//! Context types passed to widgets during layout, paint, and events.

use sikhar_core::{Color, Rect};
use sikhar_input::FocusManager;
use sikhar_layout::{ComputedLayout, LayoutTree, WidgetId};
use sikhar_render::DrawList;
use sikhar_text::{TextStyle, TextSystem};

/// Context for layout measurement.
pub struct LayoutContext<'a> {
    /// The text system for measuring text.
    pub text: &'a mut TextSystem,
    /// Available width constraint.
    pub max_width: Option<f32>,
    /// Available height constraint.
    pub max_height: Option<f32>,
}

impl<'a> LayoutContext<'a> {
    /// Measure text with the current constraints.
    pub fn measure_text(&mut self, text: &str, style: &TextStyle) -> (f32, f32) {
        self.text.measure(text, style, self.max_width)
    }
}

/// Context for painting widgets.
pub struct PaintContext<'a> {
    /// The draw list to paint to.
    pub draw_list: &'a mut DrawList,
    /// The computed layout for this widget.
    pub layout: ComputedLayout,
    /// The focus manager (for focus state).
    pub focus: &'a FocusManager,
    /// Current widget ID.
    pub widget_id: WidgetId,
    /// Scale factor for HiDPI.
    pub scale_factor: f32,
}

impl<'a> PaintContext<'a> {
    /// Get the widget's bounds.
    pub fn bounds(&self) -> Rect {
        self.layout.bounds
    }

    /// Check if this widget has keyboard focus.
    pub fn has_focus(&self) -> bool {
        self.focus.has_focus(self.widget_id)
    }

    /// Draw a filled rectangle.
    pub fn fill_rect(&mut self, bounds: Rect, color: Color) {
        self.draw_list.rect(bounds, color);
    }

    /// Draw a rounded rectangle.
    pub fn fill_rounded_rect(&mut self, bounds: Rect, color: Color, radius: f32) {
        self.draw_list.rounded_rect(bounds, color, radius);
    }

    /// Draw a rectangle with a border.
    pub fn fill_bordered_rect(
        &mut self,
        bounds: Rect,
        color: Color,
        radius: f32,
        border_width: f32,
        border_color: Color,
    ) {
        self.draw_list
            .bordered_rect(bounds, color, radius, border_width, border_color);
    }

    /// Push a clip rectangle.
    pub fn push_clip(&mut self, bounds: Rect) {
        self.draw_list.push_clip(bounds);
    }

    /// Pop the clip rectangle.
    pub fn pop_clip(&mut self) {
        self.draw_list.pop_clip();
    }
}

/// Context for handling events.
pub struct EventContext<'a> {
    /// The computed layout for this widget.
    pub layout: ComputedLayout,
    /// The layout tree for hit testing children.
    pub layout_tree: &'a LayoutTree,
    /// Focus manager.
    pub focus: &'a mut FocusManager,
    /// Current widget ID.
    pub widget_id: WidgetId,
    /// Whether this widget has pointer capture.
    pub has_capture: bool,
}

impl<'a> EventContext<'a> {
    /// Get the widget's bounds.
    pub fn bounds(&self) -> Rect {
        self.layout.bounds
    }

    /// Check if this widget has keyboard focus.
    pub fn has_focus(&self) -> bool {
        self.focus.has_focus(self.widget_id)
    }

    /// Request keyboard focus for this widget.
    pub fn request_focus(&mut self) {
        self.focus.set_focus(self.widget_id);
    }

    /// Release keyboard focus.
    pub fn release_focus(&mut self) {
        if self.has_focus() {
            self.focus.clear_focus();
        }
    }

    /// Check if a point is inside this widget's bounds.
    pub fn contains(&self, pos: glam::Vec2) -> bool {
        self.layout.bounds.contains(pos)
    }

    /// Convert a point to local coordinates.
    pub fn to_local(&self, pos: glam::Vec2) -> glam::Vec2 {
        glam::Vec2::new(
            pos.x - self.layout.bounds.x,
            pos.y - self.layout.bounds.y,
        )
    }
}

