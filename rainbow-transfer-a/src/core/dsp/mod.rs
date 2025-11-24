mod errors;
pub(crate) mod http;
mod mapper;
pub(crate) mod orchestrator;
mod persistence;
pub(crate) mod protocol_types;
mod state_machine;
mod validator;
mod services;

use crate::core::dsp::http::DspRouter;
use crate::core::dsp::orchestrator::orchestrator::OrchestratorService;
use crate::core::dsp::orchestrator::protocol::protocol::ProtocolOrchestratorService;
use crate::core::dsp::orchestrator::rpc::rpc::RPCOrchestratorService;
use crate::core::dsp::state_machine::state_machine_protocol::StateMachineForProtocolService;
use crate::core::dsp::validator::validator_protocol::ValidatorProtocolService;
use crate::core::protocol::ProtocolPluginTrait;
use crate::entities::transfer_messages::TransferAgentMessagesTrait;
use crate::entities::transfer_process::TransferAgentProcessesTrait;
use axum::Router;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::http_client::HttpClient;
use std::sync::Arc;
use crate::core::dsp::persistence::persistence_protocol::TransferPersistenceForProtocolService;
use crate::core::dsp::persistence::persistence_rpc::TransferPersistenceForRpcService;
use crate::core::dsp::state_machine::state_machine_rpc::StateMachineForRpcService;
use crate::core::dsp::validator::validator_rpc::ValidatorRpcService;

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
        let state_machine_protocol_service = Arc::new(StateMachineForProtocolService::new(
            self.transfer_agent_process_entities.clone(),
            self.config.clone(),
        ));
        let state_machine_rpc_service = Arc::new(StateMachineForRpcService::new(
            self.transfer_agent_process_entities.clone(),
            self.config.clone(),
        ));
        let validator_protocol_service = Arc::new(ValidatorProtocolService::new(
            self.transfer_agent_process_entities.clone(),
        ));
        let validator_rpc_service = Arc::new(ValidatorRpcService::new());
        let persistence_protocol_service = Arc::new(TransferPersistenceForProtocolService::new(
            self.transfer_agent_message_service.clone(),
            self.transfer_agent_process_entities.clone(),
        ));
        let persistence_rpc_service = Arc::new(TransferPersistenceForRpcService::new(
            self.transfer_agent_message_service.clone(),
            self.transfer_agent_process_entities.clone(),
        ));
        let http_orchestator = Arc::new(ProtocolOrchestratorService::new(
            state_machine_protocol_service.clone(),
            validator_protocol_service.clone(),
            persistence_protocol_service.clone(),
            self.config.clone(),
        ));
        let rpc_orchestator = Arc::new(RPCOrchestratorService::new(
            state_machine_rpc_service.clone(),
            validator_rpc_service.clone(),
            persistence_rpc_service,
            self.config.clone(),
            http_client.clone(),
        ));
        let orchestrator_service = Arc::new(OrchestratorService::new(
            http_orchestator.clone(),
            rpc_orchestator.clone(),
        ));
        let dsp_router = DspRouter::new(orchestrator_service.clone(), self.config.clone());
        Ok(dsp_router.router())
    }

    fn build_grpc_router(&self) -> anyhow::Result<Option<Router>> {
        todo!()
    }
}
