use crate::transfer::common::utils::{convert_uri_to_uuid, convert_uuid_to_uri};
use crate::transfer::protocol::messages::{
    TransferCompletionMessage, TransferMessageTypes, TransferProcessMessage,
    TransferRequestMessage, TransferStartMessage, TransferState, TransferSuspensionMessage,
    TransferTerminationMessage, TRANSFER_CONTEXT,
};
use crate::transfer::provider::data::models::{TransferMessageModel, TransferProcessModel};
use crate::transfer::provider::data::repo::{TransferProviderDataRepo, TRANSFER_PROVIDER_REPO};
use crate::transfer::provider::data::repo_postgres::TransferProviderDataRepoPostgres;
use crate::transfer::provider::err::TransferErrorType;
use crate::transfer::provider::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::transfer::provider::lib::control_plane::{get_transfer_requests_by_provider, transfer_completion, transfer_request, transfer_start, transfer_suspension, transfer_termination};
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

    match get_transfer_requests_by_provider(provider_pid).await.unwrap() {
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

    match transfer_request(input, send_transfer_start).await {
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

    let response = DATA_PLANE_HTTP_CLIENT
        .clone()
        .post(format!(
            "{}/transfers/{}/start",
            input.callback_address,
            convert_uri_to_uuid(&input.consumer_pid.to_string())?
        ))
        .header("content-type", "application/json")
        .json(&transfer_start_message)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status() == StatusCode::OK {
                let transfer_process = TRANSFER_PROVIDER_REPO
                    .update_transfer_process_by_provider_pid(&provider_pid, TransferState::STARTED, None)?
                    .unwrap();
                let created_at = chrono::Utc::now().naive_utc();
                let message_id = Uuid::new_v4();
                TRANSFER_PROVIDER_REPO.create_transfer_message(TransferMessageModel {
                    id: message_id,
                    transfer_process_id: transfer_process.provider_pid,
                    created_at,
                    message_type: TransferMessageTypes::TransferStartMessage.to_string(),
                    from: "provider".to_string(),
                    to: "consumer".to_string(),
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

    match transfer_start(&input).await {
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

    match transfer_suspension(&input).await {
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

async fn handle_transfer_completion(
    Json(input): Json<TransferCompletionMessage>,
) -> impl IntoResponse {
    info!("POST /transfers/completion");

    match transfer_completion(&input).await {
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

    match transfer_termination(&input) {
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
