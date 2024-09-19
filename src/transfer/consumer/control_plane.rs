use crate::transfer::common::utils::{does_callback_exist, is_consumer_pid_valid};
use crate::transfer::protocol::messages::{TransferCompletionMessage, TransferStartMessage, TransferSuspensionMessage, TransferTerminationMessage};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/:callback/transfers/:consumer_pid/start", post(handle_transfer_start))
        .route("/:callback/transfers/:consumer_pid/completion", post(handle_transfer_completion))
        .route("/:callback/transfers/:consumer_pid/termination", post(handle_transfer_termination))
        .route("/:callback/transfers/:consumer_pid/suspension", post(handle_transfer_suspension))
}

async fn handle_transfer_start(Path((callback, consumer_pid)): Path<(Uuid, Uuid)>, Json(input): Json<TransferStartMessage>) -> impl IntoResponse {
    // Check if callback exists
    if let Ok(callback_exists) = does_callback_exist(callback) {
        if callback_exists == false {
            // TODO CallbackClientError
            (StatusCode::BAD_REQUEST).into_response();
        }
    }

    // check consumer pid exists
    if is_consumer_pid_valid(&consumer_pid.to_string()).unwrap() == false {
        // TODO ConsumerBadError
        (StatusCode::BAD_REQUEST).into_response();
    }

    // TODO define logic for parsing and validating input


    (StatusCode::OK).into_response()
}

async fn handle_transfer_completion(Path((callback, consumer_pid)): Path<(Uuid, Uuid)>, Json(input): Json<TransferCompletionMessage>) -> impl IntoResponse {
    (StatusCode::OK).into_response()
}

async fn handle_transfer_termination(Path((callback, consumer_pid)): Path<(Uuid, Uuid)>, Json(input): Json<TransferTerminationMessage>) -> impl IntoResponse {
    (StatusCode::OK).into_response()
}

async fn handle_transfer_suspension(Path((callback, consumer_pid)): Path<(Uuid, Uuid)>, Json(input): Json<TransferSuspensionMessage>) -> impl IntoResponse {
    (StatusCode::OK).into_response()
}