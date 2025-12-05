//! Native slider widget (NSSlider on macOS, UISlider on iOS).

use sikhar_input::InputEvent;
use sikhar_layout::{taffy, WidgetId};
use sikhar_widgets::{EventContext, EventResponse, LayoutContext, Widget};
use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;

/// Default minimum width for sliders (in logical pixels)
const DEFAULT_MIN_SLIDER_WIDTH: f32 = 100.0;
/// Default minimum height for sliders (in logical pixels)
const DEFAULT_MIN_SLIDER_HEIGHT: f32 = 21.0;
/// Preferred width for sliders (in logical pixels)
const DEFAULT_PREFERRED_SLIDER_WIDTH: f32 = 200.0;

/// Native slider widget.
pub struct NativeSlider {
    id: WidgetId,
    #[cfg(target_os = "macos")]
    slider: crate::ffi::appkit::NSSlider,
    #[cfg(target_os = "ios")]
    slider: crate::ffi::uikit::UISlider,
    min_value: f64,
    max_value: f64,
    value: f64,
    on_change: Option<Box<dyn Fn(f64) + Send + Sync>>,
    /// Preferred width (can be customized)
    preferred_width: f32,
}

impl NativeSlider {
    /// Create a new native slider.
    pub fn new() -> Self {
        let mut slider = Self {
            id: WidgetId::default(),
            #[cfg(target_os = "macos")]
            slider: crate::ffi::appkit::NSSlider::new(),
            #[cfg(target_os = "ios")]
            slider: crate::ffi::uikit::UISlider::new(),
            min_value: 0.0,
            max_value: 100.0,
            value: 50.0,
            on_change: None,
            preferred_width: DEFAULT_PREFERRED_SLIDER_WIDTH,
        };
        slider.update_native_values();
        slider
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

    /// Set the current value.
    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(self.min_value, self.max_value);
        self.update_native_values();
        self
    }

    /// Set the change callback.
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(f64) + Send + Sync + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }
    
    /// Set the preferred width for the slider.
    pub fn width(mut self, width: f32) -> Self {
        self.preferred_width = width.max(DEFAULT_MIN_SLIDER_WIDTH);
        self
    }

    fn update_native_values(&mut self) {
        #[cfg(target_os = "macos")]
        {
            self.slider.set_min_value(self.min_value);
            self.slider.set_max_value(self.max_value);
            self.slider.set_double_value(self.value);
        }
        #[cfg(target_os = "ios")]
        {
            self.slider.set_minimum_value(self.min_value as f32);
            self.slider.set_maximum_value(self.max_value as f32);
            self.slider.set_value(self.value as f32);
        }
    }
    
    /// Get the preferred size for this slider.
    fn preferred_size(&self) -> (f32, f32) {
        #[cfg(target_os = "macos")]
        {
            let (intrinsic_width, intrinsic_height) = self.slider.intrinsic_content_size();
            let height = if intrinsic_height > 0.0 {
                intrinsic_height as f32
            } else {
                DEFAULT_MIN_SLIDER_HEIGHT
            };
            (self.preferred_width, height)
        }
        #[cfg(target_os = "ios")]
        {
            (self.preferred_width, 31.0) // iOS standard slider height
        }
        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        {
            (self.preferred_width, DEFAULT_MIN_SLIDER_HEIGHT)
        }
    }
}

impl Widget for NativeSlider {
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
                width: length(DEFAULT_MIN_SLIDER_WIDTH),
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

impl NativeWidget for NativeSlider {
    fn native_view(&self) -> NativeViewHandle {
        #[cfg(target_os = "macos")]
        {
            NativeViewHandle::AppKit(self.slider.view().as_ptr())
        }
        #[cfg(target_os = "ios")]
        {
            NativeViewHandle::UIKit(self.slider.view().as_ptr())
        }
    }

    fn update_layout(&mut self, layout: &taffy::Layout, scale_factor: f32) {
        let _ = (layout, scale_factor);
    }

    fn bridge_events(&mut self) -> Vec<InputEvent> {
        // Check if value changed
        #[cfg(target_os = "macos")]
        {
            let new_value = self.slider.double_value();
            if (new_value - self.value).abs() > 0.01 {
                self.value = new_value;
                if let Some(ref callback) = self.on_change {
                    callback(self.value);
                }
            }
        }
        #[cfg(target_os = "ios")]
        {
            let new_value = self.slider.value() as f64;
            if (new_value - self.value).abs() > 0.01 {
                self.value = new_value;
                if let Some(ref callback) = self.on_change {
                    callback(self.value);
                }
            }
        }
        Vec::new()
    }
}

impl NativeWidgetExt for NativeSlider {
    fn handle_event(&mut self, _ctx: &mut EventContext, _event: &InputEvent) -> EventResponse {
        // Events are handled through bridge_events
        EventResponse::default()
    }
}

