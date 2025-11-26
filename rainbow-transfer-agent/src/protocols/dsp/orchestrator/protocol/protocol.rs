use crate::protocols::dsp::orchestrator::protocol::ProtocolOrchestratorTrait;
use crate::protocols::dsp::persistence::TransferPersistenceTrait;
use crate::protocols::dsp::protocol_types::{
    TransferCompletionMessageDto, TransferProcessAckDto, TransferProcessMessageTrait, TransferProcessMessageWrapper,
    TransferRequestMessageDto, TransferStartMessageDto, TransferSuspensionMessageDto, TransferTerminationMessageDto,
};

use crate::protocols::dsp::validator::traits::validation_dsp_steps::ValidationDspSteps;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use std::sync::Arc;

pub struct ProtocolOrchestratorService {
    validator: Arc<dyn ValidationDspSteps>,
    pub persistence_service: Arc<dyn TransferPersistenceTrait>,
    pub _config: Arc<ApplicationProviderConfig>,
}

impl ProtocolOrchestratorService {
    pub fn new(
        validator: Arc<dyn ValidationDspSteps>,
        persistence_service: Arc<dyn TransferPersistenceTrait>,
        _config: Arc<ApplicationProviderConfig>,
    ) -> ProtocolOrchestratorService {
        ProtocolOrchestratorService { validator, persistence_service, _config }
    }
}

#[async_trait::async_trait]
impl ProtocolOrchestratorTrait for ProtocolOrchestratorService {
    async fn on_get_transfer_process(
        &self,
        id: &String,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        let transfer_process = self.persistence_service.fetch_process(id.as_str()).await?;
        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok(transfer_process_dto)
    }

    async fn on_transfer_request(
        &self,
        input: &TransferProcessMessageWrapper<TransferRequestMessageDto>,
    ) -> anyhow::Result<(TransferProcessMessageWrapper<TransferProcessAckDto>, bool)> {
        // transform and validate
        let input = Arc::new(input.clone());
        self.validator.on_transfer_request(&input).await?;

        // check idempotency
        let consumer_pid = input.dto.get_consumer_pid().unwrap();
        let process_result = self
            .persistence_service
            .get_transfer_process_service()
            .await?
            .get_transfer_process_by_key_id("consumerPid", &consumer_pid)
            .await;
        match process_result {
            Ok(transfer_process) => {
                let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
                return Ok((transfer_process_dto, true));
            }
            _ => {}
        }

        // persist and send
        let transfer_process = self
            .persistence_service
            .create_process(
                "DSP",
                "INBOUND",
                None,
                None,
                Arc::new(input.dto.clone()),
                serde_json::to_value(input).unwrap(),
            )
            .await?;
        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok((transfer_process_dto, false))
    }

    async fn on_transfer_start(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferStartMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        self.validator.on_transfer_start(id, input).await?;

        let transfer_process = self
            .persistence_service
            .update_process(
                id,
                Arc::new(input.dto.clone()),
                serde_json::to_value(input).unwrap(),
            )
            .await?;
        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok(transfer_process_dto)
    }

    async fn on_transfer_suspension(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferSuspensionMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        self.validator.on_transfer_suspension(id, input).await?;

        let transfer_process = self
            .persistence_service
            .update_process(
                id,
                Arc::new(input.dto.clone()),
                serde_json::to_value(input).unwrap(),
            )
            .await?;
        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok(transfer_process_dto)
    }

    async fn on_transfer_completion(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferCompletionMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        self.validator.on_transfer_completion(id, input).await?;

        let transfer_process = self
            .persistence_service
            .update_process(
                id,
                Arc::new(input.dto.clone()),
                serde_json::to_value(input).unwrap(),
            )
            .await?;
        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok(transfer_process_dto)
    }

    async fn on_transfer_termination(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferTerminationMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        self.validator.on_transfer_termination(id, input).await?;

        let transfer_process = self
            .persistence_service
            .update_process(
                id,
                Arc::new(input.dto.clone()),
                serde_json::to_value(input).unwrap(),
            )
            .await?;
        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok(transfer_process_dto)
    }
}
