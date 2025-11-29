//! Widget trait and response types.

use sikhar_input::InputEvent;
use sikhar_layout::WidgetId;

/// Response from handling an event.
#[derive(Clone, Copy, Debug, Default)]
pub struct EventResponse {
    /// Whether the event was handled and should not propagate.
    pub handled: bool,
    /// Request to capture all pointer events (e.g., during drag).
    pub capture_pointer: bool,
    /// Request to release pointer capture.
    pub release_pointer: bool,
    /// Request keyboard focus.
    pub request_focus: bool,
    /// Release keyboard focus.
    pub release_focus: bool,
    /// Request a repaint.
    pub repaint: bool,
    /// Request a layout recalculation.
    pub relayout: bool,
}

impl EventResponse {
    /// Create a new empty response.
    pub fn new() -> Self {
        Self::default()
    }

    /// The event was handled, stop propagation.
    pub fn handled() -> Self {
        Self {
            handled: true,
            repaint: true,
            ..Self::default()
        }
    }

    /// Request focus and handle the event.
    pub fn focus() -> Self {
        Self {
            handled: true,
            request_focus: true,
            repaint: true,
            ..Self::default()
        }
    }

    /// Capture pointer for dragging.
    pub fn capture() -> Self {
        Self {
            handled: true,
            capture_pointer: true,
            repaint: true,
            ..Self::default()
        }
    }

    /// Release pointer capture.
    pub fn release() -> Self {
        Self {
            handled: true,
            release_pointer: true,
            repaint: true,
            ..Self::default()
        }
    }

    /// Merge another response into this one.
    pub fn merge(&mut self, other: EventResponse) {
        self.handled |= other.handled;
        self.capture_pointer |= other.capture_pointer;
        self.release_pointer |= other.release_pointer;
        self.request_focus |= other.request_focus;
        self.release_focus |= other.release_focus;
        self.repaint |= other.repaint;
        self.relayout |= other.relayout;
    }

    /// Check if any action was requested.
    pub fn needs_action(&self) -> bool {
        self.repaint || self.relayout || self.request_focus || self.capture_pointer
    }
}

/// The core widget trait that all UI components implement.
pub trait Widget {
    /// Get the widget's unique ID.
    fn id(&self) -> WidgetId;

    /// Set the widget's ID (called by the framework during tree construction).
    fn set_id(&mut self, id: WidgetId);

    /// Get the layout style for this widget.
    fn style(&self) -> taffy::Style {
        taffy::Style::default()
    }

    /// Paint this widget to the draw list.
    fn paint(&self, ctx: &mut super::PaintContext);

    /// Handle an input event.
    fn event(&mut self, ctx: &mut super::EventContext, event: &InputEvent) -> EventResponse {
        let _ = (ctx, event);
        EventResponse::default()
    }

    /// Get child widgets (for containers).
    fn children(&self) -> &[Box<dyn Widget>] {
        &[]
    }

    /// Get mutable child widgets.
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut []
    }

    /// Called when the widget receives focus.
    fn on_focus(&mut self) {}

    /// Called when the widget loses focus.
    fn on_blur(&mut self) {}

    /// Whether this widget can receive keyboard focus.
    fn focusable(&self) -> bool {
        false
    }

    /// Measure the widget's preferred size (for intrinsic sizing).
    fn measure(&self, ctx: &mut super::LayoutContext) -> Option<(f32, f32)> {
        let _ = ctx;
        None
    }

    /// Check if this widget is a native widget (rendered by the platform).
    /// Default implementation returns false.
    fn is_native(&self) -> bool {
        false
    }

    /// Register this widget as a native widget with the given registration callback.
    /// The callback should be called with the widget ID and native view handle.
    /// Default implementation does nothing (for non-native widgets).
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    fn register_native(&self, _widget_id: WidgetId, _register: &mut dyn FnMut(WidgetId, *mut std::ffi::c_void)) {
        // Default: do nothing
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    fn register_native(&self, _widget_id: WidgetId, _register: &mut dyn FnMut(WidgetId, *mut std::ffi::c_void)) {
        // Default: do nothing
    }
}

