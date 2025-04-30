use axum::async_trait;
use rainbow_common::auth::GrantRequest;
use serde_json::Value;

pub mod manager;

#[async_trait]
pub trait RainbowSSIAuthProviderManagerTrait: Send + Sync {
    async fn generate_exchange_uri(&self, payload: GrantRequest) -> anyhow::Result<(String, String, String)>;
    async fn generate_vp_def(&self, state: String) -> anyhow::Result<Value>;
    async fn verify_all(&self, state: String, vp_token: String) -> anyhow::Result<Option<String>>;
    async fn verify_vp(&self, exchange: String, vp_token: String) -> anyhow::Result<(Vec<String>, String)>;
    async fn verify_vc(&self, vc_token: String, vp_holder: String) -> anyhow::Result<()>;
}