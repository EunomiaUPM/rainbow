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

use axum::async_trait;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::catalog::dataset_definition::Dataset;
use rainbow_common::protocol::catalog::distribution_definition::Distribution;
use urn::Urn;

pub mod ds_protocol;
pub mod ds_protocol_errors;
pub mod ds_protocol_types;

#[mockall::automock]
#[async_trait]
pub trait DSProtocolCatalogTrait: Sync + Send {
    async fn dataset_request(&self, dataset_id: Urn) -> anyhow::Result<Dataset>;
    async fn dataset_request_by_catalog(&self, catalog_id: Urn) -> anyhow::Result<Vec<Dataset>>;
    async fn data_services_request_by_catalog(&self, catalog_id: Urn) -> anyhow::Result<Vec<DataService>>;
    async fn data_services_request_by_id(&self, data_service_id: Urn) -> anyhow::Result<Option<DataService>>;
    async fn distributions_request_by_dataset(
        &self,
        dataset_id: Urn,
        catalog_id: Urn,
    ) -> anyhow::Result<Vec<Distribution>>;
    async fn catalog_request(&self) -> anyhow::Result<Catalog>;
    async fn catalog_request_by_id(&self, catalog_id: Urn) -> anyhow::Result<Catalog>;
}
