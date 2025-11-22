pub(crate) mod http;
mod mapper;
pub(crate) mod orchestrator;
pub(crate) mod protocol_types;
mod state_machine;
mod validator;
mod persistence;

use crate::core::dsp::http::DspRouter;
use crate::core::dsp::orchestrator::orchestrator::OrchestratorService;
use crate::core::dsp::orchestrator::protocol::protocol::ProtocolOrchestratorService;
use crate::core::dsp::state_machine::state_machine::StateMachineForDspService;
use crate::core::dsp::validator::validator::DspValidatorService;
use crate::core::protocol::ProtocolPluginTrait;
use crate::entities::transfer_messages::TransferAgentMessagesTrait;
use crate::entities::transfer_process::TransferAgentProcessesTrait;
use axum::Router;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use std::sync::Arc;
use crate::core::dsp::persistence::persistence::TransferPersistenceService;

pub struct TransferDSP {
    transfer_agent_process_entities: Arc<dyn TransferAgentProcessesTrait>,
    transfer_agent_message_service: Arc<dyn TransferAgentMessagesTrait>,
    config: Arc<ApplicationProviderConfig>,
}

impl TransferDSP {
    pub fn new(
        transfer_agent_message_service: Arc<dyn TransferAgentMessagesTrait>,
        transfer_agent_process_entities: Arc<dyn TransferAgentProcessesTrait>,
        config: Arc<ApplicationProviderConfig>,
    ) -> Self {
        Self { transfer_agent_message_service, transfer_agent_process_entities, config }
    }
}

impl ProtocolPluginTrait for TransferDSP {
    fn name(&self) -> &'static str {
        "Dataspace Protocol"
    }

    fn version(&self) -> &'static str {
        "1.0"
    }

    fn short_name(&self) -> &'static str {
        "DSP"
    }

    fn build_router(&self) -> anyhow::Result<Router> {
        let state_machine_service = Arc::new(StateMachineForDspService::new(
            self.transfer_agent_process_entities.clone(),
            self.config.clone(),
        ));
        let validator_service = Arc::new(DspValidatorService::new());
        let persistence_service = Arc::new(TransferPersistenceService::new(
            self.transfer_agent_message_service.clone(),
            self.transfer_agent_process_entities.clone(),
            self.config.clone(),
        ));

        let http_orchestator = Arc::new(ProtocolOrchestratorService::new(
            state_machine_service.clone(),
            validator_service.clone(),
            persistence_service,
            self.config.clone(),
        ));
        let orchestrator_service = Arc::new(OrchestratorService::new(http_orchestator.clone()));
        let dsp_router = DspRouter::new(orchestrator_service.clone(), self.config.clone());
        Ok(dsp_router.router())
    }

    fn build_grpc_router(&self) -> anyhow::Result<Option<Router>> {
        todo!()
    }
}
