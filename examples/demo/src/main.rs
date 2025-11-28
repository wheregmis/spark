//! Demo application showcasing the Sikhar UI framework.

use sikhar::prelude::*;

fn main() {
    env_logger::init();

    App::new()
        .with_title("Sikhar Demo")
        .with_size(800, 600)
        .with_background(Color::from_hex(0x1F2937)) // Dark background
        .run(build_ui);
}

fn build_ui() -> Box<dyn Widget> {
    Box::new(
        Container::new()
            .fill()
            .padding(32.0)
            .center()
            .gap(16.0)
            .background(Color::from_hex(0x1F2937))
            .child(
                // Header
                Container::new()
                    .padding(16.0)
                    .background(Color::from_hex(0x374151))
                    .corner_radius(12.0),
            )
            .child(
                // Main content area
                Container::new()
                    .row()
                    .gap(16.0)
                    .flex_grow(1.0)
                    .child(
                        // Sidebar
                        Container::new()
                            .column()
                            .gap(8.0)
                            .padding(16.0)
                            .background(Color::from_hex(0x374151))
                            .corner_radius(8.0)
                            .min_size(200.0, 0.0)
                            .child(
                                Button::new("Dashboard")
                                    .background(Color::from_hex(0x3B82F6)),
                            )
                            .child(
                                Button::new("Settings")
                                    .background(Color::from_hex(0x6B7280)),
                            )
                            .child(
                                Button::new("Profile")
                                    .background(Color::from_hex(0x6B7280)),
                            ),
                    )
                    .child(
                        // Main panel
                        Container::new()
                            .column()
                            .gap(16.0)
                            .padding(24.0)
                            .background(Color::from_hex(0x374151))
                            .corner_radius(8.0)
                            .flex_grow(1.0)
                            .child(
                                // Form
                                Container::new()
                                    .column()
                                    .gap(12.0)
                                    .child(
                                        TextInput::new()
                                            .placeholder("Enter your name...")
                                    )
                                    .child(
                                        TextInput::new()
                                            .placeholder("Enter your email...")
                                    )
                                    .child(
                                        Container::new()
                                            .row()
                                            .gap(8.0)
                                            .child(
                                                Button::new("Submit")
                                                    .background(Color::from_hex(0x10B981)),
                                            )
                                            .child(
                                                Button::new("Cancel")
                                                    .background(Color::from_hex(0xEF4444)),
                                            ),
                                    ),
                            )
                            .child(
                                // Color palette demo
                                Container::new()
                                    .row()
                                    .gap(8.0)
                                    .child(color_box(0xEF4444)) // Red
                                    .child(color_box(0xF59E0B)) // Orange
                                    .child(color_box(0x10B981)) // Green
                                    .child(color_box(0x3B82F6)) // Blue
                                    .child(color_box(0x8B5CF6)) // Purple
                                    .child(color_box(0xEC4899)), // Pink
                            ),
                    ),
            ),
    )
}

fn color_box(hex: u32) -> Container {
    Container::new()
        .size(48.0, 48.0)
        .background(Color::from_hex(hex))
        .corner_radius(8.0)
}

