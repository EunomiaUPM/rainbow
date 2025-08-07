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

use crate::provider::core::ds_protocol::ds_protocol_errors::DSProtocolCatalogErrors;
use crate::provider::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::provider::core::rainbow_entities::rainbow_catalog_types::NewResourceRequest;
use crate::provider::core::rainbow_entities::rainbow_catalog_types::EditResourceRequest;
use crate::provider::core::rainbow_entities::RainbowCatalogResourceTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::entities::catalog_record;
use rainbow_db::catalog::entities::resource::Model;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo, CatalogRecordRepo, ResourceRepo};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use sea_orm::sea_query::Mode;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogResourceService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + ResourceRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> RainbowCatalogResourceService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + ResourceRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, notification_service}
    }
}

#[async_trait]
impl<T,U> RainbowCatalogResourceTrait for RainbowCatalogResourceService<T,U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + ResourceRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    async fn get_all_resources(&self) -> anyhow::Result<Vec<Model>> {
        let resources = self.repo
            .get_all_resources(None,None)
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(resources)
    }
    async fn get_resource_by_id(&self, resource_id: Urn) -> anyhow::Result<Model> {
        let resource = self.repo
            .get_resource_by_id(resource_id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        match resource {
            Some(resource) => {
                Ok(resource)
            }
            None => bail!(CatalogError::NotFound {id: resource_id, entity: EntityTypes::Resource.to_string()}),
        }
    }
    async fn post_resource(&self, input: NewResourceRequest) -> anyhow::Result<Model> {
        let resource = self.repo
            .create_resource(input.into())
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(resource)
    }
    async fn put_resource_by_id(&self, resource_id: Urn, input: EditResourceRequest) -> anyhow::Result<Model> {
        let resource = self.repo
            .put_resource_by_id(resource_id.clone(), input.into())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(resource)
    }
    async fn delete_resoruce_by_id(&self, resource_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo
            .delete_resource_by_id(resource_id.clone())
            .await
            .map_err(|err|match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::ResourceNotfound => {
                    CatalogError::NotFound { id: resource_id.clone(), entity: EntityTypes::Resource.to_string()}
                }
                _ => CatalogError::DbErr(err),
            })?;
        Ok(())
    }
}