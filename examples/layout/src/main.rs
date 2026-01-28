//! Layout Gallery - Visual testing for layout features

use spark::prelude::*;

fn main() {
    env_logger::init();

    App::new()
        .with_title("Layout Gallery - Spark")
        .with_size(1000, 800)
        .with_background(Color::from_hex(0x1F2937))
        .run(build_ui);
}

/// Creates a labeled test section
fn test_section(title: &str, content: Container) -> Container {
    Container::new()
        .column()
        .gap(8.0)
        .padding(16.0)
        .background(Color::from_hex(0x374151))
        .corner_radius(8.0)
        .min_size(300.0, 0.0)
        .child(
            Text::new(title)
                .size(14.0)
                .bold()
                .color(Color::from_hex(0xF3F4F6)),
        )
        .child(content)
}

/// Creates a colored test box
fn color_box(hex: u32, width: f32, height: f32) -> Container {
    Container::new()
        .size(width, height)
        .background(Color::from_hex(hex))
        .corner_radius(4.0)
}

fn build_ui() -> Box<dyn Widget> {
    Box::new(
        Scroll::new().vertical().debug_overlay(true).content(
            Container::new()
                .column()
                .gap(24.0)
                .padding(32.0)
                .flex_shrink(0.0)
                .background(Color::from_hex(0x1F2937))
                // Header
                .child(
                    Text::new("Layout Gallery")
                        .size(32.0)
                        .bold()
                        .color(Color::WHITE),
                )
                // Row 1: Flex Direction
                .child(
                    Container::new()
                        .row()
                        .gap(16.0)
                        .child(test_section(
                            "Column Layout",
                            Container::new()
                                .column()
                                .gap(8.0)
                                .child(color_box(0xEF4444, 60.0, 40.0)) // Red
                                .child(color_box(0x22C55E, 60.0, 40.0)) // Green
                                .child(color_box(0x3B82F6, 60.0, 40.0)), // Blue
                        ))
                        .child(test_section(
                            "Row Layout",
                            Container::new()
                                .row()
                                .gap(8.0)
                                .child(color_box(0xEF4444, 60.0, 60.0))
                                .child(color_box(0x22C55E, 60.0, 60.0))
                                .child(color_box(0x3B82F6, 60.0, 60.0)),
                        ))
                        .child(test_section(
                            "Wrap Layout",
                            Container::new()
                                .row()
                                .wrap()
                                .gap(8.0)
                                .width(200.0)
                                .child(color_box(0xEF4444, 60.0, 40.0))
                                .child(color_box(0xF59E0B, 60.0, 40.0))
                                .child(color_box(0x22C55E, 60.0, 40.0))
                                .child(color_box(0x3B82F6, 60.0, 40.0))
                                .child(color_box(0x8B5CF6, 60.0, 40.0))
                                .child(color_box(0xEC4899, 60.0, 40.0)),
                        )),
                )
                // Row 2: Alignment
                .child(
                    Container::new()
                        .row()
                        .gap(16.0)
                        .child(test_section(
                            "Align Start",
                            Container::new()
                                .column()
                                .align_start()
                                .min_size(120.0, 150.0)
                                .background(Color::from_hex(0x1F2937))
                                .child(color_box(0xEF4444, 60.0, 40.0))
                                .child(color_box(0x22C55E, 60.0, 40.0)),
                        ))
                        .child(test_section(
                            "Align Center",
                            Container::new()
                                .column()
                                .center()
                                .min_size(120.0, 150.0)
                                .background(Color::from_hex(0x1F2937))
                                .child(color_box(0xEF4444, 60.0, 40.0))
                                .child(color_box(0x22C55E, 60.0, 40.0)),
                        ))
                        .child(test_section(
                            "Align End",
                            Container::new()
                                .column()
                                .align_items(taffy::AlignItems::FlexEnd)
                                .justify_content(taffy::JustifyContent::FlexEnd)
                                .min_size(120.0, 150.0)
                                .background(Color::from_hex(0x1F2937))
                                .child(color_box(0xEF4444, 60.0, 40.0))
                                .child(color_box(0x22C55E, 60.0, 40.0)),
                        )),
                )
                // Row 3: Spacing
                .child(
                    Container::new()
                        .row()
                        .gap(16.0)
                        .child(test_section(
                            "No Gap",
                            Container::new()
                                .row()
                                .gap(0.0)
                                .child(color_box(0xEF4444, 40.0, 40.0))
                                .child(color_box(0x22C55E, 40.0, 40.0))
                                .child(color_box(0x3B82F6, 40.0, 40.0)),
                        ))
                        .child(test_section(
                            "Gap 8px",
                            Container::new()
                                .row()
                                .gap(8.0)
                                .child(color_box(0xEF4444, 40.0, 40.0))
                                .child(color_box(0x22C55E, 40.0, 40.0))
                                .child(color_box(0x3B82F6, 40.0, 40.0)),
                        ))
                        .child(test_section(
                            "Gap 24px",
                            Container::new()
                                .row()
                                .gap(24.0)
                                .child(color_box(0xEF4444, 40.0, 40.0))
                                .child(color_box(0x22C55E, 40.0, 40.0))
                                .child(color_box(0x3B82F6, 40.0, 40.0)),
                        )),
                )
                // Row 4: Sizing
                .child(
                    Container::new()
                        .row()
                        .gap(16.0)
                        .child(test_section(
                            "Fixed Size",
                            Container::new()
                                .row()
                                .gap(8.0)
                                .child(color_box(0xEF4444, 100.0, 100.0))
                                .child(color_box(0x22C55E, 100.0, 100.0)),
                        ))
                        .child(test_section(
                            "Flex Grow",
                            Container::new()
                                .row()
                                .gap(8.0)
                                .min_size(250.0, 0.0)
                                .child(
                                    Container::new()
                                        .flex_grow(1.0)
                                        .min_size(0.0, 60.0)
                                        .background(Color::from_hex(0xEF4444))
                                        .corner_radius(4.0),
                                )
                                .child(
                                    Container::new()
                                        .flex_grow(1.0)
                                        .min_size(0.0, 60.0)
                                        .background(Color::from_hex(0x22C55E))
                                        .corner_radius(4.0),
                                ),
                        ))
                        .child(test_section(
                            "Min Size",
                            Container::new()
                                .column()
                                .gap(8.0)
                                .child(
                                    Container::new()
                                        .min_size(80.0, 40.0)
                                        .background(Color::from_hex(0xEF4444))
                                        .corner_radius(4.0),
                                )
                                .child(
                                    Container::new()
                                        .min_size(60.0, 40.0)
                                        .background(Color::from_hex(0x22C55E))
                                        .corner_radius(4.0),
                                ),
                        )),
                )
                // Row 5: Nesting
                .child(test_section(
                    "Nested Containers (3 levels)",
                    Container::new()
                        .padding(16.0)
                        .background(Color::from_hex(0x3B82F6)) // Blue
                        .corner_radius(8.0)
                        .child(
                            Container::new()
                                .padding(16.0)
                                .background(Color::from_hex(0x22C55E)) // Green
                                .corner_radius(8.0)
                                .child(
                                    Container::new()
                                        .padding(16.0)
                                        .background(Color::from_hex(0x8B5CF6)) // Purple
                                        .corner_radius(8.0)
                                        .child(Text::new("Level 3").size(14.0).color(Color::WHITE)),
                                ),
                        ),
                )),
        ),
    )
}
