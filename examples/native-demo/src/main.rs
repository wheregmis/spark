//! Comprehensive native macOS widget demo.
//!
//! This example showcases various native Apple widgets integrated with Sikhar.
//! Note: This example only works on macOS and iOS platforms.
//!
//! The layout uses adaptive sizing from Flutter's adaptive_platform_ui approach:
//! - Native widgets measure their intrinsic sizes
//! - Containers use flexible layout with proper spacing
//! - Platform-specific sizing is handled automatically

use sikhar::prelude::*;
use sikhar_widgets::Container;

#[cfg(any(target_os = "macos", target_os = "ios"))]
use sikhar_native_apple::widgets::{
    NativeButton, NativeLabel, NativeProgressIndicator, NativeSlider, NativeSwitch, NativeTextField,
};

fn main() {
    App::new()
        .with_title("Native Widget Demo")
        .with_size(900, 700)
        .run(|| {
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            {
                Box::new(
                    Container::new()
                        .fill()
                        .padding(30.0)
                        .gap(20.0)
                        .align_start() // Align children to start for proper layout
                        .child(
                            Container::new()
                                .gap(16.0)
                                .fill_width() // Fill width but let height be content-based
                                .child(NativeLabel::new("Native macOS Widgets Demo"))
                                .child(
                                    Container::new()
                                        .gap(12.0)
                                        .fill_width()
                                        // Buttons section
                                        .child(
                                            Container::new()
                                                .gap(8.0)
                                                .fill_width()
                                                .child(NativeLabel::new("Buttons:"))
                                                .child(
                                                    Container::new()
                                                        .row()
                                                        .gap(8.0)
                                                        .wrap() // Allow wrapping for smaller screens
                                                        .child(NativeButton::new("Primary Button").on_click(
                                                            || {
                                                                println!("Primary button clicked!");
                                                            },
                                                        ))
                                                        .child(NativeButton::new("Secondary Button").on_click(
                                                            || {
                                                                println!("Secondary button clicked!");
                                                            },
                                                        ))
                                                ),
                                        )
                                        // Text input section
                                        .child(
                                            Container::new()
                                                .gap(8.0)
                                                .fill_width()
                                                .child(NativeLabel::new("Text Input:"))
                                                .child(
                                                    NativeTextField::new()
                                                        .placeholder("Enter text here...")
                                                        .width(300.0) // Customizable width
                                                        .on_change(|text| {
                                                            println!("Text changed: {}", text);
                                                        }),
                                                ),
                                        )
                                        // Slider section
                                        .child(
                                            Container::new()
                                                .gap(8.0)
                                                .fill_width()
                                                .child(NativeLabel::new("Slider:"))
                                                .child(
                                                    NativeSlider::new()
                                                        .min_value(0.0)
                                                        .max_value(100.0)
                                                        .value(50.0)
                                                        .width(250.0) // Customizable width
                                                        .on_change(|value| {
                                                            println!("Slider value: {:.1}", value);
                                                        }),
                                                ),
                                        )
                                        // Switches section
                                        .child(
                                            Container::new()
                                                .gap(8.0)
                                                .fill_width()
                                                .child(NativeLabel::new("Switches:"))
                                                .child(
                                                    Container::new()
                                                        .gap(8.0)
                                                        .child(
                                                            NativeSwitch::new("Enable notifications")
                                                                .checked(false)
                                                                .on_change(|checked| {
                                                                    println!(
                                                                        "Notifications: {}",
                                                                        if checked {
                                                                            "enabled"
                                                                        } else {
                                                                            "disabled"
                                                                        }
                                                                    );
                                                                }),
                                                        )
                                                        .child(
                                                            NativeSwitch::new("Dark mode")
                                                                .checked(true)
                                                                .on_change(|checked| {
                                                                    println!(
                                                                        "Dark mode: {}",
                                                                        if checked { "on" } else { "off" }
                                                                    );
                                                                }),
                                                        ),
                                                ),
                                        )
                                        // Progress indicators section
                                        .child(
                                            Container::new()
                                                .gap(8.0)
                                                .fill_width()
                                                .child(NativeLabel::new("Progress Indicators:"))
                                                .child(
                                                    Container::new()
                                                        .gap(12.0)
                                                        .fill_width()
                                                        .child(
                                                            NativeProgressIndicator::new()
                                                                .min_value(0.0)
                                                                .max_value(100.0)
                                                                .value(45.0)
                                                                .width(250.0),
                                                        )
                                                        .child(
                                                            Container::new()
                                                                .row()
                                                                .gap(8.0)
                                                                .center()
                                                                .child(NativeProgressIndicator::new().indeterminate(true))
                                                                .child(NativeLabel::new("Loading...")),
                                                        ),
                                                ),
                                        ),
                                ),
                        ),
                )
            }

            #[cfg(not(any(target_os = "macos", target_os = "ios")))]
            {
                Box::new(
                    Container::new()
                        .fill()
                        .center()
                        .padding(20.0)
                        .child(Button::new("Native widgets not available on this platform")),
                )
            }
        });
}
