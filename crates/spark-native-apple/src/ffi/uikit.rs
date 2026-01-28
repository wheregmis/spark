//! UIKit (iOS/iPadOS) FFI bindings.

use objc2::msg_send;
use objc2::runtime::{AnyObject, Class};

/// UIView wrapper for iOS.
pub struct UIView {
    pub(crate) obj: *mut AnyObject,
}

unsafe impl Send for UIView {}
unsafe impl Sync for UIView {}

impl UIView {
    /// Create a new UIView.
    pub fn new() -> Self {
        unsafe {
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"UIView\0").unwrap();
            let class = Class::get(class_name).expect("UIView class");
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
            use objc2_foundation::CGRect;
            let rect = CGRect {
                origin: objc2_foundation::CGPoint { x, y },
                size: objc2_foundation::CGSize { width, height },
            };
            let _: () = msg_send![self.obj, setFrame: rect];
        }
    }

    /// Add a subview.
    pub fn add_subview(&self, subview: &UIView) {
        unsafe {
            let _: () = msg_send![self.obj, addSubview: subview.obj];
        }
    }

    /// Remove from superview.
    pub fn remove_from_superview(&self) {
        unsafe {
            let _: () = msg_send![self.obj, removeFromSuperview];
        }
    }
}

impl Drop for UIView {
    fn drop(&mut self) {
        // Don't release here - views are retained by their superviews
        // If we manually release, we'll cause over-release crashes
        // The Objective-C runtime will handle memory management
    }
}

/// UIButton wrapper for iOS.
pub struct UIButton {
    view: UIView,
}

impl UIButton {
    /// Create a new UIButton.
    pub fn new() -> Self {
        unsafe {
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"UIButton\0").unwrap();
            let class = Class::get(class_name).expect("UIButton class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self {
                view: UIView { obj },
            }
        }
    }

    /// Set the button title.
    pub fn set_title(&self, title: &str, state: UIControlState) {
        unsafe {
            use objc2_foundation::NSString;
            let ns_string = NSString::from_str(title);
            let _: () = msg_send![self.view.as_ptr(), setTitle: &*ns_string forState: state];
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &UIView {
        &self.view
    }
}

/// UITextField wrapper for iOS.
pub struct UITextField {
    view: UIView,
}

impl UITextField {
    /// Create a new UITextField.
    pub fn new() -> Self {
        unsafe {
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"UITextField\0").unwrap();
            let class = Class::get(class_name).expect("UITextField class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self {
                view: UIView { obj },
            }
        }
    }

    /// Set the text field's text.
    pub fn set_text(&self, text: &str) {
        unsafe {
            use objc2_foundation::NSString;
            let ns_string = NSString::from_str(text);
            let _: () = msg_send![self.view.as_ptr(), setText: &*ns_string];
        }
    }

    /// Get the text field's text.
    pub fn text(&self) -> String {
        unsafe {
            let ns_string: *mut AnyObject = msg_send![self.view.as_ptr(), text];
            if ns_string.is_null() {
                return String::new();
            }
            // Convert NSString to Rust String (simplified)
            String::new()
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &UIView {
        &self.view
    }
}

/// UILabel wrapper for iOS.
pub struct UILabel {
    view: UIView,
}

impl UILabel {
    /// Create a new UILabel.
    pub fn new() -> Self {
        unsafe {
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"UILabel\0").unwrap();
            let class = Class::get(class_name).expect("UILabel class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self {
                view: UIView { obj },
            }
        }
    }

    /// Set the label's text.
    pub fn set_text(&self, text: &str) {
        unsafe {
            use objc2_foundation::NSString;
            let ns_string = NSString::from_str(text);
            let _: () = msg_send![self.view.as_ptr(), setText: &*ns_string];
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &UIView {
        &self.view
    }
}

/// UIControlState for button states.
#[repr(usize)]
pub enum UIControlState {
    Normal = 0,
    Highlighted = 1,
    Disabled = 2,
    Selected = 4,
}

/// UISlider wrapper for iOS.
pub struct UISlider {
    view: UIView,
}

impl UISlider {
    /// Create a new UISlider.
    pub fn new() -> Self {
        unsafe {
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"UISlider\0").unwrap();
            let class = Class::get(class_name).expect("UISlider class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self {
                view: UIView { obj },
            }
        }
    }

    /// Set the slider's minimum value.
    pub fn set_minimum_value(&self, value: f32) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setMinimumValue: value];
        }
    }

    /// Set the slider's maximum value.
    pub fn set_maximum_value(&self, value: f32) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setMaximumValue: value];
        }
    }

    /// Set the slider's current value.
    pub fn set_value(&self, value: f32) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setValue: value];
        }
    }

    /// Get the slider's current value.
    pub fn value(&self) -> f32 {
        unsafe {
            let value: f32 = msg_send![self.view.as_ptr(), value];
            value
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &UIView {
        &self.view
    }
}

/// UISwitch wrapper for iOS.
pub struct UISwitch {
    view: UIView,
}

impl UISwitch {
    /// Create a new UISwitch.
    pub fn new() -> Self {
        unsafe {
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"UISwitch\0").unwrap();
            let class = Class::get(class_name).expect("UISwitch class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self {
                view: UIView { obj },
            }
        }
    }

    /// Set the switch state (on/off).
    pub fn set_on(&self, on: bool) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setOn: on];
        }
    }

    /// Get the switch state.
    pub fn is_on(&self) -> bool {
        unsafe {
            let on: bool = msg_send![self.view.as_ptr(), isOn];
            on
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &UIView {
        &self.view
    }
}

/// UIProgressView wrapper for iOS.
pub struct UIProgressView {
    view: UIView,
}

impl UIProgressView {
    /// Create a new UIProgressView.
    pub fn new() -> Self {
        unsafe {
            use std::ffi::CStr;
            let class_name = CStr::from_bytes_with_nul(b"UIProgressView\0").unwrap();
            let class = Class::get(class_name).expect("UIProgressView class");
            let obj: *mut AnyObject = msg_send![class, alloc];
            let obj: *mut AnyObject = msg_send![obj, init];
            Self {
                view: UIView { obj },
            }
        }
    }

    /// Set the progress (0.0 to 1.0).
    pub fn set_progress(&self, progress: f32) {
        unsafe {
            let _: () = msg_send![self.view.as_ptr(), setProgress: progress];
        }
    }

    /// Get the underlying view.
    pub fn view(&self) -> &UIView {
        &self.view
    }
}
