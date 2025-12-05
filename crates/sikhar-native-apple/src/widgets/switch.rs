//! Native switch widget (NSSwitch on macOS, UISwitch on iOS).

use sikhar_input::InputEvent;
use sikhar_layout::{taffy, WidgetId};
use sikhar_widgets::{EventContext, EventResponse, LayoutContext, Widget};
use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;

/// Default minimum width for switches (in logical pixels)
const DEFAULT_MIN_SWITCH_WIDTH: f32 = 40.0;
/// Default minimum height for switches (in logical pixels)
const DEFAULT_MIN_SWITCH_HEIGHT: f32 = 21.0;
/// Approximate character width for label size estimation
const CHAR_WIDTH_ESTIMATE: f32 = 7.0;

/// Native switch widget.
pub struct NativeSwitch {
    id: WidgetId,
    #[cfg(target_os = "macos")]
    switch: crate::ffi::appkit::NSSwitch,
    #[cfg(target_os = "ios")]
    switch: crate::ffi::uikit::UISwitch,
    checked: bool,
    title: String,
    on_change: Option<Box<dyn Fn(bool) + Send + Sync>>,
    /// Cached intrinsic size
    cached_size: Option<(f32, f32)>,
}

impl NativeSwitch {
    /// Create a new native switch.
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        let mut switch = Self {
            id: WidgetId::default(),
            #[cfg(target_os = "macos")]
            switch: crate::ffi::appkit::NSSwitch::new(),
            #[cfg(target_os = "ios")]
            switch: crate::ffi::uikit::UISwitch::new(),
            checked: false,
            title: title.clone(),
            on_change: None,
            cached_size: None,
        };
        switch.set_title(&title);
        switch.update_cached_size();
        switch
    }

    /// Set the switch state.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        #[cfg(target_os = "macos")]
        {
            self.switch.set_state(if checked { 1 } else { 0 });
        }
        #[cfg(target_os = "ios")]
        {
            self.switch.set_on(checked);
        }
        self
    }

    /// Set the switch title.
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        #[cfg(target_os = "macos")]
        {
            self.switch.set_title(&self.title);
        }
        self.update_cached_size();
    }

    /// Set the change callback.
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }
    
    /// Update the cached intrinsic size from the native view.
    fn update_cached_size(&mut self) {
        #[cfg(target_os = "macos")]
        {
            // Size the switch to fit its content
            self.switch.size_to_fit();
            // Get the intrinsic size
            let (width, height) = self.switch.intrinsic_content_size();
            // If intrinsic size is valid, use it; otherwise estimate
            if width > 0.0 && height > 0.0 {
                self.cached_size = Some((width as f32, height as f32));
            } else {
                // Estimate: switch + label width
                let label_width = self.title.len() as f32 * CHAR_WIDTH_ESTIMATE;
                let estimated_width = (DEFAULT_MIN_SWITCH_WIDTH + label_width + 8.0).max(DEFAULT_MIN_SWITCH_WIDTH);
                self.cached_size = Some((estimated_width, DEFAULT_MIN_SWITCH_HEIGHT));
            }
        }
        #[cfg(target_os = "ios")]
        {
            // iOS switches don't have labels by default, use standard size
            // UISwitch has a fixed size of approximately 51x31
            self.cached_size = Some((51.0, 31.0));
        }
        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        {
            let label_width = self.title.len() as f32 * CHAR_WIDTH_ESTIMATE;
            let estimated_width = (DEFAULT_MIN_SWITCH_WIDTH + label_width + 8.0).max(DEFAULT_MIN_SWITCH_WIDTH);
            self.cached_size = Some((estimated_width, DEFAULT_MIN_SWITCH_HEIGHT));
        }
    }
    
    /// Get the preferred size for this switch.
    pub fn preferred_size(&self) -> (f32, f32) {
        self.cached_size.unwrap_or((DEFAULT_MIN_SWITCH_WIDTH, DEFAULT_MIN_SWITCH_HEIGHT))
    }
}

impl Widget for NativeSwitch {
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
            // Use min_size to ensure the switch has at least its intrinsic size
            min_size: Size {
                width: length(pref_width),
                height: length(pref_height),
            },
            // Size to fit content
            size: Size {
                width: auto(),
                height: length(pref_height),
            },
            // Don't shrink below intrinsic size
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

impl NativeWidget for NativeSwitch {
    fn native_view(&self) -> NativeViewHandle {
        #[cfg(target_os = "macos")]
        {
            NativeViewHandle::AppKit(self.switch.view().as_ptr())
        }
        #[cfg(target_os = "ios")]
        {
            NativeViewHandle::UIKit(self.switch.view().as_ptr())
        }
    }

    fn update_layout(&mut self, layout: &taffy::Layout, scale_factor: f32) {
        let _ = (layout, scale_factor);
    }

    fn bridge_events(&mut self) -> Vec<InputEvent> {
        // Check if state changed
        #[cfg(target_os = "macos")]
        {
            let new_state = self.switch.state();
            let new_checked = new_state == 1;
            if new_checked != self.checked {
                self.checked = new_checked;
                if let Some(ref callback) = self.on_change {
                    callback(self.checked);
                }
            }
        }
        #[cfg(target_os = "ios")]
        {
            let new_checked = self.switch.is_on();
            if new_checked != self.checked {
                self.checked = new_checked;
                if let Some(ref callback) = self.on_change {
                    callback(self.checked);
                }
            }
        }
        Vec::new()
    }
}

impl NativeWidgetExt for NativeSwitch {
    fn handle_event(&mut self, _ctx: &mut EventContext, _event: &InputEvent) -> EventResponse {
        // Events are handled through bridge_events
        EventResponse::default()
    }
}

