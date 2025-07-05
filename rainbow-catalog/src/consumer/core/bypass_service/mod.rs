use axum::async_trait;
use serde_json::Value;

pub mod bypass_service;

#[mockall::automock]
#[async_trait]
pub trait ByPassTrait: Send + Sync + 'static {
    async fn bypass(&self, participant_id: String, path: String) -> anyhow::Result<Value>;
}