pub mod data_source_connector;

use urn::Urn;
use rainbow_common::protocol::transfer::transfer_data_address::DataAddress;

#[async_trait::async_trait]
pub trait DataSourceConnectorTrait: Send + Sync {
    async fn start_streaming(
        &self,
        session_id: &Urn,
        sink_address: &DataAddress,
    ) -> anyhow::Result<()>;
    async fn stop_streaming(&self, session_id: &Urn) -> anyhow::Result<()>;
    async fn ping_source(&self, session_id: &Urn) -> anyhow::Result<()>;
}