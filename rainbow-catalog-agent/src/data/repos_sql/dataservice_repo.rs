use crate::data::entities::dataservice::{EditDataServiceModel, NewDataServiceModel};
use crate::data::entities::{catalog, dataservice};
use crate::data::repo_traits::dataservice_repo::{DataServiceRepoErrors, DataServiceRepositoryTrait};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QuerySelect,
};
use urn::Urn;

pub struct DataServiceRepositoryForSql {
    db_connection: DatabaseConnection,
}

impl DataServiceRepositoryForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl DataServiceRepositoryTrait for DataServiceRepositoryForSql {
    async fn get_all_data_services(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<dataservice::Model>, DataServiceRepoErrors> {
        let data_services = dataservice::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match data_services {
            Ok(data_services) => Ok(data_services),
            Err(err) => Err(DataServiceRepoErrors::ErrorFetchingDataService(err.into())),
        }
    }

    async fn get_batch_data_services(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<dataservice::Model>, DataServiceRepoErrors> {
        let dataset_ids = ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let dataset_process = dataservice::Entity::find()
            .filter(dataservice::Column::Id.is_in(dataset_ids))
            .all(&self.db_connection)
            .await;
        match dataset_process {
            Ok(dataset_process) => Ok(dataset_process),
            Err(e) => Err(DataServiceRepoErrors::ErrorFetchingDataService(e.into())),
        }
    }

    async fn get_data_services_by_catalog_id(
        &self,
        catalog_id: &Urn,
    ) -> anyhow::Result<Vec<dataservice::Model>, DataServiceRepoErrors> {
        let catalog_id = catalog_id.to_string();

        let catalog = catalog::Entity::find_by_id(catalog_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| DataServiceRepoErrors::ErrorFetchingDataService(e.into()))?;
        if catalog.is_none() {
            return Err(DataServiceRepoErrors::DataServiceNotFound);
        }

        let data_services = dataservice::Entity::find()
            .filter(dataservice::Column::CatalogId.eq(catalog_id))
            .all(&self.db_connection)
            .await;
        match data_services {
            Ok(data_services) => Ok(data_services),
            Err(err) => Err(DataServiceRepoErrors::ErrorFetchingDataService(err.into())),
        }
    }

    async fn get_data_service_by_id(
        &self,
        data_service_id: &Urn,
    ) -> anyhow::Result<Option<dataservice::Model>, DataServiceRepoErrors> {
        let data_service_id = data_service_id.to_string();
        let data_service = dataservice::Entity::find_by_id(data_service_id).one(&self.db_connection).await;
        match data_service {
            Ok(data_service) => Ok(data_service),
            Err(err) => Err(DataServiceRepoErrors::ErrorFetchingDataService(err.into())),
        }
    }

    async fn put_data_service_by_id(
        &self,
        data_service_id: &Urn,
        edit_data_service_model: &EditDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, DataServiceRepoErrors> {
        let data_service_id = data_service_id.to_string();
        let old_model = dataservice::Entity::find_by_id(data_service_id).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(DataServiceRepoErrors::DataServiceNotFound),
            },
            Err(err) => return Err(DataServiceRepoErrors::ErrorFetchingDataService(err.into())),
        };
        let mut old_active_model: dataservice::ActiveModel = old_model.into();
        if let Some(dcat_endpoint_description) = edit_data_service_model.dcat_endpoint_description {
            old_active_model.dcat_endpoint_description = ActiveValue::Set(Some(dcat_endpoint_description));
        }
        if let Some(dcat_endpoint_url) = edit_data_service_model.dcat_endpoint_url {
            old_active_model.dcat_endpoint_url = ActiveValue::Set(dcat_endpoint_url);
        }
        if let Some(dct_conforms_to) = edit_data_service_model.dct_conforms_to {
            old_active_model.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
        }
        if let Some(dct_creator) = edit_data_service_model.dct_creator {
            old_active_model.dct_creator = ActiveValue::Set(Some(dct_creator));
        }
        if let Some(dct_title) = edit_data_service_model.dct_title {
            old_active_model.dct_title = ActiveValue::Set(Some(dct_title));
        }
        if let Some(dct_description) = edit_data_service_model.dct_description {
            old_active_model.dct_description = ActiveValue::Set(Some(dct_description));
        }

        old_active_model.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().into()));
        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(err) => Err(DataServiceRepoErrors::ErrorUpdatingDataService(err.into())),
        }
    }

    async fn create_data_service(
        &self,
        new_data_service_model: &NewDataServiceModel,
    ) -> anyhow::Result<dataservice::Model, DataServiceRepoErrors> {
        let model: dataservice::ActiveModel = new_data_service_model.into();
        let data_service = dataservice::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match data_service {
            Ok(data_service) => Ok(data_service),
            Err(err) => Err(DataServiceRepoErrors::ErrorCreatingDataService(err.into())),
        }
    }

    async fn delete_data_service_by_id(&self, data_service_id: &Urn) -> anyhow::Result<(), DataServiceRepoErrors> {
        let data_service_id = data_service_id.to_string();
        let data_service = dataservice::Entity::delete_by_id(data_service_id).exec(&self.db_connection).await;
        match data_service {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(DataServiceRepoErrors::DataServiceNotFound),
                _ => Ok(()),
            },
            Err(err) => Err(DataServiceRepoErrors::ErrorDeletingDataService(err.into())),
        }
    }
}
