/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::provider::core::catalog_odrl_facade::CatalogOdrlFacadeTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use urn::Urn;

pub struct CatalogOdrlFacadeService {
    client: Client,
}
impl CatalogOdrlFacadeService {
    pub fn new() -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { client }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RainbowRPCCatalogResolveOfferByIdRequest {
    #[serde(rename = "offerId")]
    pub offer_id: Urn,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RainbowRPCCatalogResolveEntityTargetRequest {
    #[serde(rename = "target")]
    pub target: Urn,
    #[serde(rename = "entityType")]
    pub entity_type: String,
}

#[async_trait]
impl CatalogOdrlFacadeTrait for CatalogOdrlFacadeService {
    async fn resolve_odrl_offers(&self, offer_id: Urn) -> anyhow::Result<OdrlOffer> {
        // TODO !important this couldn't be hardcoded...
        let res = self.client
            .post("http://127.0.0.1:1200/api/v1/catalog/rpc/resolve-offer")
            .json(&RainbowRPCCatalogResolveOfferByIdRequest { offer_id })
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
            .post("http://127.0.0.1:1200/api/v1/catalog/rpc/resolve-entity-target")
            .json(&RainbowRPCCatalogResolveEntityTargetRequest { target, entity_type })
            .send()
            .await?;
        if res.status().is_success() {
            Ok(())
        } else {
            bail!("id not found")
        }
    }
}
