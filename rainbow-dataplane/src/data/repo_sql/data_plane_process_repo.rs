use crate::data::entities::data_plane_process;
use crate::data::entities::data_plane_process::{
    EditDataPlaneProcessModel, NewDataPlaneProcessModel,
};
use crate::data::repo_traits::data_plane_process_repo::{
    DataPlaneProcessRepoErrors, DataPlaneProcessRepoTrait,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QuerySelect,
};
use urn::Urn;

pub struct DataPlaneProcessRepoForSql {
    db_connection: DatabaseConnection,
}
impl DataPlaneProcessRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl DataPlaneProcessRepoTrait for DataPlaneProcessRepoForSql {
    async fn get_all_data_plane_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<data_plane_process::Model>, DataPlaneProcessRepoErrors> {
        let processes = data_plane_process::Entity::find()
            .limit(limit.unwrap_or(20))
            .offset(page.map(|p| p * limit.unwrap_or(20)).unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match processes {
            Ok(processes) => Ok(processes),
            Err(e) => Err(DataPlaneProcessRepoErrors::ErrorFetchingDataplaneProcess(e.into())),
        }
    }

    async fn get_batch_data_plane_processes(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<data_plane_process::Model>, DataPlaneProcessRepoErrors> {
        let process_id = ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let processes = data_plane_process::Entity::find()
            .filter(data_plane_process::Column::Id.is_in(process_id))
            .all(&self.db_connection)
            .await;
        match processes {
            Ok(processes) => Ok(processes),
            Err(e) => Err(DataPlaneProcessRepoErrors::ErrorFetchingDataplaneProcess(e.into())),
        }
    }

    async fn get_data_plane_processes_by_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Option<data_plane_process::Model>, DataPlaneProcessRepoErrors> {
        let pid = process_id.to_string();
        let process = data_plane_process::Entity::find_by_id(pid).one(&self.db_connection).await;
        match process {
            Ok(process) => Ok(process),
            Err(e) => Err(DataPlaneProcessRepoErrors::ErrorFetchingDataplaneProcess(e.into())),
        }
    }

    async fn create_data_plane_processes(
        &self,
        new_data_plane_process: &NewDataPlaneProcessModel,
    ) -> anyhow::Result<data_plane_process::Model, DataPlaneProcessRepoErrors> {
        let model: data_plane_process::ActiveModel = new_data_plane_process.clone().into();
        let process = data_plane_process::Entity::insert(model)
            .exec_with_returning(&self.db_connection)
            .await;
        match process {
            Ok(process) => Ok(process),
            Err(e) => {
                return Err(DataPlaneProcessRepoErrors::ErrorCreatingDataplaneProcess(e.into()))
            }
        }
    }

    async fn put_data_plane_processes(
        &self,
        process_id: &Urn,
        edit_data_plane_process: &EditDataPlaneProcessModel,
    ) -> anyhow::Result<data_plane_process::Model, DataPlaneProcessRepoErrors> {
        let id = process_id.to_string();
        let old_model = data_plane_process::Entity::find_by_id(id).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(DataPlaneProcessRepoErrors::DataplaneProcessNotFound),
            },
            Err(e) => {
                return Err(DataPlaneProcessRepoErrors::ErrorFetchingDataplaneProcess(e.into()))
            }
        };
        let mut old_active_model: data_plane_process::ActiveModel = old_model.into();
        if let Some(state) = &edit_data_plane_process.state {
            old_active_model.state = ActiveValue::Set(state.clone());
        }
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().into()));
        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(e) => Err(DataPlaneProcessRepoErrors::ErrorUpdatingDataplaneProcess(e.into())),
        }
    }

    async fn delete_data_plane_processes(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<(), DataPlaneProcessRepoErrors> {
        let id = process_id.to_string();
        let transfer_process =
            data_plane_process::Entity::delete_by_id(id).exec(&self.db_connection).await;
        match transfer_process {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(DataPlaneProcessRepoErrors::DataplaneProcessNotFound),
                _ => Ok(()),
            },
            Err(e) => Err(DataPlaneProcessRepoErrors::ErrorDeletingDataplaneProcess(e.into())),
        }
    }
}
