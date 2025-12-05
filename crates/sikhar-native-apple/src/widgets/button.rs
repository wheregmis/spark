//! Native button widget (NSButton on macOS, UIButton on iOS).

use sikhar_input::InputEvent;
use sikhar_layout::{taffy, WidgetId};
use sikhar_widgets::{EventContext, EventResponse, LayoutContext, Widget};
use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;
use std::sync::Arc;
use std::sync::Mutex;

/// Default minimum width for buttons (in logical pixels)
const DEFAULT_MIN_BUTTON_WIDTH: f32 = 80.0;
/// Default minimum height for buttons (in logical pixels)
const DEFAULT_MIN_BUTTON_HEIGHT: f32 = 22.0;
/// Padding added to intrinsic button size
const BUTTON_PADDING: f32 = 16.0;

/// Native button widget.
pub struct NativeButton {
    id: WidgetId,
    #[cfg(target_os = "macos")]
    button: crate::ffi::appkit::NSButton,
    #[cfg(target_os = "ios")]
    button: crate::ffi::uikit::UIButton,
    title: String,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
    pending_events: Arc<Mutex<Vec<InputEvent>>>,
    /// Cached intrinsic size (width, height)
    cached_size: Option<(f32, f32)>,
}

impl NativeButton {
    /// Create a new native button.
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        let mut button = Self {
            id: WidgetId::default(),
            #[cfg(target_os = "macos")]
            button: crate::ffi::appkit::NSButton::new(),
            #[cfg(target_os = "ios")]
            button: crate::ffi::uikit::UIButton::new(),
            title: title.clone(),
            on_click: None,
            pending_events: Arc::new(Mutex::new(Vec::new())),
            cached_size: None,
        };
        // Set the title on the native button
        button.set_title(&title);
        // Cache the intrinsic size
        button.update_cached_size();
        button
    }

    /// Set the button title.
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
        #[cfg(target_os = "macos")]
        self.button.set_title(&self.title);
        #[cfg(target_os = "ios")]
        self.button.set_title(&self.title, crate::ffi::uikit::UIControlState::Normal);
        // Invalidate cached size when title changes
        self.update_cached_size();
    }

    /// Set the click callback.
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }
    
    /// Update the cached intrinsic size from the native view.
    fn update_cached_size(&mut self) {
        #[cfg(target_os = "macos")]
        {
            // Size the button to fit its content first
            self.button.size_to_fit();
            // Get the intrinsic size
            let (width, height) = self.button.intrinsic_content_size();
            // If intrinsic size is valid, use it; otherwise estimate based on title
            if width > 0.0 && height > 0.0 {
                self.cached_size = Some((width as f32, height as f32));
            } else {
                // Estimate: roughly 8 pixels per character + padding
                let estimated_width = (self.title.len() as f32 * 8.0 + BUTTON_PADDING * 2.0).max(DEFAULT_MIN_BUTTON_WIDTH);
                self.cached_size = Some((estimated_width, DEFAULT_MIN_BUTTON_HEIGHT));
            }
        }
        #[cfg(target_os = "ios")]
        {
            // On iOS, use a similar estimation approach
            let estimated_width = (self.title.len() as f32 * 8.0 + BUTTON_PADDING * 2.0).max(DEFAULT_MIN_BUTTON_WIDTH);
            self.cached_size = Some((estimated_width, 44.0)); // iOS standard button height
        }
        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        {
            let estimated_width = (self.title.len() as f32 * 8.0 + BUTTON_PADDING * 2.0).max(DEFAULT_MIN_BUTTON_WIDTH);
            self.cached_size = Some((estimated_width, DEFAULT_MIN_BUTTON_HEIGHT));
        }
    }
    
    /// Get the preferred size for this button.
    pub fn preferred_size(&self) -> (f32, f32) {
        self.cached_size.unwrap_or((DEFAULT_MIN_BUTTON_WIDTH, DEFAULT_MIN_BUTTON_HEIGHT))
    }
}

impl Widget for NativeButton {
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
            // Use min_size to ensure the button has at least its intrinsic size
            min_size: Size {
                width: length(pref_width),
                height: length(pref_height),
            },
            // Allow the button to grow but prefer its intrinsic size
            size: Size {
                width: auto(),
                height: length(pref_height),
            },
            // Allow flexible layout
            flex_shrink: 0.0,
            ..Default::default()
        }
    }

    fn paint(&self, _ctx: &mut sikhar_widgets::PaintContext) {
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
    
    fn register_native(&self, widget_id: WidgetId, register: &mut dyn FnMut(WidgetId, *mut std::ffi::c_void)) {
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

impl NativeWidget for NativeButton {
    fn native_view(&self) -> NativeViewHandle {
        #[cfg(target_os = "macos")]
        {
            NativeViewHandle::AppKit(self.button.view().as_ptr())
        }
        #[cfg(target_os = "ios")]
        {
            NativeViewHandle::UIKit(self.button.view().as_ptr())
        }
    }

    fn update_layout(&mut self, layout: &taffy::Layout, scale_factor: f32) {
        // Layout is updated by ViewManager
        let _ = (layout, scale_factor);
    }

    fn bridge_events(&mut self) -> Vec<InputEvent> {
        let mut events = self.pending_events.lock().unwrap();
        events.drain(..).collect()
    }
}

impl NativeWidgetExt for NativeButton {
    fn handle_event(&mut self, _ctx: &mut EventContext, event: &InputEvent) -> EventResponse {
        if let InputEvent::PointerDown { button: sikhar_input::PointerButton::Primary, .. } = event {
            if let Some(ref callback) = self.on_click {
                callback();
            }
            EventResponse::handled()
        } else {
            EventResponse::default()
        }
    }
}

