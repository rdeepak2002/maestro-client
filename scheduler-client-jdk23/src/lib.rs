extern crate schedulerclient;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
mod ffi_macros;
use schedulerclient::{
    workflow::Workflow as RustWorkflow, workflow::WorkflowBuilder as RustWorkflowBuilder,
};

// WorkflowBuilder FFI exports using macros
export_constructor!(RustWorkflowBuilder, workflow_builder_new);
export_string_setter!(RustWorkflowBuilder, workflow_builder_set_id, id);
export_consuming_method!(
    RustWorkflowBuilder,
    RustWorkflow,
    workflow_builder_build,
    build
);
export_destructor!(RustWorkflowBuilder, workflow_builder_free);

// Workflow FFI exports using macros
export_string_getter!(RustWorkflow, workflow_get_id, id);
export_destructor!(RustWorkflow, workflow_free);
