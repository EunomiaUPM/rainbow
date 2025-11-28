use crate::data::entities::data_plane_field;
use crate::data::entities::data_plane_field::{EditDataPlaneFieldModel, NewDataPlaneFieldModel};
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[async_trait::async_trait]
pub trait DataPlaneFieldRepoTrait: Send + Sync + 'static {
    async fn get_all_data_plane_fields(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<data_plane_field::Model>, DataPlaneFieldRepoErrors>;
    async fn get_batch_data_plane_fields(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<data_plane_field::Model>, DataPlaneFieldRepoErrors>;
    async fn get_all_data_plane_fields_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<data_plane_field::Model>, DataPlaneFieldRepoErrors>;
    async fn get_data_plane_field_by_id(
        &self,
        field_id: &Urn,
    ) -> anyhow::Result<Option<data_plane_field::Model>, DataPlaneFieldRepoErrors>;
    async fn create_data_plane_field(
        &self,
        process_id: &Urn,
        new_data_plane_field: &NewDataPlaneFieldModel,
    ) -> anyhow::Result<data_plane_field::Model, DataPlaneFieldRepoErrors>;
    async fn put_data_plane_field(
        &self,
        field_id: &Urn,
        edit_field: &EditDataPlaneFieldModel,
    ) -> anyhow::Result<data_plane_field::Model, DataPlaneFieldRepoErrors>;
    async fn delete_data_plane_field(&self, field_id: &Urn) -> anyhow::Result<(), DataPlaneFieldRepoErrors>;
}

#[derive(Debug, Error)]
pub enum DataPlaneFieldRepoErrors {
    #[error("Dataplane field not found")]
    DataplaneFieldNotFound,
    #[error("Error fetching dataplane field. {0}")]
    ErrorFetchingDataplaneField(Error),
    #[error("Error creating dataplane field. {0}")]
    ErrorCreatingDataplaneField(Error),
    #[error("Error deleting dataplane field. {0}")]
    ErrorDeletingDataplaneField(Error),
    #[error("Error updating dataplane field. {0}")]
    ErrorUpdatingDataplaneField(Error),
}
