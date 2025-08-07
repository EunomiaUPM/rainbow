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
use crate::provider::core::rainbow_entities::data_service;
use crate::provider::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::provider::core::rainbow_entities::rainbow_catalog_types::NewDatasetSeriesRequest;
use crate::provider::core::rainbow_entities::rainbow_catalog_types::EditDatasetSeriesRequest;
use crate::provider::core::rainbow_entities::RainbowCatalogDatasetSeriesTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::entities::dataset;
use rainbow_db::catalog::entities::dataset::Model as dataset_model;
use rainbow_db::catalog::entities::dataset_series;
use rainbow_db::catalog::entities::dataset_series::Model;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo, CatalogRecordRepo, DatasetSeriesRepo};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogDatasetSeriesService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + DatasetSeriesRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> RainbowCatalogDatasetSeriesService<T, U>
where 
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + DatasetSeriesRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, notification_service}
    }
}

#[async_trait]
impl<T, U> RainbowCatalogDatasetSeriesTrait for RainbowCatalogDatasetSeriesService<T, U>
where 
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + DatasetSeriesRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    async fn get_all_dataset_series(&self) -> anyhow::Result<Vec<Model>> {
        let dataset_series = self.repo
            .get_all_dataset_series(None, None)
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(dataset_series)
    }
    async fn get_dataset_series_by_id(&self, id: Urn) -> anyhow::Result<Model> {
        let dataset_series = self.repo
            .get_dataset_series_by_id(id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        match dataset_series {
            Some(dataset_series) => {
                Ok(dataset_series)
            }
            None => bail!(CatalogError::NotFound {id: id, entity: EntityTypes::DatasetSeries.to_string() })
        }
    }
    async fn get_datasets_from_dataset_series_by_id(&self, id: Urn) -> anyhow::Result<Vec<dataset_model>> {
        let datasets = self.repo
            .get_datasets_from_dataset_series_by_dataset_id(id.clone())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(datasets)
    }
    async fn post_dataset_series(&self, input: NewDatasetSeriesRequest) -> anyhow::Result<Model> {
        let dataset_series = self.repo  
            .create_dataset_series(input.into())
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(dataset_series)
    }
    async fn put_dataset_series_by_id(&self, id: Urn, input: EditDatasetSeriesRequest) -> anyhow::Result<Model> {
        let dataset_series = self.repo
            .put_dataset_series_by_id(id.clone(), input.into())
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(dataset_series)
    }
    async fn delete_dataset_series_by_id(&self, id: Urn) -> anyhow::Result<()> {
        let _ = self.repo
            .delete_dataset_series_by_id(id.clone())
            .await
            .map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogRecordNotfound => {
                    CatalogError::NotFound { id: id.clone(), entity: EntityTypes::CatalogRecord.to_string()}
                }
                _ => CatalogError::DbErr(err),
            })?;
        Ok(())
    }
}