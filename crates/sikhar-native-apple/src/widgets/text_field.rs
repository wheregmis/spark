//! Native text field widget (NSTextField on macOS, UITextField on iOS).

use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;
use sikhar_input::InputEvent;
use sikhar_layout::{taffy, WidgetId};
use sikhar_widgets::{EventContext, EventResponse, Widget};
use std::sync::Arc;
use std::sync::Mutex;

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
        taffy::Style {
            size: Size {
                width: length(200.0),
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
