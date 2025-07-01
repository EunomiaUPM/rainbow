use crate::core::vc_request_service::vc_request_types::VCRequest;
use crate::data::entities::vc_requests;
use axum::async_trait;
use urn::Urn;

pub mod vc_request_service;
pub mod vc_request_types;

#[mockall::automock]
#[async_trait]
pub trait VCRequestTrait: Send + Sync {
    async fn get_all_vc_requests(&self) -> anyhow::Result<Vec<vc_requests::Model>>;
    async fn get_vc_request_by_id(&self, vc_request_id: Urn) -> anyhow::Result<vc_requests::Model>;
    async fn validate_vc_request_by_id(&self, vc_request_id: Urn) -> anyhow::Result<vc_requests::Model>;
    async fn reject_vc_request_by_id(&self, vc_request_id: Urn) -> anyhow::Result<vc_requests::Model>;
    async fn create_vc_request(&self, input: VCRequest) -> anyhow::Result<vc_requests::Model>;
}