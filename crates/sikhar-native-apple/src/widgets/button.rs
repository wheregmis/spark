//! Native button widget (NSButton on macOS, UIButton on iOS).

use sikhar_input::InputEvent;
use sikhar_layout::{taffy, WidgetId};
use sikhar_widgets::{EventContext, EventResponse, Widget};
use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;
use std::sync::Arc;
use std::sync::Mutex;

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
        };
        // Set the title on the native button
        button.set_title(&title);
        button
    }

    /// Set the button title.
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
        #[cfg(target_os = "macos")]
        self.button.set_title(&self.title);
        #[cfg(target_os = "ios")]
        self.button.set_title(&self.title, crate::ffi::uikit::UIControlState::Normal);
    }

    /// Set the click callback.
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
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
        taffy::Style {
            size: Size {
                width: auto(),
                height: length(30.0),
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

