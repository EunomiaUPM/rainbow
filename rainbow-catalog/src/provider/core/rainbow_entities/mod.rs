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

use crate::provider::core::rainbow_entities::rainbow_catalog_types::{
    EditDataServiceRequest, EditDistributionRequest, NewCatalogRequest, NewDataServiceRequest, NewDatasetRequest,
    NewDistributionRequest,
};
use axum::async_trait;
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::catalog::dataset_definition::Dataset;
use rainbow_common::protocol::catalog::distribution_definition::Distribution;
use rainbow_common::protocol::contract::contract_odrl::{OdrlOffer, OdrlPolicyInfo};
use urn::Urn;

pub mod catalog;
pub mod data_service;
pub mod dataset;
pub mod distribution;
pub mod policies;
pub mod rainbow_catalog_err;
pub mod rainbow_catalog_types;
pub mod rainbow_policies_types;

#[mockall::automock]
#[async_trait]
pub trait RainbowCatalogTrait: Send + Sync {
    async fn get_catalog_by_id(&self, id: Urn) -> anyhow::Result<Catalog>;
    async fn post_catalog(&self, input: NewCatalogRequest, is_main: bool) -> anyhow::Result<Catalog>;
    async fn put_catalog(&self, id: Urn, input: NewCatalogRequest) -> anyhow::Result<Catalog>;
    async fn delete_catalog(&self, id: Urn) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowDatasetTrait: Send + Sync {
    async fn get_dataset_by_id(&self, dataset_id: Urn) -> anyhow::Result<Dataset>;
    async fn get_datasets_by_catalog_id(&self, catalog_id: Urn) -> anyhow::Result<Vec<Dataset>>;
    async fn post_dataset(&self, catalog_id: Urn, input: NewDatasetRequest) -> anyhow::Result<Dataset>;
    async fn put_dataset(&self, catalog_id: Urn, dataset_id: Urn, input: NewDatasetRequest) -> anyhow::Result<Dataset>;
    async fn delete_dataset(&self, catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowDataServiceTrait: Send + Sync {
    async fn get_data_service_by_id(&self, data_service_id: Urn) -> anyhow::Result<DataService>;
    async fn get_data_services_by_catalog_id(&self, catalog_id: Urn) -> anyhow::Result<Vec<DataService>>;
    async fn post_data_service(&self, catalog_id: Urn, input: NewDataServiceRequest) -> anyhow::Result<DataService>;
    async fn put_data_service(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
        input: EditDataServiceRequest,
    ) -> anyhow::Result<DataService>;
    async fn delete_data_service(&self, catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowDistributionTrait: Send + Sync {
    async fn get_distribution_by_id(&self, distribution_id: Urn) -> anyhow::Result<Distribution>;
    async fn get_distributions_by_dataset_id(&self, dataset_id: Urn) -> anyhow::Result<Vec<Distribution>>;
    async fn get_distributions_by_dataset_id_and_dct_formats(
        &self,
        dataset_id: Urn,
        dct_formats: DctFormats,
    ) -> anyhow::Result<Distribution>;
    async fn post_distribution(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        input: NewDistributionRequest,
    ) -> anyhow::Result<Distribution>;
    async fn put_distribution(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
        distribution_id: Urn,
        input: EditDistributionRequest,
    ) -> anyhow::Result<Distribution>;

    async fn delete_distribution(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
        distribution_id: Urn,
    ) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub
trait RainbowPoliciesTrait: Send + Sync {
    async fn get_any_policy(&self, policy_id: Urn) -> anyhow::Result<OdrlOffer>;
    async fn get_catalog_policies(&self, catalog_id: Urn) -> anyhow::Result<Vec<OdrlOffer>>;
    async fn post_catalog_policies(&self, catalog_id: Urn, policy: OdrlPolicyInfo) -> anyhow::Result<OdrlOffer>;
    async fn delete_catalog_policies(&self, catalog_id: Urn, policy_id: Urn) -> anyhow::Result<()>;
    async fn get_dataset_policies(&self, dataset_id: Urn) -> anyhow::Result<Vec<OdrlOffer>>;
    async fn post_dataset_policies(&self, dataset_id: Urn, policy: OdrlPolicyInfo) -> anyhow::Result<OdrlOffer>;
    async fn delete_dataset_policies(&self, dataset_id: Urn, policy_id: Urn) -> anyhow::Result<()>;
    async fn get_data_service_policies(&self, data_service_id: Urn) -> anyhow::Result<Vec<OdrlOffer>>;
    async fn post_data_service_policies(
        &self,
        data_service_id: Urn,
        policy: OdrlPolicyInfo,
    ) -> anyhow::Result<OdrlOffer>;
    async fn delete_data_service_policies(&self, data_service_id: Urn, policy_id: Urn) -> anyhow::Result<()>;
    async fn get_distribution_policies(&self, distribution_id: Urn) -> anyhow::Result<Vec<OdrlOffer>>;
    async fn post_distribution_policies(
        &self,
        distribution_id: Urn,
        policy: OdrlPolicyInfo,
    ) -> anyhow::Result<OdrlOffer>;
    async fn delete_distribution_policies(&self, distribution_id: Urn, policy_id: Urn) -> anyhow::Result<()>;
}
