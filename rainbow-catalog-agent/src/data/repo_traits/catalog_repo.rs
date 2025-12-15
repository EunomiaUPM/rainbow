use crate::data::entities::catalog;
use crate::data::entities::catalog::{EditCatalogModel, NewCatalogModel};
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[async_trait::async_trait]
pub trait CatalogRepositoryTrait: Send + Sync {
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        no_main_catalog: bool,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors>;
    async fn get_batch_catalogs(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors>;
    async fn get_catalog_by_id(&self, catalog_id: &Urn) -> anyhow::Result<Option<catalog::Model>, CatalogRepoErrors>;
    async fn get_main_catalog(&self) -> anyhow::Result<Option<catalog::Model>, CatalogRepoErrors>;

    async fn put_catalog_by_id(
        &self,
        catalog_id: &Urn,
        edit_catalog_model: &EditCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors>;
    async fn create_catalog(
        &self,
        new_catalog_model: &NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors>;

    async fn create_main_catalog(
        &self,
        new_catalog_model: &NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors>;

    async fn delete_catalog_by_id(&self, catalog_id: &Urn) -> anyhow::Result<(), CatalogRepoErrors>;
}

#[derive(Error, Debug)]
pub enum CatalogRepoErrors {
    #[error("Catalog not found")]
    CatalogNotFound,
    #[error("Error fetching catalog. {0}")]
    ErrorFetchingCatalog(Error),
    #[error("Error creating catalog. {0}")]
    ErrorCreatingCatalog(Error),
    #[error("Error deleting catalog. {0}")]
    ErrorDeletingCatalog(Error),
    #[error("Error updating catalog. {0}")]
    ErrorUpdatingCatalog(Error),
}
