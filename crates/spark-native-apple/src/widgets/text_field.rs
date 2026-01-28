//! Native text field widget (NSTextField on macOS, UITextField on iOS).

use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;
use spark_input::InputEvent;
use spark_layout::{taffy, WidgetId};
use spark_widgets::{EventContext, EventResponse, LayoutContext, Widget};
use std::sync::Arc;
use std::sync::Mutex;

/// Default minimum width for text fields (in logical pixels)
const DEFAULT_MIN_TEXT_FIELD_WIDTH: f32 = 100.0;
/// Default minimum height for text fields (in logical pixels)
const DEFAULT_MIN_TEXT_FIELD_HEIGHT: f32 = 22.0;
/// Preferred width for text fields (in logical pixels)
const DEFAULT_PREFERRED_TEXT_FIELD_WIDTH: f32 = 200.0;

/// Native text field widget.
pub struct NativeTextField {
    id: WidgetId,
    #[cfg(target_os = "macos")]
    text_field: crate::ffi::appkit::NSTextField,
    #[cfg(target_os = "ios")]
    text_field: crate::ffi::uikit::UITextField,
    text: String,
    placeholder: String,
    on_change: Option<Box<dyn Fn(&str) + Send + Sync>>,
    pending_events: Arc<Mutex<Vec<InputEvent>>>,
    /// Preferred width (can be customized)
    preferred_width: f32,
}

impl NativeTextField {
    /// Create a new native text field.
    pub fn new() -> Self {
        Self {
            id: WidgetId::default(),
            #[cfg(target_os = "macos")]
            text_field: crate::ffi::appkit::NSTextField::new(),
            #[cfg(target_os = "ios")]
            text_field: crate::ffi::uikit::UITextField::new(),
            text: String::new(),
            placeholder: String::new(),
            on_change: None,
            pending_events: Arc::new(Mutex::new(Vec::new())),
            preferred_width: DEFAULT_PREFERRED_TEXT_FIELD_WIDTH,
        }
    }

    /// Set the placeholder text.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        #[cfg(target_os = "macos")]
        {
            self.text_field.set_placeholder_string(&self.placeholder);
        }
        self
    }

    /// Set the text value.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        #[cfg(target_os = "macos")]
        self.text_field.set_string_value(&self.text);
        #[cfg(target_os = "ios")]
        self.text_field.set_text(&self.text);
    }

    /// Get the text value.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the change callback.
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }
    
    /// Set the preferred width for the text field.
    pub fn width(mut self, width: f32) -> Self {
        self.preferred_width = width.max(DEFAULT_MIN_TEXT_FIELD_WIDTH);
        self
    }
    
    /// Get the preferred size for this text field.
    fn preferred_size(&self) -> (f32, f32) {
        #[cfg(target_os = "macos")]
        {
            let (intrinsic_width, intrinsic_height) = self.text_field.intrinsic_content_size();
            let height = if intrinsic_height > 0.0 {
                intrinsic_height as f32
            } else {
                DEFAULT_MIN_TEXT_FIELD_HEIGHT
            };
            (self.preferred_width, height)
        }
        #[cfg(target_os = "ios")]
        {
            (self.preferred_width, 31.0) // iOS standard text field height
        }
        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        {
            (self.preferred_width, DEFAULT_MIN_TEXT_FIELD_HEIGHT)
        }
    }
}

impl Widget for NativeTextField {
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
            // Use min_size to ensure minimum usable size
            min_size: Size {
                width: length(DEFAULT_MIN_TEXT_FIELD_WIDTH),
                height: length(pref_height),
            },
            // Prefer the configured width
            size: Size {
                width: length(pref_width),
                height: length(pref_height),
            },
            // Allow flexible growth but don't shrink below min_size
            flex_grow: 1.0,
            flex_shrink: 0.0,
            ..Default::default()
        }
    }

    fn paint(&self, _ctx: &mut spark_widgets::PaintContext) {
        // Native widgets render themselves
    }

    fn event(&mut self, ctx: &mut EventContext, event: &InputEvent) -> EventResponse {
        <Self as NativeWidgetExt>::handle_event(self, ctx, event)
    }

    fn focusable(&self) -> bool {
        true
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

impl NativeWidget for NativeTextField {
    fn native_view(&self) -> NativeViewHandle {
        #[cfg(target_os = "macos")]
        {
            NativeViewHandle::AppKit(self.text_field.view().as_ptr())
        }
        #[cfg(target_os = "ios")]
        {
            NativeViewHandle::UIKit(self.text_field.view().as_ptr())
        }
    }

    fn update_layout(&mut self, layout: &taffy::Layout, scale_factor: f32) {
        let _ = (layout, scale_factor);
    }

    fn bridge_events(&mut self) -> Vec<InputEvent> {
        let mut events = self.pending_events.lock().unwrap();
        let mut bridged = events.drain(..).collect::<Vec<_>>();

        // Check for text changes
        #[cfg(target_os = "macos")]
        let new_text = self.text_field.string_value();
        #[cfg(target_os = "ios")]
        let new_text = self.text_field.text();

        if new_text != self.text {
            self.text = new_text.clone();
            if let Some(ref callback) = self.on_change {
                callback(&new_text);
            }
            bridged.push(InputEvent::TextInput { text: new_text });
        }

        bridged
    }
}

impl NativeWidgetExt for NativeTextField {
    fn handle_event(&mut self, _ctx: &mut EventContext, event: &InputEvent) -> EventResponse {
        match event {
            InputEvent::FocusGained => {
                // Focus the native text field
                EventResponse::focus()
            }
            InputEvent::TextInput { text } => {
                self.set_text(text);
                EventResponse::handled()
            }
            _ => EventResponse::default(),
        }
    }
}
