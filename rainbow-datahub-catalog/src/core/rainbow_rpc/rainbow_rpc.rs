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
use crate::core::rainbow_rpc::rainbow_rpc_types::{
    RainbowRPCDatahubCatalogResolveDataServiceRequest, RainbowRPCDatahubCatalogResolveOfferByIdRequest,
};
use crate::core::rainbow_rpc::RainbowRPCDatahubCatalogTrait;
use axum::async_trait;
use rainbow_catalog::provider::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use std::sync::Arc;

pub struct RainbowRPCDatahubCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
}

impl<T> RainbowRPCDatahubCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> RainbowRPCDatahubCatalogTrait for RainbowRPCDatahubCatalogService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    async fn resolve_data_service(
        &self,
        input: RainbowRPCDatahubCatalogResolveDataServiceRequest,
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

    // END DATAHUB SERVICES AND REPOS
    async fn resolve_offer(&self, input: RainbowRPCDatahubCatalogResolveOfferByIdRequest) -> anyhow::Result<OdrlOffer> {
        let offer = self
            .repo
            .get_odrl_offer_by_id(input.offer_id.clone())
            .await
            .map_err(|e| CatalogError::DbErr(e.into()))?
            .ok_or(CatalogError::NotFound { id: input.offer_id, entity: EntityTypes::DataService.to_string() })?;
        let offer = OdrlOffer::try_from(offer)?;
        Ok(offer)
    }
}
