use crate::db::entities::transfer_message::{self as transfer_message_model, NewTransferMessageModel};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use urn::Urn;

pub(crate) mod transfer_messages;

// Wrapper de respuesta
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferMessageDto {
    #[serde(flatten)]
    pub inner: transfer_message_model::Model,
}

// DTO de entrada para creación
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NewTransferMessageDto {
    pub id: Option<Urn>,
    pub transfer_agent_process_id: Urn,
    pub direction: String,
    pub protocol: String,
    pub message_type: String,
    pub state_transition_from: String,
    pub state_transition_to: String,
    pub payload: Option<Json>,
}

// Conversión de DTO a Modelo de Dominio (para pasar al Repo)
impl From<NewTransferMessageDto> for NewTransferMessageModel {
    fn from(dto: NewTransferMessageDto) -> Self {
        Self {
            id: dto.id,
            transfer_agent_process_id: dto.transfer_agent_process_id,
            direction: dto.direction,
            protocol: dto.protocol,
            message_type: dto.message_type,
            state_transition_from: dto.state_transition_from,
            state_transition_to: dto.state_transition_to,
            payload: dto.payload,
        }
    }
}

#[async_trait::async_trait]
pub trait TransferAgentMessagesTrait: Send + Sync + 'static {
    async fn get_all_transfer_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<TransferMessageDto>>;

    async fn get_messages_by_process_id(&self, process_id: &Urn) -> anyhow::Result<Vec<TransferMessageDto>>;

    async fn get_transfer_message_by_id(&self, id: &Urn) -> anyhow::Result<TransferMessageDto>;

    async fn create_transfer_message(&self, new_model: &NewTransferMessageDto) -> anyhow::Result<TransferMessageDto>;

    async fn delete_transfer_message(&self, id: &Urn) -> anyhow::Result<()>;
}
