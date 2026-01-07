use crate::protocols::dsp::facades::FacadeTrait;
use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcCatalogMessageTrait, RpcCatalogRequestMessageDto, RpcCatalogResponseMessageDto, RpcDatasetRequestMessageDto,
};
use crate::protocols::dsp::orchestrator::rpc::RPCOrchestratorTrait;
use crate::protocols::dsp::protocol_types::{CatalogMessageWrapper, CatalogRequestMessageDto, DatasetRequestMessage};
use crate::protocols::dsp::types::catalog_definition::Catalog;
use crate::protocols::dsp::types::dataset_definition::Dataset;
use crate::protocols::dsp::validator::traits::validation_dsp_steps::ValidationDspSteps;
use crate::protocols::dsp::validator::traits::validation_rpc_steps::ValidationRpcSteps;
use rainbow_common::http_client::HttpClient;
use rainbow_common::well_known::rpc::WellKnownRPCRequest;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct RPCOrchestratorService {
    validator: Arc<dyn ValidationRpcSteps>,
    http_client: Arc<HttpClient>,
    facades: Arc<dyn FacadeTrait>,
}

impl RPCOrchestratorService {
    pub fn new(
        validator: Arc<dyn ValidationRpcSteps>,
        http_client: Arc<HttpClient>,
        facades: Arc<dyn FacadeTrait>,
    ) -> RPCOrchestratorService {
        Self { validator, http_client, facades }
    }
}

#[async_trait::async_trait]
impl RPCOrchestratorTrait for RPCOrchestratorService {
    async fn setup_catalog_request_rpc(
        &self,
        input: &RpcCatalogRequestMessageDto,
    ) -> anyhow::Result<RpcCatalogResponseMessageDto<RpcCatalogRequestMessageDto, Catalog>> {
        // validation
        self.validator.on_catalog_request(input).await?;

        // send message to peer
        let participant_id = input.get_associated_agent_peer().ok_or(anyhow::Error::msg("No associated agent"))?;
        let provider_address = self
            .facades
            .get_catalog_rpc_path_facade()
            .await
            .resolve_dataspace_current_path(&WellKnownRPCRequest { participant_id })
            .await?;

        let peer_url = format!("{}/catalog/request", provider_address);
        let request_body: CatalogMessageWrapper<CatalogRequestMessageDto> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;

        let response = self
            .http_client
            .post_json::<CatalogMessageWrapper<CatalogRequestMessageDto>, Catalog>(peer_url.as_str(), &request_body)
            .await?;
        let response = RpcCatalogResponseMessageDto { request: input.clone(), response };
        Ok(response)
    }

    async fn setup_dataset_request_rpc(
        &self,
        input: &RpcDatasetRequestMessageDto,
    ) -> anyhow::Result<RpcCatalogResponseMessageDto<RpcDatasetRequestMessageDto, Dataset>> {
        // validation
        self.validator.on_dataset_request(input).await?;

        let participant_id = input.get_associated_agent_peer().ok_or(anyhow::Error::msg("No associated agent"))?;
        let provider_address = self
            .facades
            .get_catalog_rpc_path_facade()
            .await
            .resolve_dataspace_current_path(&WellKnownRPCRequest { participant_id })
            .await?;
        let dataset = input.get_dataset_id().unwrap_or("".to_string());
        let peer_url = format!("{}/catalog/datasets/{}", provider_address, dataset);
        let request_body: CatalogMessageWrapper<DatasetRequestMessage> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: Dataset = self.http_client.get_json_with_payload(peer_url.as_str(), &request_body).await?;

        let response = RpcCatalogResponseMessageDto { request: input.clone(), response };
        Ok(response)
    }
}
