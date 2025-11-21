mod http;
mod orchestrator;
mod protocol;

use crate::core::protocol::ProtocolPluginTrait;
use axum::Router;

pub struct TransferDSP {}

impl TransferDSP {}

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
        todo!()
    }

    fn build_grpc_router(&self) -> anyhow::Result<Option<Router>> {
        todo!()
    }
}
