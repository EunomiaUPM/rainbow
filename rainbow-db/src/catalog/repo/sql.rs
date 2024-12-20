use crate::catalog::entities::catalog;
use crate::catalog::entities::dataset;
use crate::catalog::repo::{
    CatalogRepo, DatasetRepo, EditCatalogModel, EditDatasetModel, NewCatalogModel, NewDatasetModel,
};
use anyhow::bail;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::utils::get_urn;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, QuerySelect};
use uuid::Uuid;
use urn::Urn;
use axum::async_trait;

pub struct CatalogRepoForSql {}

#[async_trait]
impl CatalogRepo for CatalogRepoForSql {
       
    async fn get_all_catalogs(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<catalog::Model>> {
        let db_connection = get_db_connection().await;
        let catalogs = catalog::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match catalogs {
            Ok(catalogs) => Ok(catalogs),
            Err(_) => bail!("Failed to fetch catalogs"),
        }
    }

    async fn get_catalog_by_id(&self, catalog_id: String) -> anyhow::Result<Option<catalog::Model>> {
        let db_connection = get_db_connection().await;
        let catalog = catalog::Entity::find_by_id(catalog_id).one(db_connection).await;
        match catalog {
            Ok(catalog) => Ok(catalog),
            Err(_) => bail!("Failed to fetch catalogs"),
        }
    }

    async fn put_catalog_by_id(
        &self,
        catalog_id: String,
        edit_catalog_model: EditCatalogModel,
    ) -> anyhow::Result<catalog::Model> {
        let db_connection = get_db_connection().await;

        let old_model = catalog::Entity::find_by_id(catalog_id).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => bail!("Catalog not found"),
            },
            Err(_) => bail!("Failed to fetch catalogs"),
        };

        let mut old_active_model: catalog::ActiveModel = old_model.into();
        if let Some(foaf_home_page) = edit_catalog_model.foaf_home_page {
            old_active_model.foaf_home_page = ActiveValue::Set(Some(foaf_home_page));
        }
        if let Some(dct_conforms_to) = edit_catalog_model.dct_conforms_to {
            old_active_model.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
        }
        if let Some(dct_creator) = edit_catalog_model.dct_creator {
            old_active_model.dct_creator = ActiveValue::Set(Some(dct_creator));
        }
        if let Some(dct_title) = edit_catalog_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title));
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(_) => bail!("Failed to update model"),
        }
    }

    async fn create_catalog(
        &self,
        catalog_id: String,
        new_catalog_model: NewCatalogModel,
    ) -> anyhow::Result<catalog::Model> {
        let db_connection = get_db_connection().await;
        let urn = get_urn(new_catalog_model.id.map(|urn_id|urn_id.parse::<Urn>().unwrap()));
        let model = catalog::ActiveModel {
            id: ActiveValue::Set(urn.to_string()),
            foaf_home_page: ActiveValue::Set(new_catalog_model.foaf_home_page),
            dct_conforms_to: ActiveValue::Set(new_catalog_model.dct_conforms_to),
            dct_creator: ActiveValue::Set(new_catalog_model.dct_creator),
            dct_identifier: ActiveValue::Set(Some(urn.to_string())),
            dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified: ActiveValue::Set(None),
            dct_title: ActiveValue::Set(new_catalog_model.dct_title),
            dspace_participant_id: ActiveValue::Set(Some(Uuid::new_v4().to_string())), // TODO create participant global id (create global setup)
        };
        let catalog = catalog::Entity::insert(model).exec_with_returning(db_connection).await;
        match catalog {
            Ok(catalog) => Ok(catalog),
            Err(_) => bail!("Failed to update model"),
        }
    }

    async fn delete_catalog_by_id(&self, catalog_id: String) -> anyhow::Result<()> {
        let db_connection = get_db_connection().await;
        let catalog = catalog::Entity::delete_by_id(catalog_id).exec(db_connection).await;
        match catalog {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => bail!("Not found"),
                _ => Ok(()),
            },
            Err(_) => bail!("Failed to update model"),
        }
    }
}

#[async_trait]
impl DatasetRepo for CatalogRepoForSql {
    async fn get_all_datasets(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>> {
        let db_connection = get_db_connection().await;
        let datasets = dataset::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await;
        match datasets {
            Ok(datasets) => Ok(datasets),
            Err(_) => bail!("Failed to fetch datasets"),
        }
    }

    async fn get_datasets_by_id(&self, dataset_id: String) -> anyhow::Result<Option<dataset::Model>> {
        let db_connection = get_db_connection().await;
        let dataset = dataset::Entity::find_by_id(dataset_id).one(db_connection).await;
        match dataset {
            Ok(dataset) => Ok(dataset),
            Err(_) => bail!("Failed to fetch dataset"),
        }
    }

    async fn put_datasets_by_id(
        &self,
        dataset_id: String,
        edit_dataset_model: EditDatasetModel,
    ) -> anyhow::Result<dataset::Model> {
        let db_connection = get_db_connection().await;

        let old_model = dataset::Entity::find_by_id(dataset_id).one(db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => bail!("Dataset not found"),
            },
            Err(_) => bail!("Failed to fetch datasets"),
        };

        let mut old_active_model: dataset::ActiveModel = old_model.into();
        if let Some(dct_conforms_to) = edit_dataset_model.dct_conforms_to {
            old_active_model.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
        }
        if let Some(dct_creator) = edit_dataset_model.dct_creator {
            old_active_model.dct_creator = ActiveValue::Set(Some(dct_creator));
        }
        if let Some(dct_title) = edit_dataset_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title));
        }
        if let Some(dct_description) = edit_dataset_model.dct_description {
            old_active_model.dct_description = ActiveValue::Set(Some(dct_description));
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(_) => bail!("Failed to update dataset"),
        }
    }

    async fn create_dataset(
        &self,
        catalog_id: String,
        new_dataset_model: NewDatasetModel,
    ) -> anyhow::Result<dataset::Model> {
        let db_connection = get_db_connection().await;
        let urn = get_urn(new_dataset_model.id.map(|urn_id|urn_id.parse::<Urn>().unwrap()));
        let model = dataset::ActiveModel {
            id: ActiveValue::Set(urn.to_string()),
            dct_conforms_to: ActiveValue::Set(new_dataset_model.dct_conforms_to),
            dct_creator: ActiveValue::Set(new_dataset_model.dct_creator),
            dct_identifier: ActiveValue::Set(Option::from(urn.to_string())),
            dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            dct_modified: ActiveValue::Set(None),
            dct_title: ActiveValue::Set(new_dataset_model.dct_title),
            dct_description: ActiveValue::Set(new_dataset_model.dct_description),
            catalog_id: ActiveValue::Set(catalog_id),
        };
        let dataset = dataset::Entity::insert(model).exec_with_returning(db_connection).await;
        match dataset {
            Ok(dataset) => Ok(dataset),
            Err(_) => bail!("Failed to create dataset"),
        }
    }


    async fn delete_dataset_by_id(&self, dataset_id: String) -> anyhow::Result<()> {
        let db_connection = get_db_connection().await;
        let dataset = dataset::Entity::delete_by_id(dataset_id).exec(db_connection).await;
        match dataset {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => bail!("Not found"),
                _ => Ok(()),
            },
            Err(_) => bail!("Failed to delete dataset"),
        }
    }
}
