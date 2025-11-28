//! Button widget.

use crate::{EventContext, EventResponse, PaintContext, Widget};
use sikhar_core::Color;
use sikhar_input::InputEvent;
use sikhar_layout::WidgetId;
use taffy::prelude::*;

/// Visual state of the button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonState {
    #[default]
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

/// Style configuration for a button.
#[derive(Clone, Debug)]
pub struct ButtonStyle {
    pub background: Color,
    pub background_hovered: Color,
    pub background_pressed: Color,
    pub background_disabled: Color,
    pub text_color: Color,
    pub text_color_disabled: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub corner_radius: f32,
    pub padding_h: f32,
    pub padding_v: f32,
    pub font_size: f32,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            background: Color::from_hex(0x3B82F6),         // Blue
            background_hovered: Color::from_hex(0x2563EB), // Darker blue
            background_pressed: Color::from_hex(0x1D4ED8), // Even darker
            background_disabled: Color::from_hex(0x9CA3AF), // Gray
            text_color: Color::WHITE,
            text_color_disabled: Color::from_hex(0x6B7280),
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            corner_radius: 6.0,
            padding_h: 16.0,
            padding_v: 8.0,
            font_size: 14.0,
        }
    }
}

/// A clickable button widget.
pub struct Button {
    id: WidgetId,
    label: String,
    style: ButtonStyle,
    state: ButtonState,
    on_click: Option<Box<dyn FnMut() + Send + Sync>>,
}

impl Button {
    /// Create a new button with the given label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: WidgetId::default(),
            label: label.into(),
            style: ButtonStyle::default(),
            state: ButtonState::Normal,
            on_click: None,
        }
    }

    /// Set the click handler.
    pub fn on_click(mut self, handler: impl FnMut() + Send + Sync + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Set the button style.
    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the background color.
    pub fn background(mut self, color: Color) -> Self {
        self.style.background = color;
        self
    }

    /// Set the text color.
    pub fn text_color(mut self, color: Color) -> Self {
        self.style.text_color = color;
        self
    }

    /// Set corner radius.
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.style.corner_radius = radius;
        self
    }

    /// Disable the button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        if disabled {
            self.state = ButtonState::Disabled;
        }
        self
    }

    fn current_background(&self) -> Color {
        match self.state {
            ButtonState::Normal => self.style.background,
            ButtonState::Hovered => self.style.background_hovered,
            ButtonState::Pressed => self.style.background_pressed,
            ButtonState::Disabled => self.style.background_disabled,
        }
    }

    fn current_text_color(&self) -> Color {
        match self.state {
            ButtonState::Disabled => self.style.text_color_disabled,
            _ => self.style.text_color,
        }
    }
}

impl Widget for Button {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn style(&self) -> Style {
        Style {
            padding: Rect {
                left: length(self.style.padding_h),
                right: length(self.style.padding_h),
                top: length(self.style.padding_v),
                bottom: length(self.style.padding_v),
            },
            align_items: Some(AlignItems::Center),
            justify_content: Some(JustifyContent::Center),
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let bounds = ctx.bounds();
        let bg = self.current_background();
        let _text_color = self.current_text_color();

        // Draw button background
        if self.style.border_width > 0.0 {
            ctx.fill_bordered_rect(
                bounds,
                bg,
                self.style.corner_radius,
                self.style.border_width,
                self.style.border_color,
            );
        } else {
            ctx.fill_rounded_rect(bounds, bg, self.style.corner_radius);
        }

        // Focus ring
        if ctx.has_focus() {
            let focus_bounds = sikhar_core::Rect::new(
                bounds.x - 2.0,
                bounds.y - 2.0,
                bounds.width + 4.0,
                bounds.height + 4.0,
            );
            ctx.fill_bordered_rect(
                focus_bounds,
                Color::TRANSPARENT,
                self.style.corner_radius + 2.0,
                2.0,
                Color::from_hex(0x60A5FA),
            );
        }

        // Text rendering is handled separately via TextSystem
        // For now, we just mark where text would be drawn
        // The actual text rendering will be done by the App runner
    }

    fn event(&mut self, ctx: &mut EventContext, event: &InputEvent) -> EventResponse {
        if self.state == ButtonState::Disabled {
            return EventResponse::default();
        }

        match event {
            InputEvent::PointerMove { pos } => {
                if ctx.contains(*pos) {
                    if self.state != ButtonState::Pressed {
                        self.state = ButtonState::Hovered;
                    }
                } else {
                    self.state = ButtonState::Normal;
                }
                EventResponse {
                    repaint: true,
                    ..Default::default()
                }
            }
            InputEvent::PointerDown { pos, .. } => {
                if ctx.contains(*pos) {
                    self.state = ButtonState::Pressed;
                    return EventResponse::capture();
                }
                EventResponse::default()
            }
            InputEvent::PointerUp { pos, .. } => {
                if self.state == ButtonState::Pressed {
                    if ctx.contains(*pos) {
                        // Fire click handler
                        if let Some(handler) = &mut self.on_click {
                            handler();
                        }
                        self.state = ButtonState::Hovered;
                    } else {
                        self.state = ButtonState::Normal;
                    }
                    return EventResponse::release();
                }
                EventResponse::default()
            }
            InputEvent::KeyDown { event } => {
                if ctx.has_focus() {
                    use sikhar_input::{Key, NamedKey};
                    if matches!(&event.key, Key::Named(NamedKey::Enter)) {
                        if let Some(handler) = &mut self.on_click {
                            handler();
                        }
                        return EventResponse::handled();
                    }
                }
                EventResponse::default()
            }
            _ => EventResponse::default(),
        }
    }

    fn focusable(&self) -> bool {
        self.state != ButtonState::Disabled
    }

    fn measure(&self, ctx: &mut crate::LayoutContext) -> Option<(f32, f32)> {
        use sikhar_text::TextStyle;
        let style = TextStyle::default().with_size(self.style.font_size);
        let (w, h) = ctx.text.measure(&self.label, &style, None);
        Some((
            w + self.style.padding_h * 2.0,
            h + self.style.padding_v * 2.0,
        ))
    }
}

