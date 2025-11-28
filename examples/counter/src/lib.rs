//! Counter app demonstrating stateful widgets in Sikhar.

use sikhar::prelude::*;
use sikhar::widgets::{EventContext, EventResponse, LayoutContext, PaintContext, Widget, WidgetId};
use sikhar::input::InputEvent;
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
    sikhar::init_web();
    run_counter();
}

/// The main counter application widget.
struct CounterApp {
    id: WidgetId,
    count: Arc<AtomicI32>,
    children: Vec<Box<dyn Widget>>,
}

impl CounterApp {
    fn new() -> Self {
        let count = Arc::new(AtomicI32::new(0));
        
        Self {
            id: WidgetId::default(),
            count: count.clone(),
            children: vec![
                Box::new(CounterDisplay::new(count.clone())),
                Box::new(CounterControls::new(count)),
            ],
        }
    }
}

impl Widget for CounterApp {
    fn id(&self) -> WidgetId { self.id }
    fn set_id(&mut self, id: WidgetId) { self.id = id; }

    fn style(&self) -> sikhar::layout::taffy::Style {
        use sikhar::layout::taffy::prelude::*;
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            justify_content: Some(JustifyContent::Center),
            size: Size { width: percent(1.0), height: percent(1.0) },
            gap: Size { width: length(0.0), height: length(40.0) },
            ..Default::default()
        }
    }

    fn paint(&self, _ctx: &mut PaintContext) {}

    fn children(&self) -> &[Box<dyn Widget>] { &self.children }
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] { &mut self.children }
}

/// Display widget showing the current count as a large number visualization.
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

    fn draw_digit(&self, ctx: &mut PaintContext, digit: u8, x: f32, y: f32, w: f32, h: f32, color: Color) {
        // 7-segment display style
        let seg_thick = h * 0.12;
        let seg_len_h = w - seg_thick * 2.0;
        let seg_len_v = (h - seg_thick * 3.0) / 2.0;
        let gap = seg_thick * 0.2;

        // Segments: top, top-left, top-right, middle, bottom-left, bottom-right, bottom
        let segments: [bool; 7] = match digit {
            0 => [true, true, true, false, true, true, true],
            1 => [false, false, true, false, false, true, false],
            2 => [true, false, true, true, true, false, true],
            3 => [true, false, true, true, false, true, true],
            4 => [false, true, true, true, false, true, false],
            5 => [true, true, false, true, false, true, true],
            6 => [true, true, false, true, true, true, true],
            7 => [true, false, true, false, false, true, false],
            8 => [true, true, true, true, true, true, true],
            9 => [true, true, true, true, false, true, true],
            _ => [false; 7],
        };

        let dim = Color::from_hex(0x1E293B);

        // Top horizontal
        let c = if segments[0] { color } else { dim };
        ctx.fill_rounded_rect(Rect::new(x + seg_thick + gap, y, seg_len_h - gap * 2.0, seg_thick), c, seg_thick * 0.4);

        // Top-left vertical
        let c = if segments[1] { color } else { dim };
        ctx.fill_rounded_rect(Rect::new(x, y + seg_thick + gap, seg_thick, seg_len_v - gap * 2.0), c, seg_thick * 0.4);

        // Top-right vertical
        let c = if segments[2] { color } else { dim };
        ctx.fill_rounded_rect(Rect::new(x + w - seg_thick, y + seg_thick + gap, seg_thick, seg_len_v - gap * 2.0), c, seg_thick * 0.4);

        // Middle horizontal
        let c = if segments[3] { color } else { dim };
        ctx.fill_rounded_rect(Rect::new(x + seg_thick + gap, y + seg_thick + seg_len_v, seg_len_h - gap * 2.0, seg_thick), c, seg_thick * 0.4);

        // Bottom-left vertical
        let c = if segments[4] { color } else { dim };
        ctx.fill_rounded_rect(Rect::new(x, y + seg_thick * 2.0 + seg_len_v + gap, seg_thick, seg_len_v - gap * 2.0), c, seg_thick * 0.4);

        // Bottom-right vertical
        let c = if segments[5] { color } else { dim };
        ctx.fill_rounded_rect(Rect::new(x + w - seg_thick, y + seg_thick * 2.0 + seg_len_v + gap, seg_thick, seg_len_v - gap * 2.0), c, seg_thick * 0.4);

        // Bottom horizontal
        let c = if segments[6] { color } else { dim };
        ctx.fill_rounded_rect(Rect::new(x + seg_thick + gap, y + h - seg_thick, seg_len_h - gap * 2.0, seg_thick), c, seg_thick * 0.4);
    }
}

impl Widget for CounterDisplay {
    fn id(&self) -> WidgetId { self.id }
    fn set_id(&mut self, id: WidgetId) { self.id = id; }

    fn style(&self) -> sikhar::layout::taffy::Style {
        use sikhar::layout::taffy::prelude::*;
        Style {
            min_size: Size { width: length(280.0), height: length(140.0) },
            align_items: Some(AlignItems::Center),
            justify_content: Some(JustifyContent::Center),
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let bounds = ctx.bounds();
        let value = self.count.load(Ordering::Relaxed);
        
        // Background panel
        ctx.fill_rounded_rect(bounds, Color::from_hex(0x1E293B), 20.0);

        // Inner glow effect
        let inner = Rect::new(bounds.x + 4.0, bounds.y + 4.0, bounds.width - 8.0, bounds.height - 8.0);
        ctx.fill_rounded_rect(inner, Color::from_hex(0x0F172A), 16.0);

        // Color based on value
        let color = if value > 0 {
            Color::from_hex(0x22C55E) // Green
        } else if value < 0 {
            Color::from_hex(0xEF4444) // Red  
        } else {
            Color::from_hex(0x64748B) // Gray
        };

        // Calculate digits
        let abs_value = value.abs();
        let mut digits: Vec<u8> = Vec::new();
        if abs_value == 0 {
            digits.push(0);
        } else {
            let mut n = abs_value;
            while n > 0 {
                digits.push((n % 10) as u8);
                n /= 10;
            }
            digits.reverse();
        }

        // Digit dimensions
        let digit_w = 36.0;
        let digit_h = 64.0;
        let digit_gap = 12.0;
        let sign_w = 24.0;
        
        let total_w = digits.len() as f32 * digit_w + (digits.len() - 1) as f32 * digit_gap
            + if value < 0 { sign_w + digit_gap } else { 0.0 };
        
        let start_x = bounds.x + (bounds.width - total_w) / 2.0;
        let y = bounds.y + (bounds.height - digit_h) / 2.0;
        
        let mut x = start_x;

        // Draw minus sign
        if value < 0 {
            let minus_y = y + digit_h / 2.0 - 4.0;
            ctx.fill_rounded_rect(Rect::new(x, minus_y, sign_w, 8.0), color, 4.0);
            x += sign_w + digit_gap;
        }

        // Draw each digit
        for digit in digits {
            self.draw_digit(ctx, digit, x, y, digit_w, digit_h, color);
            x += digit_w + digit_gap;
        }
    }

    fn measure(&self, _ctx: &mut LayoutContext) -> Option<(f32, f32)> {
        Some((280.0, 140.0))
    }
}

/// Control buttons for incrementing/decrementing.
struct CounterControls {
    id: WidgetId,
    children: Vec<Box<dyn Widget>>,
}

impl CounterControls {
    fn new(count: Arc<AtomicI32>) -> Self {
        let count_inc = count.clone();
        let count_dec = count.clone();
        let count_reset = count.clone();

        Self {
            id: WidgetId::default(),
            children: vec![
                Box::new(IconButton::new(
                    IconType::Minus,
                    Color::from_hex(0xEF4444),
                    move || { count_dec.fetch_sub(1, Ordering::Relaxed); },
                )),
                Box::new(IconButton::new(
                    IconType::Reset,
                    Color::from_hex(0x64748B),
                    move || { count_reset.store(0, Ordering::Relaxed); },
                )),
                Box::new(IconButton::new(
                    IconType::Plus,
                    Color::from_hex(0x22C55E),
                    move || { count_inc.fetch_add(1, Ordering::Relaxed); },
                )),
            ],
        }
    }
}

impl Widget for CounterControls {
    fn id(&self) -> WidgetId { self.id }
    fn set_id(&mut self, id: WidgetId) { self.id = id; }

    fn style(&self) -> sikhar::layout::taffy::Style {
        use sikhar::layout::taffy::prelude::*;
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            gap: Size { width: length(20.0), height: length(0.0) },
            ..Default::default()
        }
    }

    fn paint(&self, _ctx: &mut PaintContext) {}
    fn children(&self) -> &[Box<dyn Widget>] { &self.children }
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] { &mut self.children }
}

/// Icon types for buttons.
#[derive(Clone, Copy)]
enum IconType {
    Plus,
    Minus,
    Reset,
}

/// A button with an icon instead of text.
struct IconButton<F: FnMut() + Send + Sync> {
    id: WidgetId,
    icon: IconType,
    color: Color,
    hovered: bool,
    pressed: bool,
    on_click: F,
}

impl<F: FnMut() + Send + Sync> IconButton<F> {
    fn new(icon: IconType, color: Color, on_click: F) -> Self {
        Self {
            id: WidgetId::default(),
            icon,
            color,
            hovered: false,
            pressed: false,
            on_click,
        }
    }
}

impl<F: FnMut() + Send + Sync + 'static> Widget for IconButton<F> {
    fn id(&self) -> WidgetId { self.id }
    fn set_id(&mut self, id: WidgetId) { self.id = id; }

    fn style(&self) -> sikhar::layout::taffy::Style {
        use sikhar::layout::taffy::prelude::*;
        Style {
            size: Size { width: length(70.0), height: length(70.0) },
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let bounds = ctx.bounds();
        
        // Button background with state-based color
        let bg_color = if self.pressed {
            Color::rgba(self.color.r * 0.7, self.color.g * 0.7, self.color.b * 0.7, 1.0)
        } else if self.hovered {
            Color::rgba(
                (self.color.r * 1.2).min(1.0), 
                (self.color.g * 1.2).min(1.0), 
                (self.color.b * 1.2).min(1.0), 
                1.0
            )
        } else {
            self.color
        };

        // Outer shadow/glow
        if self.hovered && !self.pressed {
            let glow = Rect::new(bounds.x - 3.0, bounds.y - 3.0, bounds.width + 6.0, bounds.height + 6.0);
            ctx.fill_rounded_rect(glow, self.color.with_alpha(0.3), 18.0);
        }

        // Main button
        ctx.fill_rounded_rect(bounds, bg_color, 16.0);

        // Icon
        let icon_color = Color::WHITE;
        let cx = bounds.x + bounds.width / 2.0;
        let cy = bounds.y + bounds.height / 2.0;
        let icon_size = 24.0;
        let thickness = 6.0;

        match self.icon {
            IconType::Plus => {
                // Horizontal bar
                ctx.fill_rounded_rect(
                    Rect::new(cx - icon_size / 2.0, cy - thickness / 2.0, icon_size, thickness),
                    icon_color,
                    thickness / 2.0,
                );
                // Vertical bar
                ctx.fill_rounded_rect(
                    Rect::new(cx - thickness / 2.0, cy - icon_size / 2.0, thickness, icon_size),
                    icon_color,
                    thickness / 2.0,
                );
            }
            IconType::Minus => {
                // Horizontal bar only
                ctx.fill_rounded_rect(
                    Rect::new(cx - icon_size / 2.0, cy - thickness / 2.0, icon_size, thickness),
                    icon_color,
                    thickness / 2.0,
                );
            }
            IconType::Reset => {
                // Circle/reset icon (simplified as a ring)
                let ring_size = icon_size * 0.9;
                ctx.fill_rounded_rect(
                    Rect::new(cx - ring_size / 2.0, cy - ring_size / 2.0, ring_size, ring_size),
                    icon_color,
                    ring_size / 2.0,
                );
                // Inner cutout
                let inner_size = ring_size - thickness * 2.0;
                ctx.fill_rounded_rect(
                    Rect::new(cx - inner_size / 2.0, cy - inner_size / 2.0, inner_size, inner_size),
                    bg_color,
                    inner_size / 2.0,
                );
                // Arrow notch
                ctx.fill_rounded_rect(
                    Rect::new(cx - 2.0, cy - ring_size / 2.0 - 2.0, 8.0, 8.0),
                    bg_color,
                    2.0,
                );
            }
        }
    }

    fn event(&mut self, ctx: &mut EventContext, event: &InputEvent) -> EventResponse {
        match event {
            InputEvent::PointerMove { pos } => {
                let was_hovered = self.hovered;
                self.hovered = ctx.contains(*pos);
                if was_hovered != self.hovered {
                    return EventResponse { repaint: true, ..Default::default() };
                }
            }
            InputEvent::PointerDown { pos, .. } => {
                if ctx.contains(*pos) {
                    self.pressed = true;
                    return EventResponse { 
                        handled: true, 
                        capture_pointer: true, 
                        repaint: true,
                        ..Default::default() 
                    };
                }
            }
            InputEvent::PointerUp { pos, .. } => {
                if self.pressed {
                    self.pressed = false;
                    if ctx.contains(*pos) {
                        (self.on_click)();
                    }
                    return EventResponse { 
                        handled: true, 
                        release_pointer: true, 
                        repaint: true,
                        ..Default::default() 
                    };
                }
            }
            _ => {}
        }
        EventResponse::default()
    }

    fn focusable(&self) -> bool { true }
}

