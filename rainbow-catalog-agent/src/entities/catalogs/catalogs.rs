use crate::data::entities::catalog::{EditCatalogModel, NewCatalogModel};
use crate::data::factory_trait::CatalogAgentRepoTrait;
use crate::entities::catalogs::{CatalogDto, CatalogEntityTrait, EditCatalogDto, NewCatalogDto};
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use urn::Urn;

pub struct CatalogEntities {
    repo: Arc<dyn CatalogAgentRepoTrait>,
}

impl CatalogEntities {
    pub fn new(repo: Arc<dyn CatalogAgentRepoTrait>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl CatalogEntityTrait for CatalogEntities {
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        with_main_catalog: bool,
    ) -> anyhow::Result<Vec<CatalogDto>> {
        let catalogs =
            self.repo.get_catalog_repo().get_all_catalogs(limit, page, with_main_catalog).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let mut dtos = Vec::with_capacity(catalogs.len());
        for c in catalogs {
            let dto: CatalogDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_batch_catalogs(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<CatalogDto>> {
        let catalogs = self.repo.get_catalog_repo().get_batch_catalogs(ids).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let mut dtos = Vec::with_capacity(catalogs.len());
        for c in catalogs {
            let dto: CatalogDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_catalog_by_id(&self, catalog_id: &Urn) -> anyhow::Result<Option<CatalogDto>> {
        let catalog = self.repo.get_catalog_repo().get_catalog_by_id(catalog_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = catalog.map(|c| c.into());
        Ok(dto)
    }

    async fn get_main_catalog(&self) -> anyhow::Result<Option<CatalogDto>> {
        let catalog = self.repo.get_catalog_repo().get_main_catalog().await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = catalog.map(|c| c.into());
        Ok(dto)
    }

    async fn put_catalog_by_id(
        &self,
        catalog_id: &Urn,
        edit_catalog_model: &EditCatalogDto,
    ) -> anyhow::Result<CatalogDto> {
        let edit_model: EditCatalogModel = edit_catalog_model.clone().into();
        let catalog = self.repo.get_catalog_repo().put_catalog_by_id(catalog_id, &edit_model).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = catalog.into();
        Ok(dto)
    }

    async fn create_catalog(&self, new_catalog_model: &NewCatalogDto) -> anyhow::Result<CatalogDto> {
        let new_model: NewCatalogModel = new_catalog_model.clone().into();
        let catalog = self.repo.get_catalog_repo().create_catalog(&new_model).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        let dto = catalog.into();
        Ok(dto)
    }

    async fn create_main_catalog(&self, new_catalog_model: &NewCatalogDto) -> anyhow::Result<CatalogDto> {
        let new_model: NewCatalogModel = new_catalog_model.clone().into();
        let catalog = self.repo.get_catalog_repo().create_main_catalog(&new_model).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = catalog.into();
        Ok(dto)
    }

    async fn delete_catalog_by_id(&self, catalog_id: &Urn) -> anyhow::Result<()> {
        self.repo.get_catalog_repo().delete_catalog_by_id(catalog_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        Ok(())
    }
}
