//! Layout bridge - converts taffy layouts to native constraints/frames.

use glam::Vec2;
use sikhar_layout::taffy;

/// Bridge between taffy layout and native view frames.
pub struct LayoutBridge;

impl LayoutBridge {
    /// Convert a taffy layout to a native frame.
    ///
    /// On macOS, AppKit uses bottom-left origin, so we need to flip Y.
    /// On iOS, UIKit uses top-left origin, matching taffy.
    pub fn taffy_to_native_frame(
        layout: &taffy::Layout,
        parent_height: f32,
        scale_factor: f32,
    ) -> (f64, f64, f64, f64) {
        let x = layout.location.x as f64;
        let y = layout.location.y as f64;
        let width = layout.size.width as f64;
        let height = layout.size.height as f64;
        Self::convert_coords(x, y, width, height, parent_height, scale_factor)
    }

    /// Convert bounds (Rect) to native frame.
    pub fn taffy_to_native_frame_from_bounds(
        bounds: &sikhar_core::Rect,
        parent_height: f32,
        scale_factor: f32,
    ) -> (f64, f64, f64, f64) {
        // Note: On macOS, we might not need to apply scale_factor here if the view
        // is already in the correct coordinate space. But for now, we'll apply it.
        // The scale_factor from winit accounts for Retina displays.
        let x = bounds.x as f64;
        let y = bounds.y as f64;
        let width = bounds.width as f64;
        let height = bounds.height as f64;
        Self::convert_coords(x, y, width, height, parent_height, scale_factor)
    }

    /// Convert coordinates (helper function).
    fn convert_coords(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        parent_height: f32,
        scale_factor: f32,
    ) -> (f64, f64, f64, f64) {
        #[cfg(target_os = "macos")]
        {
            // macOS uses bottom-left origin, taffy uses top-left
            // Flip Y coordinate: y_macos = window_height - y_taffy - height
            // Note: parent_height should be the window height in logical pixels
            // Both taffy and NSView use points (logical pixels), so no scale_factor needed
            let logical_height = parent_height as f64;

            let flipped_y = logical_height - y - height;

            // Debug: log if we get negative Y (only first few times to avoid spam)
            static mut WARN_COUNT: u32 = 0;
            if flipped_y < 0.0 {
                unsafe {
                    if WARN_COUNT < 5 {
                        eprintln!("Warning: Negative Y after flip: y={}, height={}, parent_height={}, flipped_y={}", 
                            y, height, logical_height, flipped_y);
                        WARN_COUNT += 1;
                    }
                }
            }

            // Ensure minimum size and clamp Y to valid range
            let width = width.max(1.0);
            let height = height.max(1.0);
            let x = x.max(0.0);
            let y = flipped_y.max(0.0);

            (x, y, width, height)
        }

        #[cfg(target_os = "ios")]
        {
            // iOS uses top-left origin, same as taffy
            (x, y, width, height)
        }
    }

    /// Convert taffy size to native size.
    pub fn taffy_to_native_size(size: &taffy::Size<f32>, scale_factor: f32) -> (f64, f64) {
        (
            size.width as f64 * scale_factor as f64,
            size.height as f64 * scale_factor as f64,
        )
    }

    /// Convert native point to taffy point (for event coordinates).
    ///
    /// On macOS, AppKit uses bottom-left origin, so we need to flip Y.
    /// On iOS, UIKit uses top-left origin, matching taffy.
    ///
    /// Note: Both taffy and native views use logical pixels (points), so scale_factor
    /// is not needed for coordinate conversion, but is kept for API compatibility.
    pub fn native_to_taffy_point(x: f64, y: f64, parent_height: f32, _scale_factor: f32) -> Vec2 {
        #[cfg(target_os = "macos")]
        {
            // macOS uses bottom-left origin, taffy uses top-left
            // If a point is at y_macos in macOS (bottom-left origin),
            // the same point in taffy (top-left origin) is: y_taffy = parent_height - y_macos
            // Both coordinates are in logical pixels (points), so no scale_factor needed
            let logical_height = parent_height as f64;
            let flipped_y = logical_height - y;
            Vec2::new(x as f32, flipped_y as f32)
        }

        #[cfg(target_os = "ios")]
        {
            // iOS uses top-left origin, same as taffy
            Vec2::new(x as f32, y as f32)
        }
    }

    /// Update a native view's frame based on taffy layout.
    pub fn update_native_view_frame(
        view_handle: &crate::NativeViewHandle,
        layout: &taffy::Layout,
        parent_height: f32,
        scale_factor: f32,
    ) {
        let (x, y, width, height) =
            Self::taffy_to_native_frame(layout, parent_height, scale_factor);

        match view_handle {
            #[cfg(target_os = "macos")]
            crate::NativeViewHandle::AppKit(ptr) => {
                unsafe {
                    use crate::ffi::appkit::NSView;
                    // Create a temporary view wrapper to call set_frame
                    // In practice, you'd store the view properly
                    let view = NSView { obj: *ptr };
                    view.set_frame(x, y, width, height);
                }
            }
            #[cfg(target_os = "ios")]
            crate::NativeViewHandle::UIKit(ptr) => unsafe {
                use crate::ffi::uikit::UIView;
                let view = UIView { obj: *ptr };
                view.set_frame(x, y, width, height);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sikhar_core::Rect;

    #[test]
    fn test_coordinate_conversion_macos_top_left() {
        // Test: Widget at top-left of window (y=0 in taffy)
        // Window height: 600 logical pixels
        // Widget: x=0, y=0, w=100, h=50
        // Expected macOS Y: 600 - 0 - 50 = 550 (bottom-left origin)
        #[cfg(target_os = "macos")]
        {
            let (x, y, w, h) = LayoutBridge::convert_coords(0.0, 0.0, 100.0, 50.0, 600.0, 2.0);
            assert_eq!(x, 0.0, "X should be 0.0");
            assert_eq!(y, 550.0, "Y should be 550.0 (600 - 0 - 50)");
            assert_eq!(w, 100.0, "Width should be 100.0");
            assert_eq!(h, 50.0, "Height should be 50.0");
        }
    }

    #[test]
    fn test_coordinate_conversion_macos_bottom_left() {
        // Test: Widget at bottom-left of window (y=550 in taffy)
        // Window height: 600 logical pixels
        // Widget: x=0, y=550, w=100, h=50
        // Expected macOS Y: 600 - 550 - 50 = 0 (bottom-left origin)
        #[cfg(target_os = "macos")]
        {
            let (x, y, w, h) = LayoutBridge::convert_coords(0.0, 550.0, 100.0, 50.0, 600.0, 2.0);
            assert_eq!(x, 0.0, "X should be 0.0");
            assert_eq!(y, 0.0, "Y should be 0.0 (600 - 550 - 50)");
            assert_eq!(w, 100.0, "Width should be 100.0");
            assert_eq!(h, 50.0, "Height should be 50.0");
        }
    }

    #[test]
    fn test_coordinate_conversion_macos_center() {
        // Test: Widget at center of window
        // Window height: 600 logical pixels
        // Widget: x=300, y=275, w=200, h=50
        // Expected macOS Y: 600 - 275 - 50 = 275
        #[cfg(target_os = "macos")]
        {
            let (x, y, w, h) = LayoutBridge::convert_coords(300.0, 275.0, 200.0, 50.0, 600.0, 2.0);
            assert_eq!(x, 300.0, "X should be 300.0");
            assert_eq!(y, 275.0, "Y should be 275.0 (600 - 275 - 50)");
            assert_eq!(w, 200.0, "Width should be 200.0");
            assert_eq!(h, 50.0, "Height should be 50.0");
        }
    }

    #[test]
    fn test_coordinate_conversion_macos_negative_y_clamped() {
        // Test: Widget that would result in negative Y (should be clamped to 0)
        // Window height: 600 logical pixels
        // Widget: x=0, y=500, w=100, h=200 (extends beyond window)
        // Expected macOS Y: max(0, 600 - 500 - 200) = max(0, -100) = 0
        #[cfg(target_os = "macos")]
        {
            let (x, y, w, h) = LayoutBridge::convert_coords(0.0, 500.0, 100.0, 200.0, 600.0, 2.0);
            assert_eq!(x, 0.0, "X should be 0.0");
            assert_eq!(y, 0.0, "Y should be clamped to 0.0");
            assert_eq!(w, 100.0, "Width should be 100.0");
            assert_eq!(h, 200.0, "Height should be 200.0");
        }
    }

    #[test]
    fn test_coordinate_conversion_macos_zero_size_minimum() {
        // Test: Widget with zero size (should be clamped to minimum 1x1)
        #[cfg(target_os = "macos")]
        {
            let (x, y, w, h) = LayoutBridge::convert_coords(100.0, 200.0, 0.0, 0.0, 600.0, 2.0);
            assert_eq!(x, 100.0, "X should be 100.0");
            assert_eq!(
                y, 400.0,
                "Y should be 400.0 (600 - 200 - 0, but height becomes 1)"
            );
            assert_eq!(w, 1.0, "Width should be clamped to 1.0");
            assert_eq!(h, 1.0, "Height should be clamped to 1.0");
        }
    }

    #[test]
    fn test_coordinate_conversion_macos_negative_x_clamped() {
        // Test: Widget with negative X (should be clamped to 0)
        #[cfg(target_os = "macos")]
        {
            let (x, y, w, h) = LayoutBridge::convert_coords(-50.0, 100.0, 100.0, 50.0, 600.0, 2.0);
            assert_eq!(x, 0.0, "X should be clamped to 0.0");
            assert_eq!(y, 450.0, "Y should be 450.0 (600 - 100 - 50)");
            assert_eq!(w, 100.0, "Width should be 100.0");
            assert_eq!(h, 50.0, "Height should be 50.0");
        }
    }

    #[test]
    fn test_bounds_to_frame_conversion() {
        // Test: Convert Rect bounds to native frame
        #[cfg(target_os = "macos")]
        {
            let bounds = Rect::new(50.0, 100.0, 200.0, 30.0);
            let (x, y, w, h) = LayoutBridge::taffy_to_native_frame_from_bounds(&bounds, 600.0, 2.0);
            assert_eq!(x, 50.0, "X should be 50.0");
            assert_eq!(y, 470.0, "Y should be 470.0 (600 - 100 - 30)");
            assert_eq!(w, 200.0, "Width should be 200.0");
            assert_eq!(h, 30.0, "Height should be 30.0");
        }
    }

    #[test]
    fn test_coordinate_conversion_ios_no_flip() {
        // Test: iOS uses same coordinate system as taffy (no flip needed)
        #[cfg(target_os = "ios")]
        {
            let (x, y, w, h) = LayoutBridge::convert_coords(0.0, 0.0, 100.0, 50.0, 600.0, 2.0);
            assert_eq!(x, 0.0, "X should be 0.0");
            assert_eq!(y, 0.0, "Y should be 0.0 (no flip on iOS)");
            assert_eq!(w, 100.0, "Width should be 100.0");
            assert_eq!(h, 50.0, "Height should be 50.0");
        }
    }

    #[test]
    fn test_coordinate_conversion_scale_factor_independence() {
        // Test: Scale factor should not affect coordinate conversion on macOS
        // (Both taffy and NSView use points, not pixels)
        #[cfg(target_os = "macos")]
        {
            let bounds = Rect::new(100.0, 200.0, 150.0, 40.0);
            let (x1, y1, w1, h1) =
                LayoutBridge::taffy_to_native_frame_from_bounds(&bounds, 600.0, 1.0);
            let (x2, y2, w2, h2) =
                LayoutBridge::taffy_to_native_frame_from_bounds(&bounds, 600.0, 2.0);
            let (x3, y3, w3, h3) =
                LayoutBridge::taffy_to_native_frame_from_bounds(&bounds, 600.0, 3.0);

            // All should be the same regardless of scale factor
            assert_eq!(x1, x2, "X should be same for different scale factors");
            assert_eq!(x2, x3, "X should be same for different scale factors");
            assert_eq!(y1, y2, "Y should be same for different scale factors");
            assert_eq!(y2, y3, "Y should be same for different scale factors");
            assert_eq!(w1, w2, "Width should be same for different scale factors");
            assert_eq!(w2, w3, "Width should be same for different scale factors");
            assert_eq!(h1, h2, "Height should be same for different scale factors");
            assert_eq!(h2, h3, "Height should be same for different scale factors");
        }
    }

    #[test]
    fn test_coordinate_conversion_different_window_heights() {
        // Test: Coordinate conversion with different window heights
        #[cfg(target_os = "macos")]
        {
            let bounds = Rect::new(50.0, 100.0, 200.0, 30.0);

            // Window height 400
            let (x1, y1, _, _) =
                LayoutBridge::taffy_to_native_frame_from_bounds(&bounds, 400.0, 2.0);
            assert_eq!(
                y1, 270.0,
                "Y should be 270.0 for height 400 (400 - 100 - 30)"
            );

            // Window height 800
            let (x2, y2, _, _) =
                LayoutBridge::taffy_to_native_frame_from_bounds(&bounds, 800.0, 2.0);
            assert_eq!(
                y2, 670.0,
                "Y should be 670.0 for height 800 (800 - 100 - 30)"
            );

            // X should be the same
            assert_eq!(x1, x2, "X should be same regardless of window height");
        }
    }

    #[test]
    fn test_coordinate_conversion_edge_cases() {
        // Test: Various edge cases
        #[cfg(target_os = "macos")]
        {
            // Widget at very top (y=0)
            let (_, y1, _, _) = LayoutBridge::convert_coords(0.0, 0.0, 100.0, 50.0, 600.0, 2.0);
            assert_eq!(y1, 550.0, "Widget at top should have Y=550");

            // Widget at very bottom (y=550, height=50)
            let (_, y2, _, _) = LayoutBridge::convert_coords(0.0, 550.0, 100.0, 50.0, 600.0, 2.0);
            assert_eq!(y2, 0.0, "Widget at bottom should have Y=0");

            // Widget exactly filling window (y=0, height=600)
            let (_, y3, _, _) = LayoutBridge::convert_coords(0.0, 0.0, 100.0, 600.0, 600.0, 2.0);
            assert_eq!(y3, 0.0, "Widget filling window should have Y=0");
        }
    }

    #[test]
    fn test_native_to_taffy_point_conversion() {
        // Test: Convert native point back to taffy point
        #[cfg(target_os = "macos")]
        {
            // Native point at bottom-left (x=50, y=0 in macOS coordinates)
            // Should convert to taffy point at bottom (y=600 in taffy)
            // Formula: y_taffy = parent_height - y_macos = 600 - 0 = 600
            let point = LayoutBridge::native_to_taffy_point(50.0, 0.0, 600.0, 2.0);
            assert_eq!(
                point.x, 50.0,
                "X should be 50.0 (no scale factor in logical pixels)"
            );
            assert_eq!(point.y, 600.0, "Y should be 600.0 (at bottom in taffy)");

            // Native point at top-left (x=50, y=550 in macOS coordinates)
            // In taffy, this should be near the top: y_taffy = 600 - 550 = 50
            let point2 = LayoutBridge::native_to_taffy_point(50.0, 550.0, 600.0, 2.0);
            assert_eq!(point2.x, 50.0, "X should be 50.0");
            assert_eq!(point2.y, 50.0, "Y should be 50.0 (near top in taffy)");

            // Native point at middle (x=50, y=300 in macOS coordinates)
            // In taffy, this should be at middle: y_taffy = 600 - 300 = 300
            let point3 = LayoutBridge::native_to_taffy_point(50.0, 300.0, 600.0, 2.0);
            assert_eq!(point3.y, 300.0, "Y should be 300.0 (at middle in taffy)");
        }
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test: Converting taffy -> native -> taffy should give original (approximately)
        #[cfg(target_os = "macos")]
        {
            let original_bounds = Rect::new(100.0, 200.0, 150.0, 40.0);
            let (x, y, w, h) =
                LayoutBridge::taffy_to_native_frame_from_bounds(&original_bounds, 600.0, 2.0);

            // Convert back: native point at center of widget
            // The center in native coordinates
            let center_x = x + w / 2.0;
            let center_y = y + h / 2.0;

            // Convert the center point back to taffy coordinates
            let taffy_point = LayoutBridge::native_to_taffy_point(center_x, center_y, 600.0, 2.0);

            // Check that the center point is approximately correct
            let expected_center_x = original_bounds.x + original_bounds.width / 2.0;
            let expected_center_y = original_bounds.y + original_bounds.height / 2.0;

            assert!(
                (taffy_point.x - expected_center_x).abs() < 0.1,
                "Round-trip X should be close to original: got {}, expected {}",
                taffy_point.x,
                expected_center_x
            );
            assert!(
                (taffy_point.y - expected_center_y).abs() < 0.1,
                "Round-trip Y should be close to original: got {}, expected {}",
                taffy_point.y,
                expected_center_y
            );
        }
    }

    #[test]
    fn test_multiple_widgets_same_container() {
        // Test: Multiple widgets in the same container should have correct relative positions
        #[cfg(target_os = "macos")]
        {
            let parent_height = 600.0;

            // Widget 1: Top widget
            let bounds1 = Rect::new(50.0, 0.0, 200.0, 30.0);
            let (x1, y1, w1, h1) =
                LayoutBridge::taffy_to_native_frame_from_bounds(&bounds1, parent_height, 2.0);

            // Widget 2: Middle widget (below widget 1)
            let bounds2 = Rect::new(50.0, 30.0, 200.0, 30.0);
            let (x2, y2, w2, h2) =
                LayoutBridge::taffy_to_native_frame_from_bounds(&bounds2, parent_height, 2.0);

            // Widget 3: Bottom widget (below widget 2)
            let bounds3 = Rect::new(50.0, 60.0, 200.0, 30.0);
            let (x3, y3, w3, h3) =
                LayoutBridge::taffy_to_native_frame_from_bounds(&bounds3, parent_height, 2.0);

            // All should have same X
            assert_eq!(x1, x2, "Widgets should have same X");
            assert_eq!(x2, x3, "Widgets should have same X");

            // Y positions should be in reverse order (macOS bottom-left origin)
            // Widget 1 (top in taffy) should have highest Y in macOS
            // Widget 3 (bottom in taffy) should have lowest Y in macOS
            assert!(
                y1 > y2,
                "Widget 1 should be above widget 2 in macOS coordinates"
            );
            assert!(
                y2 > y3,
                "Widget 2 should be above widget 3 in macOS coordinates"
            );

            // Verify the spacing is correct
            assert_eq!(y1 - y2, 30.0, "Spacing between widget 1 and 2 should be 30");
            assert_eq!(y2 - y3, 30.0, "Spacing between widget 2 and 3 should be 30");
        }
    }

    #[test]
    fn test_very_large_coordinates() {
        // Test: Handling very large coordinates
        #[cfg(target_os = "macos")]
        {
            let bounds = Rect::new(10000.0, 50000.0, 1000.0, 500.0);
            let (x, y, w, h) = LayoutBridge::taffy_to_native_frame_from_bounds(&bounds, 600.0, 2.0);

            // Y should be clamped to 0 (would be negative)
            assert_eq!(x, 10000.0, "X should preserve large value");
            assert_eq!(y, 0.0, "Y should be clamped to 0 for out-of-bounds");
            assert_eq!(w, 1000.0, "Width should be preserved");
            assert_eq!(h, 500.0, "Height should be preserved");
        }
    }

    #[test]
    fn test_fractional_coordinates() {
        // Test: Handling fractional coordinates (sub-pixel positioning)
        #[cfg(target_os = "macos")]
        {
            let bounds = Rect::new(100.5, 200.25, 150.75, 40.5);
            let (x, y, w, h) = LayoutBridge::taffy_to_native_frame_from_bounds(&bounds, 600.0, 2.0);

            assert_eq!(x, 100.5, "X should preserve fractional value");
            // Y: 600 - 200.25 - 40.5 = 359.25
            assert!(
                (y - 359.25).abs() < 0.01,
                "Y should be correctly calculated with fractions"
            );
            assert_eq!(w, 150.75, "Width should preserve fractional value");
            assert_eq!(h, 40.5, "Height should preserve fractional value");
        }
    }
}
