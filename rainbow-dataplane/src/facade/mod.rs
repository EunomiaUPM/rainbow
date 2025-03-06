use crate::data_plane_peer::DataPlanePeer;
use axum::async_trait;
use rainbow_common::protocol::transfer::TransferRequestMessage;
use urn::Urn;
pub mod facade;

#[async_trait]
pub trait DataPlaneFacade: Send + Sync {
    async fn bootstrap_data_plane_in_consumer(
        &self,
        transfer_request: TransferRequestMessage,
    ) -> anyhow::Result<DataPlanePeer>;
    async fn bootstrap_data_plane_in_provider(
        &self,
        transfer_request: TransferRequestMessage,
        provider_pid: Urn,
    ) -> anyhow::Result<DataPlanePeer>;
    async fn set_data_plane_next_hop(
        &self,
        data_plane_peer: DataPlanePeer,
        provider_pid: Urn,
        consumer_pid: Urn,
    ) -> anyhow::Result<DataPlanePeer>;
    async fn connect_to_streaming_service(&self, data_plane_id: Urn) -> anyhow::Result<()>;
    async fn disconnect_from_streaming_service(&self, data_plane_id: Urn) -> anyhow::Result<()>;
}
