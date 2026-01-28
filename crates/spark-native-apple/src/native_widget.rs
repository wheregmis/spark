//! Base trait and types for native widgets.

use spark_input::InputEvent;
use spark_layout::{taffy, WidgetId};
use spark_widgets::{EventContext, EventResponse, PaintContext, Widget};

/// Handle to a native view (platform-specific).
#[derive(Clone, Debug)]
pub enum NativeViewHandle {
    #[cfg(target_os = "macos")]
    AppKit(*mut objc2::runtime::AnyObject),
    #[cfg(target_os = "ios")]
    UIKit(*mut objc2::runtime::AnyObject),
}

unsafe impl Send for NativeViewHandle {}
unsafe impl Sync for NativeViewHandle {}

/// Trait for widgets that wrap native Apple views.
pub trait NativeWidget: Widget {
    /// Get the native view handle for this widget.
    fn native_view(&self) -> NativeViewHandle;

    /// Update the native view's layout based on taffy layout results.
    fn update_layout(&mut self, layout: &taffy::Layout, scale_factor: f32);

    /// Bridge any pending native events to Rust InputEvent types.
    /// Returns a vector of events that should be processed.
    fn bridge_events(&mut self) -> Vec<InputEvent>;

    /// Check if this widget is a native widget.
    fn is_native(&self) -> bool {
        true
    }
}

/// Extension trait to provide default implementations for native widgets.
pub trait NativeWidgetExt: NativeWidget {
    /// Handle an event (to be implemented by concrete widget types).
    fn handle_event(&mut self, ctx: &mut EventContext, event: &InputEvent) -> EventResponse;
}
