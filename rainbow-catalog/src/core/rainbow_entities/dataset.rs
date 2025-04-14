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
use crate::core::rainbow_entities::rainbow_catalog_types::NewDatasetRequest;
use crate::core::rainbow_entities::RainbowDatasetTrait;
use crate::protocol::dataset_definition::Dataset;
use crate::protocol::policies::EntityTypes;
use anyhow::bail;
use axum::async_trait;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogDatasetService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
}

impl<T> RainbowCatalogDatasetService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> RainbowDatasetTrait for RainbowCatalogDatasetService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
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

    async fn post_dataset(&self, catalog_id: Urn, input: NewDatasetRequest) -> anyhow::Result<Dataset> {
        let dataset_entity =
            self.repo.create_dataset(catalog_id.clone(), input.into()).await.map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                    CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                }
                _ => CatalogError::DbErr(err),
            })?;
        let dataset = Dataset::try_from(dataset_entity).map_err(CatalogError::ConversionError)?;
        Ok(dataset)
    }

    async fn put_dataset(&self, catalog_id: Urn, dataset_id: Urn, input: NewDatasetRequest) -> anyhow::Result<Dataset> {
        let dataset_entity =
            self.repo.put_datasets_by_id(catalog_id.clone(), dataset_id.clone(), input.into()).await.map_err(
                |err| match err {
                    rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                        CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                    }
                    rainbow_db::catalog::repo::CatalogRepoErrors::DatasetNotFound => {
                        CatalogError::NotFound { id: dataset_id, entity: EntityTypes::Dataset.to_string() }
                    }
                    _ => CatalogError::DbErr(err),
                },
            )?;
        let dataset = Dataset::try_from(dataset_entity).map_err(CatalogError::ConversionError)?;
        Ok(dataset)
    }

    async fn delete_dataset(&self, catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<()> {
        let _ =
            self.repo.delete_dataset_by_id(catalog_id.clone(), dataset_id.clone()).await.map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                    CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DatasetNotFound => {
                    CatalogError::NotFound { id: dataset_id, entity: EntityTypes::Dataset.to_string() }
                }
                _ => CatalogError::DbErr(err),
            })?;
        Ok(())
    }
}
