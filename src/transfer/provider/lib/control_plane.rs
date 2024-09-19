use crate::transfer::common::utils::{has_data_address_in_push, is_agreement_valid, is_consumer_pid_valid};
use crate::transfer::protocol::messages::{TransferCompletionMessage, TransferRequestMessage, TransferStartMessage, TransferSuspensionMessage, TransferTerminationMessage};
use crate::transfer::provider::err::TransferErrorType;
use crate::transfer::schemas::TRANSFER_REQUEST_SCHEMA;
use anyhow::Error;
use axum::Json;
use jsonschema::output::BasicOutput;

pub fn transfer_request(Json(input): Json<TransferRequestMessage>) -> anyhow::Result<TransferRequestMessage> {
    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_REQUEST_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return Err(Error::from(TransferErrorType::ValidationError { errors }));
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid).unwrap() == false {
        return Err(Error::from(TransferErrorType::ConsumerIdUuidError));
    }

    // agreement validation - validate
    if is_agreement_valid(&input.agreement_id).unwrap() == false {
        return Err(Error::from(TransferErrorType::AgreementError));
    }

    // dct:format is push, dataAdress must be
    if has_data_address_in_push(&input.data_address, &input.format).unwrap() == false {
        return Err(Error::from(TransferErrorType::DataAddressCannotBeNullOnPushError));
    }

    // persist information

    // provide data_plane

    Ok(input)
}

pub fn transfer_start(Json(input): Json<TransferStartMessage>) -> anyhow::Result<()> {
    Ok(())
}

pub fn transfer_suspension(Json(input): Json<TransferSuspensionMessage>) -> anyhow::Result<()> {
    Ok(())
}

pub fn transfer_completion(Json(input): Json<TransferCompletionMessage>) -> anyhow::Result<()> {
    Ok(())
}

pub fn transfer_termination(Json(input): Json<TransferTerminationMessage>) -> anyhow::Result<()> {
    Ok(())
}