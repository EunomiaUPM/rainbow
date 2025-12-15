use crate::data::entities::dataset::{EditDatasetModel, NewDatasetModel};
use crate::data::entities::{catalog, dataset};
use crate::data::repo_traits::dataset_repo::{DatasetRepoErrors, DatasetRepositoryTrait};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use urn::Urn;

pub struct DatasetRepositoryForSql {
    db_connection: DatabaseConnection,
}

impl DatasetRepositoryForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl DatasetRepositoryTrait for DatasetRepositoryForSql {
    async fn get_all_datasets(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataset::Model>, DatasetRepoErrors> {
        let datasets = dataset::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match datasets {
            Ok(datasets) => Ok(datasets),
            Err(err) => Err(DatasetRepoErrors::ErrorFetchingDataset(err.into())),
        }
    }

    async fn get_batch_datasets(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<dataset::Model>, DatasetRepoErrors> {
        let dataset_ids = ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let dataset_process =
            dataset::Entity::find().filter(dataset::Column::Id.is_in(dataset_ids)).all(&self.db_connection).await;
        match dataset_process {
            Ok(dataset_process) => Ok(dataset_process),
            Err(e) => Err(DatasetRepoErrors::ErrorFetchingDataset(e.into())),
        }
    }

    async fn get_datasets_by_catalog_id(
        &self,
        catalog_id: &Urn,
    ) -> anyhow::Result<Vec<dataset::Model>, DatasetRepoErrors> {
        let catalog_id = catalog_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| DatasetRepoErrors::ErrorFetchingDataset(e.into()))?;
        if catalog.is_none() {
            return Err(DatasetRepoErrors::DatasetNotFound);
        }

        let datasets =
            dataset::Entity::find().filter(dataset::Column::CatalogId.eq(catalog_id)).all(&self.db_connection).await;
        match datasets {
            Ok(datasets) => Ok(datasets),
            Err(err) => Err(DatasetRepoErrors::ErrorFetchingDataset(err.into())),
        }
    }

    async fn get_dataset_by_id(&self, dataset_id: &Urn) -> anyhow::Result<Option<dataset::Model>, DatasetRepoErrors> {
        let dataset_id = dataset_id.to_string();
        let dataset = dataset::Entity::find_by_id(dataset_id).one(&self.db_connection).await;
        match dataset {
            Ok(dataset) => Ok(dataset),
            Err(err) => Err(DatasetRepoErrors::ErrorFetchingDataset(err.into())),
        }
    }

    async fn put_dataset_by_id(
        &self,
        dataset_id: &Urn,
        edit_dataset_model: &EditDatasetModel,
    ) -> anyhow::Result<dataset::Model, DatasetRepoErrors> {
        let dataset_id = dataset_id.to_string();

        let old_model = dataset::Entity::find_by_id(dataset_id).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(DatasetRepoErrors::DatasetNotFound),
            },
            Err(err) => return Err(DatasetRepoErrors::ErrorFetchingDataset(err.into())),
        };

        let mut old_active_model: dataset::ActiveModel = old_model.into();
        if let Some(dct_conforms_to) = &edit_dataset_model.dct_conforms_to {
            old_active_model.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to.clone()));
        }
        if let Some(dct_creator) = &edit_dataset_model.dct_creator {
            old_active_model.dct_creator = ActiveValue::Set(Some(dct_creator.clone()));
        }
        if let Some(dct_title) = &edit_dataset_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title.clone()));
        }
        if let Some(dct_description) = &edit_dataset_model.dct_description {
            old_active_model.dct_description = ActiveValue::Set(Some(dct_description.clone()));
        }
        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().into()));

        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(DatasetRepoErrors::ErrorUpdatingDataset(err.into())),
        }
    }

    async fn create_dataset(
        &self,
        new_dataset_model: &NewDatasetModel,
    ) -> anyhow::Result<dataset::Model, DatasetRepoErrors> {
        let model: dataset::ActiveModel = new_dataset_model.into();
        let dataset = dataset::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match dataset {
            Ok(dataset) => Ok(dataset),
            Err(err) => Err(DatasetRepoErrors::ErrorCreatingDataset(err.into())),
        }
    }

    async fn delete_dataset_by_id(&self, dataset_id: &Urn) -> anyhow::Result<(), DatasetRepoErrors> {
        let dataset_id = dataset_id.to_string();
        let dataset = dataset::Entity::delete_by_id(dataset_id).exec(&self.db_connection).await;
        match dataset {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(DatasetRepoErrors::DatasetNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(DatasetRepoErrors::ErrorDeletingDataset(err.into())),
        }
    }
}
