use crate::transfer::common::err::TransferErrorType;
use crate::transfer::common::utils::{does_callback_exist, is_consumer_pid_valid};
use crate::transfer::consumer::data::models::TransferCallbacksModelNewState;
use crate::transfer::consumer::data::repo::{TransferConsumerDataRepo, TRANSFER_CONSUMER_REPO};
use crate::transfer::protocol::messages::{
    TransferCompletionMessage, TransferStartMessage, TransferSuspensionMessage,
    TransferTerminationMessage,
};
use crate::transfer::schemas::{
    TRANSFER_COMPLETION_SCHEMA, TRANSFER_START_SCHEMA, TRANSFER_SUSPENSION_SCHEMA,
    TRANSFER_TERMINATION_SCHEMA,
};
use anyhow::Error;
use axum::Json;
use jsonschema::output::BasicOutput;
use uuid::Uuid;

pub fn transfer_start(
    Json(input): Json<&TransferStartMessage>,
    callback: Uuid,
    consumer_pid: Uuid,
) -> anyhow::Result<()> {


    // schema validation
    let input_as_value = serde_json::value::to_value(&input)?;
    let validation = TRANSFER_START_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return Err(Error::from(TransferErrorType::ValidationError { errors }));
    }

    // Check if callback exists
    if let Ok(callback_exists) = does_callback_exist(callback) {
        if callback_exists == false {
            // TODO CallbackClientError
            return Err(Error::from(TransferErrorType::CallbackClientError));
        }
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid)? == false {
        // TODO ConsumerBadError
        // TODO consumer pid both must be taken...
        return Err(Error::from(TransferErrorType::PidUuidError));
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
    // schema validation
    let input_as_value = serde_json::value::to_value(&input)?;
    let validation = TRANSFER_COMPLETION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return Err(Error::from(TransferErrorType::ValidationError { errors }));
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid)? == false {
        return Err(Error::from(TransferErrorType::PidUuidError));
    }

    Ok(())
}

pub fn transfer_termination(
    Json(input): Json<&TransferTerminationMessage>,
    callback: Uuid,
    consumer_pid: Uuid,
) -> anyhow::Result<()> {
    // schema validation
    let input_as_value = serde_json::value::to_value(&input)?;
    let validation = TRANSFER_TERMINATION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return Err(Error::from(TransferErrorType::ValidationError { errors }));
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid)? == false {
        return Err(Error::from(TransferErrorType::PidUuidError));
    }

    Ok(())
}

pub fn transfer_suspension(
    Json(input): Json<&TransferSuspensionMessage>,
    callback: Uuid,
    consumer_pid: Uuid,
) -> anyhow::Result<()> {
    // schema validation
    let input_as_value = serde_json::value::to_value(&input)?;
    let validation = TRANSFER_SUSPENSION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return Err(Error::from(TransferErrorType::ValidationError { errors }));
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid)? == false {
        return Err(Error::from(TransferErrorType::PidUuidError));
    }

    Ok(())
}
