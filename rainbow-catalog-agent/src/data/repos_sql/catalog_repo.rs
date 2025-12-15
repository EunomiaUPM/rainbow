use crate::data::entities::catalog;
use crate::data::entities::catalog::{EditCatalogModel, NewCatalogModel};
use crate::data::repo_traits::catalog_repo::{CatalogRepoErrors, CatalogRepositoryTrait};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use urn::Urn;

pub struct CatalogRepositoryForSql {
    db_connection: DatabaseConnection,
}

impl CatalogRepositoryForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl CatalogRepositoryTrait for CatalogRepositoryForSql {
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
        no_main_catalog: bool,
    ) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors> {
        let catalogs = match no_main_catalog {
            true => {
                catalog::Entity::find()
                    .filter(catalog::Column::DspaceMainCatalog.eq(false))
                    .limit(limit.unwrap_or(100000))
                    .offset(page.unwrap_or(0))
                    .all(&self.db_connection)
                    .await
            }
            false => {
                catalog::Entity::find()
                    .limit(limit.unwrap_or(100000))
                    .offset(page.unwrap_or(0))
                    .all(&self.db_connection)
                    .await
            }
        };

        match catalogs {
            Ok(catalogs) => Ok(catalogs),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingCatalog(err.into())),
        }
    }

    async fn get_batch_catalogs(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<catalog::Model>, CatalogRepoErrors> {
        let catalog_ids = ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let catalog_process =
            catalog::Entity::find().filter(catalog::Column::Id.is_in(catalog_ids)).all(&self.db_connection).await;
        match catalog_process {
            Ok(catalog_process) => Ok(catalog_process),
            Err(e) => Err(CatalogRepoErrors::ErrorFetchingCatalog(e.into())),
        }
    }

    async fn get_catalog_by_id(&self, catalog_id: &Urn) -> anyhow::Result<Option<catalog::Model>, CatalogRepoErrors> {
        let catalog_id = catalog_id.to_string();
        let catalog = catalog::Entity::find_by_id(catalog_id).one(&self.db_connection).await;
        match catalog {
            Ok(catalog) => Ok(catalog),
            Err(err) => Err(CatalogRepoErrors::ErrorFetchingCatalog(err.into())),
        }
    }

    async fn get_main_catalog(&self) -> anyhow::Result<Option<catalog::Model>, CatalogRepoErrors> {
        let catalog = catalog::Entity::find()
            .filter(catalog::Column::DspaceMainCatalog.eq(true))
            .one(&self.db_connection)
            .await
            .map_err(|err| CatalogRepoErrors::ErrorCreatingCatalog(err.into()))?;
        Ok(catalog)
    }

    async fn put_catalog_by_id(
        &self,
        catalog_id: &Urn,
        edit_catalog_model: &EditCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors> {
        let catalog_id = catalog_id.to_string();
        let old_model = catalog::Entity::find_by_id(catalog_id).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(CatalogRepoErrors::CatalogNotFound),
            },
            Err(err) => return Err(CatalogRepoErrors::ErrorFetchingCatalog(err.into())),
        };

        let mut old_active_model: catalog::ActiveModel = old_model.into();
        if let Some(foaf_home_page) = &edit_catalog_model.foaf_home_page {
            old_active_model.foaf_home_page = ActiveValue::Set(Some(foaf_home_page.clone()));
        }
        if let Some(dct_conforms_to) = &edit_catalog_model.dct_conforms_to {
            old_active_model.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to.clone()));
        }
        if let Some(dct_creator) = &edit_catalog_model.dct_creator {
            old_active_model.dct_creator = ActiveValue::Set(Some(dct_creator.clone()));
        }
        if let Some(dct_title) = &edit_catalog_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title.clone()));
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().into()));

        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(CatalogRepoErrors::ErrorUpdatingCatalog(err.into())),
        }
    }

    async fn create_catalog(
        &self,
        new_catalog_model: &NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors> {
        let model: catalog::ActiveModel = new_catalog_model.into();
        let catalog = catalog::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match catalog {
            Ok(catalog) => Ok(catalog),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingCatalog(err.into())),
        }
    }

    async fn create_main_catalog(
        &self,
        new_catalog_model: &NewCatalogModel,
    ) -> anyhow::Result<catalog::Model, CatalogRepoErrors> {
        let mut model: catalog::ActiveModel = new_catalog_model.into();
        model.dspace_main_catalog = ActiveValue::Set(true);
        let catalog = catalog::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match catalog {
            Ok(catalog) => Ok(catalog),
            Err(err) => Err(CatalogRepoErrors::ErrorCreatingCatalog(err.into())),
        }
    }

    async fn delete_catalog_by_id(&self, catalog_id: &Urn) -> anyhow::Result<(), CatalogRepoErrors> {
        let catalog_id = catalog_id.to_string();
        let catalog = catalog::Entity::delete_by_id(catalog_id).exec(&self.db_connection).await;
        match catalog {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(CatalogRepoErrors::CatalogNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(CatalogRepoErrors::ErrorDeletingCatalog(err.into())),
        }
    }
}
