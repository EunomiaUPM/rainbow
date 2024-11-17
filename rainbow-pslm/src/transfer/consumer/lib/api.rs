use crate::setup::config::{get_consumer_url, get_provider_url};
use crate::transfer::common::err::TransferErrorType::{
    CallbackClientError, NotCheckedError, PidUuidError, TransferProcessNotFound,
};
use crate::transfer::common::utils::convert_uuid_to_uri;
use crate::transfer::consumer::data::models::TransferCallbacksModel;
use crate::transfer::consumer::data::repo::TRANSFER_CONSUMER_REPO;
use crate::transfer::protocol::formats::DctFormats;
use crate::transfer::protocol::messages::{
    DataAddress, TransferCompletionMessage, TransferError, TransferMessageTypes,
    TransferProcessMessage, TransferRequestMessage, TransferStartMessage,
    TransferSuspensionMessage, TRANSFER_CONTEXT,
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use serde_json::to_value;
use utoipa::ToSchema;
use uuid::Uuid;

pub async fn get_all_callbacks() -> anyhow::Result<Vec<TransferCallbacksModel>> {
    let callbacks = TRANSFER_CONSUMER_REPO.get_all_callbacks(None)?;
    Ok(callbacks)
}

pub async fn get_callback_by_id(
    callback_id: Uuid,
) -> anyhow::Result<Option<TransferCallbacksModel>> {
    let callbacks = TRANSFER_CONSUMER_REPO.get_callback_by_id(callback_id)?;
    Ok(callbacks)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCallbackResponse {
    #[serde(rename = "callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "callbackId")]
    pub callback_id: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}
pub async fn create_new_callback() -> anyhow::Result<CreateCallbackResponse> {
    let callback = TRANSFER_CONSUMER_REPO.create_callback()?;
    let callback_url = format!(
        "http://{}/{}/transfers/{}",
        get_consumer_url()?,
        callback.id,
        callback.consumer_pid.unwrap()
    );
    let response = CreateCallbackResponse {
        callback_id: callback.id.to_string(),
        callback_address: callback_url,
        consumer_pid: convert_uuid_to_uri(&callback.consumer_pid.unwrap())?,
        data_address: None,
    };

    Ok(response)
}

pub async fn create_new_callback_with_address(
    data_address: DataAddress,
) -> anyhow::Result<CreateCallbackResponse> {
    let callback = TRANSFER_CONSUMER_REPO.create_callback_with_data_address(data_address)?;
    let callback_url = format!(
        "http://{}/{}/transfers/{}",
        get_consumer_url()?,
        callback.id,
        callback.consumer_pid.unwrap()
    );
    let response = CreateCallbackResponse {
        callback_id: callback.id.to_string(),
        callback_address: callback_url,
        consumer_pid: convert_uuid_to_uri(&callback.consumer_pid.unwrap())?,
        data_address: Some(serde_json::from_value::<DataAddress>(callback.data_address.unwrap())?),
    };
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RequestTransferRequest {
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    pub format: DctFormats,
    #[serde(rename = "callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RequestTransferResponse {
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<String>,
    #[serde(rename = "transferProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_process: Option<TransferProcessMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TransferError>,
}
pub async fn request_transfer(
    request: RequestTransferRequest,
) -> anyhow::Result<RequestTransferResponse, RequestTransferResponse> {
    let consumer_pid = Uuid::from_str(&request.consumer_pid); // <-----
    if consumer_pid.is_err() {
        return Err(RequestTransferResponse {
            consumer_pid: None,
            transfer_process: None,
            error: Some(TransferError::from_async(PidUuidError).await),
        });
    }
    let consumer_pid = consumer_pid.unwrap();

    let transfer_request = TransferRequestMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferRequestMessage.to_string(),
        consumer_pid: convert_uuid_to_uri(&consumer_pid).unwrap(),
        agreement_id: request.agreement_id,
        format: request.format,
        callback_address: request.callback_address,
        data_address: request.data_address,
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
            StatusCode::CREATED => Ok(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: res.json().await.unwrap(),
                error: None,
            }),
            StatusCode::BAD_REQUEST => Err(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: None,
                error: res.json().await.unwrap(),
            }),
            _ => Err(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: None,
                error: res.json().await.unwrap(),
            }),
        },
        Err(e) => Err(RequestTransferResponse {
            consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
            transfer_process: None,
            error: Some(
                TransferError::from_async(NotCheckedError {
                    inner_error: e.into(),
                })
                    .await,
            ),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DataPlaneAddressResponse {
    #[serde(rename = "dataPlaneAddress")]
    pub data_plane_address: String,
}
pub async fn get_data_address_by_consumer_pid(
    consumer_pid: Uuid,
) -> anyhow::Result<DataPlaneAddressResponse, RequestTransferResponse> {
    let callback = TRANSFER_CONSUMER_REPO
        .get_callback_by_consumer_id(consumer_pid)
        .unwrap();
    if callback.is_none() {
        return Err(RequestTransferResponse {
            consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
            transfer_process: None,
            error: Some(TransferError::from_async(CallbackClientError).await),
        });
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
    Ok(DataPlaneAddressResponse { data_plane_address })
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SuspendTransferRequest {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
}
pub async fn suspend_transfer(
    input: SuspendTransferRequest,
) -> anyhow::Result<RequestTransferResponse, RequestTransferResponse> {
    let consumer_pid = Uuid::from_str(&input.consumer_pid);
    if let Err(e) = Uuid::from_str(&input.consumer_pid) {
        return Err(RequestTransferResponse {
            consumer_pid: None,
            transfer_process: None,
            error: Some(TransferError::from_async(PidUuidError).await),
        });
    }
    let consumer_pid = consumer_pid.unwrap();

    let callback = TRANSFER_CONSUMER_REPO
        .get_callback_by_consumer_id(consumer_pid)
        .unwrap();
    if callback.is_none() {
        return Err(RequestTransferResponse {
            consumer_pid: None,
            transfer_process: None,
            error: Some(TransferError::from_async(TransferProcessNotFound).await),
        });
    }
    let callback = callback.unwrap();
    let transfer_suspend = TransferSuspensionMessage {
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
        .json(&transfer_suspend)
        .send()
        .await;

    match req {
        Ok(res) => match res.status() {
            StatusCode::OK => Ok(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: res.json().await.unwrap(),
                error: None,
            }),
            status => Err(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: None,
                error: res.json().await.unwrap(),
            }),
        },
        Err(e) => Err(RequestTransferResponse {
            consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
            transfer_process: None,
            error: Some(
                TransferError::from_async(NotCheckedError {
                    inner_error: e.into(),
                })
                    .await,
            ),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RestartTransferRequest {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
}
pub async fn restart_transfer(
    input: RestartTransferRequest,
) -> anyhow::Result<RequestTransferResponse, RequestTransferResponse> {
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
            StatusCode::OK => Ok(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: res.json().await.unwrap(),
                error: None,
            }),
            status => Err(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: None,
                error: res.json().await.unwrap(),
            }),
        },
        Err(e) => Err(RequestTransferResponse {
            consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
            transfer_process: None,
            error: Some(
                TransferError::from_async(NotCheckedError {
                    inner_error: e.into(),
                })
                    .await,
            ),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CompleteTransferRequest {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
}
pub async fn complete_transfer(
    input: CompleteTransferRequest,
) -> anyhow::Result<RequestTransferResponse, RequestTransferResponse> {
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
            StatusCode::OK => Ok(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: res.json().await.unwrap(),
                error: None,
            }),
            status => Err(RequestTransferResponse {
                consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
                transfer_process: None,
                error: res.json().await.unwrap(),
            }),
        },
        Err(e) => Err(RequestTransferResponse {
            consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
            transfer_process: None,
            error: Some(
                TransferError::from_async(NotCheckedError {
                    inner_error: e.into(),
                })
                    .await,
            ),
        }),
    }
}
