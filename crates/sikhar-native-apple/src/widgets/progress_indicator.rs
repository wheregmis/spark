//! Native progress indicator widget (NSProgressIndicator on macOS, UIProgressView on iOS).

use sikhar_input::InputEvent;
use sikhar_layout::{taffy, WidgetId};
use sikhar_widgets::{EventContext, EventResponse, Widget};
use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;

/// Native progress indicator widget.
pub struct NativeProgressIndicator {
    id: WidgetId,
    #[cfg(target_os = "macos")]
    indicator: crate::ffi::appkit::NSProgressIndicator,
    #[cfg(target_os = "ios")]
    indicator: crate::ffi::uikit::UIProgressView,
    value: f64,
    min_value: f64,
    max_value: f64,
    indeterminate: bool,
}

impl NativeProgressIndicator {
    /// Create a new native progress indicator.
    pub fn new() -> Self {
        let mut indicator = Self {
            id: WidgetId::default(),
            #[cfg(target_os = "macos")]
            indicator: crate::ffi::appkit::NSProgressIndicator::new(),
            #[cfg(target_os = "ios")]
            indicator: crate::ffi::uikit::UIProgressView::new(),
            value: 0.0,
            min_value: 0.0,
            max_value: 100.0,
            indeterminate: false,
        };
        indicator.update_native_values();
        indicator
    }

    /// Set the progress value (0.0 to 1.0 for determinate, or use min/max).
    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(self.min_value, self.max_value);
        self.update_native_values();
        self
    }

    /// Set the minimum value.
    pub fn min_value(mut self, value: f64) -> Self {
        self.min_value = value;
        self.update_native_values();
        self
    }

    /// Set the maximum value.
    pub fn max_value(mut self, value: f64) -> Self {
        self.max_value = value;
        self.update_native_values();
        self
    }

    /// Set whether the indicator is indeterminate (spinning).
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        #[cfg(target_os = "macos")]
        {
            if indeterminate {
                self.indicator.set_style(crate::ffi::appkit::NSProgressIndicatorStyle::Spinning);
                self.indicator.start_animation();
            } else {
                self.indicator.set_style(crate::ffi::appkit::NSProgressIndicatorStyle::Bar);
                self.indicator.stop_animation();
            }
        }
        self
    }

    fn update_native_values(&mut self) {
        #[cfg(target_os = "macos")]
        {
            if !self.indeterminate {
                self.indicator.set_min_value(self.min_value);
                self.indicator.set_max_value(self.max_value);
                self.indicator.set_double_value(self.value);
            }
        }
        #[cfg(target_os = "ios")]
        {
            let progress = ((self.value - self.min_value) / (self.max_value - self.min_value)) as f32;
            self.indicator.set_progress(progress);
        }
    }
}

impl Widget for NativeProgressIndicator {
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
                height: if self.indeterminate { length(20.0) } else { length(10.0) },
            },
            ..Default::default()
        }
    }

    fn paint(&self, _ctx: &mut sikhar_widgets::PaintContext) {
        // Native widgets render themselves
    }

    fn event(&mut self, _ctx: &mut EventContext, _event: &InputEvent) -> EventResponse {
        EventResponse::default()
    }

    fn focusable(&self) -> bool {
        false
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

impl NativeWidget for NativeProgressIndicator {
    fn native_view(&self) -> NativeViewHandle {
        #[cfg(target_os = "macos")]
        {
            NativeViewHandle::AppKit(self.indicator.view().as_ptr())
        }
        #[cfg(target_os = "ios")]
        {
            NativeViewHandle::UIKit(self.indicator.view().as_ptr())
        }
    }

    fn update_layout(&mut self, layout: &taffy::Layout, scale_factor: f32) {
        let _ = (layout, scale_factor);
    }

    fn bridge_events(&mut self) -> Vec<InputEvent> {
        Vec::new()
    }
}

impl NativeWidgetExt for NativeProgressIndicator {
    fn handle_event(&mut self, _ctx: &mut EventContext, _event: &InputEvent) -> EventResponse {
        EventResponse::default()
    }
}

