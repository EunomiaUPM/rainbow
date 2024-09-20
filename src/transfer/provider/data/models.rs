use crate::transfer::provider::data::schema::*;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Insertable)]
#[diesel(table_name=transfer_processes)]
#[primary_key(provider_pid)]
pub struct TransferProcess {
    pub provider_pid: Uuid,
    pub consumer_pid: Uuid,
    pub state: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Insertable)]
#[diesel(table_name=transfer_messages)]
pub struct TransferMessage {
    pub id: Uuid,
    pub transfer_process_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub message_type: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Insertable)]
#[diesel(table_name=transfer_message_fields)]
pub struct TransferField {
    pub id: Uuid,
    pub transfer_message_id: Uuid,
    pub key: String,
    pub value: String,
    pub parent: Option<Uuid>,
}
