use crate::transfer::common::err::TransferErrorType;
use crate::transfer::common::utils::{does_callback_exist, is_consumer_pid_valid};
use crate::transfer::consumer::lib::control_plane::transfer_completion;
use crate::transfer::consumer::lib::control_plane::{
    transfer_start, transfer_suspension, transfer_termination,
};
use crate::transfer::protocol::messages::{
    TransferCompletionMessage, TransferStartMessage, TransferSuspensionMessage,
    TransferTerminationMessage,
};
use anyhow::Error;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use log::info;
use tracing::error;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route(
            "/:callback/transfers/:consumer_pid/start",
            post(handle_transfer_start),
        )
        .route(
            "/:callback/transfers/:consumer_pid/completion",
            post(handle_transfer_completion),
        )
        .route(
            "/:callback/transfers/:consumer_pid/termination",
            post(handle_transfer_termination),
        )
        .route(
            "/:callback/transfers/:consumer_pid/suspension",
            post(handle_transfer_suspension),
        )
}

async fn handle_transfer_start(
    Path((callback, consumer_pid)): Path<(Uuid, Uuid)>,
    Json(input): Json<TransferStartMessage>,
) -> impl IntoResponse {
    info!(
        "/{}/transfers/{}/start",
        callback.to_string(),
        consumer_pid.to_string()
    );


    match transfer_start(Json(&input), callback, consumer_pid) {
        Ok(_) => (StatusCode::OK).into_response(),
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
    Path((callback, consumer_pid)): Path<(Uuid, Uuid)>,
    Json(input): Json<TransferCompletionMessage>,
) -> impl IntoResponse {
    info!(
        "/{}/transfers/{}/start",
        callback.to_string(),
        consumer_pid.to_string()
    );

    match transfer_completion(Json(&input), callback, consumer_pid) {
        Ok(_) => (StatusCode::OK).into_response(),
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
    Path((callback, consumer_pid)): Path<(Uuid, Uuid)>,
    Json(input): Json<TransferTerminationMessage>,
) -> impl IntoResponse {
    info!(
        "/{}/transfers/{}/start",
        callback.to_string(),
        consumer_pid.to_string()
    );

    match transfer_termination(Json(&input), callback, consumer_pid) {
        Ok(_) => (StatusCode::OK).into_response(),
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
    Path((callback, consumer_pid)): Path<(Uuid, Uuid)>,
    Json(input): Json<TransferSuspensionMessage>,
) -> impl IntoResponse {
    info!(
        "/{}/transfers/{}/start",
        callback.to_string(),
        consumer_pid.to_string()
    );

    match transfer_suspension(Json(&input), callback, consumer_pid) {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => {
                error!("Unexpected error: {:?}", e_);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
    }
}
