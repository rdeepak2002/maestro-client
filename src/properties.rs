use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Properties {
    pub owner: String,
    pub run_strategy: String,
}
