extern crate schedulerclient;

use schedulerclient::{
    workflow::Workflow as RustWorkflow, workflow::WorkflowBuilder as RustWorkflowBuilder,
};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn workflow_builder_new() -> *mut RustWorkflowBuilder {
    Box::into_raw(Box::new(RustWorkflowBuilder::default()))
}

#[no_mangle]
pub extern "C" fn workflow_builder_set_id(
    builder: *mut RustWorkflowBuilder,
    id: *const c_char,
) -> *mut RustWorkflowBuilder {
    unsafe {
        if builder.is_null() || id.is_null() {
            return std::ptr::null_mut();
        }
        let id_str = CStr::from_ptr(id).to_string_lossy().into_owned();
        let mut builder_box = Box::from_raw(builder);
        *builder_box = builder_box.id(id_str);
        Box::into_raw(builder_box)
    }
}

#[no_mangle]
pub extern "C" fn workflow_builder_build(builder: *mut RustWorkflowBuilder) -> *mut RustWorkflow {
    unsafe {
        if builder.is_null() {
            return std::ptr::null_mut();
        }
        let builder_box = Box::from_raw(builder);
        Box::into_raw(Box::new(builder_box.build()))
    }
}

#[no_mangle]
pub extern "C" fn workflow_get_id(workflow: *const RustWorkflow) -> *mut c_char {
    unsafe {
        if workflow.is_null() {
            return std::ptr::null_mut();
        }
        let workflow_ref = &*workflow;
        match CString::new(workflow_ref.id()) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn workflow_free(workflow: *mut RustWorkflow) {
    unsafe {
        if !workflow.is_null() {
            drop(Box::from_raw(workflow));
        }
    }
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            drop(CString::from_raw(s));
        }
    }
}
