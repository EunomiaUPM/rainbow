use serde_json::Value;
use urn::Urn;

pub struct PolicyTemplate {
    pub id: Urn,
    pub content: Value,
    pub created_at: chrono::NaiveDateTime,
}