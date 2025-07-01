use crate::provider::core::catalog_odrl_facade::CatalogOdrlFacadeTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use urn::Urn;

pub struct DatahubOdrlFacadeService {
    client: Client,
}
impl DatahubOdrlFacadeService {
    pub fn new() -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { client }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RainbowRPCDatahubCatalogResolveDataServiceRequest {
    #[serde(rename = "offerId")]
    pub offer_id: Urn,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RainbowRPCDatahubCatalogResolveOfferByIdRequest {
    #[serde(rename = "target")]
    pub target: Urn,
    #[serde(rename = "entityType")]
    pub entity_type: String,
}

#[async_trait]
impl CatalogOdrlFacadeTrait for DatahubOdrlFacadeService {
    async fn resolve_odrl_offers(&self, offer_id: Urn) -> anyhow::Result<OdrlOffer> {
        let res = self.client
            .post("http://127.0.0.1:1200/api/v1/datahub/rpc/resolve-offer")
            .json(&RainbowRPCDatahubCatalogResolveDataServiceRequest { offer_id })
            .send()
            .await?;
        if res.status().is_success() {
            let res_json = res.json::<OdrlOffer>().await?;
            Ok(res_json)
        } else {
            bail!("id not found")
        }
    }

    async fn resolve_catalog_target(&self, target: Urn, entity_type: String) -> anyhow::Result<()> {
        let res = self.client
            .post("http://127.0.0.1:1200/api/v1/datahub/rpc/resolve-entity-target")
            .json(&RainbowRPCDatahubCatalogResolveOfferByIdRequest { target, entity_type })
            .send()
            .await?;
        if res.status().is_success() {
            Ok(())
        } else {
            bail!("id not found")
        }
    }
}
