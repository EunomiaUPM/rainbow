pub(crate) mod persistence;

use crate::core::dsp::protocol_types::{TransferProcessMessageTrait, TransferProcessMessageWrapper};
use crate::entities::transfer_process::TransferProcessDto;
use std::sync::Arc;
use urn::Urn;

#[async_trait::async_trait]
pub trait TransferPersistenceTrait: Send + Sync {
    async fn fetch_process_by_process_id(&self, id: &str) -> anyhow::Result<TransferProcessDto>;
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
    async fn update_process_by_process_id(
        &self,
        id: &str,
        payload_dto: Arc<dyn TransferProcessMessageTrait>,
        payload_value: serde_json::Value,
    ) -> anyhow::Result<TransferProcessDto>;
}
