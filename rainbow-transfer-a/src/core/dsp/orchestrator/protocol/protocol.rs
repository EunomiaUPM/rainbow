use crate::core::dsp::orchestrator::protocol::ProtocolOrchestratorTrait;
use crate::core::dsp::protocol_types::{
    TransferCompletionMessageDto, TransferErrorDto, TransferProcessAckDto, TransferProcessMessageType,
    TransferProcessMessageWrapper, TransferRequestMessageDto, TransferStartMessageDto, TransferSuspensionMessageDto,
    TransferTerminationMessageDto,
};
use crate::core::dsp::state_machine::StateMachineTrait;
use crate::core::dsp::validator::ValidatorTrait;
use crate::entities::transfer_messages::TransferAgentMessagesTrait;
use crate::entities::transfer_process::{TransferAgentProcessesTrait, TransferProcessDto};
use crate::http::common::parse_urn;
use log::error;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::config::ConfigRoles;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::context_field::ContextField;
use std::sync::Arc;
use std::task::Context;
use tracing::debug;

pub struct ProtocolOrchestratorService {
    pub state_machine_service: Arc<dyn StateMachineTrait>,
    pub validator_service: Arc<dyn ValidatorTrait>,
    pub transfer_message_service: Arc<dyn TransferAgentMessagesTrait>,
    pub transfer_process_service: Arc<dyn TransferAgentProcessesTrait>,
    pub config: Arc<ApplicationProviderConfig>,
}

impl ProtocolOrchestratorService {
    pub fn new(
        state_machine_service: Arc<dyn StateMachineTrait>,
        validator_service: Arc<dyn ValidatorTrait>,
        transfer_message_service: Arc<dyn TransferAgentMessagesTrait>,
        transfer_process_service: Arc<dyn TransferAgentProcessesTrait>,
        config: Arc<ApplicationProviderConfig>,
    ) -> ProtocolOrchestratorService {
        ProtocolOrchestratorService {
            state_machine_service,
            validator_service,
            transfer_message_service,
            transfer_process_service,
            config,
        }
    }
}

#[async_trait::async_trait]
impl ProtocolOrchestratorTrait for ProtocolOrchestratorService {
    async fn on_get_transfer_process(
        &self,
        id: &String,
    ) -> anyhow::Result<
        TransferProcessMessageWrapper<TransferProcessAckDto>,
        TransferProcessMessageWrapper<TransferErrorDto>,
    > {
        let role = self.config.role.clone();
        let key = match role {
            ConfigRoles::Consumer => "consumerPid",
            ConfigRoles::Provider => "providerPid",
            _ => {
                let err = CommonErrors::parse_new("Something went wrong. Role should be 'Consumer' or 'provider'");
                error!("{}", err.log());
                return Err(TransferProcessMessageWrapper {
                    context: ContextField::default(),
                    _type: TransferProcessMessageType::TransferError,
                    dto: TransferErrorDto { consumer_pid: None, provider_pid: None, code: None, reason: None },
                });
            }
        };
        let urn = parse_urn(id).unwrap(); // TODO resolve parse urn
        let transfer_process =
            self.transfer_process_service.get_transfer_process_by_key_id(key, &urn).await.map_err(|e| {
                let err = CommonErrors::missing_resource_new(urn.to_string().as_str(), "Process service not found");
                error!("{}", err.log());
                TransferProcessMessageWrapper {
                    context: ContextField::default(),
                    _type: TransferProcessMessageType::TransferError,
                    dto: TransferErrorDto { consumer_pid: None, provider_pid: None, code: None, reason: None },
                }
            })?;
        debug!("{:?}", transfer_process);
        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process).unwrap();
        Ok(transfer_process_dto)
    }

    async fn on_transfer_request(
        &self,
        input: &TransferProcessMessageWrapper<TransferRequestMessageDto>,
    ) -> anyhow::Result<
        TransferProcessMessageWrapper<TransferProcessAckDto>,
        TransferProcessMessageWrapper<TransferErrorDto>,
    > {
        todo!()
    }

    async fn on_transfer_start(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferStartMessageDto>,
    ) -> anyhow::Result<
        TransferProcessMessageWrapper<TransferProcessAckDto>,
        TransferProcessMessageWrapper<TransferErrorDto>,
    > {
        todo!()
    }

    async fn on_transfer_suspension(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferSuspensionMessageDto>,
    ) -> anyhow::Result<
        TransferProcessMessageWrapper<TransferProcessAckDto>,
        TransferProcessMessageWrapper<TransferErrorDto>,
    > {
        todo!()
    }

    async fn on_transfer_completion(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferCompletionMessageDto>,
    ) -> anyhow::Result<
        TransferProcessMessageWrapper<TransferProcessAckDto>,
        TransferProcessMessageWrapper<TransferErrorDto>,
    > {
        todo!()
    }

    async fn on_transfer_termination(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferTerminationMessageDto>,
    ) -> anyhow::Result<
        TransferProcessMessageWrapper<TransferProcessAckDto>,
        TransferProcessMessageWrapper<TransferErrorDto>,
    > {
        todo!()
    }
}
