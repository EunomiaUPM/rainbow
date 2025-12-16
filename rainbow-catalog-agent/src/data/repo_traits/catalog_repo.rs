use crate::data::entities::catalog;
use crate::data::entities::catalog::{EditCatalogModel, NewCatalogModel};
use crate::data::repo_traits::catalog_db_errors::CatalogAgentRepoErrors;
use urn::Urn;

#[async_trait::async_trait]
pub trait CatalogRepositoryTrait: Send + Sync {
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        with_main_catalog: bool,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogAgentRepoErrors>;
    async fn get_batch_catalogs(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<catalog::Model>, CatalogAgentRepoErrors>;
    async fn get_catalog_by_id(
        &self,
        catalog_id: &Urn,
    ) -> anyhow::Result<Option<catalog::Model>, CatalogAgentRepoErrors>;
    async fn get_main_catalog(&self) -> anyhow::Result<Option<catalog::Model>, CatalogAgentRepoErrors>;

    async fn put_catalog_by_id(
        &self,
        catalog_id: &Urn,
        edit_catalog_model: &EditCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogAgentRepoErrors>;
    async fn create_catalog(
        &self,
        new_catalog_model: &NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogAgentRepoErrors>;

    async fn create_main_catalog(
        &self,
        new_catalog_model: &NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogAgentRepoErrors>;

    async fn delete_catalog_by_id(&self, catalog_id: &Urn) -> anyhow::Result<(), CatalogAgentRepoErrors>;
}
