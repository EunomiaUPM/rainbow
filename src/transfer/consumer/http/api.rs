use crate::setup::config::{get_consumer_url, get_provider_url};
use crate::transfer::common::utils::convert_uuid_to_uri;
use crate::transfer::consumer::lib::api::{get_all_callbacks, get_callback_by_id};
use crate::transfer::consumer::lib::callbacks_controller::create_new_callback;
use crate::transfer::protocol::formats::DctFormats;
use crate::transfer::protocol::messages::TransferRequestMessage;
use crate::transfer::protocol::messages::{DataAddress, TransferMessageTypes, TRANSFER_CONTEXT};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/callbacks", get(handle_get_all_callbacks))
        .route(
            "/api/v1/callbacks/:callback_id",
            get(handle_get_callback_by_id),
        )
        .route("/api/v1/setup-transfer", post(handle_create_callback))
        .route("/api/v1/callbacks", post(handle_create_callback))
        .route("/api/v1/request-transfer", post(handle_request_transfer))
}

async fn handle_get_all_callbacks() -> impl IntoResponse {
    let callbacks = get_all_callbacks().await;
    if callbacks.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    (StatusCode::OK, Json(callbacks.unwrap())).into_response()
}

async fn handle_get_callback_by_id(Path(callback_id): Path<Uuid>) -> impl IntoResponse {
    let callbacks = get_callback_by_id(callback_id).await;
    if callbacks.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let callbacks = callbacks.unwrap();
    if callbacks.is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }
    (StatusCode::OK, Json(callbacks.unwrap())).into_response()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseCallback {
    #[serde(rename = "callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "callbackId")]
    pub callback_id: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
}
async fn handle_create_callback(data_address: Option<Json<DataAddress>>) -> impl IntoResponse {
    match data_address {
        Some(address) => (StatusCode::NOT_IMPLEMENTED, "not implemented").into_response(),
        None => {
            let callback = create_new_callback().await;
            if callback.is_err() {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            let callback = callback.unwrap();
            let callback_url = format!(
                "http://{}/{}/transfers/{}",
                get_consumer_url().unwrap(),
                callback.id,
                callback.consumer_pid.unwrap()
            );
            (
                StatusCode::CREATED,
                Json(ApiResponseCallback {
                    callback_id: callback.id.to_string(),
                    callback_address: callback_url,
                    consumer_pid: callback.consumer_pid.unwrap().to_string(),
                }),
            )
                .into_response()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiRequestPayload {
    #[serde(rename = "agreementId")]
    agreement_id: String,
    format: DctFormats,
    #[serde(rename = "callback_address")]
    callback_address: String,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    data_address: Option<DataAddress>,
    #[serde(rename = "consumerPid")]
    consumer_pid: String,
}
async fn handle_request_transfer(Json(input): Json<ApiRequestPayload>) -> impl IntoResponse {
    let consumer_pid = Uuid::from_str(&input.consumer_pid).unwrap();
    let transfer_request = TransferRequestMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferRequestMessage.to_string(),
        consumer_pid: convert_uuid_to_uri(&consumer_pid).unwrap(),
        agreement_id: input.agreement_id,
        format: input.format,
        callback_address: input.callback_address,
        data_address: input.data_address,
    };

    let url = format!(
        "http://{}/{}",
        get_provider_url().unwrap(),
        "transfers/request"
    );

    let req = reqwest::Client::new()
        .post(url)
        .json(&transfer_request)
        .send()
        .await;

    match req {
        Ok(res) => (res.status(), res.text().await.unwrap()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
