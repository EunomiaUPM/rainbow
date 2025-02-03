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

use crate::core::rainbow_catalog_err::CatalogError;
use crate::protocol::policies::EntityTypes;
use rainbow_common::utils::get_urn;
use rainbow_db::catalog::entities::odrl_offer;
use rainbow_db::catalog::repo::{NewOdrlOfferModel, CATALOG_REPO};
use sea_orm::EntityTrait;
use serde_json::Value;
use urn::Urn;

pub async fn get_catalog_policies(catalog_id: Urn) -> anyhow::Result<Vec<odrl_offer::Model>> {
    let policies = CATALOG_REPO
        .get_all_odrl_offers_by_entity(catalog_id)
        .await
        .map_err(CatalogError::DbErr)?;
    Ok(policies)
}

pub async fn post_catalog_policies(
    catalog_id: Urn,
    policy: Value,
) -> anyhow::Result<odrl_offer::Model> {
    let new_policy = CATALOG_REPO
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

pub async fn delete_catalog_policies(catalog_id: Urn, policy_id: Urn) -> anyhow::Result<()> {
    let _ = CATALOG_REPO.delete_odrl_offer_by_id(policy_id).await.map_err(CatalogError::DbErr)?;
    Ok(())
}

pub async fn get_dataset_policies(dataset_id: Urn) -> anyhow::Result<Vec<odrl_offer::Model>> {
    let policies = CATALOG_REPO
        .get_all_odrl_offers_by_entity(dataset_id)
        .await
        .map_err(CatalogError::DbErr)?;
    Ok(policies)
}

pub async fn post_dataset_policies(
    dataset_id: Urn,
    policy: Value,
) -> anyhow::Result<odrl_offer::Model> {
    let new_policy = CATALOG_REPO
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

pub async fn delete_dataset_policies(dataset_id: Urn, policy_id: Urn) -> anyhow::Result<()> {
    let _ = CATALOG_REPO.delete_odrl_offer_by_id(policy_id).await.map_err(CatalogError::DbErr)?;
    Ok(())
}

pub async fn get_dataservices_policies(
    dataservice_id: Urn,
) -> anyhow::Result<Vec<odrl_offer::Model>> {
    let policies = CATALOG_REPO
        .get_all_odrl_offers_by_entity(dataservice_id)
        .await
        .map_err(CatalogError::DbErr)?;
    Ok(policies)
}

pub async fn post_dataservices_policies(
    dataservice_id: Urn,
    policy: Value,
) -> anyhow::Result<odrl_offer::Model> {
    let new_policy = CATALOG_REPO
        .create_odrl_offer(
            dataservice_id.clone(),
            EntityTypes::DataService.to_string(),
            NewOdrlOfferModel {
                id: Option::from(get_urn(None)),
                odrl_offers: Option::from(policy),
                entity: dataservice_id,
                entity_type: EntityTypes::DataService.to_string(),
            },
        )
        .await
        .map_err(CatalogError::DbErr)?;
    Ok(new_policy)
}


pub async fn delete_dataservices_policies(
    dataservice_id: Urn,
    policy_id: Urn,
) -> anyhow::Result<()> {
    let _ = CATALOG_REPO.delete_odrl_offer_by_id(policy_id).await.map_err(CatalogError::DbErr)?;
    Ok(())
}

pub async fn get_distributions_policies(
    distribution_id: Urn,
) -> anyhow::Result<Vec<odrl_offer::Model>> {
    let policies = CATALOG_REPO
        .get_all_odrl_offers_by_entity(distribution_id)
        .await
        .map_err(CatalogError::DbErr)?;
    Ok(policies)
}

pub async fn post_distributions_policies(
    distribution_id: Urn,
    policy: Value,
) -> anyhow::Result<odrl_offer::Model> {
    let new_policy = CATALOG_REPO
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

pub async fn delete_distributions_policies(
    distribution_id: Urn,
    policy_id: Urn,
) -> anyhow::Result<()> {
    let _ = CATALOG_REPO.delete_odrl_offer_by_id(policy_id).await.map_err(CatalogError::DbErr)?;
    Ok(())
}
