//! Scrollable container widget.

use crate::{EventContext, EventResponse, PaintContext, Widget};
use sikhar_core::{Color, Rect};
use sikhar_input::InputEvent;
use sikhar_layout::WidgetId;
use taffy::prelude::*;
use taffy::{Overflow, Point};

/// Scroll direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollDirection {
    /// Scroll vertically only.
    #[default]
    Vertical,
    /// Scroll horizontally only.
    Horizontal,
    /// Scroll in both directions.
    Both,
}

/// Style for scrollbar.
#[derive(Clone, Debug)]
pub struct ScrollbarStyle {
    pub track_color: Color,
    pub thumb_color: Color,
    pub thumb_hover_color: Color,
    pub width: f32,
    pub corner_radius: f32,
}

impl Default for ScrollbarStyle {
    fn default() -> Self {
        Self {
            track_color: Color::from_hex(0xE5E7EB),
            thumb_color: Color::from_hex(0x9CA3AF),
            thumb_hover_color: Color::from_hex(0x6B7280),
            width: 8.0,
            corner_radius: 4.0,
        }
    }
}

/// A scrollable container widget.
pub struct Scroll {
    id: WidgetId,
    content: Option<Box<dyn Widget>>,
    direction: ScrollDirection,
    offset_x: f32,
    offset_y: f32,
    content_size: (f32, f32),
    style: ScrollbarStyle,
    dragging_scrollbar: bool,
    hover_scrollbar: bool,
}

impl Default for Scroll {
    fn default() -> Self {
        Self::new()
    }
}

impl Scroll {
    /// Create a new scroll container.
    pub fn new() -> Self {
        Self {
            id: WidgetId::default(),
            content: None,
            direction: ScrollDirection::Vertical,
            offset_x: 0.0,
            offset_y: 0.0,
            content_size: (0.0, 0.0),
            style: ScrollbarStyle::default(),
            dragging_scrollbar: false,
            hover_scrollbar: false,
        }
    }

    /// Set the content widget.
    pub fn content(mut self, widget: impl Widget + 'static) -> Self {
        self.content = Some(Box::new(widget));
        self
    }

    /// Set the scroll direction.
    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set to vertical scrolling only.
    pub fn vertical(mut self) -> Self {
        self.direction = ScrollDirection::Vertical;
        self
    }

    /// Set to horizontal scrolling only.
    pub fn horizontal(mut self) -> Self {
        self.direction = ScrollDirection::Horizontal;
        self
    }

    /// Set the scrollbar style.
    pub fn scrollbar_style(mut self, style: ScrollbarStyle) -> Self {
        self.style = style;
        self
    }

    /// Get the current scroll offset.
    pub fn offset(&self) -> (f32, f32) {
        (self.offset_x, self.offset_y)
    }

    /// Set the scroll offset.
    pub fn set_offset(&mut self, x: f32, y: f32) {
        self.offset_x = x.max(0.0);
        self.offset_y = y.max(0.0);
    }

    /// Scroll to ensure a rectangle is visible.
    pub fn scroll_to_visible(&mut self, rect: Rect, viewport: Rect) {
        // Vertical
        if rect.y < self.offset_y {
            self.offset_y = rect.y;
        } else if rect.y + rect.height > self.offset_y + viewport.height {
            self.offset_y = rect.y + rect.height - viewport.height;
        }

        // Horizontal
        if rect.x < self.offset_x {
            self.offset_x = rect.x;
        } else if rect.x + rect.width > self.offset_x + viewport.width {
            self.offset_x = rect.x + rect.width - viewport.width;
        }
    }

    fn clamp_offset(&mut self, viewport: Rect) {
        let max_x = (self.content_size.0 - viewport.width).max(0.0);
        let max_y = (self.content_size.1 - viewport.height).max(0.0);
        self.offset_x = self.offset_x.clamp(0.0, max_x);
        self.offset_y = self.offset_y.clamp(0.0, max_y);
    }

    fn scrollbar_rect(&self, viewport: Rect) -> Option<Rect> {
        match self.direction {
            ScrollDirection::Vertical | ScrollDirection::Both => {
                if self.content_size.1 <= viewport.height {
                    return None;
                }

                let track_height = viewport.height;
                let thumb_height =
                    (viewport.height / self.content_size.1 * track_height).max(20.0);
                let thumb_y = (self.offset_y / (self.content_size.1 - viewport.height))
                    * (track_height - thumb_height);

                Some(Rect::new(
                    viewport.x + viewport.width - self.style.width,
                    viewport.y + thumb_y,
                    self.style.width,
                    thumb_height,
                ))
            }
            ScrollDirection::Horizontal => {
                if self.content_size.0 <= viewport.width {
                    return None;
                }

                let track_width = viewport.width;
                let thumb_width = (viewport.width / self.content_size.0 * track_width).max(20.0);
                let thumb_x = (self.offset_x / (self.content_size.0 - viewport.width))
                    * (track_width - thumb_width);

                Some(Rect::new(
                    viewport.x + thumb_x,
                    viewport.y + viewport.height - self.style.width,
                    thumb_width,
                    self.style.width,
                ))
            }
        }
    }
}

impl Widget for Scroll {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn set_id(&mut self, id: WidgetId) {
        self.id = id;
    }

    fn style(&self) -> Style {
        Style {
            overflow: Point {
                x: Overflow::Hidden,
                y: Overflow::Hidden,
            },
            ..Default::default()
        }
    }

    fn paint(&self, ctx: &mut PaintContext) {
        let bounds = ctx.bounds();

        // Clip content
        ctx.push_clip(bounds);

        // Content is painted by the framework with offset applied
        // (not implemented here - would need transform support)

        ctx.pop_clip();

        // Draw scrollbar
        if let Some(scrollbar) = self.scrollbar_rect(bounds) {
            // Track
            let track = match self.direction {
                ScrollDirection::Vertical | ScrollDirection::Both => Rect::new(
                    bounds.x + bounds.width - self.style.width,
                    bounds.y,
                    self.style.width,
                    bounds.height,
                ),
                ScrollDirection::Horizontal => Rect::new(
                    bounds.x,
                    bounds.y + bounds.height - self.style.width,
                    bounds.width,
                    self.style.width,
                ),
            };
            ctx.fill_rounded_rect(track, self.style.track_color, self.style.corner_radius);

            // Thumb
            let thumb_color = if self.hover_scrollbar || self.dragging_scrollbar {
                self.style.thumb_hover_color
            } else {
                self.style.thumb_color
            };
            ctx.fill_rounded_rect(scrollbar, thumb_color, self.style.corner_radius);
        }
    }

    fn event(&mut self, ctx: &mut EventContext, event: &InputEvent) -> EventResponse {
        let bounds = ctx.bounds();

        match event {
            InputEvent::Scroll { delta, pos } => {
                if ctx.contains(*pos) {
                    match self.direction {
                        ScrollDirection::Vertical => {
                            self.offset_y -= delta.y * 20.0;
                        }
                        ScrollDirection::Horizontal => {
                            self.offset_x -= delta.x * 20.0;
                        }
                        ScrollDirection::Both => {
                            self.offset_x -= delta.x * 20.0;
                            self.offset_y -= delta.y * 20.0;
                        }
                    }
                    self.clamp_offset(bounds);
                    return EventResponse::handled();
                }
            }
            InputEvent::PointerMove { pos } => {
                if let Some(scrollbar) = self.scrollbar_rect(bounds) {
                    let was_hover = self.hover_scrollbar;
                    self.hover_scrollbar = scrollbar.contains(*pos);
                    if was_hover != self.hover_scrollbar {
                        return EventResponse {
                            repaint: true,
                            ..Default::default()
                        };
                    }
                }
            }
            _ => {}
        }

        EventResponse::default()
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        match &self.content {
            Some(c) => std::slice::from_ref(c),
            None => &[],
        }
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        match &mut self.content {
            Some(c) => std::slice::from_mut(c),
            None => &mut [],
        }
    }
}

