//! Counter app demonstrating stateful widgets in Spark.

use spark::prelude::*;
use spark::text::TextStyle;
use spark::widgets::{LayoutContext, PaintContext, Widget, WidgetId};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub fn run_counter() {
    App::new()
        .with_title("Counter")
        .with_size(500, 400)
        .with_background(Color::from_hex(0x0F172A)) // Slate 900
        .run(|| Box::new(CounterApp::new()));
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    spark::init_web();
    run_counter();
}

/// The main counter application widget.
struct CounterApp {
    id: WidgetId,
    children: Vec<Box<dyn Widget>>,
}

impl CounterApp {
    fn new() -> Self {
        let count = Arc::new(AtomicI32::new(0));

        Self {
            id: WidgetId::default(),
            children: vec![
                // Title
                Box::new(
                    Text::new("Counter Demo")
                        .size(28.0)
                        .bold()
                        .color(Color::WHITE),
                ),
                // Counter display
                Box::new(CounterDisplay::new(count.clone())),
                // Button row
                Box::new(CounterControls::new(count)),
            ],
        }
    }
}

impl Widget for CounterApp {
    fn id(&self) -> WidgetId {
        self.id
    }
    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn style(&self) -> spark::layout::taffy::Style {
        use spark::layout::taffy::prelude::*;
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            justify_content: Some(JustifyContent::Center),
            size: Size {
                width: percent(1.0),
                height: percent(1.0),
            },
            gap: Size {
                width: length(0.0),
                height: length(30.0),
            },
            ..Default::default()
        }
    }

    fn paint(&self, _ctx: &mut PaintContext) {}

    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut self.children
    }
}

/// Display widget showing the current count as large text.
struct CounterDisplay {
    id: WidgetId,
    count: Arc<AtomicI32>,
}

impl CounterDisplay {
    fn new(count: Arc<AtomicI32>) -> Self {
        Self {
            id: WidgetId::default(),
            count,
        }
    }
}

impl Widget for CounterDisplay {
    fn id(&self) -> WidgetId {
        self.id
    }
    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn style(&self) -> spark::layout::taffy::Style {
        use spark::layout::taffy::prelude::*;
        Style {
            min_size: Size {
                width: length(200.0),
                height: length(100.0),
            },
            padding: Rect {
                left: length(32.0),
                right: length(32.0),
                top: length(24.0),
                bottom: length(24.0),
            },
            align_items: Some(AlignItems::Center),
            justify_content: Some(JustifyContent::Center),
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let bounds = ctx.bounds();
        let value = self.count.load(Ordering::Relaxed);

        // Background panel with rounded corners
        ctx.fill_rounded_rect(bounds, Color::from_hex(0x1E293B), 16.0);

        // Inner panel
        let inner = Rect::new(
            bounds.x + 4.0,
            bounds.y + 4.0,
            bounds.width - 8.0,
            bounds.height - 8.0,
        );
        ctx.fill_rounded_rect(inner, Color::from_hex(0x0F172A), 12.0);

        // Color based on value
        let color = if value > 0 {
            Color::from_hex(0x22C55E) // Green for positive
        } else if value < 0 {
            Color::from_hex(0xEF4444) // Red for negative
        } else {
            Color::from_hex(0x94A3B8) // Gray for zero
        };

        // Draw the number as text
        let text_style = TextStyle::default()
            .with_size(64.0)
            .bold()
            .with_color(color);

        ctx.draw_text_centered(&value.to_string(), &text_style, bounds);
    }

    fn measure(&self, _ctx: &mut LayoutContext) -> Option<(f32, f32)> {
        Some((200.0, 100.0))
    }
}

/// Control buttons for incrementing/decrementing.
struct CounterControls {
    id: WidgetId,
    children: Vec<Box<dyn Widget>>,
}

impl CounterControls {
    fn new(count: Arc<AtomicI32>) -> Self {
        let count_dec = count.clone();
        let count_reset = count.clone();
        let count_inc = count.clone();

        Self {
            id: WidgetId::default(),
            children: vec![
                // Decrement button
                Box::new(
                    Button::new("−") // Minus sign
                        .background(Color::from_hex(0xEF4444)) // Red
                        .corner_radius(12.0)
                        .on_click(move || {
                            count_dec.fetch_sub(1, Ordering::Relaxed);
                        }),
                ),
                // Reset button
                Box::new(
                    Button::new("⟲") // Reset symbol
                        .background(Color::from_hex(0x64748B)) // Gray
                        .corner_radius(12.0)
                        .on_click(move || {
                            count_reset.store(0, Ordering::Relaxed);
                        }),
                ),
                // Increment button
                Box::new(
                    Button::new("+")
                        .background(Color::from_hex(0x22C55E)) // Green
                        .corner_radius(12.0)
                        .on_click(move || {
                            count_inc.fetch_add(1, Ordering::Relaxed);
                        }),
                ),
            ],
        }
    }
}

impl Widget for CounterControls {
    fn id(&self) -> WidgetId {
        self.id
    }
    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn style(&self) -> spark::layout::taffy::Style {
        use spark::layout::taffy::prelude::*;
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            gap: Size {
                width: length(16.0),
                height: length(0.0),
            },
            ..Default::default()
        }
    }

    fn paint(&self, _ctx: &mut PaintContext) {}

    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut self.children
    }
}
