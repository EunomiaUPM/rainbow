use crate::data::entities::dataservice;
use crate::data::entities::dataservice::{EditDataServiceModel, NewDataServiceModel};
use crate::data::repo_traits::catalog_db_errors::CatalogAgentRepoErrors;
use urn::Urn;

#[async_trait::async_trait]
pub trait DataServiceRepositoryTrait: Send + Sync {
    async fn get_all_data_services(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataservice::Model>, CatalogAgentRepoErrors>;
    async fn get_batch_data_services(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<dataservice::Model>, CatalogAgentRepoErrors>;

    async fn get_data_services_by_catalog_id(
        &self,
        catalog_id: &Urn,
    ) -> anyhow::Result<Vec<dataservice::Model>, CatalogAgentRepoErrors>;

    async fn get_data_service_by_id(
        &self,
        data_service_id: &Urn,
    ) -> anyhow::Result<Option<dataservice::Model>, CatalogAgentRepoErrors>;
    async fn put_data_service_by_id(
        &self,
        data_service_id: &Urn,
        edit_data_service_model: &EditDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, CatalogAgentRepoErrors>;
    async fn create_data_service(
        &self,
        new_data_service_model: &NewDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, CatalogAgentRepoErrors>;
    async fn delete_data_service_by_id(&self, data_service_id: &Urn) -> anyhow::Result<(), CatalogAgentRepoErrors>;
}
