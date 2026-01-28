//! FFI bindings for AppKit and UIKit.

#[cfg(target_os = "macos")]
pub use appkit::*;

#[cfg(target_os = "ios")]
pub use uikit::*;

#[cfg(target_os = "macos")]
pub mod appkit;

#[cfg(target_os = "ios")]
pub mod uikit;
