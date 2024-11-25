use crate::consumer::data::entities::transfer_callback;
use anyhow::bail;
use axum::Json;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::err::transfer_err::TransferErrorType::CallbackClientError;
use rainbow_common::protocol::transfer::{
    TransferCompletionMessage, TransferStartMessage, TransferSuspensionMessage,
    TransferTerminationMessage,
};
use sea_orm::{ActiveValue, EntityTrait};
use uuid::Uuid;

pub async fn transfer_start(
    Json(input): Json<&TransferStartMessage>,
    callback: Uuid,
    consumer_pid: Uuid,
) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    let callback = transfer_callback::Entity::find_by_id(callback).one(db_connection).await?;
    if callback.is_none() {
        bail!(CallbackClientError)
    }
    let callback = callback.unwrap();

    let transaction = transfer_callback::Entity::update(transfer_callback::ActiveModel {
        id: ActiveValue::Set(callback.id),
        consumer_pid: ActiveValue::Set(callback.consumer_pid),
        provider_pid: ActiveValue::Set(Some(Uuid::parse_str(input.provider_pid.as_str())?)),
        created_at: ActiveValue::Set(callback.created_at),
        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
        data_plane_id: ActiveValue::Set(None),
    })
        .exec(db_connection)
        .await?;

    Ok(())
}

pub fn transfer_completion(
    Json(input): Json<&TransferCompletionMessage>,
    callback: Uuid,
    consumer_pid: Uuid,
) -> anyhow::Result<()> {
    Ok(())
}

pub fn transfer_termination(
    Json(input): Json<&TransferTerminationMessage>,
    callback: Uuid,
    consumer_pid: Uuid,
) -> anyhow::Result<()> {
    Ok(())
}

pub fn transfer_suspension(
    Json(input): Json<&TransferSuspensionMessage>,
    callback: Uuid,
    consumer_pid: Uuid,
) -> anyhow::Result<()> {
    Ok(())
}
