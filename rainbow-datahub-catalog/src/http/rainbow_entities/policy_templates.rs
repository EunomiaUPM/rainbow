use serde_json::Value;
use urn::Urn;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use crate::core::datahub_proxy::datahub_proxy_types::DatahubDataset;

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyTemplate {
    pub id: Urn,
    pub content: Value,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyTemplateDatasetRelation {
    pub relation_id: String,
    pub datahub_dataset: DatahubDataset,
    pub policy_template: PolicyTemplate,
}