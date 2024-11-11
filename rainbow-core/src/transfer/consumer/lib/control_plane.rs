use crate::transfer::common::err::TransferErrorType;
use crate::transfer::common::utils::does_callback_exist;
use crate::transfer::consumer::data::models::TransferCallbacksModelNewState;
use crate::transfer::consumer::data::repo::{TransferConsumerDataRepo, TRANSFER_CONSUMER_REPO};
use crate::transfer::protocol::messages::{
    TransferCompletionMessage, TransferStartMessage, TransferSuspensionMessage,
    TransferTerminationMessage,
};
use anyhow::Error;
use axum::Json;
use uuid::Uuid;

pub fn transfer_start(
    Json(input): Json<&TransferStartMessage>,
    callback: Uuid,
    consumer_pid: Uuid,
) -> anyhow::Result<()> {

    // Check if callback exists
    if let Ok(callback_exists) = does_callback_exist(callback) {
        if callback_exists == false {
            // TODO CallbackClientError
            return Err(Error::from(TransferErrorType::CallbackClientError));
        }
    }

    // Here i should persist something
    let transaction = TRANSFER_CONSUMER_REPO.update_callback(
        callback,
        TransferCallbacksModelNewState {
            provider_pid: Some(Uuid::parse_str(&input.provider_pid)?),
            consumer_pid: Some(Uuid::parse_str(&input.consumer_pid)?),
            data_address: Some(serde_json::to_value(&input.data_address)?),
        },
    )?;
    match transaction {
        Some(_) => Ok(()),
        // IMPROVE THIS ERROR
        None => Err(anyhow::Error::msg("no transaction returned")),
    }
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
