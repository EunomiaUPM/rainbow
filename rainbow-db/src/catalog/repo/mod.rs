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
use super::entities::catalog;
use super::entities::dataservice;
use super::entities::dataset;
use super::entities::distribution;
use super::entities::odrl_offer;
use crate::transfer_provider::repo::{TransferMessagesRepo, TransferProcessRepo};
use anyhow::Error;
use axum::async_trait;
use sea_orm::DatabaseConnection;
use thiserror::Error;
use urn::Urn;

pub mod sql;

pub trait CatalogRepoFactory:
CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static
{
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

pub struct NewCatalogModel {
    pub id: Option<Urn>,
    pub foaf_home_page: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
}

pub struct EditCatalogModel {
    pub foaf_home_page: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
}

#[async_trait]
pub trait CatalogRepo {
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors>;
    async fn get_catalog_by_id(&self, catalog_id: Urn) -> anyhow::Result<Option<catalog::Model>, CatalogRepoErrors>;
    async fn put_catalog_by_id(
        &self,
        catalog_id: Urn,
        edit_catalog_model: EditCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors>;
    async fn create_catalog(
        &self,
        new_catalog_model: NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors>;
    async fn delete_catalog_by_id(&self, catalog_id: Urn) -> anyhow::Result<(), CatalogRepoErrors>;
}

pub struct NewDatasetModel {
    pub id: Option<Urn>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
}

pub struct EditDatasetModel {
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
}

#[async_trait]
pub trait DatasetRepo {
    async fn get_all_datasets(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors>;
    async fn get_datasets_by_catalog_id(&self, catalog_id: Urn) -> anyhow::Result<Vec<dataset::Model>, CatalogRepoErrors>;
    async fn get_datasets_by_id(&self, dataset_id: Urn) -> anyhow::Result<Option<dataset::Model>, CatalogRepoErrors>;
    async fn put_datasets_by_id(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        edit_dataset_model: EditDatasetModel,
    ) -> anyhow::Result<dataset::Model, CatalogRepoErrors>;
    async fn create_dataset(
        &self,
        catalog_id: Urn,
        new_dataset_model: NewDatasetModel,
    ) -> anyhow::Result<dataset::Model, CatalogRepoErrors>;
    async fn delete_dataset_by_id(&self, catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<(), CatalogRepoErrors>;
}

#[derive(Debug)]
pub struct NewDistributionModel {
    pub id: Option<Urn>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub dcat_access_service: String,
}
pub struct EditDistributionModel {
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub dcat_access_service: Option<String>,
}

#[async_trait]
pub trait DistributionRepo {
    async fn get_all_distributions(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<distribution::Model>, CatalogRepoErrors>;
    async fn get_distributions_by_dataset_id(
        &self,
        dataset_id: Urn,
    ) -> anyhow::Result<Vec<distribution::Model>, CatalogRepoErrors>;
    async fn get_distribution_by_id(
        &self,
        distribution_id: Urn,
    ) -> anyhow::Result<Option<distribution::Model>, CatalogRepoErrors>;
    async fn put_distribution_by_id(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        distribution_id: Urn,
        edit_distribution_model: EditDistributionModel,
    ) -> anyhow::Result<distribution::Model, CatalogRepoErrors>;
    async fn create_distribution(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        new_distribution_model: NewDistributionModel,
    ) -> anyhow::Result<distribution::Model, CatalogRepoErrors>;
    async fn delete_distribution_by_id(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        distribution_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}

pub struct NewDataServiceModel {
    pub id: Option<Urn>,
    pub dcat_endpoint_description: Option<String>,
    pub dcat_endpoint_url: String,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
}
pub struct EditDataServiceModel {
    pub dcat_endpoint_description: Option<String>,
    pub dcat_endpoint_url: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
}

#[async_trait]
pub trait DataServiceRepo {
    async fn get_all_data_services(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataservice::Model>, CatalogRepoErrors>;

    async fn get_data_services_by_catalog_id(
        &self,
        catalog_id: Urn,
    ) -> anyhow::Result<Vec<dataservice::Model>, CatalogRepoErrors>;

    async fn get_data_service_by_id(
        &self,
        data_service_id: Urn,
    ) -> anyhow::Result<Option<dataservice::Model>, CatalogRepoErrors>;
    async fn put_data_service_by_id(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
        edit_data_service_model: EditDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, CatalogRepoErrors>;
    async fn create_data_service(
        &self,
        catalog_id: Urn,
        new_data_service_model: NewDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, CatalogRepoErrors>;
    async fn delete_data_service_by_id(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
    ) -> anyhow::Result<(), CatalogRepoErrors>;
}

pub struct NewOdrlOfferModel {
    pub id: Option<Urn>,
    pub odrl_offers: Option<serde_json::Value>,
    pub entity: Urn,
    pub entity_type: String,
}

#[async_trait]
pub trait OdrlOfferRepo {
    async fn get_all_odrl_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogRepoErrors>;
    async fn get_all_odrl_offers_by_entity(
        &self,
        entity: Urn,
    ) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogRepoErrors>;
    async fn get_odrl_offer_by_id(
        &self,
        odrl_offer_id: Urn,
    ) -> anyhow::Result<Option<odrl_offer::Model>, CatalogRepoErrors>;
    async fn create_odrl_offer(
        &self,
        entity_id: Urn,
        entity_type: String, // TODO EntityTypes
        new_odrl_offer_model: NewOdrlOfferModel,
    ) -> anyhow::Result<odrl_offer::Model, CatalogRepoErrors>;
    async fn delete_odrl_offer_by_id(&self, odrl_offer_id: Urn) -> anyhow::Result<(), CatalogRepoErrors>;
    async fn delete_odrl_offers_by_entity(&self, entity_id: Urn) -> anyhow::Result<(), CatalogRepoErrors>;
    async fn get_upstream_offers(&self, entity_id: Urn) -> anyhow::Result<Vec<odrl_offer::Model>, CatalogRepoErrors>;
}

#[derive(Error, Debug)]
pub enum CatalogRepoErrors {
    #[error("Catalog not found")]
    CatalogNotFound,
    #[error("Dataset not found")]
    DatasetNotFound,
    #[error("Distribution not found")]
    DistributionNotFound,
    #[error("DataService not found")]
    DataServiceNotFound,
    #[error("OdrlOffer not found")]
    OdrlOfferNotFound,

    #[error("Error fetching catalog. {0}")]
    ErrorFetchingCatalog(Error),
    #[error("Error fetching dataset. {0}")]
    ErrorFetchingDataset(Error),
    #[error("Error fetching distribution. {0}")]
    ErrorFetchingDistribution(Error),
    #[error("Error fetching data service. {0}")]
    ErrorFetchingDataService(Error),
    #[error("Error fetching odrl offer. {0}")]
    ErrorFetchingOdrlOffer(Error),

    #[error("Error creating catalog. {0}")]
    ErrorCreatingCatalog(Error),
    #[error("Error creating dataset. {0}")]
    ErrorCreatingDataset(Error),
    #[error("Error creating distribution. {0}")]
    ErrorCreatingDistribution(Error),
    #[error("Error creating data service. {0}")]
    ErrorCreatingDataService(Error),
    #[error("Error creating odrl offer. {0}")]
    ErrorCreatingOdrlOffer(Error),

    #[error("Error deleting catalog. {0}")]
    ErrorDeletingCatalog(Error),
    #[error("Error deleting dataset. {0}")]
    ErrorDeletingDataset(Error),
    #[error("Error deleting distribution. {0}")]
    ErrorDeletingDistribution(Error),
    #[error("Error deleting data service. {0}")]
    ErrorDeletingDataService(Error),
    #[error("Error deleting odrl offer. {0}")]
    ErrorDeletingOdrlOffer(Error),

    #[error("Error updating catalog. {0}")]
    ErrorUpdatingCatalog(Error),
    #[error("Error updating dataset. {0}")]
    ErrorUpdatingDataset(Error),
    #[error("Error updating distribution. {0}")]
    ErrorUpdatingDistribution(Error),
    #[error("Error updating data service. {0}")]
    ErrorUpdatingDataService(Error),
    #[error("Error updating odrl offer. {0}")]
    ErrorUpdatingOdrlOffer(Error),
}
