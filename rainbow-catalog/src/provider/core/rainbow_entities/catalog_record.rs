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
use crate::provider::core::rainbow_entities::rainbow_catalog_types::NewCatalogRecordRequest;
use crate::provider::core::rainbow_entities::rainbow_catalog_types::EditCatalogRecordRequest;
use crate::provider::core::rainbow_entities::RainbowCatalogRecrodTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::entities::catalog_record;
use rainbow_db::catalog::entities::catalog_record::Model;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo, CatalogRecordRepo};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogCatalogRecordService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> RainbowCatalogCatalogRecordService<T, U>
where 
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, notification_service}
    }
}

#[async_trait]
impl<T, U> RainbowCatalogRecrodTrait for RainbowCatalogCatalogRecordService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait  + Send + Sync,
{
    async fn get_catalog_records(&self) -> anyhow::Result<Vec<Model>> {
        let catalog_records = self.repo
        .get_all_catalog_records(None,None)
        .await
        .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(catalog_records)
    }
    async fn get_catalog_records_by_catalog(&self, catalog_id: Urn) -> anyhow::Result<Vec<Model>> {
        let catalog_records = self.repo
            .get_all_catalog_records_by_catalog_id(catalog_id, None, None)
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(catalog_records)
    }
    async fn get_catalog_records_by_id(&self, catalog_record_id: Urn) -> anyhow::Result<Model> {
        let catalog_record = self.repo
            .get_catalog_record_by_id(catalog_record_id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        match catalog_record {
            Some(catalog_record) => {
                Ok(catalog_record)
            }
            None => bail!(CatalogError::NotFound { id: catalog_record_id, entity: EntityTypes::CatalogRecord.to_string() }),
        }
    }
    async fn post_catalog_record(&self, input: NewCatalogRecordRequest) -> anyhow::Result<Model> {
        let catalog_record = self.repo
            .create_catalog_record(input.into())
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(catalog_record)
    }
    async fn put_catalog_record_by_id(&self, catalog_record_id: Urn, input: EditCatalogRecordRequest) -> anyhow::Result<Model> {
        let catalog_record = self.repo
            .put_catalog_record_by_id(catalog_record_id.clone(), input.into())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        // aqui se puede añadir el servicio de notificación, tal y como aparece para las otras clases como dataset
        Ok(catalog_record)
    }
    async fn delete_catalog_record_by_id(&self, catalog_record_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo
            .delete_catalog_record_by_id(catalog_record_id.clone())
            .await
            .map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogRecordNotfound => {
                    CatalogError::NotFound { id: catalog_record_id.clone(), entity: EntityTypes::CatalogRecord.to_string()}
                }
                _ => CatalogError::DbErr(err),
            })?;
        Ok(())
    }
}

