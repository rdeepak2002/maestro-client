use std::ffi::CString;
use std::os::raw::c_char;

/// Macro to generate constructor FFI functions
#[macro_export]
macro_rules! export_constructor {
    ($type:ty, $func_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_name() -> *mut $type {
            Box::into_raw(Box::new(<$type>::default()))
        }
    };
    ($type:ty, $func_name:ident, $constructor:expr) => {
        #[no_mangle]
        pub extern "C" fn $func_name() -> *mut $type {
            Box::into_raw(Box::new($constructor))
        }
    };
}

/// Macro to generate destructor FFI functions
#[macro_export]
macro_rules! export_destructor {
    ($type:ty, $func_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_name(obj: *mut $type) {
            unsafe {
                if !obj.is_null() {
                    drop(Box::from_raw(obj));
                }
            }
        }
    };
}

/// Macro to generate method FFI functions that return strings
#[macro_export]
macro_rules! export_string_getter {
    ($type:ty, $func_name:ident, $method:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_name(obj: *const $type) -> *mut c_char {
            unsafe {
                if obj.is_null() {
                    return std::ptr::null_mut();
                }
                let obj_ref = &*obj;
                match CString::new(obj_ref.$method()) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            }
        }
    };
}

/// Macro to generate method FFI functions that take a string and return self
#[macro_export]
macro_rules! export_string_setter {
    ($type:ty, $func_name:ident, $method:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_name(obj: *mut $type, value: *const c_char) -> *mut $type {
            unsafe {
                if obj.is_null() || value.is_null() {
                    return std::ptr::null_mut();
                }
                let value_str = CStr::from_ptr(value).to_string_lossy().into_owned();
                let mut obj_box = Box::from_raw(obj);
                *obj_box = obj_box.$method(value_str);
                Box::into_raw(obj_box)
            }
        }
    };
}

/// Macro to generate method FFI functions that consume self and return another type
#[macro_export]
macro_rules! export_consuming_method {
    ($from_type:ty, $to_type:ty, $func_name:ident, $method:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_name(obj: *mut $from_type) -> *mut $to_type {
            unsafe {
                if obj.is_null() {
                    return std::ptr::null_mut();
                }
                let obj_box = Box::from_raw(obj);
                Box::into_raw(Box::new(obj_box.$method()))
            }
        }
    };
}

/// Utility function to free strings returned by FFI
#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            drop(CString::from_raw(s));
        }
    }
}

// pub use export_constructor;
// pub use export_consuming_method;
// pub use export_destructor;
// pub use export_string_getter;
// pub use export_string_setter;
