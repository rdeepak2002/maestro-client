use crate::properties::Properties;
use crate::workflow::{Workflow, WorkflowBuilder};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(uniffi::Record, Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowPushResponse {
    pub status: u16,
    pub text: Option<String>,
}

#[derive(uniffi::Record, Clone, Debug, Serialize, Deserialize)]
pub(crate) struct WorkflowPayload {
    pub properties: Properties,
    pub workflow: Workflow,
}

pub(crate) fn push_workflow(builder: &WorkflowBuilder) -> WorkflowPushResponse {
    let client = Client::new();
    let api_url = env::var("MAESTRO_CLIENT_API_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080/api/v3/workflows".to_string());

    let payload = WorkflowPayload {
        properties: Properties {
            owner: "tester".to_owned(),
            run_strategy: "sequential".to_owned(),
        },
        workflow: builder.build(),
    };

    let response = client
        .post(api_url)
        .header("user", "tester")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .expect("Failed to push workflow");

    WorkflowPushResponse {
        status: response.status().as_u16(),
        text: response.text().ok(),
    }
}
