use crate::db::get_db_connection;
use crate::transfer::protocol::messages::{TransferMessageTypes, TransferState};
use crate::transfer::provider::data::models::{TransferMessage, TransferProcess};
use crate::transfer::provider::data::schema::transfer_messages::dsl::{
    id as message_id, message_type, transfer_messages,
};
use crate::transfer::provider::data::schema::transfer_processes::dsl::transfer_processes;
use crate::transfer::provider::data::schema::transfer_processes::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

// TODO create all this with a trait...

pub fn get_all_transfer_processes(limit: Option<i64>) -> anyhow::Result<Vec<TransferProcess>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_processes
        .limit(limit.unwrap_or(20))
        .select(TransferProcess::as_select())
        .load(connection)?;

    Ok(transaction)
}

pub fn get_transfer_process_by_consumer_pid(
    consumer_pid_in: Uuid,
) -> anyhow::Result<Option<TransferProcess>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_processes
        .filter(consumer_pid.eq(consumer_pid_in))
        .select(TransferProcess::as_select())
        .first(connection)
        .optional()?;

    Ok(transaction)
}

pub fn get_transfer_process_by_provider_pid(
    provider_pid_in: Uuid,
) -> anyhow::Result<Option<TransferProcess>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_processes
        .filter(provider_pid.eq(provider_pid_in))
        .select(TransferProcess::as_select())
        .first(connection)
        .optional()?;

    Ok(transaction)
}

pub fn create_transfer_process(transfer_process: TransferProcess) -> anyhow::Result<()> {
    let connection = &mut get_db_connection().get()?;
    let _ = diesel::insert_into(transfer_processes)
        .values(&transfer_process)
        .execute(connection)?;

    Ok(())
}

pub fn update_transfer_process_by_provider_pid(
    provider_pid_in: &Uuid,
    new_state: TransferState,
) -> anyhow::Result<Option<TransferProcess>> {
    let connection = &mut get_db_connection().get()?;
    let find = transfer_processes.find(provider_pid_in);
    let values = (
        state.eq(new_state.to_string()),
        updated_at.eq(chrono::Utc::now().naive_utc()),
    );
    let transaction = diesel::update(find)
        .set(values)
        .returning(TransferProcess::as_returning())
        .get_result(connection)
        .optional()?;

    Ok(transaction)
}

pub fn get_all_transfer_messages(limit: Option<i64>) -> anyhow::Result<Vec<TransferMessage>> {
    // TODO create joins and return format ok
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_messages
        .limit(limit.unwrap_or(20))
        .select(TransferMessage::as_select())
        .load(connection)?;

    Ok(transaction)
}

pub fn get_all_transfer_messages_by_type(
    message_type_in: TransferMessageTypes,
    limit: Option<i64>,
) -> anyhow::Result<Vec<TransferMessage>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_messages
        .filter(message_type.eq(message_type_in.to_string()))
        .limit(limit.unwrap_or(20))
        .select(TransferMessage::as_select())
        .load(connection)?;

    Ok(transaction)
}

pub fn get_transfer_message_by_id(message_id_in: Uuid) -> anyhow::Result<Option<TransferMessage>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_messages
        .filter(message_id.eq(message_id_in))
        .select(TransferMessage::as_select())
        .first(connection)
        .optional()?;

    Ok(transaction)
}

pub fn create_transfer_message(message: TransferMessage) -> anyhow::Result<()> {
    let connection = &mut get_db_connection().get()?;
    let _ = diesel::insert_into(transfer_messages)
        .values(&message)
        .execute(connection)?;

    Ok(())
}
