use crate::entities::transfer_process::TransferProcessDto;
use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcTransferCompletionMessageDto, RpcTransferMessageDto, RpcTransferRequestMessageDto, RpcTransferStartMessageDto,
    RpcTransferSuspensionMessageDto, RpcTransferTerminationMessageDto,
};
use crate::protocols::dsp::orchestrator::rpc::RPCOrchestratorTrait;
use crate::protocols::dsp::persistence::TransferPersistenceTrait;
use crate::protocols::dsp::protocol_types::{
    TransferCompletionMessageDto, TransferProcessAckDto, TransferProcessMessageTrait, TransferProcessMessageWrapper,
    TransferRequestMessageDto, TransferStartMessageDto, TransferSuspensionMessageDto, TransferTerminationMessageDto,
};
use crate::protocols::dsp::state_machine::StateMachineTrait;
use crate::protocols::dsp::validator::ValidatorTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::http_client::HttpClient;
use rainbow_common::protocol::context_field::ContextField;
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;

#[allow(unused)]
pub struct RPCOrchestratorService {
    pub state_machine_service: Arc<dyn StateMachineTrait>,
    pub validator_service: Arc<dyn ValidatorTrait>,
    pub persistence_service: Arc<dyn TransferPersistenceTrait>,
    pub _config: Arc<ApplicationProviderConfig>,
    pub http_client: Arc<HttpClient>,
}

impl RPCOrchestratorService {
    pub fn new(
        state_machine_service: Arc<dyn StateMachineTrait>,
        validator_service: Arc<dyn ValidatorTrait>,
        persistence_service: Arc<dyn TransferPersistenceTrait>,
        _config: Arc<ApplicationProviderConfig>,
        http_client: Arc<HttpClient>,
    ) -> RPCOrchestratorService {
        RPCOrchestratorService { state_machine_service, validator_service, persistence_service, _config, http_client }
    }
}

#[async_trait::async_trait]
impl RPCOrchestratorTrait for RPCOrchestratorService {
    async fn setup_transfer_request(
        &self,
        input: &RpcTransferRequestMessageDto,
    ) -> anyhow::Result<RpcTransferMessageDto<RpcTransferRequestMessageDto>> {
        // get from input
        let request_body: TransferProcessMessageWrapper<TransferRequestMessageDto> = input.clone().into();
        let provider_address = input.provider_address.clone();
        // validate
        // self.state_machine_service.validate_transition(None, Arc::new(request_body.dto.clone())).await?;
        // self.validator_service.validate(None, Arc::new(request_body.dto.clone())).await?;
        // create url
        let peer_url = format!("{}/transfers/request", provider_address);
        // request
        let response: TransferProcessMessageWrapper<TransferProcessAckDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;
        // persist
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
        // get from input
        let input_data_address = input.data_address.clone();
        let input_transfer_id = input.consumer_pid.clone();
        // fetch current process
        let transfer_process = self.persistence_service.fetch_process(input_transfer_id.to_string().as_str()).await?;
        let provider_pid = transfer_process.identifiers.get("providerPid").unwrap();
        let consumer_pid = transfer_process.identifiers.get("consumerPid").unwrap();
        // create message
        let transfer_process_into_trait = TransferStartMessageDto {
            provider_pid: Urn::from_str(provider_pid.as_str())?,
            consumer_pid: Urn::from_str(consumer_pid.as_str())?,
            data_address: input_data_address,
        };
        // get uri
        let identifier_key = match transfer_process.inner.role.as_str() {
            "Provider" => "consumerPid",
            "Consumer" => "providerPid",
            _ => "providerPid",
        };
        let peer_url_id = transfer_process.identifiers.get(identifier_key).unwrap();
        // validate, send and persist
        let (response, transfer_process) = self
            .validate_and_send(
                &transfer_process,
                Arc::new(transfer_process_into_trait.clone()),
                peer_url_id,
                "start",
            )
            .await?;
        // bye!
        let response =
            RpcTransferMessageDto { request: input.clone(), response, transfer_agent_model: transfer_process };
        Ok(response)
    }

    async fn setup_transfer_suspension(
        &self,
        input: &RpcTransferSuspensionMessageDto,
    ) -> anyhow::Result<RpcTransferMessageDto<RpcTransferSuspensionMessageDto>> {
        // get from input
        let input_transfer_id = input.consumer_pid.clone();
        let input_code = input.code.clone();
        let input_reason = input.reason.clone();
        // fetch current process
        let transfer_process = self.persistence_service.fetch_process(input_transfer_id.to_string().as_str()).await?;
        let provider_pid = transfer_process.identifiers.get("providerPid").unwrap();
        let consumer_pid = transfer_process.identifiers.get("consumerPid").unwrap();
        // create message
        let transfer_process_into_trait = TransferSuspensionMessageDto {
            provider_pid: Urn::from_str(provider_pid.as_str())?,
            consumer_pid: Urn::from_str(consumer_pid.as_str())?,
            code: input_code,
            reason: input_reason,
        };
        // get uri
        let identifier_key = match transfer_process.inner.role.as_str() {
            "Provider" => "consumerPid",
            "Consumer" => "providerPid",
            _ => "providerPid",
        };
        let peer_url_id = transfer_process.identifiers.get(identifier_key).unwrap();
        // validate, send and persist
        let (response, transfer_process) = self
            .validate_and_send(
                &transfer_process,
                Arc::new(transfer_process_into_trait.clone()),
                peer_url_id,
                "suspension",
            )
            .await?;
        // bye!
        let response =
            RpcTransferMessageDto { request: input.clone(), response, transfer_agent_model: transfer_process };
        Ok(response)
    }

    async fn setup_transfer_completion(
        &self,
        input: &RpcTransferCompletionMessageDto,
    ) -> anyhow::Result<RpcTransferMessageDto<RpcTransferCompletionMessageDto>> {
        // get from input
        let input_transfer_id = input.consumer_pid.clone();
        // fetch current process
        let transfer_process = self.persistence_service.fetch_process(input_transfer_id.to_string().as_str()).await?;
        let provider_pid = transfer_process.identifiers.get("providerPid").unwrap();
        let consumer_pid = transfer_process.identifiers.get("consumerPid").unwrap();
        // create message
        let transfer_process_into_trait = TransferCompletionMessageDto {
            provider_pid: Urn::from_str(provider_pid.as_str())?,
            consumer_pid: Urn::from_str(consumer_pid.as_str())?,
        };
        // get uri
        let identifier_key = match transfer_process.inner.role.as_str() {
            "Provider" => "consumerPid",
            "Consumer" => "providerPid",
            _ => "providerPid",
        };
        let peer_url_id = transfer_process.identifiers.get(identifier_key).unwrap();
        // validate, send and persist
        let (response, transfer_process) = self
            .validate_and_send(
                &transfer_process,
                Arc::new(transfer_process_into_trait.clone()),
                peer_url_id,
                "completion",
            )
            .await?;
        // bye!
        let response =
            RpcTransferMessageDto { request: input.clone(), response, transfer_agent_model: transfer_process };
        Ok(response)
    }

    async fn setup_transfer_termination(
        &self,
        input: &RpcTransferTerminationMessageDto,
    ) -> anyhow::Result<RpcTransferMessageDto<RpcTransferTerminationMessageDto>> {
        // get from input
        let input_transfer_id = input.consumer_pid.clone();
        let input_code = input.code.clone();
        let input_reason = input.reason.clone();
        // fetch current process
        let transfer_process = self.persistence_service.fetch_process(input_transfer_id.to_string().as_str()).await?;
        let provider_pid = transfer_process.identifiers.get("providerPid").unwrap();
        let consumer_pid = transfer_process.identifiers.get("consumerPid").unwrap();
        // create message
        let transfer_process_into_trait = TransferTerminationMessageDto {
            provider_pid: Urn::from_str(provider_pid.as_str()).unwrap(),
            consumer_pid: Urn::from_str(consumer_pid.as_str()).unwrap(),
            code: input_code,
            reason: input_reason,
        };
        // get uri
        let identifier_key = match transfer_process.inner.role.as_str() {
            "Provider" => "consumerPid",
            "Consumer" => "providerPid",
            _ => "providerPid",
        };
        let peer_url_id = transfer_process.identifiers.get(identifier_key).unwrap();
        // validate, send and persist
        let (response, transfer_process) = self
            .validate_and_send(
                &transfer_process,
                Arc::new(transfer_process_into_trait.clone()),
                peer_url_id,
                "termination",
            )
            .await?;
        // bye!
        let response =
            RpcTransferMessageDto { request: input.clone(), response, transfer_agent_model: transfer_process };
        Ok(response)
    }
}

impl RPCOrchestratorService {
    async fn validate_and_send<T>(
        &self,
        transfer_process: &TransferProcessDto,
        payload: Arc<T>,
        peer_url_id: &str,
        url_suffix: &str,
    ) -> anyhow::Result<(
        TransferProcessMessageWrapper<TransferProcessAckDto>,
        TransferProcessDto,
    )>
    where
        T: TransferProcessMessageTrait + Clone + serde::Serialize + 'static,
    {
        // self.state_machine_service.validate_transition(None, payload.clone()).await?;
        // self.validator_service.validate(None, payload.clone()).await?;
        // where to send
        let callback_url = transfer_process.inner.callback_address.clone().unwrap_or("".to_string());
        let peer_url = format!("{}/transfers/{}/{}", callback_url, peer_url_id, url_suffix);
        // create final message
        let message = TransferProcessMessageWrapper {
            context: ContextField::default(),
            _type: payload.get_message(),
            dto: payload.as_ref().clone(),
        };
        // send message to peer url
        let response: TransferProcessMessageWrapper<TransferProcessAckDto> =
            self.http_client.post_json(peer_url.as_str(), &message).await?;
        // persist
        let transfer_process = self
            .persistence_service
            .update_process(
                transfer_process.inner.id.as_str(),
                payload,
                serde_json::to_value(message.clone()).unwrap(),
            )
            .await?;
        // bye!
        Ok((response, transfer_process))
    }
}
