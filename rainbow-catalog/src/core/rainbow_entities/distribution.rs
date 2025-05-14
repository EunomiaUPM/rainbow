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
use crate::core::rainbow_entities::rainbow_catalog_types::{EditDistributionRequest, NewDistributionRequest};
use crate::core::rainbow_entities::RainbowDistributionTrait;
use axum::async_trait;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::catalog::distribution_definition::Distribution;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogDistributionService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}


impl<T, U> RainbowCatalogDistributionService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, notification_service }
    }
    async fn data_services_request_by_id(&self, data_service_id: Urn) -> anyhow::Result<Option<DataService>> {
        let data_service = self.repo.get_data_service_by_id(data_service_id).await.map_err(CatalogError::DbErr)?;
        let data_service = data_service.map(|m| DataService::try_from(m).unwrap());
        Ok(data_service)
    }
}

#[async_trait]
impl<T, U> RainbowDistributionTrait for RainbowCatalogDistributionService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,

{
    async fn get_distribution_by_id(&self, distribution_id: Urn) -> anyhow::Result<Distribution> {
        let distribution = self
            .repo
            .get_distribution_by_id(distribution_id.clone())
            .await
            .map_err(CatalogError::DbErr)?
            .ok_or(CatalogError::NotFound { id: distribution_id, entity: EntityTypes::Distribution.to_string() })?;
        let data_service_id = get_urn_from_string(&distribution.dcat_access_service)?;
        let mut distribution = Distribution::try_from(distribution).map_err(CatalogError::ConversionError)?;
        distribution.dcat.access_service = self.data_services_request_by_id(data_service_id).await?;
        Ok(distribution)
    }

    async fn get_distributions_by_dataset_id(&self, dataset_id: Urn) -> anyhow::Result<Vec<Distribution>> {
        let mut distributions_out: Vec<Distribution> = vec![];
        let distributions = self.repo
            .get_distributions_by_dataset_id(dataset_id.clone())
            .await
            .map_err(CatalogError::DbErr)?;
        for distribution in distributions {
            let data_service_id = get_urn_from_string(&distribution.dcat_access_service)?;
            let mut distribution = Distribution::try_from(distribution.clone()).map_err(CatalogError::ConversionError)?;
            distribution.dcat.access_service = self.data_services_request_by_id(data_service_id).await?;
            distributions_out.push(distribution);
        }
        Ok(distributions_out)
    }

    async fn post_distribution(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        input: NewDistributionRequest,
    ) -> anyhow::Result<Distribution> {
        let distribution_entity =
            self.repo.create_distribution(catalog_id.clone(), dataset_id.clone(), input.clone().into()).await.map_err(
                |err| match err {
                    rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                        CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                    }
                    rainbow_db::catalog::repo::CatalogRepoErrors::DatasetNotFound => {
                        CatalogError::NotFound { id: dataset_id, entity: EntityTypes::Dataset.to_string() }
                    }
                    rainbow_db::catalog::repo::CatalogRepoErrors::DataServiceNotFound => {
                        CatalogError::NotFound { id: input.dcat_access_service.clone(), entity: EntityTypes::DataService.to_string() }
                    }
                    _ => CatalogError::DbErr(err),
                },
            )?;
        let mut distribution = Distribution::try_from(distribution_entity).map_err(CatalogError::ConversionError)?;
        distribution.dcat.access_service = self.data_services_request_by_id(input.dcat_access_service).await?;
        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::Catalog,
            subcategory: "Distribution".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
            message_content: to_value(&distribution)?,
            message_operation: RainbowEventsNotificationMessageOperation::Creation,
        }).await?;
        Ok(distribution)
    }

    async fn put_distribution(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        distribution_id: Urn,
        input: EditDistributionRequest,
    ) -> anyhow::Result<Distribution> {
        let distribution_entity = self
            .repo
            .put_distribution_by_id(
                catalog_id.clone(),
                dataset_id.clone(),
                distribution_id.clone(),
                input.clone().into(),
            )
            .await
            .map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                    CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DatasetNotFound => {
                    CatalogError::NotFound { id: dataset_id, entity: EntityTypes::Dataset.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DataServiceNotFound => {
                    CatalogError::NotFound { id: input.dcat_access_service.clone().unwrap(), entity: EntityTypes::DataService.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DistributionNotFound => {
                    CatalogError::NotFound { id: distribution_id, entity: EntityTypes::Distribution.to_string() }
                }
                _ => CatalogError::DbErr(err),
            })?;
        let data_service_id = get_urn_from_string(&distribution_entity.dcat_access_service)?;
        let mut distribution =
            Distribution::try_from(distribution_entity.clone()).map_err(CatalogError::ConversionError)?;
        distribution.dcat.access_service = self.data_services_request_by_id(data_service_id).await?;
        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::Catalog,
            subcategory: "Distribution".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
            message_content: to_value(&distribution)?,
            message_operation: RainbowEventsNotificationMessageOperation::Update,
        }).await?;
        Ok(distribution)
    }

    async fn delete_distribution(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
        distribution_id: Urn,
    ) -> anyhow::Result<()> {
        let _ = self
            .repo
            .delete_distribution_by_id(
                catalog_id.clone(),
                data_service_id.clone(),
                distribution_id.clone(),
            )
            .await
            .map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                    CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DatasetNotFound => {
                    CatalogError::NotFound { id: data_service_id, entity: EntityTypes::Dataset.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DistributionNotFound => {
                    CatalogError::NotFound { id: distribution_id.clone(), entity: EntityTypes::Distribution.to_string() }
                }
                _ => CatalogError::DbErr(err),
            })?;
        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::Catalog,
            subcategory: "Distribution".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
            message_content: json!({
                "@type": "dcat:Distribution",
                "@id": distribution_id.to_string()
            }),
            message_operation: RainbowEventsNotificationMessageOperation::Deletion,
        }).await?;
        Ok(())
    }
}
