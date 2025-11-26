pub(crate) mod persistence_protocol;
pub(crate) mod persistence_rpc;

use crate::entities::transfer_messages::TransferAgentMessagesTrait;
use crate::entities::transfer_process::{TransferAgentProcessesTrait, TransferProcessDto};
use crate::protocols::dsp::protocol_types::TransferProcessMessageTrait;
use std::sync::Arc;
use urn::Urn;

#[async_trait::async_trait]
#[allow(unused)]
pub trait TransferPersistenceTrait: Send + Sync {
    async fn get_transfer_process_service(&self) -> anyhow::Result<Arc<dyn TransferAgentProcessesTrait>>;
    async fn get_transfer_message_service(&self) -> anyhow::Result<Arc<dyn TransferAgentMessagesTrait>>;
    async fn fetch_process(&self, id: &str) -> anyhow::Result<TransferProcessDto>;
    async fn create_process(
        &self,
        protocol: &str,
        direction: &str,
        provider_pid: Option<Urn>,
        provider_address: Option<String>,
        payload_dto: Arc<dyn TransferProcessMessageTrait>,
        payload_value: serde_json::Value,
    ) -> anyhow::Result<TransferProcessDto>;
    async fn update_process(
        &self,
        id: &str,
        payload_dto: Arc<dyn TransferProcessMessageTrait>,
        payload_value: serde_json::Value,
    ) -> anyhow::Result<TransferProcessDto>;
}
