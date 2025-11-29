//! Comprehensive native macOS widget demo.
//!
//! This example showcases various native Apple widgets integrated with Sikhar.
//! Note: This example only works on macOS and iOS platforms.

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
                    Container::new().fill().padding(30.0).gap(20.0).child(
                        Container::new()
                            .gap(16.0)
                            .child(NativeLabel::new("Native macOS Widgets Demo"))
                            .child(
                                Container::new()
                                    .gap(12.0)
                                    .child(
                                        Container::new()
                                            .gap(8.0)
                                            .child(NativeLabel::new("Buttons:"))
                                            .child(NativeButton::new("Primary Button").on_click(
                                                || {
                                                    println!("Primary button clicked!");
                                                },
                                            ))
                                            .child(NativeButton::new("Secondary Button").on_click(
                                                || {
                                                    println!("Secondary button clicked!");
                                                },
                                            )),
                                    )
                                    .child(
                                        Container::new()
                                            .gap(8.0)
                                            .child(NativeLabel::new("Text Input:"))
                                            .child(
                                                NativeTextField::new()
                                                    .placeholder("Enter text here...")
                                                    .on_change(|text| {
                                                        println!("Text changed: {}", text);
                                                    }),
                                            ),
                                    )
                                    .child(
                                        Container::new()
                                            .gap(8.0)
                                            .child(NativeLabel::new("Slider:"))
                                            .child(
                                                NativeSlider::new()
                                                    .min_value(0.0)
                                                    .max_value(100.0)
                                                    .value(50.0)
                                                    .on_change(|value| {
                                                        println!("Slider value: {:.1}", value);
                                                    }),
                                            ),
                                    )
                                    .child(
                                        Container::new()
                                            .gap(8.0)
                                            .child(NativeLabel::new("Switches:"))
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
                                    )
                                    .child(
                                        Container::new()
                                            .gap(8.0)
                                            .child(NativeLabel::new("Progress Indicators:"))
                                            .child(
                                                NativeProgressIndicator::new()
                                                    .min_value(0.0)
                                                    .max_value(100.0)
                                                    .value(45.0),
                                            )
                                            .child(
                                                NativeProgressIndicator::new().indeterminate(true),
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
