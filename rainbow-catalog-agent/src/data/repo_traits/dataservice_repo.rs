use crate::data::entities::dataservice;
use crate::data::entities::dataservice::{EditDataServiceModel, NewDataServiceModel};
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[async_trait::async_trait]
pub trait DataServiceRepositoryTrait: Send + Sync {
    async fn get_all_data_services(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataservice::Model>, DataServiceRepoErrors>;
    async fn get_batch_data_services(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<dataservice::Model>, DataServiceRepoErrors>;

    async fn get_data_services_by_catalog_id(
        &self,
        catalog_id: &Urn,
    ) -> anyhow::Result<Vec<dataservice::Model>, DataServiceRepoErrors>;

    async fn get_data_service_by_id(
        &self,
        data_service_id: &Urn,
    ) -> anyhow::Result<Option<dataservice::Model>, DataServiceRepoErrors>;
    async fn put_data_service_by_id(
        &self,
        data_service_id: &Urn,
        edit_data_service_model: &EditDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, DataServiceRepoErrors>;
    async fn create_data_service(
        &self,
        new_data_service_model: &NewDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, DataServiceRepoErrors>;
    async fn delete_data_service_by_id(&self, data_service_id: &Urn) -> anyhow::Result<(), DataServiceRepoErrors>;
}

#[derive(Error, Debug)]
pub enum DataServiceRepoErrors {
    #[error("DataService not found")]
    DataServiceNotFound,
    #[error("Error fetching data service. {0}")]
    ErrorFetchingDataService(Error),
    #[error("Error creating data service. {0}")]
    ErrorCreatingDataService(Error),
    #[error("Error deleting data service. {0}")]
    ErrorDeletingDataService(Error),
    #[error("Error updating data service. {0}")]
    ErrorUpdatingDataService(Error),
}
