//! Native switch widget (NSSwitch on macOS, UISwitch on iOS).

use sikhar_input::InputEvent;
use sikhar_layout::{taffy, WidgetId};
use sikhar_widgets::{EventContext, EventResponse, Widget};
use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;

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
        };
        switch.set_title(&title);
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
    }

    /// Set the change callback.
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
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
        taffy::Style {
            size: Size {
                width: auto(),
                height: length(20.0),
            },
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

