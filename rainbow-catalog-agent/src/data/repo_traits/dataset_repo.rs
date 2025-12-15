use crate::data::entities::catalog::{EditCatalogModel, NewCatalogModel};
use crate::data::entities::dataset;
use crate::data::entities::dataset::{EditDatasetModel, NewDatasetModel};
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[async_trait::async_trait]
pub trait DatasetRepositoryTrait: Send + Sync {
    async fn get_all_datasets(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>, DatasetRepoErrors>;
    async fn get_batch_datasets(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<dataset::Model>, DatasetRepoErrors>;
    async fn get_datasets_by_catalog_id(
        &self,
        catalog_id: &Urn,
    ) -> anyhow::Result<Vec<dataset::Model>, DatasetRepoErrors>;
    async fn get_dataset_by_id(&self, dataset_id: &Urn) -> anyhow::Result<Option<dataset::Model>, DatasetRepoErrors>;

    async fn put_dataset_by_id(
        &self,
        dataset_id: &Urn,
        edit_dataset_model: &EditDatasetModel,
    ) -> anyhow::Result<dataset::Model, DatasetRepoErrors>;
    async fn create_dataset(
        &self,
        new_dataset_model: &NewDatasetModel,
    ) -> anyhow::Result<dataset::Model, DatasetRepoErrors>;

    async fn delete_dataset_by_id(&self, dataset_id: &Urn) -> anyhow::Result<(), DatasetRepoErrors>;
}

#[derive(Error, Debug)]
pub enum DatasetRepoErrors {
    #[error("Dataset not found")]
    DatasetNotFound,
    #[error("Error fetching dataset. {0}")]
    ErrorFetchingDataset(Error),
    #[error("Error creating dataset. {0}")]
    ErrorCreatingDataset(Error),
    #[error("Error deleting dataset. {0}")]
    ErrorDeletingDataset(Error),
    #[error("Error updating dataset. {0}")]
    ErrorUpdatingDataset(Error),
}
