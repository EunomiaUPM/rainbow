use crate::coordinator::data_source_connector::DataSourceConnectorTrait;
use rainbow_common::protocol::transfer::transfer_data_address::DataAddress;
use urn::Urn;

pub struct DataSourceConnector;
impl DataSourceConnector {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl DataSourceConnectorTrait for DataSourceConnector {
    async fn start_streaming(&self, session_id: &Urn, sink_address: &DataAddress) -> anyhow::Result<()> {
        todo!()
    }

    async fn stop_streaming(&self, session_id: &Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn ping_source(&self, session_id: &Urn) -> anyhow::Result<()> {
        todo!()
    }
}
