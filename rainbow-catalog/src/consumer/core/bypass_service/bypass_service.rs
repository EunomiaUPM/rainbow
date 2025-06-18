use crate::consumer::core::bypass_service::ByPassTrait;
use crate::consumer::core::mates_facade::MatesFacadeTrait;
use anyhow::bail;
use axum::async_trait;
use log::debug;
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use urn::Urn;

pub struct CatalogBypassService<T>
where
    T: MatesFacadeTrait + Send + Sync + 'static,
{
    mates_facade: Arc<T>,
    client: Client,
}

impl<T> CatalogBypassService<T>
where
    T: MatesFacadeTrait + Send + Sync + 'static,
{
    pub fn new(mates_facade: Arc<T>) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { mates_facade, client }
    }
}

#[async_trait]
impl<T> ByPassTrait for CatalogBypassService<T>
where
    T: MatesFacadeTrait + Send + Sync + 'static,
{
    async fn bypass(&self, participant_id: Urn, path: String) -> anyhow::Result<Value> {
        let mate = self.mates_facade.get_mate_by_id(participant_id).await;
        let mate = match mate {
            Ok(mate) => mate,
            Err(e) => bail!("Mate not found: {}", e.to_string())
        };
        let base_url = mate.base_url.expect("Base url not found");
        let request_url = format!("{}/api/v1/{}", base_url, path);
        debug!("{}", request_url);
        let request = self.client
            .get(&request_url)
            .send()
            .await;
        let request = match request {
            Ok(request) => request,
            Err(e) => bail!("Not able to connect: {}", e.to_string())
        };
        if request.status().is_success() == false {
            bail!("Bypass failed: {}", request.text().await?);
        }
        let response = request.json().await?;
        Ok(response)
    }
}
