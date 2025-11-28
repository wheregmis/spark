//! Sikhar Input - Event handling, focus management, and hit testing.
//!
//! Uses [ui-events](https://github.com/endoli/ui-events) from the Linebender
//! ecosystem for W3C-compliant UI event types.

mod events;
mod focus;
mod hit_test;

// Re-export ui-events types
pub use ui_events;
pub use ui_events_winit;

// Our wrapper types
pub use events::{
    shortcuts, CompositionEvent, InputEvent, Key, KeyState, KeyboardEvent, Modifiers,
    NamedKey, PointerButton, PointerId, PointerState, PointerType, ScrollDelta,
};
pub use focus::FocusManager;
pub use hit_test::{hit_test, hit_test_all, hit_test_filtered, HitTestResult};

