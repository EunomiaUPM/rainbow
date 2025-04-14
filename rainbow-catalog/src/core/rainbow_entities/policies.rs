/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use crate::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::core::rainbow_entities::RainbowPoliciesTrait;
use crate::protocol::policies::EntityTypes;
use axum::async_trait;
use rainbow_common::utils::get_urn;
use rainbow_db::catalog::entities::odrl_offer::Model;
use rainbow_db::catalog::repo::{NewOdrlOfferModel, OdrlOfferRepo};
use serde_json::Value;
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogPoliciesService<T>
where
    T: OdrlOfferRepo + Send + Sync,
{
    repo: Arc<T>,
}

impl<T> RainbowCatalogPoliciesService<T>
where
    T: OdrlOfferRepo + Send + Sync,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> RainbowPoliciesTrait for RainbowCatalogPoliciesService<T>
where
    T: OdrlOfferRepo + Send + Sync,
{
    async fn get_catalog_policies(&self, catalog_id: Urn) -> anyhow::Result<Vec<Model>> {
        let policies = self.repo.get_all_odrl_offers_by_entity(catalog_id).await.map_err(CatalogError::DbErr)?;
        Ok(policies)
    }

    async fn post_catalog_policies(&self, catalog_id: Urn, policy: Value) -> anyhow::Result<Model> {
        let new_policy = self
            .repo
            .create_odrl_offer(
                catalog_id.clone(),
                EntityTypes::Catalog.to_string(),
                NewOdrlOfferModel {
                    id: Option::from(get_urn(None)),
                    odrl_offers: Option::from(policy),
                    entity: catalog_id,
                    entity_type: EntityTypes::Catalog.to_string(),
                },
            )
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(new_policy)
    }

    async fn delete_catalog_policies(&self, catalog_id: Urn, policy_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_odrl_offer_by_id(policy_id).await.map_err(CatalogError::DbErr)?;
        Ok(())
    }

    async fn get_dataset_policies(&self, dataset_id: Urn) -> anyhow::Result<Vec<Model>> {
        let policies = self.repo.get_all_odrl_offers_by_entity(dataset_id).await.map_err(CatalogError::DbErr)?;
        Ok(policies)
    }

    async fn post_dataset_policies(&self, dataset_id: Urn, policy: Value) -> anyhow::Result<Model> {
        let new_policy = self
            .repo
            .create_odrl_offer(
                dataset_id.clone(),
                EntityTypes::Dataset.to_string(),
                NewOdrlOfferModel {
                    id: Option::from(get_urn(None)),
                    odrl_offers: Option::from(policy),
                    entity: dataset_id,
                    entity_type: EntityTypes::Dataset.to_string(),
                },
            )
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(new_policy)
    }

    async fn delete_dataset_policies(&self, dataset_id: Urn, policy_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_odrl_offer_by_id(policy_id).await.map_err(CatalogError::DbErr)?;
        Ok(())
    }

    async fn get_data_service_policies(&self, data_service_id: Urn) -> anyhow::Result<Vec<Model>> {
        let policies = self.repo.get_all_odrl_offers_by_entity(data_service_id).await.map_err(CatalogError::DbErr)?;
        Ok(policies)
    }

    async fn post_data_service_policies(&self, data_service_id: Urn, policy: Value) -> anyhow::Result<Model> {
        let new_policy = self
            .repo
            .create_odrl_offer(
                data_service_id.clone(),
                EntityTypes::DataService.to_string(),
                NewOdrlOfferModel {
                    id: Option::from(get_urn(None)),
                    odrl_offers: Option::from(policy),
                    entity: data_service_id,
                    entity_type: EntityTypes::DataService.to_string(),
                },
            )
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(new_policy)
    }

    async fn delete_data_service_policies(&self, data_service_id: Urn, policy_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_odrl_offer_by_id(policy_id).await.map_err(CatalogError::DbErr)?;
        Ok(())
    }

    async fn get_distribution_policies(&self, distribution_id: Urn) -> anyhow::Result<Vec<Model>> {
        let policies = self.repo.get_all_odrl_offers_by_entity(distribution_id).await.map_err(CatalogError::DbErr)?;
        Ok(policies)
    }

    async fn post_distribution_policies(&self, distribution_id: Urn, policy: Value) -> anyhow::Result<Model> {
        let new_policy = self
            .repo
            .create_odrl_offer(
                distribution_id.clone(),
                EntityTypes::Distribution.to_string(),
                NewOdrlOfferModel {
                    id: Option::from(get_urn(None)),
                    odrl_offers: Option::from(policy),
                    entity: distribution_id,
                    entity_type: EntityTypes::Distribution.to_string(),
                },
            )
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(new_policy)
    }

    async fn delete_distribution_policies(&self, distribution_id: Urn, policy_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_odrl_offer_by_id(policy_id).await.map_err(CatalogError::DbErr)?;
        Ok(())
    }
}
