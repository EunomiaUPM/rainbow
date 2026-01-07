use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcCatalogMessageTrait, RpcCatalogRequestMessageDto, RpcCatalogResponseMessageDto, RpcDatasetRequestMessageDto,
};
use crate::protocols::dsp::orchestrator::rpc::RPCOrchestratorTrait;
use crate::protocols::dsp::protocol_types::{CatalogMessageDto, CatalogMessageWrapper, CatalogRequestMessageDto, DatasetMessageDto, DatasetRequestMessage};
use rainbow_common::http_client::HttpClient;
use std::marker::PhantomData;
use std::sync::Arc;
use crate::protocols::dsp::validator::traits::validation_dsp_steps::ValidationDspSteps;
use crate::protocols::dsp::validator::traits::validation_rpc_steps::ValidationRpcSteps;

pub struct RPCOrchestratorService {
    validator: Arc<dyn ValidationRpcSteps>,
    http_client: Arc<HttpClient>,
}

impl RPCOrchestratorService {
    pub fn new(validator: Arc<dyn ValidationRpcSteps>, http_client: Arc<HttpClient>) -> RPCOrchestratorService {
        Self { validator, http_client }
    }
}

#[async_trait::async_trait]
impl RPCOrchestratorTrait for RPCOrchestratorService {
    async fn setup_catalog_request_rpc(
        &self,
        input: &RpcCatalogRequestMessageDto,
    ) -> anyhow::Result<RpcCatalogResponseMessageDto<RpcCatalogRequestMessageDto, CatalogMessageDto>> {
        // validation
        self.validator.on_catalog_request(input).await?;

        // send message to peer
        let provider_address = "";
        let peer_url = format!("{}/catalog/request", provider_address);
        let request_body: CatalogMessageWrapper<CatalogRequestMessageDto> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: CatalogMessageDto = self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // todo persist

        let response = RpcCatalogResponseMessageDto { request: input.clone(), response };
        Ok(response)
    }

    async fn setup_dataset_request_rpc(
        &self,
        input: &RpcDatasetRequestMessageDto,
    ) -> anyhow::Result<RpcCatalogResponseMessageDto<RpcDatasetRequestMessageDto, DatasetMessageDto>> {
        // validation
        self.validator.on_dataset_request(input).await?;

        let provider_address = "";
        let dataset = input.get_dataset_id().unwrap_or("".to_string());
        let peer_url = format!("{}/catalog/datasets/{}", provider_address, dataset);
        let request_body: CatalogMessageWrapper<DatasetRequestMessage> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: DatasetMessageDto = self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // todo persist

        let response = RpcCatalogResponseMessageDto { request: input.clone(), response };
        Ok(response)
    }
}
