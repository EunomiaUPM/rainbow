/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! The catalog contract over the catalog agent's management API.

use async_trait::async_trait;
use axum::http::StatusCode;
use urn::Urn;
use ymir::config::types::HostType;
use ymir::errors::{Errors, Outcome, PetitionFailure};
use ymir::services::client::ClientExt;
use ymir::types::oauth::UserInfo;
use ymir::utils::http_client;

use crate::config::types::min_known_config::MinKnownConfig;
use crate::config::types::traits::MinKnownConfigTrait;
use crate::facades::catalog::{
    CatalogDataset, CatalogDistribution, CatalogFacadeTrait, CatalogOffer,
};

/// Reads the catalog agent's API. Calls carry no user token yet, as for the auth facades.
pub struct CatalogRemoteFacade {
    base_url: String,
}

impl CatalogRemoteFacade {
    /// Path segment the catalog agent mounts its API under.
    const SERVICE: &'static str = "catalog-agent";

    pub fn new(catalog: &MinKnownConfig) -> Self {
        Self {
            base_url: format!(
                "{}{}/{}",
                catalog.get_host(HostType::Http),
                catalog.get_api_version(),
                Self::SERVICE
            ),
        }
    }

    /// GETs `path`; a 404 becomes the missing-resource error the local adapter returns.
    async fn get<T: serde::de::DeserializeOwned + Send>(
        &self,
        path: &str,
        what: &str,
    ) -> Outcome<T> {
        let url = format!("{}/{path}", self.base_url);
        match http_client().get_json(&url, None).await {
            Err(Errors::PetitionError {
                failure: PetitionFailure::HttpStatus(StatusCode::NOT_FOUND),
                ..
            }) => Err(Errors::missing_resource(
                path,
                format!("{what} not found in the catalog"),
                None,
            )),
            other => other,
        }
    }
}

#[async_trait]
impl CatalogFacadeTrait for CatalogRemoteFacade {
    #[tracing::instrument(level = "info", skip_all, err, fields(peer.service = "catalog"))]
    async fn get_dataset(&self, dataset_id: &Urn) -> Outcome<CatalogDataset> {
        self.get(&format!("datasets/{dataset_id}"), "Dataset").await
    }

    #[tracing::instrument(level = "info", skip_all, err, fields(peer.service = "catalog"))]
    async fn get_distribution_by_format(
        &self,
        dataset_id: &Urn,
        format: &str,
    ) -> Outcome<CatalogDistribution> {
        let path = format!("distributions/dataset/{dataset_id}/format/{format}");
        self.get(&path, "Distribution").await
    }

    #[tracing::instrument(level = "info", skip_all, err, fields(peer.service = "catalog"))]
    async fn get_distribution(
        &self,
        _user: &UserInfo,
        distribution_id: &Urn,
    ) -> Outcome<CatalogDistribution> {
        self.get(&format!("distributions/{distribution_id}"), "Distribution")
            .await
    }

    #[tracing::instrument(level = "info", skip_all, err, fields(peer.service = "catalog"))]
    async fn get_offer(&self, offer_id: &Urn) -> Outcome<CatalogOffer> {
        self.get(&format!("odrl-policies/{offer_id}"), "Offer")
            .await
    }
}
