use crate::db::get_db_connection;
use crate::transfer::protocol::messages::{TransferMessageTypes, TransferState};
use crate::transfer::provider::data::models::{
    TransferFieldModel, TransferMessageModel, TransferProcessModel,
};
use crate::transfer::provider::data::schema::transfer_message_fields::dsl::transfer_message_fields;
use crate::transfer::provider::data::schema::transfer_messages::dsl::{
    id as message_id, message_type, transfer_messages,
};
use crate::transfer::provider::data::schema::transfer_processes::dsl::transfer_processes;
use crate::transfer::provider::data::schema::transfer_processes::dsl::*;
use diesel::prelude::*;
use serde_json::Value;
use uuid::Uuid;
// TODO create all this with a trait...

pub fn get_all_transfer_processes(limit: Option<i64>) -> anyhow::Result<Vec<TransferProcessModel>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_processes
        .limit(limit.unwrap_or(20))
        .select(TransferProcessModel::as_select())
        .load(connection)?;

    Ok(transaction)
}

pub fn get_transfer_process_by_consumer_pid(
    consumer_pid_in: Uuid,
) -> anyhow::Result<Option<TransferProcessModel>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_processes
        .filter(consumer_pid.eq(consumer_pid_in))
        .select(TransferProcessModel::as_select())
        .first(connection)
        .optional()?;

    Ok(transaction)
}

pub fn get_transfer_process_by_provider_pid(
    provider_pid_in: Uuid,
) -> anyhow::Result<Option<TransferProcessModel>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_processes
        .filter(provider_pid.eq(provider_pid_in))
        .select(TransferProcessModel::as_select())
        .first(connection)
        .optional()?;

    Ok(transaction)
}

pub fn create_transfer_process(transfer_process: TransferProcessModel) -> anyhow::Result<()> {
    let connection = &mut get_db_connection().get()?;
    let _ = diesel::insert_into(transfer_processes)
        .values(&transfer_process)
        .execute(connection)?;

    Ok(())
}

pub fn update_transfer_process_by_provider_pid(
    provider_pid_in: &Uuid,
    new_state: TransferState,
) -> anyhow::Result<Option<TransferProcessModel>> {
    let connection = &mut get_db_connection().get()?;
    let find = transfer_processes.find(provider_pid_in);
    let values = (
        state.eq(new_state.to_string()),
        updated_at.eq(chrono::Utc::now().naive_utc()),
    );
    let transaction = diesel::update(find)
        .set(values)
        .returning(TransferProcessModel::as_returning())
        .get_result(connection)
        .optional()?;

    Ok(transaction)
}

pub fn get_all_transfer_messages(limit: Option<i64>) -> anyhow::Result<Vec<TransferMessageModel>> {
    // TODO create joins and return format ok
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_messages
        .limit(limit.unwrap_or(20))
        .select(TransferMessageModel::as_select())
        .load(connection)?;

    Ok(transaction)
}

pub fn get_all_transfer_messages_by_type(
    message_type_in: TransferMessageTypes,
    limit: Option<i64>,
) -> anyhow::Result<Vec<TransferMessageModel>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_messages
        .filter(message_type.eq(message_type_in.to_string()))
        .limit(limit.unwrap_or(20))
        .select(TransferMessageModel::as_select())
        .load(connection)?;

    Ok(transaction)
}

pub fn get_transfer_message_by_id(
    message_id_in: Uuid,
) -> anyhow::Result<Option<TransferMessageModel>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_messages
        .filter(message_id.eq(message_id_in))
        .select(TransferMessageModel::as_select())
        .first(connection)
        .optional()?;

    Ok(transaction)
}

pub fn create_transfer_message(message: TransferMessageModel) -> anyhow::Result<()> {
    let connection = &mut get_db_connection().get()?;
    let _ = diesel::insert_into(transfer_messages)
        .values(&message)
        .execute(connection)?;

    Ok(())
}

pub fn create_transfer_fields(input: &Value, id: Uuid) -> anyhow::Result<()> {
    let connection = &mut get_db_connection().get()?;

    // IM HERE!!! refactor this to work properly
    if let Value::Object(map) = input {
        for (key, value) in map.iter() {
            println!("{}: {}", key, value);
            let mut fields: Vec<TransferFieldModel> = vec![];
            let parent_id = Uuid::new_v4();

            if Value::is_object(value) {
                if let Value::Object(submap) = input {
                    for (subkey, subvalue) in submap.iter() {
                        fields.push(TransferFieldModel {
                            id: Uuid::new_v4(),
                            transfer_message_id: id,
                            key: subkey.to_string(),
                            value: subvalue.to_string(),
                            parent: Some(parent_id),
                        })
                    }
                }
            } else if Value::is_string(value) {
                fields.push(TransferFieldModel {
                    id: parent_id,
                    transfer_message_id: id,
                    key: key.to_string(),
                    value: value.to_string(),
                    parent: None,
                });
            }

            for field in fields.iter() {
                let _ = diesel::insert_into(transfer_message_fields)
                    .values(field)
                    .execute(connection)?;
            }
        }
    }

    Ok(())
}
