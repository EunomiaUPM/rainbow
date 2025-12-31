use crate::cache::factory_trait::CatalogAgentCacheTrait;
use crate::data::factory_trait::CatalogAgentRepoTrait;
use crate::entities::datasets::{DatasetDto, DatasetEntityTrait, EditDatasetDto, NewDatasetDto};
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use urn::Urn;

pub struct DatasetEntities {
    repo: Arc<dyn CatalogAgentRepoTrait>,
    cache: Arc<dyn CatalogAgentCacheTrait>,
}

impl DatasetEntities {
    pub fn new(repo: Arc<dyn CatalogAgentRepoTrait>, cache: Arc<dyn CatalogAgentCacheTrait>) -> Self {
        Self { repo, cache }
    }
}

#[async_trait::async_trait]
impl DatasetEntityTrait for DatasetEntities {
    async fn get_all_datasets(&self, limit: Option<u64>, page: Option<u64>) -> anyhow::Result<Vec<DatasetDto>> {
        let datasets = self.repo.get_dataset_repo().get_all_datasets(limit, page).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let mut dtos = Vec::with_capacity(datasets.len());
        for c in datasets {
            let dto: DatasetDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_batch_datasets(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<DatasetDto>> {
        let datasets = self.repo.get_dataset_repo().get_batch_datasets(ids).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let mut dtos = Vec::with_capacity(datasets.len());
        for c in datasets {
            let dto: DatasetDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_datasets_by_catalog_id(&self, catalog_id: &Urn) -> anyhow::Result<Vec<DatasetDto>> {
        let datasets = self.repo.get_dataset_repo().get_datasets_by_catalog_id(catalog_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let mut dtos = Vec::with_capacity(datasets.len());
        for c in datasets {
            let dto: DatasetDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_dataset_by_id(&self, dataset_id: &Urn) -> anyhow::Result<Option<DatasetDto>> {
        let dataset = self.repo.get_dataset_repo().get_dataset_by_id(dataset_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = dataset.map(|dto| dto.into());
        Ok(dto)
    }

    async fn put_dataset_by_id(
        &self,
        dataset_id: &Urn,
        edit_dataset_model: &EditDatasetDto,
    ) -> anyhow::Result<DatasetDto> {
        let edit_model = edit_dataset_model.clone().into();
        let dataset = self.repo.get_dataset_repo().put_dataset_by_id(dataset_id, &edit_model).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = dataset.into();
        Ok(dto)
    }

    async fn create_dataset(&self, new_dataset_model: &NewDatasetDto) -> anyhow::Result<DatasetDto> {
        let new_model = new_dataset_model.clone().into();
        let dataset = self.repo.get_dataset_repo().create_dataset(&new_model).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = dataset.into();
        Ok(dto)
    }

    async fn delete_dataset_by_id(&self, dataset_id: &Urn) -> anyhow::Result<()> {
        self.repo.get_dataset_repo().delete_dataset_by_id(dataset_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        Ok(())
    }
}
