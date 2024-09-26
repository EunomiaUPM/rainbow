use crate::transfer::common::utils::convert_uuid_to_uri;
use crate::transfer::common::utils::{
    has_data_address_in_push, is_agreement_valid, is_consumer_pid_valid, is_provider_valid,
};
use crate::transfer::protocol::messages::TransferMessageTypes;
use crate::transfer::protocol::messages::{
    TransferCompletionMessage, TransferProcessMessage, TransferRequestMessage,
    TransferStartMessage, TransferState, TransferSuspensionMessage, TransferTerminationMessage,
    TRANSFER_CONTEXT,
};
use crate::transfer::provider::data::models::{TransferMessageModel, TransferProcessModel};
use crate::transfer::provider::data::repo::{
    create_transfer_fields, create_transfer_message, create_transfer_process,
    get_transfer_process_by_provider_pid, update_transfer_process_by_provider_pid,
};
use crate::transfer::provider::err::TransferErrorType;
use crate::transfer::schemas::{
    TRANSFER_COMPLETION_SCHEMA, TRANSFER_REQUEST_SCHEMA, TRANSFER_START_SCHEMA,
    TRANSFER_SUSPENSION_SCHEMA, TRANSFER_TERMINATION_SCHEMA,
};
use anyhow::Error;
use axum::extract::Path;
use axum::Json;
use jsonschema::output::BasicOutput;
use serde_json::Value;
use tracing::{debug, error};
use uuid::Uuid;

pub fn get_transfer_requests_by_provider(
    Path(provider_pid): Path<Uuid>,
) -> anyhow::Result<Option<TransferProcessModel>> {
    // access info
    let transaction = get_transfer_process_by_provider_pid(provider_pid)?;
    Ok(transaction)
}

pub fn transfer_request(
    Json(input): Json<&TransferRequestMessage>,
) -> anyhow::Result<TransferProcessMessage> {
    // schema validation
    let input_as_value = serde_json::value::to_value(&input)?;
    let validation = TRANSFER_REQUEST_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return Err(Error::from(TransferErrorType::ValidationError { errors }));
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid)? == false {
        return Err(Error::from(TransferErrorType::ConsumerIdUuidError));
    }

    // agreement validation - validate
    if is_agreement_valid(&input.agreement_id)? == false {
        return Err(Error::from(TransferErrorType::AgreementError));
    }

    // dct:format is push, dataAdress must be
    if has_data_address_in_push(&input.data_address, &input.format)? == false {
        return Err(Error::from(
            TransferErrorType::DataAddressCannotBeNullOnPushError,
        ));
    }

    // persist information
    let provider_pid = Uuid::new_v4();
    let created_at = chrono::Utc::now().naive_utc();
    let message_type = input._type.clone();

    create_transfer_process(TransferProcessModel {
        provider_pid,
        consumer_pid: input.consumer_pid.parse()?,
        state: TransferState::REQUESTED.to_string(),
        created_at,
        updated_at: None,
    })?;

    let message_id = Uuid::new_v4();
    create_transfer_message(TransferMessageModel {
        id: message_id,
        transfer_process_id: provider_pid,
        created_at,
        message_type,
    })?;
    create_transfer_fields(&serde_json::to_value(&input)?, message_id)?;

    // provide data_plane
    // TODO manage data plane

    // send back TransferProcessMessage
    let tp = TransferProcessMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcessMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&provider_pid)?,
        consumer_pid: input.consumer_pid.clone(),
        state: TransferState::REQUESTED,
    };

    Ok(tp)
}

pub fn transfer_start(Json(input): Json<&TransferStartMessage>) -> anyhow::Result<()> {
    // schema validation
    let input_as_value = serde_json::value::to_value(&input)?;
    let validation = TRANSFER_START_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return Err(Error::from(TransferErrorType::ValidationError { errors }));
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid)? == false {
        return Err(Error::from(TransferErrorType::ConsumerIdUuidError));
    }

    // has provider - validate - TODO check in database
    if is_provider_valid(&input.provider_pid)? == false {
        return Err(Error::from(TransferErrorType::ProviderIdUuidError));
    }

    // persist information
    let transaction = update_transfer_process_by_provider_pid(
        &input.provider_pid.parse()?,
        TransferState::STARTED,
    )?;
    if let Some(_) = transaction {
        create_transfer_message(TransferMessageModel {
            id: Uuid::new_v4(),
            transfer_process_id: input.provider_pid.parse()?,
            created_at: chrono::Utc::now().naive_utc(),
            message_type: input._type.clone(),
        })?;
    } else {
        // TODO send back error or in guard up
        // TODO improve erroring...
        error!("Not provider");
        return Err(Error::from(TransferErrorType::ProviderIdUuidError));
    }

    Ok(())
}

pub fn transfer_suspension(Json(input): Json<&TransferSuspensionMessage>) -> anyhow::Result<()> {
    // schema validation
    let input_as_value = serde_json::value::to_value(&input)?;
    let validation = TRANSFER_SUSPENSION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return Err(Error::from(TransferErrorType::ValidationError { errors }));
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid)? == false {
        return Err(Error::from(TransferErrorType::ConsumerIdUuidError));
    }

    // has provider - validate - TODO check in database
    if is_provider_valid(&input.provider_pid)? == false {
        return Err(Error::from(TransferErrorType::ProviderIdUuidError));
    }

    let transaction = update_transfer_process_by_provider_pid(
        &input.provider_pid.parse()?,
        TransferState::SUSPENDED,
    )?;
    if let Some(_) = transaction {
        create_transfer_message(TransferMessageModel {
            id: Uuid::new_v4(),
            transfer_process_id: input.provider_pid.parse()?,
            created_at: chrono::Utc::now().naive_utc(),
            message_type: input._type.clone(),
        })?;
    } else {
        // TODO send back error or in guard up
        error!("Not provider");
        return Err(Error::from(TransferErrorType::ProviderIdUuidError));
    }

    Ok(())
}

pub fn transfer_completion(Json(input): Json<&TransferCompletionMessage>) -> anyhow::Result<()> {
    // schema validation
    let input_as_value = serde_json::value::to_value(&input)?;
    let validation = TRANSFER_COMPLETION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return Err(Error::from(TransferErrorType::ValidationError { errors }));
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid)? == false {
        return Err(Error::from(TransferErrorType::ConsumerIdUuidError));
    }

    // has provider - validate - TODO check in database
    if is_provider_valid(&input.provider_pid)? == false {
        return Err(Error::from(TransferErrorType::ProviderIdUuidError));
    }

    let transaction = update_transfer_process_by_provider_pid(
        &input.provider_pid.parse()?,
        TransferState::COMPLETED,
    )?;
    if let Some(_) = transaction {
        create_transfer_message(TransferMessageModel {
            id: Uuid::new_v4(),
            transfer_process_id: input.provider_pid.parse()?,
            created_at: chrono::Utc::now().naive_utc(),
            message_type: input._type.clone(),
        })?;
    } else {
        // TODO send back error
        error!("Not provider");
        return Err(Error::from(TransferErrorType::ProviderIdUuidError));
    }

    Ok(())
}

pub fn transfer_termination(Json(input): Json<&TransferTerminationMessage>) -> anyhow::Result<()> {
    // schema validation
    let input_as_value = serde_json::value::to_value(&input)?;
    let validation = TRANSFER_TERMINATION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return Err(Error::from(TransferErrorType::ValidationError { errors }));
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid)? == false {
        return Err(Error::from(TransferErrorType::ConsumerIdUuidError));
    }

    // has provider - validate - TODO check in database
    if is_provider_valid(&input.provider_pid)? == false {
        return Err(Error::from(TransferErrorType::ProviderIdUuidError));
    }

    let transaction = update_transfer_process_by_provider_pid(
        &input.provider_pid.parse()?,
        TransferState::TERMINATED,
    )?;
    if let Some(_) = transaction {
        create_transfer_message(TransferMessageModel {
            id: Uuid::new_v4(),
            transfer_process_id: input.provider_pid.parse()?,
            created_at: chrono::Utc::now().naive_utc(),
            message_type: input._type.clone(),
        })?;
    } else {
        // TODO send back error or in guard up
        error!("Not provider");
        return Err(Error::from(TransferErrorType::ProviderIdUuidError));
    }

    Ok(())
}
