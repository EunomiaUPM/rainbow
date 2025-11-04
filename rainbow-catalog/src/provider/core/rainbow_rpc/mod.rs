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

use crate::provider::core::rainbow_rpc::rainbow_rpc_types::{
    RainbowRPCCatalogResolveDataServiceRequest, RainbowRPCCatalogResolveOfferByIdRequest,
};
use axum::async_trait;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;

pub mod rainbow_rpc;
pub mod rainbow_rpc_types;

#[mockall::automock]
#[async_trait]
pub trait RainbowRPCCatalogTrait: Send + Sync {
    async fn resolve_data_service(
        &self,
        input: RainbowRPCCatalogResolveDataServiceRequest,
    ) -> anyhow::Result<DataService>;
    async fn resolve_offer(&self, input: RainbowRPCCatalogResolveOfferByIdRequest) -> anyhow::Result<OdrlOffer>;
}
