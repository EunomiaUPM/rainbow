use crate::transfer::common::utils::convert_uuid_to_uri;
use crate::transfer::protocol::messages::{
    TransferCompletionMessage, TransferMessageTypes, TransferProcessMessage,
    TransferRequestMessage, TransferStartMessage, TransferState, TransferSuspensionMessage,
    TransferTerminationMessage, TRANSFER_CONTEXT,
};
use crate::transfer::provider::data::models::{TransferMessageModel, TransferProcessModel};
use crate::transfer::provider::data::repo::{
    create_transfer_message, create_transfer_process, get_transfer_process_by_provider_pid,
    update_transfer_process_by_provider_pid,
};
use crate::transfer::provider::err::TransferErrorType;
use crate::transfer::provider::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::transfer::provider::lib::control_plane::{
    transfer_completion, transfer_request, transfer_start, transfer_suspension,
    transfer_termination,
};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use reqwest::{Error, Response};
use tracing::{debug, error, info};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route(
            "/transfers/:provider_pid",
            get(handle_get_transfer_by_provider),
        )
        .route("/transfers/request", post(handle_transfer_request))
        .route("/transfers/start", post(handle_transfer_start))
        .route("/transfers/suspension", post(handle_transfer_suspension))
        .route("/transfers/completion", post(handle_transfer_completion))
        .route("/transfers/termination", post(handle_transfer_termination))
}

async fn handle_get_transfer_by_provider(Path(provider_pid): Path<Uuid>) -> impl IntoResponse {
    info!("GET /transfers/{}", provider_pid.to_string());

    // TODO REFACTOR IN CONTROL PLANE
    let transfer = get_transfer_process_by_provider_pid(provider_pid).unwrap();
    match transfer {
        Some(transfer_process) => (
            StatusCode::OK,
            Json(TransferProcessMessage {
                context: TRANSFER_CONTEXT.to_string(),
                _type: TransferMessageTypes::TransferProcessMessage.to_string(),
                provider_pid: transfer_process.provider_pid.to_string(),
                consumer_pid: transfer_process.consumer_pid.to_string(),
                state: TransferState::try_from(transfer_process.state).unwrap(),
            }),
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

async fn handle_transfer_request(Json(input): Json<TransferRequestMessage>) -> impl IntoResponse {
    info!("POST /transfers/request");

    match transfer_request(Json(input), send_transfer_start).await {
        Ok(tp) => (StatusCode::CREATED, Json(tp)).into_response(),
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => {
                error!("Unexpected error: {:?}", e_);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
    }
}

async fn send_transfer_start(
    Json(input): Json<TransferRequestMessage>,
    provider_pid: Uuid,
) -> anyhow::Result<()> {
    // TODO REFACTOR IN CONTROL PLANE
    let transfer_start_message = TransferStartMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferStartMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&provider_pid)?,
        consumer_pid: input.consumer_pid.to_string(),
        data_address: input.data_address,
    };

    println!("{}", serde_json::to_string_pretty(&transfer_start_message)?);

    let response = DATA_PLANE_HTTP_CLIENT
        .clone()
        .post(format!(
            "{}/transfers/{}/start",
            input.callback_address,
            input.consumer_pid.to_string()
        ))
        .header("content-type", "application/json")
        .json(&transfer_start_message)
        .send()
        .await;

    match response {
        Ok(res) => {
            println!("{:?}", res.status());
            if res.status() == StatusCode::OK {
                let transfer_process =
                    update_transfer_process_by_provider_pid(&provider_pid, TransferState::STARTED)?
                        .unwrap();
                let created_at = chrono::Utc::now().naive_utc();
                let message_id = Uuid::new_v4();
                create_transfer_message(TransferMessageModel {
                    id: message_id,
                    transfer_process_id: transfer_process.provider_pid,
                    created_at,
                    message_type: TransferMessageTypes::TransferStartMessage.to_string(),
                    content: serde_json::to_value(&transfer_start_message)?,
                })?;
            } else {
                println!("not started...."); // TODO Error
            }
        }
        Err(_) => {
            println!("boom");
        }
    }
    Ok(())
}

async fn handle_transfer_start(Json(input): Json<TransferStartMessage>) -> impl IntoResponse {
    info!("POST /transfers/start");

    match transfer_start(Json(&input)) {
        Ok(_) => (StatusCode::OK, Json(input)).into_response(),
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => {
                error!("Unexpected error: {:?}", e_);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
    }
}

async fn handle_transfer_suspension(
    Json(input): Json<TransferSuspensionMessage>,
) -> impl IntoResponse {
    info!("POST /transfers/suspension");

    match transfer_suspension(Json(input.clone()), send_transfer_suspension) {
        Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => {
                error!("Unexpected error: {:?}", e_);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
    }
}

async fn send_transfer_suspension(
    Json(input): Json<TransferSuspensionMessage>,
    provider_pid: Uuid,
) -> anyhow::Result<()> {
    println!("Transfer {} suspended", input.provider_pid);
    Ok(())
}

async fn handle_transfer_completion(
    Json(input): Json<TransferCompletionMessage>,
) -> impl IntoResponse {
    info!("POST /transfers/completion");

    match transfer_completion(Json(&input)) {
        Ok(_) => (StatusCode::OK, Json(input)).into_response(),
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => {
                error!("Unexpected error: {:?}", e_);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
    }
}

async fn handle_transfer_termination(
    Json(input): Json<TransferTerminationMessage>,
) -> impl IntoResponse {
    info!("POST /transfers/termination");

    match transfer_termination(Json(&input)) {
        Ok(_) => (StatusCode::OK, Json(input)).into_response(),
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => {
                error!("Unexpected error: {:?}", e_);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
    }
}
