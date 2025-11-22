use std::collections::HashMap;
use std::str::FromStr;
use crate::core::dsp::persistence::TransferPersistenceTrait;
use crate::core::dsp::protocol_types::{TransferProcessMessageTrait, TransferProcessState};
use crate::entities::transfer_messages::{NewTransferMessageDto, TransferAgentMessagesTrait};
use crate::entities::transfer_process::{EditTransferProcessDto, NewTransferProcessDto, TransferAgentProcessesTrait, TransferProcessDto};
use crate::http::common::parse_urn;
use log::error;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use urn::Urn;
use rainbow_common::protocol::transfer::TransferState;

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
        payload_dto: Arc<dyn TransferProcessMessageTrait>,
        payload_value: serde_json::Value,
    ) -> anyhow::Result<TransferProcessDto> {
        let consumer_pid = payload_dto.get_consumer_pid().unwrap(); // always
        let format = payload_dto.get_format().unwrap();
        let agreement_id = payload_dto.get_agreement_id().unwrap();
        let message_type = payload_dto.get_message();
        let callback_address = payload_dto.get_callback_address();

        let mut identifiers = HashMap::new();
        let provider_pid = format!("urn:provider-pid:{}", uuid::Uuid::new_v4());
        identifiers.insert("providerPid".to_string(), provider_pid);
        identifiers.insert(
            "consumerPid".to_string(),
            consumer_pid.to_string(),
        );
        let transfer_process_id = Urn::from_str(format!("urn:transfer-process:{}", uuid::Uuid::new_v4()).as_str())
            .expect("Failed to parse transfer-process id");
        let transfer_process = self
            .transfer_process_service
            .create_transfer_process(&NewTransferProcessDto {
                id: Some(transfer_process_id.clone()),
                state: TransferState::REQUESTED.to_string(),
                associated_agent_peer: "".to_string(),
                protocol: "DSP".to_string(),
                transfer_direction: format,
                agreement_id,
                callback_address,
                role: "Provider".to_string(),
                state_attribute: Some("REQUESTED".to_string()),
                properties: None,
                identifiers: Some(identifiers),
            })
            .await
            .expect("Failed to create transfer process");
        self
            .transfer_message_service
            .create_transfer_message(&NewTransferMessageDto {
                id: None,
                transfer_agent_process_id: transfer_process_id.clone(),
                direction: "INBOUND".to_string(),
                protocol: "DSP".to_string(),
                message_type: message_type.to_string(),
                state_transition_from: "-".to_string(),
                state_transition_to: TransferState::REQUESTED.to_string(),
                payload: Some(payload_value),
            })
            .await
            .expect("Failed to create transfer process");
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
        let transfer_process = self
            .transfer_process_service
            .get_transfer_process_by_key_value(&urn_id)
            .await
            .expect("Failed to fetch transfer process");

        let transfer_process_urn = Urn::from_str(transfer_process.inner.id.as_str()).expect("Failed to parse urnID");
        let new_transfer_process = self
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
            .await
            .expect("Failed to create transfer process");
        self
            .transfer_message_service
            .create_transfer_message(&NewTransferMessageDto {
                id: None,
                transfer_agent_process_id: transfer_process_urn.clone(),
                direction: "INBOUND".to_string(),
                protocol: "DSP".to_string(),
                message_type: message_type.to_string(),
                state_transition_from: transfer_process.inner.state.to_string(),
                state_transition_to: new_state.to_string(),
                payload: Some(payload_value),
            })
            .await
            .expect("Failed to create transfer process");
        Ok(new_transfer_process)
    }
}
