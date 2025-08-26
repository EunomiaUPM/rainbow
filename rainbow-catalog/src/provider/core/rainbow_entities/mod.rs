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

use crate::provider::core::rainbow_entities::rainbow_catalog_types::{
    EditCatalogRecordRequest, EditCatalogRequest, 
    EditDataServiceRequest, EditDatasetRequest, 
    EditDatasetSeriesRequest, EditDistributionRequest, 
    EditReferenceRequest, EditResourceRequest, 
    NewCatalogRecordRequest, NewCatalogRequest, 
    NewDataServiceRequest, NewDatasetRequest, 
    NewDatasetSeriesRequest, NewDistributionRequest, 
    NewKeywordRequest, NewThemeRequest,
    NewReferenceRequest, NewResourceRequest,
    NewRelationRequest, EditRelationRequest,
    NewQualifiedRelationRequest, EditQualifiedRelationRequest
};

use rainbow_db::catalog::entities::{
    catalog_record as catalog_record_model,
     dataset as dataset_model, 
     dataset_series as dataset_series_model, 
     keyword,
     theme,
     reference as referece_model, 
     resource,
     relation,
     qualified_relation,
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
pub mod catalog_record;
pub mod rainbow_catalog_err;
pub mod rainbow_catalog_types;
pub mod rainbow_policies_types;
pub mod resources; 
pub mod dataset_series;
pub mod references;
pub mod keywords_themes;
pub mod relations;
pub mod qualified_relations;

#[mockall::automock]
#[async_trait]
pub trait RainbowCatalogTrait: Send + Sync {
    async fn get_all_catalogs(&self) -> anyhow::Result<Vec<Catalog>>;
    async fn get_catalog_by_id(&self, id: Urn) -> anyhow::Result<Catalog>;
    async fn post_catalog(&self, input: NewCatalogRequest, is_main: bool) -> anyhow::Result<Catalog>;
    async fn put_catalog(&self, id: Urn, input: EditCatalogRequest) -> anyhow::Result<Catalog>;
    async fn delete_catalog(&self, id: Urn) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowDatasetTrait: Send + Sync {
    async fn get_dataset_by_id(&self, dataset_id: Urn) -> anyhow::Result<Dataset>;
    async fn get_datasets_by_catalog_id(&self, catalog_id: Urn) -> anyhow::Result<Vec<Dataset>>;
    async fn post_dataset(&self, catalog_id: Urn, input: NewDatasetRequest) -> anyhow::Result<Dataset>;
    async fn put_dataset(&self, dataset_id: Urn, input: EditDatasetRequest) -> anyhow::Result<Dataset>;
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
    async fn get_distributions_by_dataset_id_and_dct_formats(&self, dataset_id: Urn, dct_formats: DctFormats) -> anyhow::Result<Distribution>;
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
pub trait RainbowPoliciesTrait: Send + Sync {
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

#[mockall::automock]
#[async_trait]
pub trait RainbowCatalogRecrodTrait: Send + Sync {
    async fn get_catalog_records(&self) -> anyhow::Result<Vec<catalog_record_model::Model>>;
    async fn get_catalog_records_by_catalog(&self, catalog_id: Urn) -> anyhow::Result<Vec<catalog_record_model::Model>>;
    async fn get_catalog_records_by_id(&self, id: Urn) -> anyhow::Result<catalog_record_model::Model>;
    async fn post_catalog_record(&self, input: NewCatalogRecordRequest) ->  anyhow::Result<catalog_record_model::Model>;
    async fn put_catalog_record_by_id(&self, catalog_record_id: Urn, input: EditCatalogRecordRequest) -> anyhow::Result<catalog_record_model::Model>;
    async fn delete_catalog_record_by_id(&self, catalog_record_id: Urn) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowCatalogResourceTrait: Send + Sync {
    async fn get_all_resources(&self) -> anyhow::Result<Vec<resource::Model>>;
    async fn post_resource(&self, input: NewResourceRequest) -> anyhow::Result<resource::Model>;
    async fn get_resource_by_id(&self, id: Urn) -> anyhow::Result<resource::Model>;
    async fn put_resource_by_id(&self, id: Urn, input: EditResourceRequest) -> anyhow::Result<resource::Model>;
    async fn delete_resoruce_by_id(&self, id: Urn) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowCatalogDatasetSeriesTrait: Send + Sync {
    async fn get_all_dataset_series(&self) -> anyhow::Result<Vec<dataset_series_model::Model>>;
    async fn post_dataset_series(&self, input: NewDatasetSeriesRequest) -> anyhow::Result<dataset_series_model::Model>;
    async fn get_dataset_series_by_id(&self, dataset_series_id: Urn) -> anyhow::Result<dataset_series_model::Model>;
    async fn get_datasets_from_dataset_series_by_id(&self, dataset_series_id: Urn) -> anyhow::Result<Vec<dataset_model::Model>>;
    async fn put_dataset_series_by_id(&self, dataset_series_id: Urn, input: EditDatasetSeriesRequest) -> anyhow::Result<dataset_series_model::Model>;
    async fn delete_dataset_series_by_id(&self, dataset_series_id: Urn) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowCatalogReferencesTrait: Send + Sync {
    async fn get_all_references(&self) -> anyhow::Result<Vec<referece_model::Model>>;
    async fn get_reference_by_id(&self, reference_id: Urn) -> anyhow::Result<referece_model::Model>;
    async fn post_reference(&self, input: NewReferenceRequest) -> anyhow::Result<referece_model::Model>;
    async fn put_reference_by_id(&self, reference_id: Urn, input: EditReferenceRequest) -> anyhow::Result<referece_model::Model>;
    async fn delete_reference_by_id(&self, reference_id: Urn) -> anyhow::Result<()>;
    async fn get_all_references_by_reosurce(&self, resource_id: Urn) -> anyhow::Result<Vec<referece_model::Model>>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowCatalogKeywordsThemesTrait: Send + Sync {
    async fn get_all_keywords(&self) -> anyhow::Result<Vec<keyword::Model>>;
    async fn post_keyword(&self, input: NewKeywordRequest) -> anyhow::Result<keyword::Model>;
    async fn delete_keyword(&self, id: Urn) -> anyhow::Result<()>;
    async fn get_all_themes(&self) -> anyhow::Result<Vec<theme::Model>>;
    async fn post_theme(&self, input: NewThemeRequest) -> anyhow::Result<theme::Model>;
    async fn delete_theme(&self, id: Urn) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowRelationsTrait: Send + Sync {
    async fn get_relations(&self) -> anyhow::Result<Vec<relation::Model>>;
    async fn post_relation(&self, input: NewRelationRequest) -> anyhow::Result<relation::Model>;
    async fn get_relation_by_id(&self, id: Urn) -> anyhow::Result<relation::Model>;
    async fn put_relation_by_id(&self, id: Urn, inpur: EditRelationRequest) -> anyhow::Result<relation::Model>;
    async fn delete_relation(&self, id: Urn) -> anyhow::Result<()>;
    async fn get_relations_from_resource(&self, resource_id: Urn) -> anyhow::Result<Vec<relation::Model>>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowQualifiedRelationsTrait: Send + Sync {
    async fn get_qualified_relations(&self) -> anyhow::Result<Vec<qualified_relation::Model>>;
    async fn post_qualified_relation(&self, input: NewQualifiedRelationRequest) -> anyhow::Result<qualified_relation::Model>;
    async fn get_qualified_relation_by_id(&self, id: Urn) -> anyhow::Result<qualified_relation::Model>;
    async fn put_qualified_relation_by_id(&self, id: Urn, inpur: EditQualifiedRelationRequest) -> anyhow::Result<qualified_relation::Model>;
    async fn delete_qualified_relation(&self, id: Urn) -> anyhow::Result<()>;
    async fn get_qualified_relations_from_resource(&self, resource_id: Urn) -> anyhow::Result<Vec<qualified_relation::Model>>;
}