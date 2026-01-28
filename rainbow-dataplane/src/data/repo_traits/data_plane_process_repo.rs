use crate::data::entities::data_plane_process;
use crate::data::entities::data_plane_process::{
    EditDataPlaneProcessModel, NewDataPlaneProcessModel,
};
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[async_trait::async_trait]
pub trait DataPlaneProcessRepoTrait: Send + Sync + 'static {
    async fn get_all_data_plane_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<data_plane_process::Model>, DataPlaneProcessRepoErrors>;
    async fn get_batch_data_plane_processes(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<data_plane_process::Model>, DataPlaneProcessRepoErrors>;
    async fn get_data_plane_processes_by_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Option<data_plane_process::Model>, DataPlaneProcessRepoErrors>;
    async fn create_data_plane_processes(
        &self,
        new_data_plane_process: &NewDataPlaneProcessModel,
    ) -> anyhow::Result<data_plane_process::Model, DataPlaneProcessRepoErrors>;
    async fn put_data_plane_processes(
        &self,
        process_id: &Urn,
        new_data_plane_process: &EditDataPlaneProcessModel,
    ) -> anyhow::Result<data_plane_process::Model, DataPlaneProcessRepoErrors>;
    async fn delete_data_plane_processes(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<(), DataPlaneProcessRepoErrors>;
}

#[derive(Debug, Error)]
pub enum DataPlaneProcessRepoErrors {
    #[error("Dataplane process not found")]
    DataplaneProcessNotFound,
    #[error("Error fetching dataplane process. {0}")]
    ErrorFetchingDataplaneProcess(Error),
    #[error("Error creating dataplane process. {0}")]
    ErrorCreatingDataplaneProcess(Error),
    #[error("Error deleting dataplane process. {0}")]
    ErrorDeletingDataplaneProcess(Error),
    #[error("Error updating dataplane process. {0}")]
    ErrorUpdatingDataplaneProcess(Error),
}
