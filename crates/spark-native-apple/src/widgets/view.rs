//! Native view container widget (NSView on macOS, UIView on iOS).

use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;
use spark_input::InputEvent;
use spark_layout::{taffy, WidgetId};
use spark_widgets::{EventContext, EventResponse, PaintContext, Widget};

/// Native view container widget.
pub struct NativeView {
    id: WidgetId,
    #[cfg(target_os = "macos")]
    view: crate::ffi::appkit::NSView,
    #[cfg(target_os = "ios")]
    view: crate::ffi::uikit::UIView,
    children: Vec<Box<dyn Widget>>,
}

impl NativeView {
    /// Create a new native view container.
    pub fn new() -> Self {
        Self {
            id: WidgetId::default(),
            #[cfg(target_os = "macos")]
            view: crate::ffi::appkit::NSView::new(),
            #[cfg(target_os = "ios")]
            view: crate::ffi::uikit::UIView::new(),
            children: Vec::new(),
        }
    }

    /// Add a child widget.
    pub fn child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
}

impl Widget for NativeView {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn style(&self) -> taffy::Style {
        taffy::Style::default()
    }

    fn paint(&self, _ctx: &mut PaintContext) {
        // Native widgets render themselves
    }

    fn event(&mut self, _ctx: &mut EventContext, _event: &InputEvent) -> EventResponse {
        EventResponse::default()
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut self.children
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

impl NativeWidget for NativeView {
    fn native_view(&self) -> NativeViewHandle {
        #[cfg(target_os = "macos")]
        {
            NativeViewHandle::AppKit(self.view.as_ptr())
        }
        #[cfg(target_os = "ios")]
        {
            NativeViewHandle::UIKit(self.view.as_ptr())
        }
    }

    fn update_layout(&mut self, layout: &taffy::Layout, scale_factor: f32) {
        let _ = (layout, scale_factor);
    }

    fn bridge_events(&mut self) -> Vec<InputEvent> {
        Vec::new()
    }
}

impl NativeWidgetExt for NativeView {
    fn handle_event(&mut self, _ctx: &mut EventContext, _event: &InputEvent) -> EventResponse {
        EventResponse::default()
    }
}
