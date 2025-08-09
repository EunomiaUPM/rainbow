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
use crate::provider::core::rainbow_entities::rainbow_catalog_types::NewRelationRequest;
use crate::provider::core::rainbow_entities::rainbow_catalog_types::EditRelationRequest;
use crate::provider::core::rainbow_entities::RainbowRelationsTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::entities::relation;
use rainbow_db::catalog::entities::relation::Model;
use rainbow_db::catalog::repo::CatalogRepoErrors;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo, CatalogRecordRepo, RelationRepo};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use sea_orm::sea_query::Mode;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogRelationsService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + RelationRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> RainbowCatalogRelationsService<T, U>
where 
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + RelationRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self {repo, notification_service}
    }
}

#[async_trait]
impl<T, U> RainbowRelationsTrait for RainbowCatalogRelationsService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + RelationRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait  + Send + Sync,
{
    async fn get_relations(&self) -> anyhow::Result<Vec<Model>> {
        let relations = self.repo
            .get_all_relations(None, None)
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(relations)
    }
    async fn post_relation(&self, input: NewRelationRequest) -> anyhow::Result<Model> {
        let relation = self.repo
            .create_relation(input.into())
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(relation)
    }
    async fn get_relation_by_id(&self, id: Urn) -> anyhow::Result<relation::Model> {
        let relation = self.repo
            .get_relation_by_id(id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        match relation {
            Some(relation) => {
                Ok(relation)
            }
            None => bail!(CatalogError::NotFound { id: id.clone(), entity: EntityTypes::Relation.to_string() })
        }
    }   
    async fn put_relation_by_id(&self, id: Urn, input: EditRelationRequest) -> anyhow::Result<relation::Model> {
        let relation = self.repo
            .put_relation_by_id(id.clone(), input.into())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(relation)
    }
    async fn delete_relation(&self, id: Urn) -> anyhow::Result<()> {
        let _ = self.repo 
            .delete_relation_by_id(id.clone())
            .await
            .map_err(|err| match err {
                    CatalogRepoErrors::CatalogRecordNotfound => {
                    CatalogError::NotFound { id: id.clone(), entity: EntityTypes::Relation.to_string()}
                }
                _ => CatalogError::DbErr(err),
            })?;
        Ok(())
    }
    async fn get_relations_from_resource(&self, resource_id: Urn) -> anyhow::Result<Vec<relation::Model>> {
        let relations = self.repo
            .get_relations_by_resource(None, None, resource_id)
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(relations)
    }

}