use crate::data::entities::transfer_message as transfer_message_model;
use crate::data::entities::transfer_process as transfer_process_model;
use crate::data::entities::transfer_process::{EditTransferProcessModel, NewTransferProcessModel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use urn::Urn;

pub(crate) mod transfer_process;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferProcessDto {
    #[serde(flatten)]
    pub inner: transfer_process_model::Model,
    pub identifiers: HashMap<String, String>,
    pub messages: Vec<transfer_message_model::Model>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NewTransferProcessDto {
    pub id: Option<Urn>,
    pub state: String,
    pub associated_agent_peer: String,
    pub protocol: String,
    pub transfer_direction: String,
    pub agreement_id: Urn,
    pub callback_address: Option<String>,
    pub role: String,
    pub state_attribute: Option<String>,
    pub properties: Option<serde_json::Value>,
    pub identifiers: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EditTransferProcessDto {
    pub state: Option<String>,
    pub state_attribute: Option<String>,
    pub properties: Option<serde_json::Value>,
    pub error_details: Option<serde_json::Value>,
    pub identifiers: Option<HashMap<String, String>>,
}

impl From<NewTransferProcessDto> for NewTransferProcessModel {
    fn from(dto: NewTransferProcessDto) -> Self {
        Self {
            id: dto.id,
            state: dto.state,
            state_attribute: dto.state_attribute,
            associated_agent_peer: dto.associated_agent_peer,
            protocol: dto.protocol,
            transfer_direction: dto.transfer_direction,
            agreement_id: dto.agreement_id,
            callback_address: dto.callback_address,
            role: dto.role,
            properties: dto.properties.unwrap_or(serde_json::json!({})),
            error_details: None,
        }
    }
}

impl From<EditTransferProcessDto> for EditTransferProcessModel {
    fn from(dto: EditTransferProcessDto) -> Self {
        Self {
            state: dto.state,
            state_attribute: dto.state_attribute,
            properties: dto.properties,
            error_details: dto.error_details,
        }
    }
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait TransferAgentProcessesTrait: Send + Sync + 'static {
    async fn get_all_transfer_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<TransferProcessDto>>;
    async fn get_batch_transfer_processes(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<TransferProcessDto>>;
    async fn get_transfer_process_by_id(&self, id: &Urn) -> anyhow::Result<TransferProcessDto>;
    async fn get_transfer_process_by_key_id(&self, key_id: &str, id: &Urn) -> anyhow::Result<TransferProcessDto>;
    async fn get_transfer_process_by_key_value(&self, id: &Urn) -> anyhow::Result<TransferProcessDto>;

    async fn create_transfer_process(&self, new_model: &NewTransferProcessDto) -> anyhow::Result<TransferProcessDto>;
    async fn put_transfer_process(
        &self,
        id: &Urn,
        edit_model: &EditTransferProcessDto,
    ) -> anyhow::Result<TransferProcessDto>;
    async fn delete_transfer_process(&self, id: &Urn) -> anyhow::Result<()>;
}
