use pyo3::prelude::*;
use schedulerclient::{
    workflow::Workflow as RustWorkflow, workflow::WorkflowBuilder as RustWorkflowBuilder,
};

#[pyclass]
struct Workflow {
    inner: RustWorkflow,
}

#[pymethods]
impl Workflow {
    #[staticmethod]
    fn builder() -> WorkflowBuilder {
        WorkflowBuilder {
            inner: RustWorkflow::builder(),
        }
    }

    #[getter]
    fn id(&self) -> String {
        self.inner.id().to_string()
    }
}

#[pyclass]
struct WorkflowBuilder {
    inner: RustWorkflowBuilder,
}

#[pymethods]
impl WorkflowBuilder {
    fn id(mut self_: PyRefMut<Self>, id: String) -> PyRefMut<Self> {
        self_.inner = std::mem::take(&mut self_.inner).id(id);
        self_
    }

    fn build(mut self_: PyRefMut<Self>) -> Workflow {
        let builder = std::mem::replace(
            &mut *self_,
            WorkflowBuilder {
                inner: RustWorkflowBuilder::default(),
            },
        );
        Workflow {
            inner: builder.inner.build(),
        }
    }
}

#[pymodule]
mod schedulerclient_py {
    #[pymodule_export]
    use super::{Workflow, WorkflowBuilder};
}
