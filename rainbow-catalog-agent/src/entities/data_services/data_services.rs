use crate::cache::factory_trait::CatalogAgentCacheTrait;
use crate::data::factory_trait::CatalogAgentRepoTrait;
use crate::entities::data_services::{DataServiceDto, DataServiceEntityTrait, EditDataServiceDto, NewDataServiceDto};
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use urn::Urn;

pub struct DataServiceEntities {
    repo: Arc<dyn CatalogAgentRepoTrait>,
    cache: Arc<dyn CatalogAgentCacheTrait>,
}

impl DataServiceEntities {
    pub fn new(repo: Arc<dyn CatalogAgentRepoTrait>, cache: Arc<dyn CatalogAgentCacheTrait>) -> Self {
        Self { repo, cache }
    }
}

#[async_trait::async_trait]
impl DataServiceEntityTrait for DataServiceEntities {
    async fn get_all_data_services(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<DataServiceDto>> {
        let data_services = self.repo.get_dataservice_repo().get_all_data_services(limit, page).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let mut dtos = Vec::with_capacity(data_services.len());
        for c in data_services {
            let dto: DataServiceDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_batch_data_services(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<DataServiceDto>> {
        let data_services = self.repo.get_dataservice_repo().get_batch_data_services(ids).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let mut dtos = Vec::with_capacity(data_services.len());
        for c in data_services {
            let dto: DataServiceDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_data_services_by_catalog_id(&self, catalog_id: &Urn) -> anyhow::Result<Vec<DataServiceDto>> {
        let data_services =
            self.repo.get_dataservice_repo().get_data_services_by_catalog_id(catalog_id).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let mut dtos = Vec::with_capacity(data_services.len());
        for c in data_services {
            let dto: DataServiceDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_main_data_service(&self) -> anyhow::Result<Option<DataServiceDto>> {
        let data_service = self.repo.get_dataservice_repo().get_main_data_service().await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = data_service.map(|dto| dto.into());
        Ok(dto)
    }

    async fn get_data_service_by_id(&self, data_service_id: &Urn) -> anyhow::Result<Option<DataServiceDto>> {
        let data_service =
            self.repo.get_dataservice_repo().get_data_service_by_id(data_service_id).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let dto = data_service.map(|dto| dto.into());
        Ok(dto)
    }

    async fn put_data_service_by_id(
        &self,
        data_service_id: &Urn,
        edit_data_service_model: &EditDataServiceDto,
    ) -> anyhow::Result<DataServiceDto> {
        let edit_model = edit_data_service_model.clone().into();
        let data_service =
            self.repo.get_dataservice_repo().put_data_service_by_id(data_service_id, &edit_model).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;
        let dto = data_service.into();
        Ok(dto)
    }

    async fn create_data_service(&self, new_data_service_model: &NewDataServiceDto) -> anyhow::Result<DataServiceDto> {
        let new_model = new_data_service_model.clone().into();
        let data_service = self.repo.get_dataservice_repo().create_data_service(&new_model).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = data_service.into();
        Ok(dto)
    }

    async fn create_main_data_service(
        &self,
        new_data_service_model: &NewDataServiceDto,
    ) -> anyhow::Result<DataServiceDto> {
        let new_model = new_data_service_model.clone().into();
        let data_service =
            self.repo.get_dataservice_repo().create_main_data_service(&new_model).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let dto = data_service.into();
        Ok(dto)
    }

    async fn delete_data_service_by_id(&self, data_service_id: &Urn) -> anyhow::Result<()> {
        self.repo.get_dataservice_repo().delete_data_service_by_id(data_service_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        Ok(())
    }
}
