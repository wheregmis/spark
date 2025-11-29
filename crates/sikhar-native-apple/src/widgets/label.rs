//! Native label widget (NSTextField on macOS, UILabel on iOS).

use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;
use sikhar_input::InputEvent;
use sikhar_layout::{taffy, WidgetId};
use sikhar_widgets::{EventContext, EventResponse, PaintContext, Widget};

/// Native label widget.
pub struct NativeLabel {
    id: WidgetId,
    #[cfg(target_os = "macos")]
    text_field: crate::ffi::appkit::NSTextField,
    #[cfg(target_os = "ios")]
    label: crate::ffi::uikit::UILabel,
    text: String,
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
        };
        label.set_text(&text);
        label
    }

    /// Set the label text.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        #[cfg(target_os = "macos")]
        self.text_field.set_string_value(text);
        #[cfg(target_os = "ios")]
        self.label.set_text(text);
    }

    /// Get the label text.
    pub fn text(&self) -> &str {
        &self.text
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
        taffy::Style {
            size: Size {
                width: auto(),
                height: auto(),
            },
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
