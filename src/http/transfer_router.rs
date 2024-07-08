use axum::response::{IntoResponse, Response};
use axum::{Router};
use axum::body::{Body, Bytes};
use axum::extract::{Json, Request};
use axum::routing::{get, post};
use garde::{Report, Validate};
use axum::http::StatusCode;
use tracing::{debug, info};
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

    // validation

    // send ACK o ERROR

    // try to send errors with ERROR Response...
    debug!("{:#?}", &input);
    return "ok"
}

async fn handle_transfer_start(Json(input): Json<TransferStartMessage>) -> impl IntoResponse {
    info!("POST /transfer/start")
}

async fn handle_transfer_suspension(Json(input): Json<TransferSuspensionMessage>) -> impl IntoResponse {
    info!("POST /transfer/suspension")
}

async fn handle_transfer_completion(Json(input): Json<TransferCompletionMessage>) -> impl IntoResponse {
    info!("POST /transfer/completion")
}

async fn handle_transfer_termination(Json(input): Json<TransferTerminationMessage>) -> impl IntoResponse {
    info!("POST /transfer/termination")
}

