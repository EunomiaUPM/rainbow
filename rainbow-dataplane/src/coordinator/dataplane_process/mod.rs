use crate::coordinator::dataplane_process::dataplane_process::DataPlaneProcess;
use crate::coordinator::transfer_event::TransferEvent;
use axum::async_trait;
use rainbow_common::adv_protocol::interplane::{DataPlaneProcessDirection, DataPlaneProcessState};
use serde::Serialize;
use urn::Urn;

pub mod dataplane_process;
pub mod dataplane_process_service;

pub struct DataPlaneProcessRequest {
    pub session_id: Urn,
    pub process_address: DataPlaneProcessAddress,
    pub downstream_hop: DataPlaneProcessAddress,
    pub process_direction: DataPlaneProcessDirection,
}

#[derive(Debug, Serialize)]
pub struct DataPlaneProcessAddress {
    pub protocol: String,
    pub url: String,
    pub auth_type: String,
    pub auth_content: String,
}

impl Default for DataPlaneProcessAddress {
    fn default() -> Self {
        Self {
            protocol: "".to_string(),
            url: "".to_string(),
            auth_type: "".to_string(),
            auth_content: "".to_string(),
        }
    }
}

#[async_trait]
pub trait DataPlaneDefaultBehaviour: Send + Sync {
    async fn create_dataplane_process(input: DataPlaneProcessRequest) -> anyhow::Result<DataPlaneProcess>;
    async fn get_dataplane_by_id(&self, dataplane_id: Urn) -> anyhow::Result<DataPlaneProcess>;
    async fn on_pull_data(&self, dataplane_id: Urn, event: TransferEvent) -> anyhow::Result<()>;
    async fn on_push_data(&self, dataplane_id: Urn, event: TransferEvent) -> anyhow::Result<()>;
    async fn tear_down_data_plane(&self, dataplane_id: Urn) -> anyhow::Result<()>;
    async fn connect_to_streaming_service(&self, dataplane_id: Urn) -> anyhow::Result<()>;
    async fn disconnect_from_streaming_service(&self, dataplane_id: Urn) -> anyhow::Result<()>;
}

#[async_trait]
pub trait DataPlaneProcessTrait: Send + Sync {
    async fn create_dataplane_process(&self, input: DataPlaneProcess) -> anyhow::Result<DataPlaneProcess>;
    async fn get_dataplane_processes(&self) -> anyhow::Result<Vec<DataPlaneProcess>>;
    async fn get_dataplane_process_by_id(&self, id: Urn) -> anyhow::Result<DataPlaneProcess>;
    async fn set_dataplane_process_status(
        &self,
        id: Urn,
        new_state: DataPlaneProcessState,
    ) -> anyhow::Result<DataPlaneProcess>;
}
