mod ffi_macros;
use pyo3::prelude::*;
use schedulerclient::{
    workflow::Workflow as RustWorkflow, workflow::WorkflowBuilder as RustWorkflowBuilder,
};

py_wrapper!(Workflow, RustWorkflow, {
    getters: [id],
    setters: [],
    consumers: [],
    statics: [(builder, WorkflowBuilder)]
});

py_wrapper!(WorkflowBuilder, RustWorkflowBuilder, {
    getters: [],
    setters: [id],
    consumers: [(build, Workflow)],
    statics: []
});

py_exports!(schedulerclient_py: Workflow, WorkflowBuilder);
