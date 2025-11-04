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
use crate::provider::core::rainbow_entities::rainbow_catalog_types::NewCatalogRequest;
use crate::provider::core::rainbow_entities::RainbowCatalogTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogCatalogService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> RainbowCatalogCatalogService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, notification_service }
    }
}

#[async_trait]
impl<T, U> RainbowCatalogTrait for RainbowCatalogCatalogService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    async fn get_catalog_by_id(&self, id: Urn) -> anyhow::Result<Catalog> {
        let catalog = self.repo.get_catalog_by_id(id.clone()).await.map_err(CatalogError::DbErr)?;

        match catalog {
            Some(catalog_entity) => {
                let mut catalog_out =
                    Catalog::try_from(catalog_entity.clone()).map_err(CatalogError::ConversionError)?;
                let catalog_id = get_urn_from_string(&catalog_entity.id.clone())?;
                let odrl = self.repo.get_all_odrl_offers_by_entity(catalog_id).await.map_err(CatalogError::DbErr)?;
                let odrl = Some(odrl.iter().map(|o| OdrlOffer::try_from(o.to_owned()).unwrap()).collect());
                catalog_out.odrl_offer = odrl;
                // catalog_out.datasets = dataset_request_by_catalog(catalog_out.id.parse()?).await?;
                // catalog_out.data_services = dataservices_request_by_catalog(catalog_out.id.parse()?).await?;
                Ok(catalog_out)
            }
            None => bail!(CatalogError::NotFound { id, entity: EntityTypes::Catalog.to_string() }),
        }
    }

    async fn post_catalog(&self, input: NewCatalogRequest, is_main: bool) -> anyhow::Result<Catalog> {
        let catalog_entity = match is_main {
            true => self.repo.create_main_catalog(input.into()).await.map_err(CatalogError::DbErr)?,
            false => self.repo.create_catalog(input.into()).await.map_err(CatalogError::DbErr)?,
        };
        let catalog = Catalog::try_from(catalog_entity).map_err(CatalogError::ConversionError)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "Catalog".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&catalog)?,
                message_operation: RainbowEventsNotificationMessageOperation::Creation,
            })
            .await?;
        Ok(catalog)
    }

    async fn put_catalog(&self, id: Urn, input: NewCatalogRequest) -> anyhow::Result<Catalog> {
        let catalog_entity = self.repo.put_catalog_by_id(id.clone(), input.into()).await.map_err(|err| match err {
            rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                CatalogError::NotFound { id, entity: EntityTypes::Catalog.to_string() }
            }
            _ => CatalogError::DbErr(err),
        })?;
        let catalog = Catalog::try_from(catalog_entity).map_err(CatalogError::ConversionError)?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "Catalog".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: to_value(&catalog)?,
                message_operation: RainbowEventsNotificationMessageOperation::Update,
            })
            .await?;
        Ok(catalog)
    }

    async fn delete_catalog(&self, id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_catalog_by_id(id.clone()).await.map_err(|err| match err {
            rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                CatalogError::NotFound { id: id.clone(), entity: EntityTypes::Catalog.to_string() }
            }
            _ => CatalogError::DbErr(err),
        })?;
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::Catalog,
                subcategory: "Catalog".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage,
                message_content: json!({
                    "@type": "dcat:Catalog",
                    "@id": id.to_string()
                }),
                message_operation: RainbowEventsNotificationMessageOperation::Deletion,
            })
            .await?;
        Ok(())
    }
}
