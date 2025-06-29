use crate::api::WorkflowPushResponse;
use crate::step::Step;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(uniffi::Record, Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowParameter {
    pub expression: String,
    #[serde(rename = "type")]
    pub param_type: String,
}

#[derive(uniffi::Record, Clone, Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: Option<String>,
    pub params: HashMap<String, WorkflowParameter>,
    pub steps: Vec<Step>,
}

#[derive(uniffi::Object, Clone, Debug)]
pub struct WorkflowBuilder {
    inner: Workflow,
}

#[uniffi::export]
impl WorkflowBuilder {
    #[uniffi::constructor]
    pub fn new(id: String) -> Arc<Self> {
        Arc::new(Self {
            inner: Workflow {
                id,
                name: None,
                params: HashMap::new(),
                steps: vec![],
            },
        })
    }

    #[uniffi::method]
    pub fn add_step(&self, step: Step) -> Arc<WorkflowBuilder> {
        let mut workflow = self.inner.clone();
        workflow.steps.push(step);
        Arc::new(WorkflowBuilder { inner: workflow })
    }

    #[uniffi::method]
    pub fn add_param(
        &self,
        key: String,
        expression: String,
        param_type: String,
    ) -> Arc<WorkflowBuilder> {
        let mut workflow = self.inner.clone();
        workflow.params.insert(
            key,
            WorkflowParameter {
                expression,
                param_type,
            },
        );
        Arc::new(WorkflowBuilder { inner: workflow })
    }

    #[uniffi::method]
    pub fn set_name(&self, name: String) -> Arc<WorkflowBuilder> {
        let mut workflow = self.inner.clone();
        workflow.name = Some(name);
        Arc::new(WorkflowBuilder { inner: workflow })
    }

    #[uniffi::method]
    pub fn build(&self) -> Workflow {
        self.inner.clone()
    }

    #[uniffi::method]
    pub fn push(&self) -> WorkflowPushResponse {
        crate::api::push_workflow(self)
    }
}
