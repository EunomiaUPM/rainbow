use serde_json::Value;
use urn::Urn;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct PolicyTemplate {
    pub id: Urn,
    pub content: Value,
    pub created_at: chrono::NaiveDateTime,
}