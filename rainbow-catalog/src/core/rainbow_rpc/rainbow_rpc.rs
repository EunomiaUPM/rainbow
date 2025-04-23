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
use crate::core::rainbow_rpc::rainbow_rpc_types::RainbowRPCCatalogResolveDataServiceRequest;
use crate::core::rainbow_rpc::RainbowRPCCatalogTrait;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::policies::EntityTypes;
use axum::async_trait;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use std::sync::Arc;

pub struct RainbowRPCCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
}

impl<T> RainbowRPCCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> RainbowRPCCatalogTrait for RainbowRPCCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    async fn resolve_data_service(
        &self,
        input: RainbowRPCCatalogResolveDataServiceRequest,
    ) -> anyhow::Result<DataService> {
        let data_service = self
            .repo
            .get_data_service_by_id(input.data_service_id.clone())
            .await
            .map_err(|e| CatalogError::DbErr(e.into()))?
            .ok_or(CatalogError::NotFound {
                id: input.data_service_id,
                entity: EntityTypes::DataService.to_string(),
            })?;
        let data_service = DataService::try_from(data_service)?;
        Ok(data_service)
    }
}
