use crate::core::rainbow_rpc::rainbow_rpc_types::RainbowRPCCatalogResolveDataServiceRequest;
use crate::protocol::dataservice_definition::DataService;
use axum::async_trait;

pub mod rainbow_rpc;
pub mod rainbow_rpc_types;

#[async_trait]
#[mockall::automock]
pub trait RainbowRPCCatalogTrait: Send + Sync {
    async fn resolve_data_service(&self, input: RainbowRPCCatalogResolveDataServiceRequest) -> anyhow::Result<DataService>;
}