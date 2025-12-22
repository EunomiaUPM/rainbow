use crate::dsp_common::well_known_types::DSPProtocolVersions;
use crate::errors::{CommonErrors, ErrorLog};
use crate::facades::ssi_auth_facade::MatesFacadeTrait;
use crate::http_client::HttpClient;
use crate::well_known::rpc::{WellKnownRPCRequest, WellKnownRPCTrait};
use std::sync::Arc;
use tracing::error;

pub struct WellKnownRPCService {
    http_client: Arc<HttpClient>,
    mates_facade: Arc<dyn MatesFacadeTrait>,
}

impl WellKnownRPCService {
    pub fn new(http_client: Arc<HttpClient>, mates_facade: Arc<dyn MatesFacadeTrait>) -> Self {
        Self { http_client, mates_facade }
    }
}

#[async_trait::async_trait]
impl WellKnownRPCTrait for WellKnownRPCService {
    async fn fetch_dataspace_well_known(&self, input: &WellKnownRPCRequest) -> anyhow::Result<DSPProtocolVersions> {
        let mate_id = input.participant_id.clone();
        let participant = self.mates_facade.get_mate_by_id(mate_id).await.map_err(|_e| {
            let err = CommonErrors::missing_resource_new("", "Mate not found");
            error!("{}", err.log());
            anyhow::anyhow!(err)
        })?;
        let base_url = participant.base_url.clone().ok_or_else(|| {
            let err = CommonErrors::missing_resource_new("", "Mate should have base url defined");
            error!("{}", err.log());
            anyhow::anyhow!(err)
        })?;
        let response = self.http_client.get_json::<DSPProtocolVersions>(base_url.as_str()).await?;
        Ok(response)
    }
}
