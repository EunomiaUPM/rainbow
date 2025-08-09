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
use crate::provider::core::rainbow_entities::rainbow_catalog_types::NewReferenceRequest;
use crate::provider::core::rainbow_entities::rainbow_catalog_types::EditReferenceRequest;
use crate::provider::core::rainbow_entities::RainbowCatalogRecrodTrait;
use crate::provider::core::rainbow_entities::RainbowCatalogReferencesTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::entities::reference;
use rainbow_db::catalog::entities::reference::Model;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo, CatalogRecordRepo, ReferenceRepo};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogReferenceService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + ReferenceRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T,U> RainbowCatalogReferenceService<T,U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + ReferenceRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self {repo, notification_service}
    }
}

#[async_trait]
impl<T,U> RainbowCatalogReferencesTrait for RainbowCatalogReferenceService<T,U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + ReferenceRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait  + Send + Sync,
{
    async fn get_all_references(&self) -> anyhow::Result<Vec<Model>> {
        let references = self.repo
            .get_all_references(None, None)
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(references)
    }
    async fn get_reference_by_id(&self, id: Urn) -> anyhow::Result<Model> {
        let reference = self.repo
            .get_reference_by_id(id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        match reference {
            Some(reference) => {
                Ok(reference)
            }
            None => bail!(CatalogError::NotFound { id: id, entity: EntityTypes::Reference.to_string() })
        }
    }
    async fn post_reference(&self, input: NewReferenceRequest) -> anyhow::Result<Model> {
        let reference = self.repo
            .create_reference(input.into())
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(reference)
    } 
    async fn get_all_references_by_reosurce(&self, id: Urn) -> anyhow::Result<Vec<Model>> {
        let references = self.repo
            .get_all_references_by_referenced_resource(id.clone(), None, None)
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(references)
    }
    async fn put_reference_by_id(&self, id: Urn, input: EditReferenceRequest) -> anyhow::Result<Model> {
        let refernce = self.repo
            .put_reference_by_id(id.clone(), input.into())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(refernce)
    }
    async fn delete_reference_by_id(&self, id:Urn) -> anyhow::Result<()> {
        let _ = self.repo
            .delete_reference(id.clone())
            .await
            .map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogRecordNotfound => {
                    CatalogError::NotFound { id: id.clone(), entity: EntityTypes::Reference.to_string()}
                }
                _ => CatalogError::DbErr(err),
            })?;
        Ok(())
    }
}
