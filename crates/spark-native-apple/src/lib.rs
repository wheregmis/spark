//! Native Apple widget integration for Spark.
//!
//! This crate provides bindings to AppKit (macOS) and UIKit (iOS/iPadOS)
//! to enable native widget integration with Spark's widget system.

#![cfg(any(target_os = "macos", target_os = "ios"))]

mod events;
mod layout;
mod native_widget;
mod view_manager;

#[cfg(target_os = "macos")]
pub mod ffi {
    pub mod appkit;
}

#[cfg(target_os = "ios")]
pub mod ffi {
    pub mod uikit;
}

pub mod widgets;

pub use events::EventBridge;
pub use layout::LayoutBridge;
pub use native_widget::{NativeViewHandle, NativeWidget, NativeWidgetExt};
pub use view_manager::ViewManager;

// Re-export for convenience
pub use spark_layout;

/// Trait for widgets that can register themselves as native widgets.
/// This is used internally to register native widgets with the view manager.
pub trait NativeWidgetRegistration {
    /// Register this widget with the view manager.
    fn register_with_manager(&self, widget_id: spark_layout::WidgetId, manager: &mut ViewManager);
}

// Implement for all native widget types
impl NativeWidgetRegistration for widgets::NativeButton {
    fn register_with_manager(&self, widget_id: spark_layout::WidgetId, manager: &mut ViewManager) {
        let view_handle = <Self as NativeWidget>::native_view(self);
        manager.register_widget(widget_id, view_handle);
    }
}

impl NativeWidgetRegistration for widgets::NativeLabel {
    fn register_with_manager(&self, widget_id: spark_layout::WidgetId, manager: &mut ViewManager) {
        let view_handle = <Self as NativeWidget>::native_view(self);
        manager.register_widget(widget_id, view_handle);
    }
}

impl NativeWidgetRegistration for widgets::NativeTextField {
    fn register_with_manager(&self, widget_id: spark_layout::WidgetId, manager: &mut ViewManager) {
        let view_handle = <Self as NativeWidget>::native_view(self);
        manager.register_widget(widget_id, view_handle);
    }
}

impl NativeWidgetRegistration for widgets::NativeView {
    fn register_with_manager(&self, widget_id: spark_layout::WidgetId, manager: &mut ViewManager) {
        let view_handle = <Self as NativeWidget>::native_view(self);
        manager.register_widget(widget_id, view_handle);
    }
}

