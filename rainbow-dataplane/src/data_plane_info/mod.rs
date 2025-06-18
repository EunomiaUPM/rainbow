use crate::coordinator::dataplane_process::dataplane_process::DataPlaneProcess;
use axum::async_trait;
use urn::Urn;

pub mod data_plane_info;

#[async_trait]
pub trait DataPlaneInfoTrait: Send + Sync {
    async fn get_data_plane_info_by_session_id(&self, session_id: Urn) -> anyhow::Result<DataPlaneProcess>;
}