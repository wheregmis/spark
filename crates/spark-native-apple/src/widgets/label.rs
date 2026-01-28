//! Native label widget (NSTextField on macOS, UILabel on iOS).

use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;
use spark_input::InputEvent;
use spark_layout::{taffy, WidgetId};
use spark_widgets::{EventContext, EventResponse, LayoutContext, PaintContext, Widget};

/// Default minimum height for labels (in logical pixels)
const DEFAULT_MIN_LABEL_HEIGHT: f32 = 17.0;
/// Approximate character width for size estimation
const CHAR_WIDTH_ESTIMATE: f32 = 7.0;

/// Native label widget.
pub struct NativeLabel {
    id: WidgetId,
    #[cfg(target_os = "macos")]
    text_field: crate::ffi::appkit::NSTextField,
    #[cfg(target_os = "ios")]
    label: crate::ffi::uikit::UILabel,
    text: String,
    /// Cached intrinsic size (width, height)
    cached_size: Option<(f32, f32)>,
}

impl NativeLabel {
    /// Create a new native label.
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let mut label = Self {
            id: WidgetId::default(),
            #[cfg(target_os = "macos")]
            text_field: crate::ffi::appkit::NSTextField::new(),
            #[cfg(target_os = "ios")]
            label: crate::ffi::uikit::UILabel::new(),
            text: text.clone(),
            cached_size: None,
        };
        label.set_text(&text);
        label.update_cached_size();
        label
    }

    /// Set the label text.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        #[cfg(target_os = "macos")]
        self.text_field.set_string_value(text);
        #[cfg(target_os = "ios")]
        self.label.set_text(text);
        self.update_cached_size();
    }

    /// Get the label text.
    pub fn text(&self) -> &str {
        &self.text
    }
    
    /// Update the cached intrinsic size from the native view.
    fn update_cached_size(&mut self) {
        #[cfg(target_os = "macos")]
        {
            // Size the text field to fit its content
            self.text_field.size_to_fit();
            // Get the intrinsic size
            let (width, height) = self.text_field.intrinsic_content_size();
            // If intrinsic size is valid, use it; otherwise estimate based on text
            if width > 0.0 && height > 0.0 {
                self.cached_size = Some((width as f32, height as f32));
            } else {
                // Estimate: roughly 7 pixels per character
                let estimated_width = (self.text.len() as f32 * CHAR_WIDTH_ESTIMATE).max(10.0);
                self.cached_size = Some((estimated_width, DEFAULT_MIN_LABEL_HEIGHT));
            }
        }
        #[cfg(target_os = "ios")]
        {
            // On iOS, estimate based on text
            let estimated_width = (self.text.len() as f32 * CHAR_WIDTH_ESTIMATE).max(10.0);
            self.cached_size = Some((estimated_width, 21.0)); // iOS standard label height
        }
        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        {
            let estimated_width = (self.text.len() as f32 * CHAR_WIDTH_ESTIMATE).max(10.0);
            self.cached_size = Some((estimated_width, DEFAULT_MIN_LABEL_HEIGHT));
        }
    }
    
    /// Get the preferred size for this label.
    pub fn preferred_size(&self) -> (f32, f32) {
        self.cached_size.unwrap_or((100.0, DEFAULT_MIN_LABEL_HEIGHT))
    }
}

impl Widget for NativeLabel {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn style(&self) -> taffy::Style {
        use taffy::prelude::*;
        let (pref_width, pref_height) = self.preferred_size();
        taffy::Style {
            // Use min_size to ensure the label has at least its intrinsic size
            min_size: Size {
                width: length(pref_width),
                height: length(pref_height),
            },
            // Allow the label to shrink and grow
            size: Size {
                width: auto(),
                height: auto(),
            },
            // Don't shrink below intrinsic size
            flex_shrink: 0.0,
            ..Default::default()
        }
    }

    fn paint(&self, _ctx: &mut PaintContext) {
        // Native widgets render themselves
    }

    fn event(&mut self, _ctx: &mut EventContext, _event: &InputEvent) -> EventResponse {
        EventResponse::default()
    }

    fn is_native(&self) -> bool {
        true
    }
    
    fn measure(&self, _ctx: &mut LayoutContext) -> Option<(f32, f32)> {
        Some(self.preferred_size())
    }

    fn register_native(
        &self,
        widget_id: WidgetId,
        register: &mut dyn FnMut(WidgetId, *mut std::ffi::c_void),
    ) {
        let view_handle = <Self as NativeWidget>::native_view(self);
        match view_handle {
            #[cfg(target_os = "macos")]
            crate::NativeViewHandle::AppKit(ptr) => {
                register(widget_id, ptr as *mut std::ffi::c_void);
            }
            #[cfg(target_os = "ios")]
            crate::NativeViewHandle::UIKit(ptr) => {
                register(widget_id, ptr as *mut std::ffi::c_void);
            }
        }
    }
}

impl NativeWidget for NativeLabel {
    fn native_view(&self) -> NativeViewHandle {
        #[cfg(target_os = "macos")]
        {
            NativeViewHandle::AppKit(self.text_field.view().as_ptr())
        }
        #[cfg(target_os = "ios")]
        {
            NativeViewHandle::UIKit(self.label.view().as_ptr())
        }
    }

    fn update_layout(&mut self, layout: &taffy::Layout, scale_factor: f32) {
        let _ = (layout, scale_factor);
    }

    fn bridge_events(&mut self) -> Vec<InputEvent> {
        Vec::new()
    }
}

impl NativeWidgetExt for NativeLabel {
    fn handle_event(&mut self, _ctx: &mut EventContext, _event: &InputEvent) -> EventResponse {
        EventResponse::default()
    }
}
