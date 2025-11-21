use crate::core::dsp::orchestrator::protocol::ProtocolOrchestratorTrait;
use crate::core::dsp::orchestrator::rpc::RPCOrchestratorTrait;
use crate::core::dsp::orchestrator::OrchestratorTrait;
use std::sync::Arc;

pub struct OrchestratorService {
    protocol_service: Arc<dyn ProtocolOrchestratorTrait>,
}

impl OrchestratorService {
    pub fn new(protocol_service: Arc<dyn ProtocolOrchestratorTrait>) -> OrchestratorService {
        Self { protocol_service }
    }
}

impl OrchestratorTrait for OrchestratorService {
    fn get_protocol_service(&self) -> Arc<dyn ProtocolOrchestratorTrait> {
        self.protocol_service.clone()
    }

    fn get_rpc_service(&self) -> Arc<dyn RPCOrchestratorTrait> {
        todo!()
    }
}
