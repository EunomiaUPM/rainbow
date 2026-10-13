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

//! The catalog's published contract served in-process, over its own services.

use std::sync::Arc;

use common::facades::catalog::{
    CatalogDataset, CatalogDistribution, CatalogFacadeTrait, CatalogOffer,
};
use common::oauth::UserInfo;
use urn::Urn;
use ymir::errors::{Errors, Outcome};

use crate::entities::datasets::DatasetDto;
use crate::entities::distributions::DistributionDto;
use crate::entities::odrl_policies::OdrlPolicyDto;
use crate::services::datasets::DatasetServiceTrait;
use crate::services::distributions::DistributionServiceTrait;
use crate::services::odrl_policies::OdrlPolicyServiceTrait;

/// The catalog for the agents and the connector sharing its process.
pub struct CatalogLocalFacade {
    datasets: Arc<dyn DatasetServiceTrait>,
    distributions: Arc<dyn DistributionServiceTrait>,
    offers: Arc<dyn OdrlPolicyServiceTrait>,
}

impl CatalogLocalFacade {
    pub fn new(
        datasets: Arc<dyn DatasetServiceTrait>,
        distributions: Arc<dyn DistributionServiceTrait>,
        offers: Arc<dyn OdrlPolicyServiceTrait>,
    ) -> Self {
        Self {
            datasets,
            distributions,
            offers,
        }
    }

    /// A missing record keeps its missing-resource error, with `why` as the reason.
    fn not_found<T>(result: Outcome<T>, id: &Urn, why: &str) -> Outcome<T> {
        match result {
            Err(Errors::MissingResourceError { .. }) => {
                Err(Errors::missing_resource(id.to_string(), why, None))
            }
            other => other,
        }
    }

    fn dataset(dto: DatasetDto) -> CatalogDataset {
        CatalogDataset {
            id: dto.inner.id,
            catalog_id: dto.inner.catalog_id,
            dct_title: dto.inner.dct_title,
        }
    }

    fn distribution(dto: DistributionDto) -> CatalogDistribution {
        CatalogDistribution {
            id: dto.inner.id,
            dataset_id: dto.inner.dataset_id,
            dct_format: dto.inner.dct_format,
            dcat_access_service: dto.inner.dcat_access_service,
        }
    }

    fn offer(dto: OdrlPolicyDto) -> CatalogOffer {
        CatalogOffer {
            id: dto.inner.id,
            entity: dto.inner.entity,
            entity_type: dto.inner.entity_type,
            odrl_offer: dto.inner.odrl_offer,
        }
    }
}

#[async_trait::async_trait]
impl CatalogFacadeTrait for CatalogLocalFacade {
    #[tracing::instrument(level = "info", skip_all, err, fields(peer.service = "catalog"))]
    async fn get_dataset(&self, dataset_id: &Urn) -> Outcome<CatalogDataset> {
        let found = self
            .datasets
            .get_dataset_by_id(&UserInfo::system(), dataset_id)
            .await;
        Self::not_found(found, dataset_id, "Dataset not found in the catalog").map(Self::dataset)
    }

    #[tracing::instrument(level = "info", skip_all, err, fields(peer.service = "catalog"))]
    async fn get_distribution_by_format(
        &self,
        dataset_id: &Urn,
        format: &str,
    ) -> Outcome<CatalogDistribution> {
        self.distributions
            .get_distribution_by_dataset_id_and_dct_format(&UserInfo::system(), dataset_id, format)
            .await
            .map(Self::distribution)
    }

    #[tracing::instrument(
        level = "info",
        skip_all,
        err,
        fields(peer.service = "catalog", user = %user.id())
    )]
    async fn get_distribution(
        &self,
        user: &UserInfo,
        distribution_id: &Urn,
    ) -> Outcome<CatalogDistribution> {
        let found = self
            .distributions
            .get_distribution_by_id(user, distribution_id)
            .await;
        let why = "Distribution not found among those the user sees";
        Self::not_found(found, distribution_id, why).map(Self::distribution)
    }

    #[tracing::instrument(level = "info", skip_all, err, fields(peer.service = "catalog"))]
    async fn get_offer(&self, offer_id: &Urn) -> Outcome<CatalogOffer> {
        let found = self
            .offers
            .get_odrl_offer_by_id(&UserInfo::system(), offer_id)
            .await;
        Self::not_found(found, offer_id, "Offer not found in the catalog").map(Self::offer)
    }
}
