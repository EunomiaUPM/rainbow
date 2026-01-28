pub(crate) mod data_plane_process_entity;

use crate::data::entities::data_plane_process;
use crate::data::entities::data_plane_process::NewDataPlaneProcessModel;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPlaneProcessDto {
    #[serde(flatten)]
    pub inner: data_plane_process::Model,
    pub data_plane_fields: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NewDataPlaneProcessDto {
    pub id: Urn,
    pub direction: String,
    pub state: String,
    pub fields: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EditDataPlaneProcessDto {
    pub state: Option<String>,
    pub fields: Option<HashMap<String, String>>,
}

impl From<NewDataPlaneProcessDto> for NewDataPlaneProcessModel {
    fn from(value: NewDataPlaneProcessDto) -> Self {
        Self { id: value.id, direction: value.direction, state: value.state }
    }
}

#[async_trait::async_trait]
pub trait DataPlaneProcessEntitiesTrait: Send + Sync + 'static {
    async fn get_all_data_plane_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<DataPlaneProcessDto>>;

    async fn get_batch_data_plane_processes(
        &self,
        ids: Vec<Urn>,
    ) -> anyhow::Result<Vec<DataPlaneProcessDto>>;

    async fn get_data_plane_process_by_id(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<DataPlaneProcessDto>>;

    async fn create_data_plane_process(
        &self,
        new_data_plane_process: &NewDataPlaneProcessDto,
    ) -> anyhow::Result<DataPlaneProcessDto>;

    async fn put_data_plane_process(
        &self,
        id: &Urn,
        edit_data_plane_process: &EditDataPlaneProcessDto,
    ) -> anyhow::Result<DataPlaneProcessDto>;

    async fn delete_data_plane_process(&self, id: &Urn) -> anyhow::Result<()>;
}
