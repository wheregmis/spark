//! Native progress indicator widget (NSProgressIndicator on macOS, UIProgressView on iOS).

use spark_input::InputEvent;
use spark_layout::{taffy, WidgetId};
use spark_widgets::{EventContext, EventResponse, LayoutContext, Widget};
use crate::native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
use crate::NativeWidgetExt as _;

/// Default minimum width for progress indicators (in logical pixels)
const DEFAULT_MIN_PROGRESS_WIDTH: f32 = 100.0;
/// Default height for bar-style progress indicators (in logical pixels)
const DEFAULT_BAR_HEIGHT: f32 = 5.0;
/// Default size for spinning progress indicators (in logical pixels)
const DEFAULT_SPINNER_SIZE: f32 = 20.0;
/// Preferred width for progress indicators (in logical pixels)
const DEFAULT_PREFERRED_PROGRESS_WIDTH: f32 = 200.0;

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
    /// Preferred width (can be customized)
    preferred_width: f32,
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
            preferred_width: DEFAULT_PREFERRED_PROGRESS_WIDTH,
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
    
    /// Set the preferred width for the progress indicator.
    pub fn width(mut self, width: f32) -> Self {
        self.preferred_width = width.max(DEFAULT_MIN_PROGRESS_WIDTH);
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
    
    /// Get the preferred size for this progress indicator.
    fn preferred_size(&self) -> (f32, f32) {
        if self.indeterminate {
            // Spinning indicator is square
            (DEFAULT_SPINNER_SIZE, DEFAULT_SPINNER_SIZE)
        } else {
            #[cfg(target_os = "macos")]
            {
                let (intrinsic_width, intrinsic_height) = self.indicator.intrinsic_content_size();
                let height = if intrinsic_height > 0.0 {
                    intrinsic_height as f32
                } else {
                    DEFAULT_BAR_HEIGHT
                };
                (self.preferred_width, height)
            }
            #[cfg(target_os = "ios")]
            {
                (self.preferred_width, 4.0) // iOS standard progress bar height
            }
            #[cfg(not(any(target_os = "macos", target_os = "ios")))]
            {
                (self.preferred_width, DEFAULT_BAR_HEIGHT)
            }
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
        let (pref_width, pref_height) = self.preferred_size();
        
        if self.indeterminate {
            // Spinning indicator has fixed size
            taffy::Style {
                size: Size {
                    width: length(pref_width),
                    height: length(pref_height),
                },
                flex_shrink: 0.0,
                ..Default::default()
            }
        } else {
            // Bar indicator can flex
            taffy::Style {
                min_size: Size {
                    width: length(DEFAULT_MIN_PROGRESS_WIDTH),
                    height: length(pref_height),
                },
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
    }

    fn paint(&self, _ctx: &mut spark_widgets::PaintContext) {
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

