pub mod data_source_connector;

use rainbow_common::protocol::data_address::DataAddress;
use urn::Urn;

#[async_trait::async_trait]
pub trait DataSourceConnectorTrait: Send + Sync {
    async fn start_streaming(&self, session_id: &Urn, sink_address: &DataAddress) -> anyhow::Result<()>;
    async fn stop_streaming(&self, session_id: &Urn) -> anyhow::Result<()>;
    async fn ping_source(&self, session_id: &Urn) -> anyhow::Result<()>;
}
