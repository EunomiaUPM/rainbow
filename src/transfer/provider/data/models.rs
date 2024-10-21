use crate::transfer::provider::data::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Queryable,
    Identifiable,
    Selectable,
    Debug,
    PartialEq,
    Insertable,
    Serialize,
    Deserialize,
)]
#[diesel(table_name=transfer_processes)]
#[diesel(primary_key(provider_pid))]
pub struct TransferProcessModel {
    pub provider_pid: Uuid,
    pub consumer_pid: Uuid,
    pub agreement_id: Uuid,
    pub data_plane_id: Option<Uuid>,
    pub state: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(
    Queryable, Identifiable, Selectable, Debug, PartialEq, Insertable, Serialize, Deserialize,
)]
#[diesel(table_name=transfer_messages)]
pub struct TransferMessageModel {
    pub id: Uuid,
    pub transfer_process_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub message_type: String,
    pub from: String,
    pub to: String,
    pub content: serde_json::Value,
}
