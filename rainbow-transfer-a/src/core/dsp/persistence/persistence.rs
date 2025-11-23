use crate::core::dsp::persistence::TransferPersistenceTrait;
use crate::core::dsp::protocol_types::{TransferProcessMessageTrait, TransferProcessState};
use crate::entities::transfer_messages::{NewTransferMessageDto, TransferAgentMessagesTrait};
use crate::entities::transfer_process::{
    EditTransferProcessDto, NewTransferProcessDto, TransferAgentProcessesTrait, TransferProcessDto,
};
use crate::http::common::parse_urn;
use log::error;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::transfer::TransferState;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;

pub struct TransferPersistenceService {
    pub transfer_message_service: Arc<dyn TransferAgentMessagesTrait>,
    pub transfer_process_service: Arc<dyn TransferAgentProcessesTrait>,
    pub _config: Arc<ApplicationProviderConfig>,
}

impl TransferPersistenceService {
    pub fn new(
        transfer_message_service: Arc<dyn TransferAgentMessagesTrait>,
        transfer_process_service: Arc<dyn TransferAgentProcessesTrait>,
        config: Arc<ApplicationProviderConfig>,
    ) -> Self {
        Self { transfer_message_service, transfer_process_service, _config: config }
    }
}

#[async_trait::async_trait]
impl TransferPersistenceTrait for TransferPersistenceService {
    async fn fetch_process_by_process_id(&self, id: &str) -> anyhow::Result<TransferProcessDto> {
        let urn = parse_urn(id).unwrap();
        let transfer_process = self.transfer_process_service.get_transfer_process_by_id(&urn).await.map_err(|e| {
            let err = CommonErrors::missing_resource_new(urn.to_string().as_str(), "Process service not found");
            error!("{}", err.log());
            err
        })?;
        Ok(transfer_process)
    }
    async fn fetch_process(&self, id: &str) -> anyhow::Result<TransferProcessDto> {
        let urn = parse_urn(id).unwrap();
        let transfer_process =
            self.transfer_process_service.get_transfer_process_by_key_value(&urn).await.map_err(|e| {
                let err = CommonErrors::missing_resource_new(urn.to_string().as_str(), "Process service not found");
                error!("{}", err.log());
                err
            })?;
        Ok(transfer_process)
    }

    async fn create_process(
        &self,
        protocol: &str,
        direction: &str,
        provider_pid: Option<Urn>,
        provider_address: Option<String>,
        payload_dto: Arc<dyn TransferProcessMessageTrait>,
        payload_value: serde_json::Value,
    ) -> anyhow::Result<TransferProcessDto> {
        let consumer_pid = payload_dto.get_consumer_pid().unwrap(); // always
        let format = payload_dto.get_format().unwrap();
        let agreement_id = payload_dto.get_agreement_id().unwrap();
        let message_type = payload_dto.get_message();
        let mut identifiers = HashMap::new();
        let role = if direction == "INBOUND" { "Provider" } else { "Consumer" };
        let callback_address = provider_address.unwrap_or(payload_dto.get_callback_address().unwrap());

        if direction == "INBOUND" {
            let provider_pid = format!("urn:provider-pid:{}", uuid::Uuid::new_v4());
            identifiers.insert("providerPid".to_string(), provider_pid);
        } else {
            identifiers.insert("providerPid".to_string(), provider_pid.unwrap().to_string());
        }
        identifiers.insert("consumerPid".to_string(), consumer_pid.to_string());
        let transfer_process_id = Urn::from_str(format!("urn:transfer-process:{}", uuid::Uuid::new_v4()).as_str())?;
        let mut transfer_process = self
            .transfer_process_service
            .create_transfer_process(&NewTransferProcessDto {
                id: Some(transfer_process_id.clone()),
                state: TransferState::REQUESTED.to_string(),
                associated_agent_peer: "".to_string(),
                protocol: protocol.to_string(),
                transfer_direction: format,
                agreement_id,
                callback_address: Some(callback_address),
                role: role.to_string(),
                state_attribute: Some("REQUESTED".to_string()),
                properties: None,
                identifiers: Some(identifiers),
            })
            .await?;
        let mut transfer_message = self
            .transfer_message_service
            .create_transfer_message(&NewTransferMessageDto {
                id: None,
                transfer_agent_process_id: transfer_process_id.clone(),
                direction: direction.to_string(),
                protocol: protocol.to_string(),
                message_type: message_type.to_string(),
                state_transition_from: "-".to_string(),
                state_transition_to: TransferState::REQUESTED.to_string(),
                payload: Some(payload_value),
            })
            .await?;
        transfer_process.messages.push(transfer_message.inner);
        Ok(transfer_process)
    }

    async fn update_process(
        &self,
        id: &str,
        payload_dto: Arc<dyn TransferProcessMessageTrait>,
        payload_value: serde_json::Value,
    ) -> anyhow::Result<TransferProcessDto> {
        let message_type = payload_dto.get_message();
        let new_state: TransferProcessState = TransferProcessState::from(message_type.clone());

        let urn_id = Urn::from_str(id).expect("Failed to parse urnID");
        let mut transfer_process = self.transfer_process_service.get_transfer_process_by_key_value(&urn_id).await?;

        let transfer_process_urn = Urn::from_str(transfer_process.inner.id.as_str()).expect("Failed to parse urnID");
        let mut new_transfer_process = self
            .transfer_process_service
            .put_transfer_process(
                &transfer_process_urn,
                &EditTransferProcessDto {
                    state: Some(new_state.to_string()),
                    state_attribute: Some("BY_PROVIDER".to_string()),
                    properties: None,
                    error_details: None,
                    identifiers: None,
                },
            )
            .await?;
        let mut message = self
            .transfer_message_service
            .create_transfer_message(&NewTransferMessageDto {
                id: None,
                transfer_agent_process_id: transfer_process_urn.clone(),
                direction: "INBOUND".to_string(),
                protocol: "DSP".to_string(),
                message_type: message_type.to_string(),
                state_transition_from: transfer_process.inner.state.to_string(),
                state_transition_to: new_state.to_string(),
                payload: Some(payload_value.clone()),
            })
            .await?;
        new_transfer_process.messages.push(message.inner);
        Ok(new_transfer_process)
    }

    async fn update_process_by_process_id(
        &self,
        id: &str,
        payload_dto: Arc<dyn TransferProcessMessageTrait>,
        payload_value: serde_json::Value,
    ) -> anyhow::Result<TransferProcessDto> {
        let message_type = payload_dto.get_message();
        let new_state: TransferProcessState = TransferProcessState::from(message_type.clone());

        let urn_id = Urn::from_str(id).expect("Failed to parse urnID");
        let mut transfer_process = self.transfer_process_service.get_transfer_process_by_id(&urn_id).await?;

        let transfer_process_urn = Urn::from_str(transfer_process.inner.id.as_str())?;
        let mut new_transfer_process = self
            .transfer_process_service
            .put_transfer_process(
                &transfer_process_urn,
                &EditTransferProcessDto {
                    state: Some(new_state.to_string()),
                    state_attribute: Some("BY_PROVIDER".to_string()),
                    properties: None,
                    error_details: None,
                    identifiers: None,
                },
            )
            .await?;
        let mut message = self
            .transfer_message_service
            .create_transfer_message(&NewTransferMessageDto {
                id: None,
                transfer_agent_process_id: transfer_process_urn.clone(),
                direction: "OUTBOUND".to_string(),
                protocol: "DSP".to_string(),
                message_type: message_type.to_string(),
                state_transition_from: transfer_process.inner.state.to_string(),
                state_transition_to: new_state.to_string(),
                payload: Some(payload_value),
            })
            .await?;
        new_transfer_process.messages.push(message.inner);
        Ok(new_transfer_process)
    }
}
