use crate::setup::config::{get_consumer_url, get_provider_url};
use crate::transfer::common::err::TransferErrorType::{CallbackClientError, NotCheckedError, PidUuidError, TransferProcessNotFound};
use crate::transfer::common::utils::convert_uuid_to_uri;
use crate::transfer::consumer::data::repo::TRANSFER_CONSUMER_REPO;
use crate::transfer::consumer::lib::api::{get_all_callbacks, get_callback_by_id};
use crate::transfer::consumer::lib::callbacks_controller::create_new_callback;
use crate::transfer::protocol::formats::DctFormats;
use crate::transfer::protocol::messages::{
    DataAddress, TransferError, TransferMessageTypes, TransferProcessMessage, TRANSFER_CONTEXT,
};
use crate::transfer::protocol::messages::{TransferCompletionMessage, TransferSuspensionMessage};
use crate::transfer::protocol::messages::{TransferRequestMessage, TransferStartMessage};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

pub fn router() -> Router {
    let group_1 = Router::new()
        .route("/api/v1/callbacks", get(handle_get_all_callbacks))
        .route(
            "/api/v1/callbacks/:callback_id",
            get(handle_get_callback_by_id),
        )
        .route("/api/v1/callbacks", post(handle_create_callback));

    let group_2 = Router::new()
        .route("/api/v1/setup-transfer", post(handle_create_callback))
        .route("/api/v1/request-transfer", post(handle_request_transfer))
        .route(
            "/api/v1/data-address/:consumer_pid",
            get(handle_get_data_address_by_consumer_pid),
        )
        .route("/api/v1/suspend-transfer", post(handle_suspend_transfer))
        .route("/api/v1/restart-transfer", post(handle_restart_transfer))
        .route("/api/v1/complete-transfer", post(handle_complete_transfer));

    Router::new().merge(group_1).merge(group_2)
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
pub struct CreateCallbackResponse {
    #[serde(rename = "callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "callbackId")]
    pub callback_id: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
}
async fn handle_create_callback(data_address: Option<Json<DataAddress>>) -> impl IntoResponse {
    match data_address {
        Some(address) => {
            todo!("not implemented");
            (StatusCode::NOT_IMPLEMENTED, "not implemented").into_response()
        }
        None => {
            let callback = create_new_callback()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
                .unwrap();

            let callback_url = format!(
                "http://{}/{}/transfers/{}",
                get_consumer_url().unwrap(),
                callback.id,
                callback.consumer_pid.unwrap()
            );
            (
                StatusCode::CREATED,
                Json(CreateCallbackResponse {
                    callback_id: callback.id.to_string(),
                    callback_address: callback_url,
                    consumer_pid: convert_uuid_to_uri(&callback.consumer_pid.unwrap()).unwrap(),
                }),
            )
                .into_response()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestTransferRequest {
    #[serde(rename = "agreementId")]
    agreement_id: String,
    format: DctFormats,
    #[serde(rename = "callbackAddress")]
    callback_address: String,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    data_address: Option<DataAddress>,
    #[serde(rename = "consumerPid")]
    consumer_pid: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct RequestTransferResponse {
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_pid: Option<String>,
    #[serde(rename = "transferProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_process: Option<TransferProcessMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<TransferError>,
}
async fn handle_request_transfer(Json(input): Json<RequestTransferRequest>) -> impl IntoResponse {
    let consumer_pid = Uuid::from_str(&input.consumer_pid);
    if let Err(e) = Uuid::from_str(&input.consumer_pid) {
        return (
            StatusCode::BAD_REQUEST,
            Json(RequestTransferResponse {
                consumer_pid: None,
                transfer_process: None,
                error: Some(TransferError::from_async(PidUuidError).await),
            }),
        );
    }
    let consumer_pid = consumer_pid.unwrap();

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
        Ok(res) => match res.status() {
            StatusCode::CREATED => (
                StatusCode::CREATED,
                Json(RequestTransferResponse {
                    consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                    transfer_process: res.json().await.unwrap(),
                    error: None,
                }),
            ),
            StatusCode::BAD_REQUEST => (
                StatusCode::BAD_REQUEST,
                Json(RequestTransferResponse {
                    consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                    transfer_process: None,
                    error: res.json().await.unwrap(),
                }),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RequestTransferResponse {
                    consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                    transfer_process: None,
                    error: res.json().await.unwrap(),
                }),
            ),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: None,
                error: Some(
                    TransferError::from_async(NotCheckedError {
                        inner_error: e.into(),
                    })
                        .await,
                ),
            }),
        ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DataPlaneAddressResponse {
    #[serde(rename = "dataPlaneAddress")]
    data_plane_address: String,
}
async fn handle_get_data_address_by_consumer_pid(
    Path(consumer_pid): Path<Uuid>,
) -> impl IntoResponse {
    let callback = TRANSFER_CONSUMER_REPO
        .get_callback_by_consumer_id(consumer_pid)
        .unwrap();

    if callback.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: None,
                error: Some(TransferError::from_async(CallbackClientError).await),
            }),
        ).into_response();
    }


    let callback = callback.unwrap();
    let consumer_pid = callback.consumer_pid.unwrap().to_string();
    let callback_id = callback.id.to_string();
    let data_plane_address = format!(
        "http://{}/{}/data/{}",
        get_consumer_url().unwrap(),
        callback_id,
        consumer_pid
    );

    (
        StatusCode::OK,
        Json(DataPlaneAddressResponse { data_plane_address }),
    ).into_response()
}

#[derive(Debug, Serialize, Deserialize)]
struct SuspendTransferRequest {
    #[serde(rename = "consumerPid")]
    consumer_pid: String,
}
async fn handle_suspend_transfer(Json(input): Json<SuspendTransferRequest>) -> impl IntoResponse {
    // here i am...
    let consumer_pid = Uuid::from_str(&input.consumer_pid);
    if let Err(e) = Uuid::from_str(&input.consumer_pid) {
        return (
            StatusCode::BAD_REQUEST,
            Json(RequestTransferResponse {
                consumer_pid: None,
                transfer_process: None,
                error: Some(TransferError::from_async(PidUuidError).await),
            }),
        ).into_response();
    }
    let consumer_pid = consumer_pid.unwrap();

    let callback = TRANSFER_CONSUMER_REPO
        .get_callback_by_consumer_id(consumer_pid)
        .unwrap();
    if callback.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(RequestTransferResponse {
                consumer_pid: None,
                transfer_process: None,
                error: Some(TransferError::from_async(TransferProcessNotFound).await),
            }),
        ).into_response();
    }
    let callback = callback.unwrap();
    let trasnfer_suspend = TransferSuspensionMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&callback.provider_pid.unwrap()).unwrap(),
        consumer_pid: convert_uuid_to_uri(&callback.consumer_pid.unwrap()).unwrap(),
        code: "TRANSFER_SUSPENSION_FROM_CONSUMER_CLIENT".to_string(),
        reason: vec![],
    };

    let url = format!(
        "http://{}/{}",
        get_provider_url().unwrap(),
        "transfers/suspension"
    );

    let req = reqwest::Client::new()
        .post(url)
        .json(&trasnfer_suspend)
        .send()
        .await;

    match req {
        Ok(res) => match res.status() {
            StatusCode::OK => (
                StatusCode::OK,
                Json(RequestTransferResponse {
                    consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                    transfer_process: res.json().await.unwrap(),
                    error: None,
                }),
            ).into_response(),
            status => (
                StatusCode::BAD_REQUEST,
                Json(RequestTransferResponse {
                    consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                    transfer_process: None,
                    error: res.json().await.unwrap(),
                }),
            ).into_response()
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: None,
                error: Some(
                    TransferError::from_async(NotCheckedError {
                        inner_error: e.into(),
                    })
                        .await,
                ),
            }),
        ).into_response(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RestartTransferRequest {
    #[serde(rename = "consumerPid")]
    consumer_pid: String,
}
async fn handle_restart_transfer(Json(input): Json<RestartTransferRequest>) -> impl IntoResponse {
    let consumer_pid = Uuid::from_str(&input.consumer_pid)
        .map_err(|_| return (StatusCode::BAD_REQUEST, "invalid consumer_pid"))
        .unwrap();

    let callback = TRANSFER_CONSUMER_REPO
        .get_callback_by_consumer_id(consumer_pid)
        .unwrap()
        .unwrap();

    let transfer_start = TransferStartMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferStartMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&callback.provider_pid.unwrap()).unwrap(),
        consumer_pid: convert_uuid_to_uri(&callback.consumer_pid.unwrap()).unwrap(),
        data_address: None,
    };

    let url = format!(
        "http://{}/{}",
        get_provider_url().unwrap(),
        "transfers/start"
    );

    let req = reqwest::Client::new()
        .post(url)
        .json(&transfer_start)
        .send()
        .await;

    match req {
        Ok(res) => match res.status() {
            StatusCode::OK => (
                StatusCode::OK,
                Json(RequestTransferResponse {
                    consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                    transfer_process: res.json().await.unwrap(),
                    error: None,
                }),
            ).into_response(),
            status => (
                StatusCode::BAD_REQUEST,
                Json(RequestTransferResponse {
                    consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                    transfer_process: None,
                    error: res.json().await.unwrap(),
                }),
            ).into_response()
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: None,
                error: Some(
                    TransferError::from_async(NotCheckedError {
                        inner_error: e.into(),
                    })
                        .await,
                ),
            }),
        ).into_response(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CompleteTransferRequest {
    #[serde(rename = "consumerPid")]
    consumer_pid: String,
}
async fn handle_complete_transfer(Json(input): Json<CompleteTransferRequest>) -> impl IntoResponse {
    let consumer_pid = Uuid::from_str(&input.consumer_pid)
        .map_err(|_| return (StatusCode::BAD_REQUEST, "invalid consumer_pid"))
        .unwrap();

    let callback = TRANSFER_CONSUMER_REPO
        .get_callback_by_consumer_id(consumer_pid)
        .unwrap()
        .unwrap();

    let transfer_complete = TransferCompletionMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferCompletionMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&callback.provider_pid.unwrap()).unwrap(),
        consumer_pid: convert_uuid_to_uri(&callback.consumer_pid.unwrap()).unwrap(),
    };

    let url = format!(
        "http://{}/{}",
        get_provider_url().unwrap(),
        "transfers/completion"
    );

    let req = reqwest::Client::new()
        .post(url)
        .json(&transfer_complete)
        .send()
        .await;

    match req {
        Ok(res) => match res.status() {
            StatusCode::OK => (
                StatusCode::OK,
                Json(RequestTransferResponse {
                    consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                    transfer_process: res.json().await.unwrap(),
                    error: None,
                }),
            ).into_response(),
            status => (
                StatusCode::BAD_REQUEST,
                Json(RequestTransferResponse {
                    consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                    transfer_process: None,
                    error: res.json().await.unwrap(),
                }),
            ).into_response()
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: None,
                error: Some(
                    TransferError::from_async(NotCheckedError {
                        inner_error: e.into(),
                    })
                        .await,
                ),
            }),
        ).into_response(),
    }
}
