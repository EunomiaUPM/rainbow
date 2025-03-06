use axum::async_trait;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use urn::Urn;

pub mod data_service;

#[mockall::automock]
#[async_trait]
pub trait DataServiceFacadeTrait: Send + Sync {
    async fn resolve_data_service_by_agreement_id(&self, agreement_id: Urn) -> anyhow::Result<DataService>;
}
