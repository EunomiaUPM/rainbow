use crate::core::rainbow_entities::rainbow_catalog_types::{EditDataServiceRequest, EditDistributionRequest, NewCatalogRequest, NewDataServiceRequest, NewDatasetRequest, NewDistributionRequest};
use crate::protocol::catalog_definition::Catalog;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::dataset_definition::Dataset;
use crate::protocol::distribution_definition::Distribution;
use axum::async_trait;
use rainbow_db::catalog::entities::odrl_offer;
use serde_json::Value;
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
    async fn post_catalog(&self, input: NewCatalogRequest) -> anyhow::Result<Catalog>;
    async fn put_catalog(&self, id: Urn, input: NewCatalogRequest) -> anyhow::Result<Catalog>;
    async fn delete_catalog(&self, id: Urn) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowDatasetTrait: Send + Sync {
    async fn get_dataset_by_id(&self, dataset_id: Urn) -> anyhow::Result<Dataset>;
    async fn post_dataset(&self, catalog_id: Urn, input: NewDatasetRequest) -> anyhow::Result<Dataset>;
    async fn put_dataset(&self, catalog_id: Urn, dataset_id: Urn, input: NewDatasetRequest) -> anyhow::Result<Dataset>;
    async fn delete_dataset(&self, catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<()>;
}

#[mockall::automock]
#[async_trait]
pub trait RainbowDataServiceTrait: Send + Sync {
    async fn get_data_service_by_id(&self, data_service_id: Urn) -> anyhow::Result<DataService>;
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
    async fn get_catalog_policies(&self, catalog_id: Urn) -> anyhow::Result<Vec<odrl_offer::Model>>;
    async fn post_catalog_policies(&self, catalog_id: Urn, policy: Value) -> anyhow::Result<odrl_offer::Model>;
    async fn delete_catalog_policies(&self, catalog_id: Urn, policy_id: Urn) -> anyhow::Result<()>;
    async fn get_dataset_policies(&self, dataset_id: Urn) -> anyhow::Result<Vec<odrl_offer::Model>>;
    async fn post_dataset_policies(&self, dataset_id: Urn, policy: Value) -> anyhow::Result<odrl_offer::Model>;
    async fn delete_dataset_policies(&self, dataset_id: Urn, policy_id: Urn) -> anyhow::Result<()>;
    async fn get_data_service_policies(&self, data_service_id: Urn) -> anyhow::Result<Vec<odrl_offer::Model>>;
    async fn post_data_service_policies(
        &self,
        data_service_id: Urn,
        policy: Value,
    ) -> anyhow::Result<odrl_offer::Model>;
    async fn delete_data_service_policies(&self, data_service_id: Urn, policy_id: Urn) -> anyhow::Result<()>;
    async fn get_distribution_policies(&self, distribution_id: Urn) -> anyhow::Result<Vec<odrl_offer::Model>>;
    async fn post_distribution_policies(
        &self,
        distribution_id: Urn,
        policy: Value,
    ) -> anyhow::Result<odrl_offer::Model>;
    async fn delete_distribution_policies(&self, distribution_id: Urn, policy_id: Urn) -> anyhow::Result<()>;
}
