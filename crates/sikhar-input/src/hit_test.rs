//! Hit testing for finding widgets under a point.

use glam::Vec2;
use sikhar_core::Rect;
use sikhar_layout::{ComputedLayout, LayoutTree, WidgetId};

/// Result of a hit test.
#[derive(Clone, Copy, Debug)]
pub struct HitTestResult {
    /// The widget that was hit.
    pub widget_id: WidgetId,
    /// The position relative to the widget's bounds.
    pub local_pos: Vec2,
    /// The depth of the widget in the tree (higher = more nested).
    pub depth: usize,
}

/// Perform a hit test to find the deepest widget at the given position.
pub fn hit_test(layout_tree: &LayoutTree, pos: Vec2) -> Option<HitTestResult> {
    let mut result: Option<HitTestResult> = None;

    layout_tree.traverse(|widget_id, computed, depth| {
        if computed.bounds.contains(pos) {
            // Prefer deeper widgets (more nested = more specific)
            if result.is_none() || depth > result.as_ref().unwrap().depth {
                result = Some(HitTestResult {
                    widget_id,
                    local_pos: Vec2::new(
                        pos.x - computed.bounds.x,
                        pos.y - computed.bounds.y,
                    ),
                    depth,
                });
            }
        }
    });

    result
}

/// Perform a hit test with a custom filter.
pub fn hit_test_filtered<F>(
    layout_tree: &LayoutTree,
    pos: Vec2,
    filter: F,
) -> Option<HitTestResult>
where
    F: Fn(WidgetId) -> bool,
{
    let mut result: Option<HitTestResult> = None;

    layout_tree.traverse(|widget_id, computed, depth| {
        if computed.bounds.contains(pos) && filter(widget_id) {
            if result.is_none() || depth > result.as_ref().unwrap().depth {
                result = Some(HitTestResult {
                    widget_id,
                    local_pos: Vec2::new(
                        pos.x - computed.bounds.x,
                        pos.y - computed.bounds.y,
                    ),
                    depth,
                });
            }
        }
    });

    result
}

/// Check if a point is inside a rectangle.
pub fn point_in_rect(pos: Vec2, rect: &Rect) -> bool {
    rect.contains(pos)
}

/// Check if a point is inside a computed layout.
pub fn point_in_layout(pos: Vec2, layout: &ComputedLayout) -> bool {
    layout.bounds.contains(pos)
}

/// Get all widgets at a position (from front to back).
pub fn hit_test_all(layout_tree: &LayoutTree, pos: Vec2) -> Vec<HitTestResult> {
    let mut results = Vec::new();

    layout_tree.traverse(|widget_id, computed, depth| {
        if computed.bounds.contains(pos) {
            results.push(HitTestResult {
                widget_id,
                local_pos: Vec2::new(
                    pos.x - computed.bounds.x,
                    pos.y - computed.bounds.y,
                ),
                depth,
            });
        }
    });

    // Sort by depth descending (deepest first)
    results.sort_by(|a, b| b.depth.cmp(&a.depth));

    results
}

