use crate::dsp_common::well_known_types::{VersionPath, VersionResponse};
use crate::errors::{CommonErrors, ErrorLog};
use crate::facades::ssi_auth_facade::MatesFacadeTrait;
use crate::http_client::HttpClient;
use crate::well_known::rpc::{WellKnownRPCRequest, WellKnownRPCTrait, DSP_CURRENT_VERSION};
use anyhow::bail;
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
    async fn get_base_url(&self, mate_id: String) -> anyhow::Result<String> {
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
        Ok(base_url)
    }
}

#[async_trait::async_trait]
impl WellKnownRPCTrait for WellKnownRPCService {
    async fn fetch_dataspace_well_known(
        &self,
        input: &WellKnownRPCRequest,
    ) -> anyhow::Result<(VersionResponse, String)> {
        let mate_id = input.participant_id.clone();
        let base_url = self.get_base_url(mate_id).await?;
        let url = format!("{}/.well-known/dspace-version", base_url);
        let response = self.http_client.get_json::<VersionResponse>(url.as_str()).await?;
        Ok((response, base_url))
    }

    async fn fetch_dataspace_current_path(&self, input: &WellKnownRPCRequest) -> anyhow::Result<VersionPath> {
        let (wk, base_url) = self.fetch_dataspace_well_known(input).await?;

        let current = wk.protocol_versions.iter().find(|p| p.version == DSP_CURRENT_VERSION);
        if current.is_none() {
            let err = CommonErrors::parse_new("Could not find protocol version");
            error!("{}", err.log());
            bail!(err);
        }
        let current = current.unwrap();
        let path = format!("{}{}", base_url, current.path);
        Ok(VersionPath { path })
    }
}
