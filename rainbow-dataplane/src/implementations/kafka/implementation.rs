use crate::core::{DataPlanePeer, DataPlanePeerDefaultBehavior};
use crate::implementations::kafka::KafkaDataPlane;
use axum::async_trait;
use axum::extract::Request;
use axum::response::Response;
use rainbow_common::protocol::transfer::TransferRequestMessage;
use uuid::Uuid;

#[async_trait]
impl DataPlanePeerDefaultBehavior for KafkaDataPlane {
    async fn bootstrap_data_plane_in_consumer(transfer_request: TransferRequestMessage) -> anyhow::Result<DataPlanePeer> {
        todo!()
    }

    async fn bootstrap_data_plane_in_provider(transfer_request: TransferRequestMessage, provider_pid: Uuid) -> anyhow::Result<DataPlanePeer> {
        todo!()
    }

    async fn set_data_plane_next_hop(data_plane_peer: DataPlanePeer, provider_pid: Uuid, consumer_pid: Uuid) -> anyhow::Result<DataPlanePeer> {
        todo!()
    }

    async fn connect_to_streaming_service(data_plane_id: Uuid) -> anyhow::Result<()> {
        todo!()
    }

    async fn disconnect_from_streaming_service(data_plane_id: Uuid) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_pull_data(data_plane_peer: DataPlanePeer, request: Request, extras: Option<String>) -> anyhow::Result<Response> {
        todo!()
    }

    async fn on_push_data(data_plane_peer: DataPlanePeer, request: Request, extras: Option<String>) -> anyhow::Result<Response> {
        todo!()
    }
}