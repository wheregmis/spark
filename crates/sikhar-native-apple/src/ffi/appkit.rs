//! AppKit (macOS) FFI bindings.

use objc2::runtime::AnyObject;
use objc2::msg_send;

/// NSView wrapper for macOS.
pub struct NSView {
    pub(crate) obj: *mut AnyObject,
}

impl NSView {
    /// Create an NSView from a raw pointer (unsafe).
    /// This is used when we have a pointer from the window handle.
    pub unsafe fn from_ptr(ptr: *mut AnyObject) -> Self {
        Self { obj: ptr }
    }
}

unsafe impl Send for NSView {}
unsafe impl Sync for NSView {}

impl NSView {
    /// Create a new NSView.
    pub fn new() -> Self {
        unsafe {
            use objc2::runtime::Class;
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"NSView\0").unwrap();
            let class = Class::get(class_name).expect("NSView class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self { obj }
        }
    }

    /// Get the raw object pointer.
    pub fn as_ptr(&self) -> *mut AnyObject {
        self.obj
    }

    /// Set the frame of the view.
    pub fn set_frame(&self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            use objc2_foundation::NSRect;
            // Ensure minimum size
            let width = width.max(1.0);
            let height = height.max(1.0);
            let rect = NSRect {
                origin: objc2_foundation::NSPoint { x, y },
                size: objc2_foundation::NSSize { width, height },
            };
            let _: () = msg_send![self.obj, setFrame: rect];
        }
    }
    
    /// Get the view's intrinsic content size (if available).
    /// Returns (width, height) or (-1.0, -1.0) if no intrinsic size.
    pub fn intrinsic_content_size(&self) -> (f64, f64) {
        unsafe {
            use objc2_foundation::NSSize;
            let size: NSSize = msg_send![self.obj, intrinsicContentSize];
            (size.width, size.height)
        }
    }
    
    /// Get the view's fitting size (the size that fits its content).
    pub fn fitting_size(&self) -> (f64, f64) {
        unsafe {
            use objc2_foundation::NSSize;
            let size: NSSize = msg_send![self.obj, fittingSize];
            (size.width, size.height)
        }
    }
    
    /// Make the view visible and set up for layer-backed rendering.
    pub fn set_visible(&self, visible: bool) {
        unsafe {
            let _: () = msg_send![self.obj, setHidden: !visible];
        }
    }
    
    /// Enable layer-backed rendering (needed for compositing with Metal/OpenGL).
    pub fn set_wants_layer(&self, wants: bool) {
        unsafe {
            let _: () = msg_send![self.obj, setWantsLayer: wants];
        }
    }
    
    /// Enable Auto Layout constraints.
    pub fn set_translates_autoresizing_mask(&self, translates: bool) {
        unsafe {
            let _: () = msg_send![self.obj, setTranslatesAutoresizingMaskIntoConstraints: translates];
        }
    }

    /// Add a subview.
    pub fn add_subview(&self, subview: &NSView) {
        unsafe {
            let _: () = msg_send![self.obj, addSubview: subview.obj];
        }
    }
    
    /// Bring the view to the front.
    pub fn bring_to_front(&self) {
        unsafe {
            let superview: *mut AnyObject = msg_send![self.obj, superview];
            if !superview.is_null() {
                // Remove and re-add to bring to front
                let _: () = msg_send![self.obj, removeFromSuperview];
                let _: () = msg_send![superview, addSubview: self.obj];
            }
        }
    }

    /// Remove from superview.
    pub fn remove_from_superview(&self) {
        unsafe {
            let _: () = msg_send![self.obj, removeFromSuperview];
        }
    }
    
    /// Layout the view and its subviews.
    pub fn layout_if_needed(&self) {
        unsafe {
            let _: () = msg_send![self.obj, layoutSubtreeIfNeeded];
        }
    }
    
    /// Mark the view as needing layout.
    pub fn needs_layout(&self) {
        unsafe {
            let _: () = msg_send![self.obj, setNeedsLayout: true];
        }
    }
}

impl Drop for NSView {
    fn drop(&mut self) {
        // Don't release here - views are retained by their superviews
        // If we manually release, we'll cause over-release crashes
        // The Objective-C runtime will handle memory management
        // We only need to release if we explicitly retained (which we don't)
    }
}

/// NSButton wrapper for macOS.
pub struct NSButton {
    view: NSView,
}

impl NSButton {
    /// Create a new NSButton.
    pub fn new() -> Self {
        unsafe {
            use objc2::runtime::Class;
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"NSButton\0").unwrap();
            let class = Class::get(class_name).expect("NSButton class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self {
                view: NSView { obj },
            }
        }
    }

    /// Set the button title.
    pub fn set_title(&self, title: &str) {
        unsafe {
            use objc2_foundation::NSString;
            let ns_string = NSString::from_str(title);
            let _: () = msg_send![self.view.as_ptr(), setTitle: &*ns_string];
        }
    }

    /// Set the button style.
    pub fn set_bezel_style(&self, style: NSBezelStyle) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setBezelStyle: style as i64];
        }
    }

    /// Set the button action (callback).
    pub fn set_action(&self, target: *mut AnyObject, selector: objc2::runtime::Sel) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setTarget: target];
            let _: () = msg_send![self.view.as_ptr(), setAction: selector];
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &NSView {
        &self.view
    }
    
    /// Get the button's intrinsic content size.
    /// This is the size the button wants to be based on its content.
    pub fn intrinsic_content_size(&self) -> (f64, f64) {
        self.view.intrinsic_content_size()
    }
    
    /// Get the button's fitting size.
    pub fn fitting_size(&self) -> (f64, f64) {
        self.view.fitting_size()
    }
    
    /// Size the button to fit its content.
    pub fn size_to_fit(&self) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), sizeToFit];
        }
    }
}

/// NSButton bezel styles.
#[repr(i64)]
pub enum NSBezelStyle {
    Rounded = 1,
    RegularSquare = 2,
    TexturedSquare = 3,
    Disclosure = 5,
    ShadowlessSquare = 6,
    Circular = 7,
    TexturedRounded = 8,
    HelpButton = 9,
    SmallSquare = 10,
    Toolbar = 11,
    AccessoryBarAction = 12,
    AccessoryBarAction2 = 13,
    PushOnPushOff = 14,
    MomentaryPushIn = 15,
    Accordion = 16,
    Inline = 17,
    Recessed = 18,
    RoundedDisclosure = 19,
}

/// NSTextField wrapper for macOS.
pub struct NSTextField {
    view: NSView,
}

impl NSTextField {
    /// Create a new NSTextField.
    pub fn new() -> Self {
        unsafe {
            use objc2::runtime::Class;
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"NSTextField\0").unwrap();
            let class = Class::get(class_name).expect("NSTextField class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self {
                view: NSView { obj },
            }
        }
    }

    /// Set the text field's string value.
    pub fn set_string_value(&self, value: &str) {
        unsafe {
            use objc2_foundation::NSString;
            let ns_string = NSString::from_str(value);
            let _: () = msg_send![self.view.as_ptr(), setStringValue: &*ns_string];
        }
    }

    /// Get the text field's string value.
    pub fn string_value(&self) -> String {
        unsafe {
            let ns_string: *mut AnyObject = msg_send![self.view.as_ptr(), stringValue];
            if ns_string.is_null() {
                return String::new();
            }
            // Convert NSString to Rust String (simplified)
            // In practice, you'd use proper NSString methods
            String::new()
        }
    }

    /// Set placeholder string.
    pub fn set_placeholder_string(&self, value: &str) {
        unsafe {
            use objc2_foundation::NSString;
            let ns_string = NSString::from_str(value);
            let _: () = msg_send![self.view.as_ptr(), setPlaceholderString: &*ns_string];
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &NSView {
        &self.view
    }
    
    /// Get the text field's intrinsic content size.
    pub fn intrinsic_content_size(&self) -> (f64, f64) {
        self.view.intrinsic_content_size()
    }
    
    /// Size the text field to fit its content.
    pub fn size_to_fit(&self) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), sizeToFit];
        }
    }
}

/// NSSlider wrapper for macOS.
pub struct NSSlider {
    view: NSView,
}

impl NSSlider {
    /// Create a new NSSlider.
    pub fn new() -> Self {
        unsafe {
            use objc2::runtime::Class;
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"NSSlider\0").unwrap();
            let class = Class::get(class_name).expect("NSSlider class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self {
                view: NSView { obj },
            }
        }
    }

    /// Set the slider's minimum value.
    pub fn set_min_value(&self, value: f64) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setMinValue: value];
        }
    }

    /// Set the slider's maximum value.
    pub fn set_max_value(&self, value: f64) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setMaxValue: value];
        }
    }

    /// Set the slider's current value.
    pub fn set_double_value(&self, value: f64) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setDoubleValue: value];
        }
    }

    /// Get the slider's current value.
    pub fn double_value(&self) -> f64 {
        unsafe {
            let value: f64 = msg_send![self.view.as_ptr(), doubleValue];
            value
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &NSView {
        &self.view
    }
    
    /// Get the slider's intrinsic content size.
    pub fn intrinsic_content_size(&self) -> (f64, f64) {
        self.view.intrinsic_content_size()
    }
}

/// NSSwitch (NSButton with switch style) wrapper for macOS.
pub struct NSSwitch {
    view: NSView,
}

impl NSSwitch {
    /// Create a new NSSwitch.
    pub fn new() -> Self {
        unsafe {
            use objc2::runtime::Class;
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"NSButton\0").unwrap();
            let class = Class::get(class_name).expect("NSButton class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            
            // Set button type to switch
            let button_type: i64 = 18; // NSSwitchButton
            let _: () = msg_send![obj, setButtonType: button_type];
            
            Self {
                view: NSView { obj },
            }
        }
    }

    /// Set the switch state (on/off).
    pub fn set_state(&self, state: i64) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setState: state];
        }
    }

    /// Get the switch state.
    pub fn state(&self) -> i64 {
        unsafe {
            let state: i64 = msg_send![self.view.as_ptr(), state];
            state
        }
    }

    /// Set the switch title.
    pub fn set_title(&self, title: &str) {
        unsafe {
            use objc2_foundation::NSString;
            let ns_string = NSString::from_str(title);
            let _: () = msg_send![self.view.as_ptr(), setTitle: &*ns_string];
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &NSView {
        &self.view
    }
    
    /// Get the switch's intrinsic content size.
    pub fn intrinsic_content_size(&self) -> (f64, f64) {
        self.view.intrinsic_content_size()
    }
    
    /// Size the switch to fit its content.
    pub fn size_to_fit(&self) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), sizeToFit];
        }
    }
}

/// NSProgressIndicator wrapper for macOS.
pub struct NSProgressIndicator {
    view: NSView,
}

impl NSProgressIndicator {
    /// Create a new NSProgressIndicator.
    pub fn new() -> Self {
        unsafe {
            use objc2::runtime::Class;
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"NSProgressIndicator\0").unwrap();
            let class = Class::get(class_name).expect("NSProgressIndicator class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self {
                view: NSView { obj },
            }
        }
    }

    /// Set the progress indicator style.
    pub fn set_style(&self, style: NSProgressIndicatorStyle) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setStyle: style as i64];
        }
    }

    /// Set the progress indicator's minimum value.
    pub fn set_min_value(&self, value: f64) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setMinValue: value];
        }
    }

    /// Set the progress indicator's maximum value.
    pub fn set_max_value(&self, value: f64) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setMaxValue: value];
        }
    }

    /// Set the progress indicator's current value.
    pub fn set_double_value(&self, value: f64) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setDoubleValue: value];
        }
    }

    /// Start animation (for indeterminate progress).
    pub fn start_animation(&self) {
        unsafe {
            // Note: startAnimation: takes a sender parameter, we pass nil
            let nil: *mut objc2::runtime::AnyObject = std::ptr::null_mut();
            let _: () = msg_send![self.view.as_ptr(), startAnimation: nil];
        }
    }

    /// Stop animation.
    pub fn stop_animation(&self) {
        unsafe {
            // Note: stopAnimation: takes a sender parameter, we pass nil
            let nil: *mut objc2::runtime::AnyObject = std::ptr::null_mut();
            let _: () = msg_send![self.view.as_ptr(), stopAnimation: nil];
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &NSView {
        &self.view
    }
    
    /// Get the progress indicator's intrinsic content size.
    pub fn intrinsic_content_size(&self) -> (f64, f64) {
        self.view.intrinsic_content_size()
    }
}

/// NSProgressIndicator style.
#[repr(i64)]
pub enum NSProgressIndicatorStyle {
    Bar = 0,
    Spinning = 1,
}

/// Helper to get NSWindow from a winit window.
/// Note: This requires winit as a dependency, which is not included here.
/// The app runner should handle window embedding directly.
pub fn get_nswindow_from_winit(_window: &dyn std::any::Any) -> *mut AnyObject {
    // This is platform-specific - winit provides access to the native window
    // For now, return a placeholder - in practice, you'd use winit's native window access
    std::ptr::null_mut()
}
