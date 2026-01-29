//! Event bridge - converts native events to Rust InputEvent types.

use spark_input::{InputEvent, PointerButton};

/// Bridge for converting native events to Spark InputEvent types.
pub struct EventBridge;

impl EventBridge {
    /// Convert a native mouse/touch event to InputEvent.
    pub fn native_to_pointer_move(
        x: f64,
        y: f64,
        parent_height: f32,
        scale_factor: f32,
    ) -> InputEvent {
        let pos =
            crate::layout::LayoutBridge::native_to_taffy_point(x, y, parent_height, scale_factor);
        InputEvent::PointerMove { pos }
    }

    /// Convert a native mouse/touch down event to InputEvent.
    pub fn native_to_pointer_down(
        x: f64,
        y: f64,
        button: NativeButton,
        parent_height: f32,
        scale_factor: f32,
    ) -> InputEvent {
        let pos =
            crate::layout::LayoutBridge::native_to_taffy_point(x, y, parent_height, scale_factor);
        let button = match button {
            NativeButton::Left => PointerButton::Primary,
            NativeButton::Right => PointerButton::Secondary,
            NativeButton::Middle => PointerButton::Auxiliary,
        };
        InputEvent::PointerDown { pos, button }
    }

    /// Convert a native mouse/touch up event to InputEvent.
    pub fn native_to_pointer_up(
        x: f64,
        y: f64,
        button: NativeButton,
        parent_height: f32,
        scale_factor: f32,
    ) -> InputEvent {
        let pos =
            crate::layout::LayoutBridge::native_to_taffy_point(x, y, parent_height, scale_factor);
        let button = match button {
            NativeButton::Left => PointerButton::Primary,
            NativeButton::Right => PointerButton::Secondary,
            NativeButton::Middle => PointerButton::Auxiliary,
        };
        InputEvent::PointerUp { pos, button }
    }

    /// Convert native text input to InputEvent.
    pub fn native_to_text_input(text: String) -> InputEvent {
        InputEvent::TextInput { text }
    }

    /// Convert native focus event to InputEvent.
    pub fn native_to_focus_gained() -> InputEvent {
        InputEvent::FocusGained
    }

    /// Convert native blur event to InputEvent.
    pub fn native_to_focus_lost() -> InputEvent {
        InputEvent::FocusLost
    }
}

/// Native button type (simplified).
#[derive(Clone, Copy, Debug)]
pub enum NativeButton {
    Left,
    Right,
    Middle,
}
