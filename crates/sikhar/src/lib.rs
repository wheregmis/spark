//! Sikhar - A GPU-first cross-platform UI framework.
//!
//! # Example
//!
//! ```rust,no_run
//! use sikhar::prelude::*;
//!
//! fn main() {
//!     // On web, call init_web() first
//!     #[cfg(target_arch = "wasm32")]
//!     sikhar::init_web();
//!     
//!     App::new()
//!         .with_title("My App")
//!         .run(|| {
//!             Box::new(Container::new()
//!                 .child(Button::new("Click me!")))
//!         });
//! }
//! ```

mod app;

#[cfg(target_arch = "wasm32")]
mod web;

pub use app::{App, AppConfig};

#[cfg(target_arch = "wasm32")]
pub use web::init_web;

/// Re-exports of commonly used types.
pub mod prelude {
    pub use crate::{App, AppConfig};
    pub use sikhar_core::{Color, Rect};
    pub use sikhar_input::{InputEvent, Key, Modifiers, PointerButton};
    pub use sikhar_layout::taffy;
    pub use sikhar_widgets::{
        Button, ButtonStyle, Container, EventResponse, Scroll, ScrollDirection, TextInput, Widget,
    };
}

// Re-export sub-crates
pub use sikhar_core as core;
pub use sikhar_input as input;
pub use sikhar_layout as layout;
pub use sikhar_render as render;
pub use sikhar_text as text;
pub use sikhar_widgets as widgets;

