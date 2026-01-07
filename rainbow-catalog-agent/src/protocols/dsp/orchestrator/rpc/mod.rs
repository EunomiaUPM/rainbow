use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcCatalogRequestMessageDto, RpcCatalogResponseMessageDto, RpcDatasetRequestMessageDto,
};
use crate::protocols::dsp::protocol_types::{
    CatalogMessageDto, CatalogMessageWrapper, CatalogRequestMessageDto, DatasetMessageDto,
};

pub(crate) mod rpc;
pub(crate) mod types;
pub(crate) mod persistence;

#[async_trait::async_trait]
pub trait RPCOrchestratorTrait: Send + Sync + 'static {
    async fn setup_catalog_request_rpc(
        &self,
        input: &RpcCatalogRequestMessageDto,
    ) -> anyhow::Result<
        RpcCatalogResponseMessageDto<RpcCatalogRequestMessageDto, CatalogMessageDto>,
    >;

    async fn setup_dataset_request_rpc(
        &self,
        input: &RpcDatasetRequestMessageDto,
    ) -> anyhow::Result<
        RpcCatalogResponseMessageDto<RpcDatasetRequestMessageDto, DatasetMessageDto>,
    >;
}
