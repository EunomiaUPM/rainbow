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
use crate::core::rainbow_entities::rainbow_catalog_types::{EditDataServiceRequest, NewDataServiceRequest};
use crate::core::rainbow_entities::RainbowDataServiceTrait;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::policies::EntityTypes;
use anyhow::bail;
use axum::async_trait;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogDataServiceService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
}

impl<T> RainbowCatalogDataServiceService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> RainbowDataServiceTrait for RainbowCatalogDataServiceService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    async fn get_data_service_by_id(&self, data_service_id: Urn) -> anyhow::Result<DataService> {
        let data_service_entity =
            self.repo.get_data_service_by_id(data_service_id.clone()).await.map_err(CatalogError::DbErr)?;
        match data_service_entity {
            Some(data_service_entity) => {
                let data_service = DataService::try_from(data_service_entity).map_err(CatalogError::ConversionError)?;
                Ok(data_service)
            }
            None => bail!(CatalogError::NotFound { id: data_service_id, entity: EntityTypes::DataService.to_string() }),
        }
    }

    async fn post_data_service(&self, catalog_id: Urn, input: NewDataServiceRequest) -> anyhow::Result<DataService> {
        let data_service_entity =
            self.repo.create_data_service(catalog_id.clone(), input.into()).await.map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                    CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                }
                _ => CatalogError::DbErr(err),
            })?;
        let dataservice = DataService::try_from(data_service_entity).map_err(CatalogError::ConversionError)?;
        Ok(dataservice)
    }

    async fn put_data_service(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
        input: EditDataServiceRequest,
    ) -> anyhow::Result<DataService> {
        let data_service_entity =
            self.repo.put_data_service_by_id(catalog_id.clone(), data_service_id, input.into()).await.map_err(
                |err| match err {
                    rainbow_db::catalog::repo::CatalogRepoErrors::DataServiceNotFound => {
                        CatalogError::NotFound { id: catalog_id, entity: EntityTypes::DataService.to_string() }
                    }
                    _ => CatalogError::DbErr(err),
                },
            )?;
        let dataservice = DataService::try_from(data_service_entity).map_err(CatalogError::ConversionError)?;
        Ok(dataservice)
    }

    async fn delete_data_service(&self, catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_data_service_by_id(catalog_id.clone(), dataset_id).await.map_err(|err| match err {
            rainbow_db::catalog::repo::CatalogRepoErrors::DataServiceNotFound => {
                CatalogError::NotFound { id: catalog_id, entity: EntityTypes::DataService.to_string() }
            }
            _ => CatalogError::DbErr(err),
        })?;
        Ok(())
    }
}
