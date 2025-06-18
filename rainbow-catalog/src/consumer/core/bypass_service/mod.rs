use axum::async_trait;
use serde_json::Value;
use urn::Urn;

pub mod bypass_service;

#[mockall::automock]
#[async_trait]
pub trait ByPassTrait: Send + Sync + 'static {
    async fn bypass(&self, participant_id: Urn, path: String) -> anyhow::Result<Value>;
}