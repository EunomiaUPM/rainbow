pub(crate) mod transfer_event_entity;

use crate::data::entities::transfer_event;
use crate::data::entities::transfer_event::NewTransferEventModel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferEventDto {
    #[serde(flatten)]
    pub inner: transfer_event::Model,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NewTransferEventDto {
    pub id: Urn,
    pub dataplane_process_id: Urn,
    pub from: String,
    pub to: String,
    pub payload: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EditTransferEventDto {
    pub from: Option<String>,
    pub to: Option<String>,
    pub payload: Option<Value>,
}

impl From<NewTransferEventDto> for NewTransferEventModel {
    fn from(value: NewTransferEventDto) -> Self {
        Self { from: value.from, to: value.to, payload: value.payload }
    }
}

#[async_trait::async_trait]
pub trait TransferEventEntitiesTrait: Send + Sync + 'static {
    async fn get_all_transfer_events(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<TransferEventDto>>;

    async fn get_batch_transfer_events(
        &self,
        ids: Vec<Urn>,
    ) -> anyhow::Result<Vec<TransferEventDto>>;

    async fn get_transfer_event_by_id(&self, id: &Urn) -> anyhow::Result<Option<TransferEventDto>>;

    async fn get_transfer_events_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<TransferEventDto>>;

    async fn create_transfer_event(
        &self,
        new_transfer_event: &NewTransferEventDto,
    ) -> anyhow::Result<TransferEventDto>;
}
