mod errors;
pub(crate) mod facades;
pub(crate) mod http;
pub(crate) mod orchestrator;
mod persistence;
pub(crate) mod protocol_types;
pub(crate) mod validator;

use crate::entities::transfer_messages::TransferAgentMessagesTrait;
use crate::entities::transfer_process::TransferAgentProcessesTrait;
use crate::protocols::dsp::http::protocol::DspRouter;
use crate::protocols::dsp::http::rpc::RpcRouter;
use crate::protocols::dsp::orchestrator::orchestrator::OrchestratorService;
use crate::protocols::dsp::orchestrator::protocol::protocol::ProtocolOrchestratorService;
use crate::protocols::dsp::orchestrator::rpc::rpc::RPCOrchestratorService;
use crate::protocols::dsp::persistence::persistence_protocol::TransferPersistenceForProtocolService;
use crate::protocols::dsp::persistence::persistence_rpc::TransferPersistenceForRpcService;
use crate::protocols::dsp::validator::validators::protocol::validation_dsp_steps::ValidationDspStepsService;
use crate::protocols::dsp::validator::validators::rpc::validate_state_transition::ValidatedStateTransitionServiceForRcp;
use crate::protocols::dsp::validator::validators::validate_payload::ValidatePayloadService;
use crate::protocols::dsp::validator::validators::validation_helpers::ValidationHelperService;
use crate::protocols::protocol::ProtocolPluginTrait;
use axum::Router;
use rainbow_common::config::provider::config::ApplicationProviderConfig;
use rainbow_common::http_client::HttpClient;
use std::sync::Arc;
use validator::validators::protocol::validate_state_transition::ValidatedStateTransitionServiceForDsp;
use validator::validators::rpc::validation_rpc_steps::ValidationRpcStepsService;

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
        let http_client = Arc::new(HttpClient::new(10, 10));

        let validator_helper = Arc::new(ValidationHelperService::new(
            self.transfer_agent_process_entities.clone(),
        ));
        let validator_payload = Arc::new(ValidatePayloadService::new(validator_helper.clone()));
        let validator_state_machine_dsp = Arc::new(ValidatedStateTransitionServiceForDsp::new(
            validator_helper.clone(),
        ));
        let dsp_validator = Arc::new(ValidationDspStepsService::new(
            validator_payload.clone(),
            validator_state_machine_dsp.clone(),
            validator_helper.clone(),
        ));
        let validator_state_machine_rcp = Arc::new(ValidatedStateTransitionServiceForRcp::new(
            validator_helper.clone(),
        ));
        let rcp_validator = Arc::new(ValidationRpcStepsService::new(
            validator_payload.clone(),
            validator_state_machine_rcp.clone(),
            validator_helper.clone(),
        ));

        let persistence_protocol_service = Arc::new(TransferPersistenceForProtocolService::new(
            self.transfer_agent_message_service.clone(),
            self.transfer_agent_process_entities.clone(),
        ));
        let persistence_rpc_service = Arc::new(TransferPersistenceForRpcService::new(
            self.transfer_agent_message_service.clone(),
            self.transfer_agent_process_entities.clone(),
        ));
        let http_orchestator = Arc::new(ProtocolOrchestratorService::new(
            dsp_validator.clone(),
            persistence_protocol_service.clone(),
            self.config.clone(),
        ));
        let rpc_orchestator = Arc::new(RPCOrchestratorService::new(
            rcp_validator.clone(),
            persistence_rpc_service,
            self.config.clone(),
            http_client.clone(),
        ));
        let orchestrator_service = Arc::new(OrchestratorService::new(
            http_orchestator.clone(),
            rpc_orchestator.clone(),
        ));
        let dsp_router = DspRouter::new(orchestrator_service.clone(), self.config.clone());
        let rcp_router = RpcRouter::new(orchestrator_service.clone(), self.config.clone());

        Ok(Router::new().merge(dsp_router.router()).merge(rcp_router.router()))
    }

    fn build_grpc_router(&self) -> anyhow::Result<Option<Router>> {
        todo!()
    }
}
