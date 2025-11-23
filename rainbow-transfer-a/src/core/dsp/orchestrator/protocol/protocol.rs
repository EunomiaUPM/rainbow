use crate::core::dsp::orchestrator::protocol::ProtocolOrchestratorTrait;
use crate::core::dsp::persistence::TransferPersistenceTrait;
use crate::core::dsp::protocol_types::{
    TransferCompletionMessageDto, TransferErrorDto, TransferProcessAckDto, TransferProcessMessageType,
    TransferProcessMessageWrapper, TransferRequestMessageDto, TransferStartMessageDto, TransferSuspensionMessageDto,
    TransferTerminationMessageDto,
};
use crate::core::dsp::state_machine::StateMachineTrait;
use crate::core::dsp::validator::ValidatorTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::errors::CommonErrors;
use rainbow_common::protocol::context_field::ContextField;
use std::sync::Arc;

pub struct ProtocolOrchestratorService {
    pub state_machine_service: Arc<dyn StateMachineTrait>,
    pub validator_service: Arc<dyn ValidatorTrait>,
    pub persistence_service: Arc<dyn TransferPersistenceTrait>,
    pub config: Arc<ApplicationProviderConfig>,
}

impl ProtocolOrchestratorService {
    pub fn new(
        state_machine_service: Arc<dyn StateMachineTrait>,
        validator_service: Arc<dyn ValidatorTrait>,
        persistence_service: Arc<dyn TransferPersistenceTrait>,
        config: Arc<ApplicationProviderConfig>,
    ) -> ProtocolOrchestratorService {
        ProtocolOrchestratorService { state_machine_service, validator_service, persistence_service, config }
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
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        let input = Arc::new(input.clone());
        self.state_machine_service.validate_transition(None, Arc::new(input.dto.clone())).await?;
        self.validator_service.validate(None, Arc::new(input.dto.clone())).await?;
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
        Ok(transfer_process_dto)
    }

    async fn on_transfer_start(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferStartMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        let input = Arc::new(input.clone());
        self.state_machine_service.validate_transition(None, Arc::new(input.dto.clone())).await?;
        self.validator_service.validate(None, Arc::new(input.dto.clone())).await?;

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
        let input = Arc::new(input.clone());
        self.state_machine_service.validate_transition(None, Arc::new(input.dto.clone())).await?;
        self.validator_service.validate(None, Arc::new(input.dto.clone())).await?;

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
        let input = Arc::new(input.clone());
        self.state_machine_service.validate_transition(None, Arc::new(input.dto.clone())).await?;
        self.validator_service.validate(None, Arc::new(input.dto.clone())).await?;

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
        let input = Arc::new(input.clone());
        self.state_machine_service.validate_transition(None, Arc::new(input.dto.clone())).await?;
        self.validator_service.validate(None, Arc::new(input.dto.clone())).await?;

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
