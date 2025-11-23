use crate::core::dsp::orchestrator::rpc::types::{
    RpcTransferCompletionMessageDto, RpcTransferMessageDto, RpcTransferRequestMessageDto, RpcTransferStartMessageDto,
    RpcTransferSuspensionMessageDto, RpcTransferTerminationMessageDto,
};
use crate::core::dsp::orchestrator::rpc::RPCOrchestratorTrait;
use crate::core::dsp::persistence::TransferPersistenceTrait;
use crate::core::dsp::protocol_types::{
    TransferCompletionMessageDto, TransferProcessAckDto, TransferProcessMessageType, TransferProcessMessageWrapper,
    TransferRequestMessageDto, TransferStartMessageDto, TransferSuspensionMessageDto, TransferTerminationMessageDto,
};
use crate::core::dsp::state_machine::StateMachineTrait;
use crate::core::dsp::validator::ValidatorTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::http_client::HttpClient;
use rainbow_common::protocol::context_field::ContextField;
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;

pub struct RPCOrchestratorService {
    pub state_machine_service: Arc<dyn StateMachineTrait>,
    pub validator_service: Arc<dyn ValidatorTrait>,
    pub persistence_service: Arc<dyn TransferPersistenceTrait>,
    pub config: Arc<ApplicationProviderConfig>,
    pub http_client: Arc<HttpClient>,
}

impl RPCOrchestratorService {
    pub fn new(
        state_machine_service: Arc<dyn StateMachineTrait>,
        validator_service: Arc<dyn ValidatorTrait>,
        persistence_service: Arc<dyn TransferPersistenceTrait>,
        config: Arc<ApplicationProviderConfig>,
        http_client: Arc<HttpClient>,
    ) -> RPCOrchestratorService {
        RPCOrchestratorService { state_machine_service, validator_service, persistence_service, config, http_client }
    }
}

#[async_trait::async_trait]
impl RPCOrchestratorTrait for RPCOrchestratorService {
    async fn setup_transfer_request(
        &self,
        input: &RpcTransferRequestMessageDto,
    ) -> anyhow::Result<RpcTransferMessageDto<RpcTransferRequestMessageDto>> {
        let request_body: TransferProcessMessageWrapper<TransferRequestMessageDto> = input.clone().into();
        let provider_address = input.provider_address.clone();
        self.state_machine_service.validate_rpc_transition(None, Arc::new(request_body.dto.clone())).await?;
        self.validator_service.validate(None, Arc::new(request_body.dto.clone())).await?;
        let peer_url = format!("{}/transfers/request", provider_address);
        let response: TransferProcessMessageWrapper<TransferProcessAckDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;
        let transfer_process = self
            .persistence_service
            .create_process(
                "DSP",
                "OUTBOUND",
                Some(response.dto.provider_pid.clone()),
                Some(provider_address),
                Arc::new(request_body.clone().dto),
                serde_json::to_value(request_body.clone()).unwrap(),
            )
            .await?;
        let response =
            RpcTransferMessageDto { request: input.clone(), response, transfer_agent_model: transfer_process };
        Ok(response)
    }

    async fn setup_transfer_start(
        &self,
        input: &RpcTransferStartMessageDto,
    ) -> anyhow::Result<RpcTransferMessageDto<RpcTransferStartMessageDto>> {
        let input_data_address = input.data_address.clone();
        // current state
        let input_transfer_id = input.consumer_pid.clone();
        let transfer_process = self.persistence_service.fetch_process(input_transfer_id.to_string().as_str()).await?;
        // can be sent?
        let provider_pid = transfer_process.identifiers.get("providerPid").unwrap();
        let consumer_pid = transfer_process.identifiers.get("consumerPid").unwrap();
        let transfer_process_into_trait = TransferStartMessageDto {
            provider_pid: Urn::from_str(provider_pid.as_str()).unwrap(),
            consumer_pid: Urn::from_str(consumer_pid.as_str()).unwrap(),
            data_address: input_data_address,
        };
        self.state_machine_service.validate_rpc_transition(None, Arc::new(transfer_process_into_trait.clone())).await?;
        self.validator_service.validate(None, Arc::new(transfer_process_into_trait.clone())).await?;
        // where to be sent
        let peer_url_id = transfer_process.identifiers.get("consumerPid").unwrap();
        let callback_url = transfer_process.inner.callback_address.clone().unwrap_or("".to_string());
        let peer_url = format!(
            "{}/transfers/{}/start",
            callback_url,
            peer_url_id.to_string()
        );
        dbg!(&peer_url);
        let message = TransferProcessMessageWrapper {
            context: ContextField::default(),
            _type: TransferProcessMessageType::TransferStartMessage,
            dto: transfer_process_into_trait.clone(),
        };
        // send
        let response: TransferProcessMessageWrapper<TransferProcessAckDto> =
            self.http_client.post_json(peer_url.as_str(), &message).await?;
        // persist response
        let transfer_process = self
            .persistence_service
            .update_process_by_process_id(
                transfer_process.inner.id.as_str(),
                Arc::new(transfer_process_into_trait.clone()),
                serde_json::to_value(message.clone()).unwrap(),
            )
            .await?;
        // bye
        let response =
            RpcTransferMessageDto { request: input.clone(), response, transfer_agent_model: transfer_process };
        Ok(response)
    }

    async fn setup_transfer_suspension(
        &self,
        input: &RpcTransferSuspensionMessageDto,
    ) -> anyhow::Result<RpcTransferMessageDto<RpcTransferSuspensionMessageDto>> {
        // current state
        let input_transfer_id = input.consumer_pid.clone();
        let input_code = input.code.clone();
        let input_reason = input.reason.clone();
        let transfer_process = self.persistence_service.fetch_process(input_transfer_id.to_string().as_str()).await?;
        // can be sent?
        let provider_pid = transfer_process.identifiers.get("providerPid").unwrap();
        let consumer_pid = transfer_process.identifiers.get("consumerPid").unwrap();
        let transfer_process_into_trait = TransferSuspensionMessageDto {
            provider_pid: Urn::from_str(provider_pid.as_str())?,
            consumer_pid: Urn::from_str(consumer_pid.as_str())?,
            code: input_code,
            reason: input_reason,
        };
        self.state_machine_service.validate_rpc_transition(None, Arc::new(transfer_process_into_trait.clone())).await?;
        self.validator_service.validate(None, Arc::new(transfer_process_into_trait.clone())).await?;

        // where to be sent (depend o being provider or consumer
        let identifier_key = match transfer_process.inner.role.as_str() {
            "Provider" => "providerPid",
            "Consumer" => "consumerPid",
            _ => "providerPid", // should panic... but still...
        };
        let peer_url_id = transfer_process.identifiers.get(identifier_key).unwrap();
        let callback_url = transfer_process.inner.callback_address.clone().unwrap_or("".to_string());
        let peer_url = format!(
            "{}/transfers/{}/suspension",
            callback_url,
            peer_url_id.to_string()
        );
        // what to be sent
        let message = TransferProcessMessageWrapper {
            context: ContextField::default(),
            _type: TransferProcessMessageType::TransferSuspensionMessage,
            dto: transfer_process_into_trait.clone(),
        };
        // send
        let response: TransferProcessMessageWrapper<TransferProcessAckDto> =
            self.http_client.post_json(peer_url.as_str(), &message).await?;
        // persist response
        let transfer_process = self
            .persistence_service
            .update_process_by_process_id(
                transfer_process.inner.id.as_str(),
                Arc::new(transfer_process_into_trait.clone()),
                serde_json::to_value(message.clone()).unwrap(),
            )
            .await?;
        // bye
        let response =
            RpcTransferMessageDto { request: input.clone(), response, transfer_agent_model: transfer_process };
        Ok(response)
    }

    async fn setup_transfer_completion(
        &self,
        input: &RpcTransferCompletionMessageDto,
    ) -> anyhow::Result<RpcTransferMessageDto<RpcTransferCompletionMessageDto>> {
        // current state
        let input_transfer_id = input.consumer_pid.clone();
        let transfer_process = self.persistence_service.fetch_process(input_transfer_id.to_string().as_str()).await?;
        // can be sent?
        let provider_pid = transfer_process.identifiers.get("providerPid").unwrap();
        let consumer_pid = transfer_process.identifiers.get("consumerPid").unwrap();
        let transfer_process_into_trait = TransferCompletionMessageDto {
            provider_pid: Urn::from_str(provider_pid.as_str())?,
            consumer_pid: Urn::from_str(consumer_pid.as_str())?,
        };
        self.state_machine_service.validate_rpc_transition(None, Arc::new(transfer_process_into_trait.clone())).await?;
        self.validator_service.validate(None, Arc::new(transfer_process_into_trait.clone())).await?;
        // where to be sent
        let peer_url_id = transfer_process.identifiers.get("providerPid").unwrap();
        let callback_url = transfer_process.inner.callback_address.clone().unwrap_or("".to_string());
        let peer_url = format!(
            "{}/transfers/{}/completion",
            callback_url,
            peer_url_id.to_string()
        );
        let message = TransferProcessMessageWrapper {
            context: ContextField::default(),
            _type: TransferProcessMessageType::TransferCompletionMessage,
            dto: transfer_process_into_trait.clone(),
        };
        // send
        let response: TransferProcessMessageWrapper<TransferProcessAckDto> =
            self.http_client.post_json(peer_url.as_str(), &message).await?;
        // persist response
        let transfer_process = self
            .persistence_service
            .update_process_by_process_id(
                transfer_process.inner.id.as_str(),
                Arc::new(transfer_process_into_trait.clone()),
                serde_json::to_value(message.clone()).unwrap(),
            )
            .await?;
        // bye
        let response =
            RpcTransferMessageDto { request: input.clone(), response, transfer_agent_model: transfer_process };
        Ok(response)
    }

    async fn setup_transfer_termination(
        &self,
        input: &RpcTransferTerminationMessageDto,
    ) -> anyhow::Result<RpcTransferMessageDto<RpcTransferTerminationMessageDto>> {
        // current state
        let input_transfer_id = input.consumer_pid.clone();
        let input_code = input.code.clone();
        let input_reason = input.reason.clone();
        let transfer_process = self.persistence_service.fetch_process(input_transfer_id.to_string().as_str()).await?;
        // can be sent?
        let provider_pid = transfer_process.identifiers.get("providerPid").unwrap();
        let consumer_pid = transfer_process.identifiers.get("consumerPid").unwrap();
        let transfer_process_into_trait = TransferTerminationMessageDto {
            provider_pid: Urn::from_str(provider_pid.as_str()).unwrap(),
            consumer_pid: Urn::from_str(consumer_pid.as_str()).unwrap(),
            code: input_code,
            reason: input_reason,
        };
        self.state_machine_service.validate_rpc_transition(None, Arc::new(transfer_process_into_trait.clone())).await?;
        self.validator_service.validate(None, Arc::new(transfer_process_into_trait.clone())).await?;

        // where to be sent
        let peer_url_id = transfer_process.identifiers.get("providerPid").unwrap();
        let callback_url = transfer_process.inner.callback_address.clone().unwrap_or("".to_string());
        let peer_url = format!(
            "{}/transfers/{}/termination",
            callback_url,
            peer_url_id.to_string()
        );
        let message = TransferProcessMessageWrapper {
            context: ContextField::default(),
            _type: TransferProcessMessageType::TransferTerminationMessage,
            dto: transfer_process_into_trait.clone(),
        };
        // send
        let response: TransferProcessMessageWrapper<TransferProcessAckDto> =
            self.http_client.post_json(peer_url.as_str(), &message).await?;
        // persist response
        let transfer_process = self
            .persistence_service
            .update_process_by_process_id(
                transfer_process.inner.id.as_str(),
                Arc::new(transfer_process_into_trait.clone()),
                serde_json::to_value(message.clone()).unwrap(),
            )
            .await?;
        // bye
        let response =
            RpcTransferMessageDto { request: input.clone(), response, transfer_agent_model: transfer_process };
        Ok(response)
    }
}
