use crate::ssi_auth::consumer::core::{consumer_vc_request, ConsumerSSIVCRequest};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType;
use reqwest::StatusCode;
use tracing::info;

pub fn create_ssi_auth_router() -> Router {
    Router::new()
        .route("/ssi-auth/vc-request", post(handle_consumer_vc_request))
        .route("/ssi-auth/wf-exchange", post(handle_consumer_wf_exchange))
}


async fn handle_consumer_vc_request(Json(input): Json<ConsumerSSIVCRequest>) -> impl IntoResponse {
    info!("POST /ssi-auth/vc-request");

    match consumer_vc_request(input).await {
        Ok(_) => (StatusCode::CREATED, "OK").into_response(),
        Err(e) => TransferErrorType::NotCheckedError { inner_error: e }.into_response(),
    }
}

async fn handle_consumer_wf_exchange() -> impl IntoResponse {
    info!("POST /ssi-auth/wf-exchange");

    match consumer_vc_request().await {
        Ok(_) => (StatusCode::CREATED, "OK").into_response(),
        Err(e) => TransferErrorType::NotCheckedError { inner_error: e }.into_response(),
    }
} 