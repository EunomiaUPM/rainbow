pub(crate) mod persistence;

use std::sync::Arc;
use crate::core::dsp::protocol_types::{TransferProcessMessageTrait, TransferProcessMessageWrapper};
use crate::entities::transfer_process::TransferProcessDto;

#[async_trait::async_trait]
pub trait TransferPersistenceTrait: Send + Sync {
    async fn fetch_process(&self, id: &str) -> anyhow::Result<TransferProcessDto>;
    async fn create_process(
        &self,
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
