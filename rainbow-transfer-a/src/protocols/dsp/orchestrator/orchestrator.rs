use crate::protocols::dsp::orchestrator::protocol::ProtocolOrchestratorTrait;
use crate::protocols::dsp::orchestrator::rpc::RPCOrchestratorTrait;
use crate::protocols::dsp::orchestrator::OrchestratorTrait;
use std::sync::Arc;

pub struct OrchestratorService {
    protocol_service: Arc<dyn ProtocolOrchestratorTrait>,
    rpc_service: Arc<dyn RPCOrchestratorTrait>,
}

impl OrchestratorService {
    pub fn new(
        protocol_service: Arc<dyn ProtocolOrchestratorTrait>,
        rpc_service: Arc<dyn RPCOrchestratorTrait>,
    ) -> OrchestratorService {
        Self { protocol_service, rpc_service }
    }
}

impl OrchestratorTrait for OrchestratorService {
    fn get_protocol_service(&self) -> Arc<dyn ProtocolOrchestratorTrait> {
        self.protocol_service.clone()
    }

    fn get_rpc_service(&self) -> Arc<dyn RPCOrchestratorTrait> {
        self.rpc_service.clone()
    }
}
