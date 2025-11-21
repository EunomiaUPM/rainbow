use crate::core::dsp::orchestrator::protocol::ProtocolOrchestratorTrait;
use crate::core::dsp::orchestrator::rpc::RPCOrchestratorTrait;
use std::sync::Arc;

pub(crate) mod orchestrator;
pub(crate) mod protocol;
pub(crate) mod rpc;

pub trait OrchestratorTrait: Send + Sync + 'static {
    fn get_protocol_service(&self) -> Arc<dyn ProtocolOrchestratorTrait>;
    fn get_rpc_service(&self) -> Arc<dyn RPCOrchestratorTrait>;
}
