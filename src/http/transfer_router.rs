use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post};
use jsonschema::output::BasicOutput;
use tracing::{debug, info};

use crate::http::err::HttpError;
use crate::transfer::compiled_schemas::*;
use crate::transfer::messages::*;

pub fn router() -> Router
{
    Router::new()
        .route("/transfer/request", post(handle_transfer_request))
        .route("/transfer/start", post(handle_transfer_start))
        .route("/transfer/suspension", post(handle_transfer_suspension))
        .route("/transfer/completion", post(handle_transfer_completion))
        .route("/transfer/termination", post(handle_transfer_termination))
}


async fn handle_transfer_request(Json(input): Json<TransferRequestMessage>) -> impl IntoResponse {
    info!("POST /transfer/request");

    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_REQUEST_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return HttpError::ValidationError { errors }.into_response();
    }

    // response builder
    (StatusCode::OK, Json(TransferProcess {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcess.to_string(),
        provider_pid: "123".to_string(),
        consumer_pid: "123".to_string(),
        state: TransferState::REQUESTED,
    })).into_response()
}

async fn handle_transfer_start(Json(input): Json<TransferStartMessage>) -> impl IntoResponse {
    info!("POST /transfer/start");

    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_START_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return HttpError::ValidationError { errors }.into_response();
    }

    // response builder
    (StatusCode::OK, Json(TransferProcess {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcess.to_string(),
        provider_pid: "123".to_string(),
        consumer_pid: "123".to_string(),
        state: TransferState::STARTED,
    })).into_response()
}

async fn handle_transfer_suspension(Json(input): Json<TransferSuspensionMessage>) -> impl IntoResponse {
    info!("POST /transfer/suspension");

    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_SUSPENSION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return HttpError::ValidationError { errors }.into_response();
    }

    // response builder
    (StatusCode::OK, Json(TransferProcess {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcess.to_string(),
        provider_pid: "123".to_string(),
        consumer_pid: "123".to_string(),
        state: TransferState::SUSPENDED,
    })).into_response()
}

async fn handle_transfer_completion(Json(input): Json<TransferCompletionMessage>) -> impl IntoResponse {
    info!("POST /transfer/completion");

    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_COMPLETION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return HttpError::ValidationError { errors }.into_response();
    }

    // response builder
    (StatusCode::OK, Json(TransferProcess {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcess.to_string(),
        provider_pid: "123".to_string(),
        consumer_pid: "123".to_string(),
        state: TransferState::COMPLETED,
    })).into_response()
}

async fn handle_transfer_termination(Json(input): Json<TransferTerminationMessage>) -> impl IntoResponse {
    info!("POST /transfer/termination");

    // schema validation
    let input_as_value = serde_json::value::to_value(&input).unwrap();
    let validation = TRANSFER_TERMINATION_SCHEMA.apply(&input_as_value).basic();
    if let BasicOutput::Invalid(errors) = validation {
        return HttpError::ValidationError { errors }.into_response();
    }

    // response builder
    (StatusCode::OK, Json(TransferProcess {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcess.to_string(),
        provider_pid: "123".to_string(),
        consumer_pid: "123".to_string(),
        state: TransferState::TERMINATED,
    })).into_response()
}

