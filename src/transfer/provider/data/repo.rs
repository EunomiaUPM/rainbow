use crate::db::get_db_connection;
use crate::transfer::protocol::messages::TransferState;
use crate::transfer::provider::data::models::TransferProcess;
use crate::transfer::provider::data::schema::transfer_processes::dsl::transfer_processes;
use diesel::prelude::*;
use serde_json::Value;
use uuid::Uuid;

use crate::transfer::provider::data::schema::transfer_processes::dsl::*;

pub fn get_all_transfer_processes() -> anyhow::Result<Vec<TransferProcess>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_processes
        .limit(20)
        .select(TransferProcess::as_select())
        .load(connection)?;

    Ok(transaction)
}

pub fn get_transfer_process_by_consumer_pid(consumer_pid_in: Uuid) -> anyhow::Result<Option<TransferProcess>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_processes
        .filter(consumer_pid.eq(consumer_pid_in))
        .select(TransferProcess::as_select())
        .first(connection)
        .optional()?;
    Ok(transaction)
}

// sigo aqui....
pub fn create_transfer_process(transfer_process: TransferProcess) -> anyhow::Result<()> {
    Ok(())
}

pub fn update_transfer_process_by_consumer_pid(consumer_pid_in: Uuid, new_state: TransferState) -> anyhow::Result<Option<TransferProcess>> {
    Ok(None)
}

pub fn get_all_transfer_messages() -> anyhow::Result<Vec<Value>> {
    Ok(vec![])
}

pub fn get_all_transfer_messages_by_type() -> anyhow::Result<Vec<Value>> {
    Ok(vec![])
}

pub fn get_transfer_message_by_id(message_id: Uuid) -> anyhow::Result<Option<Value>> {
    Ok(None)
}

pub fn create_transfer_message(message: Value) -> anyhow::Result<()> {
    Ok(())
}