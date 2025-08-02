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

use crate::provider::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::provider::core::rainbow_entities::rainbow_catalog_types::{NewDatasetRequest, EditDatasetRequest};
use crate::provider::core::rainbow_entities::RainbowDatasetTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::catalog::dataset_definition::Dataset;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_db::catalog::repo::{CatalogRepo, CatalogRepoErrors, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogDatasetService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}


impl<T, U> RainbowCatalogDatasetService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, notification_service }
    }
}

#[async_trait]
impl<T, U> RainbowDatasetTrait for RainbowCatalogDatasetService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    async fn get_dataset_by_id(&self, dataset_id: Urn) -> anyhow::Result<Dataset> {
        let dataset_entity = self.repo.get_datasets_by_id(dataset_id.clone()).await.map_err(CatalogError::DbErr)?;
        match dataset_entity {
            Some(dataset_entity) => {
                let dataset = Dataset::try_from(dataset_entity).map_err(CatalogError::ConversionError)?;
                Ok(dataset)
            }
            None => bail!(CatalogError::NotFound { id: dataset_id, entity: EntityTypes::Dataset.to_string() }),
        }
    }

    async fn get_datasets_by_catalog_id(&self, catalog_id: Urn) -> anyhow::Result<Vec<Dataset>> {
        let datasets = self.repo.get_datasets_by_catalog_id(catalog_id.clone())
            .await
            .map_err(|e| match e {
                CatalogRepoErrors::CatalogNotFound => CatalogError::NotFound {
                    id: catalog_id,
                    entity: EntityTypes::Catalog.to_string(),
                },
                err => CatalogError::DbErr(err),
            })?;
        let datasets = datasets.iter()
            .map(|d| Dataset::try_from(d.to_owned()).unwrap())
            .collect();
        Ok(datasets)
    }

    async fn post_dataset(&self, catalog_id: Urn, input: NewDatasetRequest) -> anyhow::Result<Dataset> {
        let dataset_entity =
            self.repo.create_dataset(catalog_id.clone(), input.into()).await.map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                    CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                }
                _ => CatalogError::DbErr(err),
            })?;
        let dataset = Dataset::try_from(dataset_entity).map_err(CatalogError::ConversionError)?;
        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::Catalog,
            subcategory: "Dataset".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
            message_content: to_value(&dataset)?,
            message_operation: RainbowEventsNotificationMessageOperation::Creation,
        }).await?;
        Ok(dataset)
    }

    async fn put_dataset(&self, dataset_id: Urn, input: EditDatasetRequest) -> anyhow::Result<Dataset> {
        let dataset_entity =
            self.repo.put_datasets_by_id(dataset_id.clone(), input.into()).await.map_err(
                |err| match err {
                    rainbow_db::catalog::repo::CatalogRepoErrors::DatasetNotFound => {
                        CatalogError::NotFound { id: dataset_id, entity: EntityTypes::Dataset.to_string() }
                    }
                    _ => CatalogError::DbErr(err),
                },
            )?;
        let dataset = Dataset::try_from(dataset_entity).map_err(CatalogError::ConversionError)?;
        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::Catalog,
            subcategory: "Dataset".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
            message_content: to_value(&dataset)?,
            message_operation: RainbowEventsNotificationMessageOperation::Update,
        }).await?;
        Ok(dataset)
    }

    async fn delete_dataset(&self, catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<()> {
        let _ =
            self.repo.delete_dataset_by_id(catalog_id.clone(), dataset_id.clone()).await.map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                    CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DatasetNotFound => {
                    CatalogError::NotFound { id: dataset_id.clone(), entity: EntityTypes::Dataset.to_string() }
                }
                _ => CatalogError::DbErr(err),
            })?;
        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::Catalog,
            subcategory: "Dataset".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
            message_content: json!({
                "@type": "dcat:Dataset",
                "@id": dataset_id.to_string()
            }),
            message_operation: RainbowEventsNotificationMessageOperation::Deletion,
        }).await?;
        Ok(())
    }
}
