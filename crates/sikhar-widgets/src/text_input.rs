//! Text input widget.

use crate::{EventContext, EventResponse, PaintContext, Widget};
use sikhar_core::Color;
use sikhar_input::{shortcuts, InputEvent, Key};
use sikhar_layout::WidgetId;
use taffy::prelude::*;

/// Style configuration for text input.
#[derive(Clone, Debug)]
pub struct TextInputStyle {
    pub background: Color,
    pub background_focused: Color,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub border_color: Color,
    pub border_color_focused: Color,
    pub border_width: f32,
    pub corner_radius: f32,
    pub padding_h: f32,
    pub padding_v: f32,
    pub font_size: f32,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            background: Color::WHITE,
            background_focused: Color::WHITE,
            text_color: Color::from_hex(0x1F2937),
            placeholder_color: Color::from_hex(0x9CA3AF),
            border_color: Color::from_hex(0xD1D5DB),
            border_color_focused: Color::from_hex(0x3B82F6),
            border_width: 1.0,
            corner_radius: 6.0,
            padding_h: 12.0,
            padding_v: 8.0,
            font_size: 14.0,
        }
    }
}

/// A single-line text input widget.
pub struct TextInput {
    id: WidgetId,
    value: String,
    placeholder: String,
    style: TextInputStyle,
    cursor_pos: usize,
    selection_start: Option<usize>,
    on_change: Option<Box<dyn FnMut(&str) + Send + Sync>>,
    on_submit: Option<Box<dyn FnMut(&str) + Send + Sync>>,
}

impl TextInput {
    /// Create a new text input.
    pub fn new() -> Self {
        Self {
            id: WidgetId::default(),
            value: String::new(),
            placeholder: String::new(),
            style: TextInputStyle::default(),
            cursor_pos: 0,
            selection_start: None,
            on_change: None,
            on_submit: None,
        }
    }

    /// Set the initial value.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor_pos = self.value.len();
        self
    }

    /// Set the placeholder text.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the change handler.
    pub fn on_change(mut self, handler: impl FnMut(&str) + Send + Sync + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Set the submit handler (called on Enter).
    pub fn on_submit(mut self, handler: impl FnMut(&str) + Send + Sync + 'static) -> Self {
        self.on_submit = Some(Box::new(handler));
        self
    }

    /// Set the style.
    pub fn with_style(mut self, style: TextInputStyle) -> Self {
        self.style = style;
        self
    }

    /// Get the current value.
    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn insert_char(&mut self, c: char) {
        self.delete_selection();
        self.value.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
        self.fire_change();
    }

    fn insert_str(&mut self, s: &str) {
        self.delete_selection();
        self.value.insert_str(self.cursor_pos, s);
        self.cursor_pos += s.len();
        self.fire_change();
    }

    fn delete_selection(&mut self) {
        if let Some(start) = self.selection_start.take() {
            let (from, to) = if start < self.cursor_pos {
                (start, self.cursor_pos)
            } else {
                (self.cursor_pos, start)
            };
            self.value.drain(from..to);
            self.cursor_pos = from;
        }
    }

    fn backspace(&mut self) {
        if self.selection_start.is_some() {
            self.delete_selection();
            self.fire_change();
        } else if self.cursor_pos > 0 {
            // Find the previous character boundary
            let prev = self.value[..self.cursor_pos]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.value.drain(prev..self.cursor_pos);
            self.cursor_pos = prev;
            self.fire_change();
        }
    }

    fn delete(&mut self) {
        if self.selection_start.is_some() {
            self.delete_selection();
            self.fire_change();
        } else if self.cursor_pos < self.value.len() {
            // Find the next character boundary
            let next = self.value[self.cursor_pos..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| self.cursor_pos + i)
                .unwrap_or(self.value.len());
            self.value.drain(self.cursor_pos..next);
            self.fire_change();
        }
    }

    fn move_cursor_left(&mut self, shift: bool) {
        if !shift {
            self.selection_start = None;
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_pos);
        }

        if self.cursor_pos > 0 {
            self.cursor_pos = self.value[..self.cursor_pos]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    fn move_cursor_right(&mut self, shift: bool) {
        if !shift {
            self.selection_start = None;
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_pos);
        }

        if self.cursor_pos < self.value.len() {
            self.cursor_pos = self.value[self.cursor_pos..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| self.cursor_pos + i)
                .unwrap_or(self.value.len());
        }
    }

    fn select_all(&mut self) {
        self.selection_start = Some(0);
        self.cursor_pos = self.value.len();
    }

    fn fire_change(&mut self) {
        if let Some(handler) = &mut self.on_change {
            handler(&self.value);
        }
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for TextInput {
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
            min_size: Size {
                width: length(100.0),
                height: auto(),
            },
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let bounds = ctx.bounds();
        let focused = ctx.has_focus();

        let bg = if focused {
            self.style.background_focused
        } else {
            self.style.background
        };

        let border = if focused {
            self.style.border_color_focused
        } else {
            self.style.border_color
        };

        // Draw background
        ctx.fill_bordered_rect(
            bounds,
            bg,
            self.style.corner_radius,
            self.style.border_width,
            border,
        );

        // Focus ring
        if focused {
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
                Color::from_hex(0x60A5FA).with_alpha(0.5),
            );
        }

        // Text and cursor rendering would be handled by the app runner
        // using the TextSystem
    }

    fn event(&mut self, ctx: &mut EventContext, event: &InputEvent) -> EventResponse {
        match event {
            InputEvent::PointerDown { pos, .. } => {
                if ctx.contains(*pos) {
                    ctx.request_focus();
                    // TODO: Position cursor based on click position
                    self.cursor_pos = self.value.len();
                    self.selection_start = None;
                    return EventResponse::focus();
                }
                EventResponse::default()
            }
            InputEvent::KeyDown { event } => {
                if !ctx.has_focus() {
                    return EventResponse::default();
                }

                use sikhar_input::NamedKey;

                // Handle shortcuts
                if shortcuts::is_select_all(event) {
                    self.select_all();
                    return EventResponse::handled();
                }

                if shortcuts::is_backspace(event) {
                    self.backspace();
                    return EventResponse::handled();
                }

                if shortcuts::is_delete(event) {
                    self.delete();
                    return EventResponse::handled();
                }

                // Arrow keys
                match &event.key {
                    Key::Named(NamedKey::ArrowLeft) => {
                        self.move_cursor_left(event.modifiers.shift());
                        return EventResponse::handled();
                    }
                    Key::Named(NamedKey::ArrowRight) => {
                        self.move_cursor_right(event.modifiers.shift());
                        return EventResponse::handled();
                    }
                    Key::Named(NamedKey::Home) => {
                        if !event.modifiers.shift() {
                            self.selection_start = None;
                        } else if self.selection_start.is_none() {
                            self.selection_start = Some(self.cursor_pos);
                        }
                        self.cursor_pos = 0;
                        return EventResponse::handled();
                    }
                    Key::Named(NamedKey::End) => {
                        if !event.modifiers.shift() {
                            self.selection_start = None;
                        } else if self.selection_start.is_none() {
                            self.selection_start = Some(self.cursor_pos);
                        }
                        self.cursor_pos = self.value.len();
                        return EventResponse::handled();
                    }
                    Key::Named(NamedKey::Enter) => {
                        if let Some(handler) = &mut self.on_submit {
                            handler(&self.value);
                        }
                        return EventResponse::handled();
                    }
                    Key::Named(NamedKey::Escape) => {
                        ctx.release_focus();
                        return EventResponse {
                            release_focus: true,
                            repaint: true,
                            ..Default::default()
                        };
                    }
                    _ => {}
                }

                EventResponse::default()
            }
            InputEvent::TextInput { text } => {
                if ctx.has_focus() {
                    // Filter out control characters
                    for c in text.chars() {
                        if !c.is_control() {
                            self.insert_char(c);
                        }
                    }
                    return EventResponse::handled();
                }
                EventResponse::default()
            }
            _ => EventResponse::default(),
        }
    }

    fn focusable(&self) -> bool {
        true
    }

    fn on_focus(&mut self) {
        // Select all on focus
        self.select_all();
    }

    fn on_blur(&mut self) {
        self.selection_start = None;
    }
}

