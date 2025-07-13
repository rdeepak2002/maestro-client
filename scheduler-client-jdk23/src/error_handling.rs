use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Result type for FFI operations
#[repr(C)]
pub struct FFIResult<T> {
    pub success: bool,
    pub value: T,
    pub error_message: *mut c_char,
}

impl<T: Default> FFIResult<T> {
    pub fn success(value: T) -> Self {
        FFIResult {
            success: true,
            value,
            error_message: std::ptr::null_mut(),
        }
    }

    pub fn error(message: &str) -> Self {
        let c_message = CString::new(message)
            .unwrap_or_else(|_| CString::new("Invalid error message").unwrap())
            .into_raw();
        
        FFIResult {
            success: false,
            value: T::default(),
            error_message: c_message,
        }
    }
}

/// Macro to generate error-aware FFI functions
macro_rules! export_safe_constructor {
    ($type:ty, $func_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_name() -> FFIResult<*mut $type> {
            match std::panic::catch_unwind(|| {
                Box::into_raw(Box::new(<$type>::default()))
            }) {
                Ok(ptr) => FFIResult::success(ptr),
                Err(_) => FFIResult::error("Failed to create object"),
            }
        }
    };
}

/// Macro to generate error-aware string getter functions
macro_rules! export_safe_string_getter {
    ($type:ty, $func_name:ident, $method:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_name(obj: *const $type) -> FFIResult<*mut c_char> {
            if obj.is_null() {
                return FFIResult::error("Null pointer provided");
            }

            match std::panic::catch_unwind(|| unsafe {
                let obj_ref = &*obj;
                CString::new(obj_ref.$method())
                    .map(|s| s.into_raw())
                    .unwrap_or(std::ptr::null_mut())
            }) {
                Ok(ptr) => {
                    if ptr.is_null() {
                        FFIResult::error("Failed to convert string")
                    } else {
                        FFIResult::success(ptr)
                    }
                },
                Err(_) => FFIResult::error("Panic occurred while getting string"),
            }
        }
    };
}

/// Free the error message from an FFIResult
#[no_mangle]
pub extern "C" fn free_error_message(error_msg: *mut c_char) {
    unsafe {
        if !error_msg.is_null() {
            drop(CString::from_raw(error_msg));
        }
    }
}

pub use export_safe_constructor;
pub use export_safe_string_getter;