pub mod rpc;

use crate::dsp_common::well_known_types::{DSPProtocolVersions, VersionPath, VersionResponse};
use serde::{Deserialize, Serialize};

pub const DSP_CURRENT_VERSION: DSPProtocolVersions = DSPProtocolVersions::V2025_1;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WellKnownRPCRequest {
    pub participant_id: String,
}
#[async_trait::async_trait]
pub trait WellKnownRPCTrait: Send + Sync {
    async fn fetch_dataspace_well_known(
        &self,
        input: &WellKnownRPCRequest,
    ) -> anyhow::Result<(VersionResponse, String)>;
    async fn fetch_dataspace_current_path(&self, input: &WellKnownRPCRequest) -> anyhow::Result<VersionPath>;
}
