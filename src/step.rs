use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(uniffi::Record, Clone, Debug, Serialize, Deserialize)]
pub struct Transition {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub successors: HashMap<String, String>,
}

#[derive(uniffi::Record, Clone, Debug, Serialize, Deserialize)]
pub struct StepParameter {
    pub value: Option<String>,
    pub expression: Option<String>,
    #[serde(rename = "type")]
    pub param_type: String,
}

#[derive(uniffi::Record, Clone, Debug, Serialize, Deserialize)]
pub struct InnerStep {
    pub id: String,
    #[serde(rename = "type")]
    pub step_type: String,
    pub transition: Transition,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub params: HashMap<String, StepParameter>,
}

#[derive(uniffi::Record, Clone, Debug, Serialize, Deserialize)]
pub struct Step {
    pub step: InnerStep,
}

#[derive(uniffi::Object, Clone, Debug)]
pub struct StepBuilder {
    inner: Step,
}

#[uniffi::export]
impl StepBuilder {
    #[uniffi::constructor]
    pub fn new(id: String, step_type: String) -> Arc<Self> {
        Arc::new(Self {
            inner: Step {
                step: InnerStep {
                    id,
                    step_type,
                    transition: Transition {
                        successors: HashMap::new(),
                    },
                    params: HashMap::new(),
                },
            },
        })
    }

    #[uniffi::method]
    pub fn add_param_expression(
        &self,
        key: String,
        expression: String,
        param_type: String,
    ) -> Arc<StepBuilder> {
        let mut step = self.inner.clone();
        step.step.params.insert(
            key,
            StepParameter {
                value: None,
                expression: Some(expression),
                param_type,
            },
        );
        Arc::new(StepBuilder { inner: step })
    }

    #[uniffi::method]
    pub fn add_param_value(
        &self,
        key: String,
        value: String,
        param_type: String,
    ) -> Arc<StepBuilder> {
        let mut step = self.inner.clone();
        step.step.params.insert(
            key,
            StepParameter {
                expression: None,
                value: Some(value),
                param_type,
            },
        );
        Arc::new(StepBuilder { inner: step })
    }

    #[uniffi::method]
    pub fn add_transition(&self, to_step_id: String, condition: String) -> Arc<StepBuilder> {
        let mut step = self.inner.clone();
        step.step
            .transition
            .successors
            .insert(to_step_id, condition);
        Arc::new(StepBuilder { inner: step })
    }

    #[uniffi::method]
    pub fn build(&self) -> Step {
        self.inner.clone()
    }
}
