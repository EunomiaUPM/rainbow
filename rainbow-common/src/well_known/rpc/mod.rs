pub mod rpc;

use crate::dsp_common::well_known_types::DSPProtocolVersions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WellKnownRPCRequest {
    pub participant_id: String,
}

#[async_trait::async_trait]
pub trait WellKnownRPCTrait: Send + Sync {
    async fn fetch_dataspace_well_known(&self, input: &WellKnownRPCRequest) -> anyhow::Result<DSPProtocolVersions>;
}
