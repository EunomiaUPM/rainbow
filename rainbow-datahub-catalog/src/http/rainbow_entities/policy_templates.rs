use crate::core::datahub_proxy::datahub_proxy_types::DatahubDataset;
use rainbow_common::protocol::contract::contract_odrl::OdrlPolicyInfo;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyTemplate {
    pub id: String,
    pub content: OdrlPolicyInfo,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyTemplateDatasetRelation {
    pub relation_id: String,
    pub datahub_dataset: DatahubDataset,
    pub policy_template: PolicyTemplate,
}