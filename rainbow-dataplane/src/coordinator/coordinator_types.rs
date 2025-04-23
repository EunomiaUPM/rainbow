use axum::async_trait;


pub struct DataPlanePeer;
pub struct DataPlaneBootstrapRequest;
pub struct DataPlaneNextHop;

#[async_trait]
pub trait DataPlaneDefaultBehaviour: Send + Sync {
    async fn bootstrap_data_plane(input: DataPlaneBootstrapRequest) -> anyhow::Result<DataPlanePeer>;
    async fn set_next_hop(input: DataPlaneNextHop) -> anyhow::Result<DataPlanePeer>;
    async fn on_pull_data(input: DataPlanePeer) -> anyhow::Result<DataPlanePeer>;
    async fn on_push_data(input: DataPlanePeer) -> anyhow::Result<DataPlanePeer>;
    async fn tear_down_data_plane(input: DataPlanePeer) -> anyhow::Result<()>;
}

#[async_trait]
pub trait DataPlaneProviderBehaviour: Send + Sync {
    async fn connect_to_streaming_service(input: DataPlanePeer) -> anyhow::Result<()>;
    async fn disconnect_from_streaming_service(input: DataPlanePeer) -> anyhow::Result<()>;
}