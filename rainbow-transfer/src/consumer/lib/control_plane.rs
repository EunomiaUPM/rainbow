use crate::common::err::TransferErrorType::CallbackClientError;
use crate::consumer::data::entities::transfer_callback;
use crate::protocol::messages::{
    TransferCompletionMessage, TransferStartMessage, TransferSuspensionMessage,
    TransferTerminationMessage,
};
use crate::setup::databases::get_db_connection;
use anyhow::bail;
use axum::Json;
use sea_orm::{ActiveValue, EntityTrait};
use uuid::Uuid;

pub async fn transfer_start(
    Json(input): Json<&TransferStartMessage>,
    callback: Uuid,
    consumer_pid: Uuid,
) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;

    // // Check if callback exists
    // if let Ok(callback_exists) = does_callback_exist(callback) {
    //     if callback_exists == false {
    //         bail!(CallbackClientError)
    //     }
    // }

    // Here i should persist something
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
        data_address: ActiveValue::Set(Some(serde_json::to_value(&input.data_address)?)),
    })
        .exec(db_connection)
        .await?;

    // let transaction = TRANSFER_CONSUMER_REPO.update_callback(
    //     callback,
    //     TransferCallbacksModelNewState {
    //         provider_pid: Some(Uuid::parse_str(&input.provider_pid)?),
    //         consumer_pid: Some(Uuid::parse_str(&input.consumer_pid)?),
    //         data_address: Some(serde_json::to_value(&input.data_address)?),
    //     },
    // )?;
    //
    Ok(())
    // match transaction {
    //     Some(_) => Ok(()),
    //     // TODO IMPROVE THIS ERROR
    //     None => Err(anyhow::Error::msg("no transaction returned")),
    // }
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
