//! View manager - manages native view hierarchy and lifecycle.

use spark_layout::WidgetId;
use std::collections::HashMap;
use crate::NativeViewHandle;

/// Manages the native view hierarchy and maps widget IDs to native views.
pub struct ViewManager {
    /// Map from widget ID to native view handle.
    views: HashMap<WidgetId, NativeViewHandle>,
    /// Map from widget ID to parent widget ID.
    parent_map: HashMap<WidgetId, WidgetId>,
    /// Root view handle (NSView/UIView that contains all native widgets).
    root_view: Option<NativeViewHandle>,
}

impl ViewManager {
    /// Create a new view manager.
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
            parent_map: HashMap::new(),
            root_view: None,
        }
    }

    /// Register a native widget with its view handle.
    pub fn register_widget(&mut self, widget_id: WidgetId, view_handle: NativeViewHandle) {
        // Make the view visible and set up for rendering
        #[cfg(target_os = "macos")]
        {
            let crate::NativeViewHandle::AppKit(ptr) = &view_handle;
            use crate::ffi::appkit::NSView;
            // NSView wrapper creation is safe as it just wraps the pointer
            let view = NSView { obj: *ptr };
            view.set_visible(true);
            view.set_wants_layer(true);
            // Set a background color temporarily for debugging
            // view.set_background_color(1.0, 0.0, 0.0, 0.5); // Red with transparency
        }
        
        self.views.insert(widget_id, view_handle);
        
        // If we have a root view, add this view to it
        if let Some(root) = &self.root_view {
            self.add_to_parent_internal(widget_id, root);
        }
    }
    
    /// Internal helper to add a view to a parent.
    fn add_to_parent_internal(&self, widget_id: WidgetId, parent_handle: &NativeViewHandle) {
        if let Some(child_handle) = self.views.get(&widget_id) {
            match (child_handle, parent_handle) {
                #[cfg(target_os = "macos")]
                (NativeViewHandle::AppKit(child_ptr), NativeViewHandle::AppKit(parent_ptr)) => {
                    use crate::ffi::appkit::NSView;
                    let child = NSView { obj: *child_ptr };
                    let parent = NSView { obj: *parent_ptr };
                    parent.add_subview(&child);
                }
                #[cfg(target_os = "ios")]
                (NativeViewHandle::UIKit(child_ptr), NativeViewHandle::UIKit(parent_ptr)) => {
                    unsafe {
                        use crate::ffi::uikit::UIView;
                        let child = UIView { obj: *child_ptr };
                        let parent = UIView { obj: *parent_ptr };
                        parent.add_subview(&child);
                    }
                }
            }
        }
    }
    
    /// Get all registered view handles (for embedding into window).
    pub fn get_all_views(&self) -> &HashMap<WidgetId, NativeViewHandle> {
        &self.views
    }

    /// Unregister a native widget.
    pub fn unregister_widget(&mut self, widget_id: WidgetId) {
        if let Some(view_handle) = self.views.remove(&widget_id) {
            // Remove from native view hierarchy
            match view_handle {
                #[cfg(target_os = "macos")]
                NativeViewHandle::AppKit(ptr) => {
                    use crate::ffi::appkit::NSView;
                    let view = NSView { obj: ptr };
                    view.remove_from_superview();
                }
                #[cfg(target_os = "ios")]
                NativeViewHandle::UIKit(ptr) => {
                    unsafe {
                        use crate::ffi::uikit::UIView;
                        let view = UIView { obj: ptr };
                        view.remove_from_superview();
                    }
                }
            }
        }
        self.parent_map.remove(&widget_id);
    }

    /// Set the parent of a widget.
    pub fn set_parent(&mut self, widget_id: WidgetId, parent_id: WidgetId) {
        self.parent_map.insert(widget_id, parent_id);
    }

    /// Get the view handle for a widget.
    pub fn get_view(&self, widget_id: WidgetId) -> Option<&NativeViewHandle> {
        self.views.get(&widget_id)
    }

    /// Get the view handle for a widget (mutable).
    pub fn get_view_mut(&mut self, widget_id: WidgetId) -> Option<&mut NativeViewHandle> {
        self.views.get_mut(&widget_id)
    }

    /// Set the root view (container for all native widgets).
    pub fn set_root_view(&mut self, root_view: NativeViewHandle) {
        self.root_view = Some(root_view);
    }

    /// Get the root view.
    pub fn get_root_view(&self) -> Option<&NativeViewHandle> {
        self.root_view.as_ref()
    }

    /// Add a view to its parent in the native hierarchy.
    pub fn add_to_parent(&self, widget_id: WidgetId, parent_id: WidgetId) {
        if let (Some(child_handle), Some(parent_handle)) = (
            self.views.get(&widget_id),
            self.views.get(&parent_id),
        ) {
            match (child_handle, parent_handle) {
                #[cfg(target_os = "macos")]
                (NativeViewHandle::AppKit(child_ptr), NativeViewHandle::AppKit(parent_ptr)) => {
                    use crate::ffi::appkit::NSView;
                    let child = NSView { obj: *child_ptr };
                    let parent = NSView { obj: *parent_ptr };
                    parent.add_subview(&child);
                }
                #[cfg(target_os = "ios")]
                (NativeViewHandle::UIKit(child_ptr), NativeViewHandle::UIKit(parent_ptr)) => {
                    unsafe {
                        use crate::ffi::uikit::UIView;
                        let child = UIView { obj: *child_ptr };
                        let parent = UIView { obj: *parent_ptr };
                        parent.add_subview(&child);
                    }
                }
                #[allow(unreachable_patterns)]
                _ => {
                    // Mismatched platforms - shouldn't happen
                }
            }
        } else if let Some(child_handle) = self.views.get(&widget_id) {
            // Add to root view if no parent found
            if let Some(root) = &self.root_view {
                match (child_handle, root) {
                    #[cfg(target_os = "macos")]
                    (NativeViewHandle::AppKit(child_ptr), NativeViewHandle::AppKit(root_ptr)) => {
                        use crate::ffi::appkit::NSView;
                        let child = NSView { obj: *child_ptr };
                        let root = NSView { obj: *root_ptr };
                        root.add_subview(&child);
                    }
                    #[cfg(target_os = "ios")]
                    (NativeViewHandle::UIKit(child_ptr), NativeViewHandle::UIKit(root_ptr)) => {
                        unsafe {
                            use crate::ffi::uikit::UIView;
                            let child = UIView { obj: *child_ptr };
                            let root = UIView { obj: *root_ptr };
                            root.add_subview(&child);
                        }
                    }
                    #[allow(unreachable_patterns)]
                    _ => {}
                }
            }
        }
    }

    /// Update all native view layouts based on computed layout results.
    pub fn update_layouts(
        &self,
        layouts: &HashMap<WidgetId, spark_layout::ComputedLayout>,
        parent_height: f32,
        scale_factor: f32,
    ) {
        for (widget_id, computed) in layouts {
            if let Some(view_handle) = self.views.get(widget_id) {
                // Convert ComputedLayout to the format needed for native views
                let (x, y, width, height) = crate::layout::LayoutBridge::taffy_to_native_frame_from_bounds(
                    &computed.bounds,
                    parent_height,
                    scale_factor,
                );
                
                match view_handle {
                    #[cfg(target_os = "macos")]
                    crate::NativeViewHandle::AppKit(ptr) => {
                        use crate::ffi::appkit::NSView;
                        let view = NSView { obj: *ptr };
                        // Debug: log the frame being set (only for first few to avoid spam)
                        if layouts.len() <= 3 {
                            eprintln!("Setting native view frame: x={:.1}, y={:.1}, w={:.1}, h={:.1}, parent_height={:.1}, scale={:.1}", 
                                x, y, width, height, parent_height, scale_factor);
                        }
                        view.set_frame(x, y, width, height);
                        // Ensure view is visible and bring to front
                        view.set_visible(true);
                        view.bring_to_front();
                    }
                    #[cfg(target_os = "ios")]
                    crate::NativeViewHandle::UIKit(ptr) => {
                        unsafe {
                            use crate::ffi::uikit::UIView;
                            let view = UIView { obj: *ptr };
                            view.set_frame(x, y, width, height);
                        }
                    }
                }
            }
        }
    }
}

impl Default for ViewManager {
    fn default() -> Self {
        Self::new()
    }
}

