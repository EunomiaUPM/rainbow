use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use diesel::prelude::*;
use diesel::SelectableHelper;
use jsonschema::output::BasicOutput;
use reqwest::Client;
use serde_json::Value;
use tracing::{debug, info};
use uuid::{uuid, Uuid};

use crate::db::get_db_connection;
use crate::db::models::{CreateTransferSession, TransferSession};
use crate::transfer::common::utils::{has_data_address_in_push, is_agreement_valid, is_consumer_pid_valid, is_provider_valid};
use crate::transfer::persistence::sql_persistence::SQLPersistence;
use crate::transfer::persistence::Persistence;
use crate::transfer::protocol::messages::*;
use crate::transfer::provider::err::TransferErrorType;
use crate::transfer::schemas::*;

pub fn router() -> Router {
    Router::new()
        .route("/transfers/request", post(handle_transfer_request))
        // TODO implement "GET /transfers/:providerPid"
        .route("/transfers/start", post(handle_transfer_start))
        .route("/transfers/suspension", post(handle_transfer_suspension))
        .route("/transfers/completion", post(handle_transfer_completion))
        .route("/transfers/termination", post(handle_transfer_termination))
}

async fn handle_transfer_request(Json(input): Json<TransferRequestMessage>) -> impl IntoResponse {
    info!("POST /transfer/request");

    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_REQUEST_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return TransferErrorType::ValidationError { errors }.into_response();
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid).unwrap() == false {
        return TransferErrorType::ConsumerIdUuidError.into_response();
    }

    // agreement validation - validate
    if is_agreement_valid(&input.agreement_id).unwrap() == false {
        return TransferErrorType::AgreementError.into_response();
    }

    // dct:format is push, dataAdress must be
    if has_data_address_in_push(&input.data_address, &input.format).unwrap() == false {
        return TransferErrorType::DataAddressCannotBeNullOnPushError.into_response();
    }

    // TODO refactor for proper forwarding...
    // forwarding data
    if let Some(data_address) = input.data_address {
        let endpoint = data_address.endpoint;
        debug!(endpoint);
        let res = Client::new().get(endpoint).send().await;

        if let Err(err) = res {
            debug!("{}", err.to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
        let data = res.unwrap().bytes().await.unwrap();
        debug!("{:?}", data);
    }


    let id_uuid = Uuid::new_v4();
    let provider_uuid = Uuid::new_v4();
    let provider_pid = format!("urn:uuid:{}", provider_uuid.to_string());

    // persist
    let new_transaction = CreateTransferSession {
        id: id_uuid,
        provider_pid: provider_pid.parse().unwrap(),
        consumer_pid: input.consumer_pid.parse().unwrap(),
        state: TransferState::REQUESTED.to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };
    let db_transaction = SQLPersistence::persist_transfer_request(new_transaction);
    debug!("{:?}", db_transaction);

    // response builder
    (
        StatusCode::OK,
        Json(TransferProcess {
            context: TRANSFER_CONTEXT.to_string(),
            _type: TransferMessageTypes::TransferProcess.to_string(),
            provider_pid,
            consumer_pid: input.consumer_pid,
            state: TransferState::REQUESTED,
        }),
    )
        .into_response()
}

async fn handle_transfer_start(Json(input): Json<TransferStartMessage>) -> impl IntoResponse {
    info!("POST /transfer/start");

    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_START_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return TransferErrorType::ValidationError { errors }.into_response();
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid).unwrap() == false {
        return TransferErrorType::ConsumerIdUuidError.into_response();
    }

    // has consumerId - validate - TODO check in database
    if is_provider_valid(&input.consumer_pid).unwrap() == false {
        return TransferErrorType::ProviderIdUuidError.into_response();
    }

    // persist

    let a = SQLPersistence::persist_transfer_start();
    debug!("{:?}", a);

    // response builder
    (
        StatusCode::OK,
        Json(TransferProcess {
            context: TRANSFER_CONTEXT.to_string(),
            _type: TransferMessageTypes::TransferProcess.to_string(),
            provider_pid: "123".to_string(),
            consumer_pid: "123".to_string(),
            state: TransferState::STARTED,
        }),
    )
        .into_response()
}

async fn handle_transfer_suspension(
    Json(input): Json<TransferSuspensionMessage>,
) -> impl IntoResponse {
    info!("POST /transfer/suspension");

    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_SUSPENSION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return TransferErrorType::ValidationError { errors }.into_response();
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid).unwrap() == false {
        return TransferErrorType::ConsumerIdUuidError.into_response();
    }

    // has consumerId - validate - TODO check in database
    if is_provider_valid(&input.consumer_pid).unwrap() == false {
        return TransferErrorType::ProviderIdUuidError.into_response();
    }

    // persist
    let a = SQLPersistence::persist_transfer_suspension();
    debug!("{:?}", a);

    // response builder
    (
        StatusCode::OK,
        Json(TransferProcess {
            context: TRANSFER_CONTEXT.to_string(),
            _type: TransferMessageTypes::TransferProcess.to_string(),
            provider_pid: "123".to_string(),
            consumer_pid: "123".to_string(),
            state: TransferState::SUSPENDED,
        }),
    )
        .into_response()
}

async fn handle_transfer_completion(
    Json(input): Json<TransferCompletionMessage>,
) -> impl IntoResponse {
    info!("POST /transfer/completion");

    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_COMPLETION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return TransferErrorType::ValidationError { errors }.into_response();
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid).unwrap() == false {
        return TransferErrorType::ConsumerIdUuidError.into_response();
    }

    // has consumerId - validate - TODO check in database
    if is_provider_valid(&input.consumer_pid).unwrap() == false {
        return TransferErrorType::ProviderIdUuidError.into_response();
    }

    // persist
    let a = SQLPersistence::persist_transfer_completion();
    debug!("{:?}", a);

    // response builder
    (
        StatusCode::OK,
        Json(TransferProcess {
            context: TRANSFER_CONTEXT.to_string(),
            _type: TransferMessageTypes::TransferProcess.to_string(),
            provider_pid: "123".to_string(),
            consumer_pid: "123".to_string(),
            state: TransferState::COMPLETED,
        }),
    )
        .into_response()
}

async fn handle_transfer_termination(
    Json(input): Json<TransferTerminationMessage>,
) -> impl IntoResponse {
    info!("POST /transfer/termination");

    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_TERMINATION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return TransferErrorType::ValidationError { errors }.into_response();
    }

    // has consumerId - validate
    if is_consumer_pid_valid(&input.consumer_pid).unwrap() == false {
        return TransferErrorType::ConsumerIdUuidError.into_response();
    }

    // has consumerId - validate - TODO check in database
    if is_provider_valid(&input.consumer_pid).unwrap() == false {
        return TransferErrorType::ProviderIdUuidError.into_response();
    }

    // persist
    let a = SQLPersistence::persist_transfer_termination();
    debug!("{:?}", a);

    // response builder
    (
        StatusCode::OK,
        Json(TransferProcess {
            context: TRANSFER_CONTEXT.to_string(),
            _type: TransferMessageTypes::TransferProcess.to_string(),
            provider_pid: "123".to_string(),
            consumer_pid: "123".to_string(),
            state: TransferState::TERMINATED,
        }),
    )
        .into_response()
}
