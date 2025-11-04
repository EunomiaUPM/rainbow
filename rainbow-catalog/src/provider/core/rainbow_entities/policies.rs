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

use crate::provider::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::provider::core::rainbow_entities::RainbowPoliciesTrait;
use axum::async_trait;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::protocol::contract::contract_odrl::{OdrlOffer, OdrlPolicyInfo};
use rainbow_common::utils::get_urn;
use rainbow_db::catalog::repo::{NewOdrlOfferModel, OdrlOfferRepo};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogPoliciesService<T, U>
where
    T: OdrlOfferRepo + Send + Sync,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> RainbowCatalogPoliciesService<T, U>
where
    T: OdrlOfferRepo + Send + Sync,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, notification_service }
    }
}

#[async_trait]
impl<T, U> RainbowPoliciesTrait for RainbowCatalogPoliciesService<T, U>
where
    T: OdrlOfferRepo + Send + Sync,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    async fn get_catalog_policies(&self, catalog_id: Urn) -> anyhow::Result<Vec<OdrlOffer>> {
        let policies = self.repo.get_all_odrl_offers_by_entity(catalog_id).await.map_err(CatalogError::DbErr)?;
        let policies: Vec<OdrlOffer> = policies.iter().map(|p| OdrlOffer::try_from(p.to_owned()).unwrap()).collect();
        Ok(policies)
    }

    async fn post_catalog_policies(&self, catalog_id: Urn, policy: OdrlPolicyInfo) -> anyhow::Result<OdrlOffer> {
        let new_policy = self
            .repo
            .create_odrl_offer(
                catalog_id.clone(),
                EntityTypes::Catalog.to_string(),
                NewOdrlOfferModel {
                    id: Option::from(get_urn(None)),
                    odrl_offers: Option::from(to_value(&policy)?),
                    entity: catalog_id,
                    entity_type: EntityTypes::Catalog.to_string(),
                },
            )
            .await
            .map_err(CatalogError::DbErr)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "CatalogPolicies".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&new_policy)?,
                message_operation: RainbowEventsNotificationMessageOperation::Creation,
            })
            .await?;
        let new_policy = OdrlOffer::try_from(new_policy)?;
        Ok(new_policy)
    }

    async fn delete_catalog_policies(&self, catalog_id: Urn, policy_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_odrl_offer_by_id(policy_id.clone()).await.map_err(CatalogError::DbErr)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "CatalogPolicies".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: json!({
                    "@type": "OdrlPolicy",
                    "@id": policy_id.to_string()
                }),
                message_operation: RainbowEventsNotificationMessageOperation::Deletion,
            })
            .await?;
        Ok(())
    }

    async fn get_dataset_policies(&self, dataset_id: Urn) -> anyhow::Result<Vec<OdrlOffer>> {
        let policies = self.repo.get_all_odrl_offers_by_entity(dataset_id).await.map_err(CatalogError::DbErr)?;
        let policies: Vec<OdrlOffer> = policies.iter().map(|p| OdrlOffer::try_from(p.to_owned()).unwrap()).collect();
        Ok(policies)
    }

    async fn post_dataset_policies(&self, dataset_id: Urn, policy: OdrlPolicyInfo) -> anyhow::Result<OdrlOffer> {
        let new_policy = self
            .repo
            .create_odrl_offer(
                dataset_id.clone(),
                EntityTypes::Dataset.to_string(),
                NewOdrlOfferModel {
                    id: Option::from(get_urn(None)),
                    odrl_offers: Option::from(to_value(&policy)?),
                    entity: dataset_id,
                    entity_type: EntityTypes::Dataset.to_string(),
                },
            )
            .await
            .map_err(CatalogError::DbErr)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "DatasetPolicies".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&new_policy)?,
                message_operation: RainbowEventsNotificationMessageOperation::Creation,
            })
            .await?;
        let new_policy = OdrlOffer::try_from(new_policy)?;
        Ok(new_policy)
    }

    async fn delete_dataset_policies(&self, dataset_id: Urn, policy_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_odrl_offer_by_id(policy_id.clone()).await.map_err(CatalogError::DbErr)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "DatasetPolicies".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: json!({
                    "@type": "OdrlPolicy",
                    "@id": policy_id.to_string()
                }),
                message_operation: RainbowEventsNotificationMessageOperation::Deletion,
            })
            .await?;
        Ok(())
    }

    async fn get_data_service_policies(&self, data_service_id: Urn) -> anyhow::Result<Vec<OdrlOffer>> {
        let policies = self.repo.get_all_odrl_offers_by_entity(data_service_id).await.map_err(CatalogError::DbErr)?;
        let policies: Vec<OdrlOffer> = policies.iter().map(|p| OdrlOffer::try_from(p.to_owned()).unwrap()).collect();
        Ok(policies)
    }

    async fn post_data_service_policies(
        &self,
        data_service_id: Urn,
        policy: OdrlPolicyInfo,
    ) -> anyhow::Result<OdrlOffer> {
        let new_policy = self
            .repo
            .create_odrl_offer(
                data_service_id.clone(),
                EntityTypes::DataService.to_string(),
                NewOdrlOfferModel {
                    id: Option::from(get_urn(None)),
                    odrl_offers: Option::from(to_value(&policy)?),
                    entity: data_service_id,
                    entity_type: EntityTypes::DataService.to_string(),
                },
            )
            .await
            .map_err(CatalogError::DbErr)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "DataServicePolicies".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&new_policy)?,
                message_operation: RainbowEventsNotificationMessageOperation::Creation,
            })
            .await?;
        let new_policy = OdrlOffer::try_from(new_policy)?;
        Ok(new_policy)
    }

    async fn delete_data_service_policies(&self, data_service_id: Urn, policy_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_odrl_offer_by_id(policy_id.clone()).await.map_err(CatalogError::DbErr)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "DataServicePolicies".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: json!({
                    "@type": "OdrlPolicy",
                    "@id": policy_id.to_string()
                }),
                message_operation: RainbowEventsNotificationMessageOperation::Deletion,
            })
            .await?;
        Ok(())
    }

    async fn get_distribution_policies(&self, distribution_id: Urn) -> anyhow::Result<Vec<OdrlOffer>> {
        let policies = self.repo.get_all_odrl_offers_by_entity(distribution_id).await.map_err(CatalogError::DbErr)?;
        let policies: Vec<OdrlOffer> = policies.iter().map(|p| OdrlOffer::try_from(p.to_owned()).unwrap()).collect();
        Ok(policies)
    }

    async fn post_distribution_policies(
        &self,
        distribution_id: Urn,
        policy: OdrlPolicyInfo,
    ) -> anyhow::Result<OdrlOffer> {
        let new_policy = self
            .repo
            .create_odrl_offer(
                distribution_id.clone(),
                EntityTypes::Distribution.to_string(),
                NewOdrlOfferModel {
                    id: Option::from(get_urn(None)),
                    odrl_offers: Option::from(to_value(&policy)?),
                    entity: distribution_id,
                    entity_type: EntityTypes::Distribution.to_string(),
                },
            )
            .await
            .map_err(CatalogError::DbErr)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "DistributionPolicies".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&new_policy)?,
                message_operation: RainbowEventsNotificationMessageOperation::Creation,
            })
            .await?;
        let new_policy = OdrlOffer::try_from(new_policy)?;
        Ok(new_policy)
    }

    async fn delete_distribution_policies(&self, distribution_id: Urn, policy_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_odrl_offer_by_id(policy_id.clone()).await.map_err(CatalogError::DbErr)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "DistributionPolicies".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: json!({
                    "@type": "OdrlPolicy",
                    "@id": policy_id.to_string()
                }),
                message_operation: RainbowEventsNotificationMessageOperation::Deletion,
            })
            .await?;
        Ok(())
    }
}
