/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use anyhow::anyhow;
use axum::http::StatusCode;
use rainbow_common::config::config::{get_consumer_url, get_provider_url};
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::err::transfer_err::TransferErrorType::{
    CallbackClientError, NotCheckedError, PidUuidError, TransferProcessNotFound,
};
use rainbow_common::protocol::transfer::{
    DataAddress, TransferCompletionMessage, TransferError, TransferMessageTypes,
    TransferProcessMessage, TransferRequestMessage, TransferStartMessage,
    TransferSuspensionMessage, TRANSFER_CONTEXT,
};
use rainbow_common::utils::get_urn_from_string;
// use rainbow_dataplane::{
//     bootstrap_data_plane_in_consumer, get_data_plane_peer, set_data_plane_next_hop,
// };
use rainbow_db::transfer_consumer::entities::transfer_callback;
use rainbow_db::transfer_consumer::repo::{
    EditTransferCallback, NewTransferCallback, TRANSFER_CONSUMER_REPO,
};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use std::str::FromStr;
use urn::Urn;

pub async fn get_all_callbacks() -> anyhow::Result<Vec<transfer_callback::Model>> {
    TRANSFER_CONSUMER_REPO.get_all_transfer_callbacks(None, None).await
}

pub async fn get_callback_by_id(
    callback_id: Urn,
) -> anyhow::Result<Option<transfer_callback::Model>> {
    TRANSFER_CONSUMER_REPO.get_transfer_callbacks_by_id(callback_id).await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCallbackResponse {
    #[serde(rename = "callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "callbackId")]
    pub callback_id: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}
pub async fn create_new_callback() -> anyhow::Result<CreateCallbackResponse> {
    let callback = TRANSFER_CONSUMER_REPO
        .create_transfer_callback(NewTransferCallback { data_address: None })
        .await?;

    let callback_url = format!("http://{}/{}", get_consumer_url()?, callback.id, );
    let response = CreateCallbackResponse {
        callback_id: get_urn_from_string(&callback.id)?,
        callback_address: callback_url,
        consumer_pid: get_urn_from_string(&callback.consumer_pid)?,
        data_address: None,
    };

    Ok(response)
}

pub async fn create_new_callback_with_address(
    data_address: DataAddress,
) -> anyhow::Result<CreateCallbackResponse> {
    let callback = TRANSFER_CONSUMER_REPO
        .create_transfer_callback(NewTransferCallback {
            data_address: Option::from(to_value(data_address).unwrap()),
        })
        .await?;

    let callback_address = format!("http://{}/{}", get_consumer_url()?, callback.id, );
    let data_plane_address = format!("{}/data/push/{}", callback_address, callback.consumer_pid);

    let response = CreateCallbackResponse {
        callback_id: get_urn_from_string(&callback.id)?,
        callback_address,
        consumer_pid: get_urn_from_string(&callback.consumer_pid)?,
        data_address: Option::from(serde_json::from_value::<DataAddress>(
            callback.data_address.unwrap(),
        )?),
    };
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTransferResponse {
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "transferProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_process: Option<TransferProcessMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TransferError>,
}
pub async fn request_transfer(
    request: RequestTransferRequest,
) -> anyhow::Result<RequestTransferResponse, RequestTransferResponse> {
    let db_connection = get_db_connection().await;
    let request_format = request.format;
    let request_callback_address = request.callback_address;
    let consumer_pid = match get_urn_from_string(&request.consumer_pid) {
        Ok(p) => p,
        Err(e) => {
            return Err(RequestTransferResponse {
                consumer_pid: None,
                transfer_process: None,
                error: Some(TransferError::from_async(PidUuidError).await),
            })
        }
    };
    let agreement_id = match get_urn_from_string(&request.agreement_id) {
        Ok(p) => p,
        Err(e) => {
            return Err(RequestTransferResponse {
                consumer_pid: Some(consumer_pid),
                transfer_process: None,
                error: Some(TransferError::from_async(PidUuidError).await),
            })
        }
    };

    let transfer_request = TransferRequestMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferRequestMessage.to_string(),
        consumer_pid: consumer_pid.to_string(),
        agreement_id: agreement_id.to_string(),
        format: request_format.clone(),
        callback_address: request_callback_address.clone(),
        data_address: request.data_address,
    };

    let url = format!(
        "http://{}/{}",
        get_provider_url().unwrap(),
        "transfers/request"
    );

    // request to provider
    let req = reqwest::Client::new().post(url).json(&transfer_request).send().await;

    match req {
        Ok(res) => match res.status() {
            StatusCode::CREATED => {
                // setup data plane
                let transfer_process = res.json::<TransferProcessMessage>().await.unwrap();
                let provider_pid = transfer_process.provider_pid.clone();
                let consumer_pid = transfer_process.consumer_pid.clone();
                let provider_pid = get_urn_from_string(&provider_pid).unwrap();
                let consumer_pid = get_urn_from_string(&consumer_pid).unwrap();
                let data_plane_peer =
                    bootstrap_data_plane_in_consumer(transfer_request).await.unwrap();
                let data_plane_id = data_plane_peer.id.clone();
                set_data_plane_next_hop(data_plane_peer, provider_pid, consumer_pid.clone())
                    .await
                    .unwrap();

                // persist data plane in transfer
                match TRANSFER_CONSUMER_REPO
                    .put_transfer_callback_by_consumer(
                        consumer_pid.clone(),
                        EditTransferCallback {
                            data_plane_id: Some(data_plane_id),
                            ..Default::default()
                        },
                    )
                    .await
                {
                    Ok(res) => Ok(RequestTransferResponse {
                        consumer_pid: Some(get_urn_from_string(&res.consumer_pid).unwrap()),
                        transfer_process: Some(transfer_process),
                        error: None,
                    }),
                    Err(_) => Err(RequestTransferResponse {
                        consumer_pid: Some(consumer_pid.clone()),
                        transfer_process: None,
                        error: Some(
                            TransferError::from_async(NotCheckedError {
                                inner_error: anyhow!("Db error"),
                            })
                                .await,
                        ),
                    }),
                }
            }
            StatusCode::BAD_REQUEST => Err(RequestTransferResponse {
                consumer_pid: Some(consumer_pid),
                transfer_process: None,
                error: res.json().await.unwrap(),
            }),
            _ => Err(RequestTransferResponse {
                consumer_pid: Some(consumer_pid),
                transfer_process: None,
                error: res.json().await.unwrap(),
            }),
        },
        Err(e) => Err(RequestTransferResponse {
            consumer_pid: Some(consumer_pid),
            transfer_process: None,
            error: Some(TransferError::from_async(NotCheckedError { inner_error: e.into() }).await),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPlaneAddressResponse {
    #[serde(rename = "dataPlaneAddress")]
    pub data_plane_address: String,
}
pub async fn get_data_address_by_consumer_pid(
    consumer_pid: Urn,
) -> anyhow::Result<DataPlaneAddressResponse, RequestTransferResponse> {
    let callback =
        TRANSFER_CONSUMER_REPO.get_transfer_callbacks_by_consumer_id(consumer_pid.clone()).await;
    match callback.unwrap() {
        Some(callback) => {
            let data_plane_id = get_urn_from_string(&callback.data_plane_id.unwrap()).unwrap();
            let dataplane = get_data_plane_peer(data_plane_id).await.unwrap();
            Ok(DataPlaneAddressResponse { data_plane_address: dataplane.local_address.unwrap() })
        }
        None => Err(RequestTransferResponse {
            consumer_pid: Some(consumer_pid),
            transfer_process: None,
            error: Some(TransferError::from_async(CallbackClientError).await),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspendTransferRequest {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
}
pub async fn suspend_transfer(
    input: SuspendTransferRequest,
) -> anyhow::Result<RequestTransferResponse, RequestTransferResponse> {
    let consumer_pid = match get_urn_from_string(&input.consumer_pid) {
        Ok(p) => p,
        Err(e) => {
            return Err(RequestTransferResponse {
                consumer_pid: None,
                transfer_process: None,
                error: Some(TransferError::from_async(PidUuidError).await),
            })
        }
    };
    let callback = match TRANSFER_CONSUMER_REPO
        .get_transfer_callbacks_by_consumer_id(consumer_pid.clone())
        .await
    {
        Ok(callback) => match callback {
            Some(callback) => callback,
            None => {
                return Err(RequestTransferResponse {
                    consumer_pid: None,
                    transfer_process: None,
                    error: Some(TransferError::from_async(TransferProcessNotFound).await),
                })
            }
        },
        Err(e) => {
            return Err(RequestTransferResponse {
                consumer_pid: None,
                transfer_process: None,
                error: Some(TransferError::from_async(NotCheckedError { inner_error: e }).await),
            })
        }
    };

    let transfer_suspend = TransferSuspensionMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
        provider_pid: callback.provider_pid.clone().unwrap(),
        consumer_pid: callback.consumer_pid,
        code: "TRANSFER_SUSPENSION_FROM_CONSUMER_CLIENT".to_string(),
        reason: vec![],
    };

    let url = format!(
        "http://{}/transfers/{}/suspension",
        get_provider_url().unwrap(),
        callback.provider_pid.unwrap()
    );

    let req = reqwest::Client::new().post(url).json(&transfer_suspend).send().await;

    match req {
        Ok(res) => match res.status() {
            StatusCode::OK => Ok(RequestTransferResponse {
                consumer_pid: Some(consumer_pid),
                transfer_process: res.json().await.unwrap(),
                error: None,
            }),
            status => Err(RequestTransferResponse {
                consumer_pid: Some(consumer_pid),
                transfer_process: None,
                error: res.json().await.unwrap(),
            }),
        },
        Err(e) => Err(RequestTransferResponse {
            consumer_pid: Some(consumer_pid),
            transfer_process: None,
            error: Some(TransferError::from_async(NotCheckedError { inner_error: e.into() }).await),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestartTransferRequest {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
    // TODO create new data address
}
pub async fn restart_transfer(
    input: RestartTransferRequest,
) -> anyhow::Result<RequestTransferResponse, RequestTransferResponse> {
    let consumer_pid = match get_urn_from_string(&input.consumer_pid) {
        Ok(p) => p,
        Err(e) => {
            return Err(RequestTransferResponse {
                consumer_pid: None,
                transfer_process: None,
                error: Some(TransferError::from_async(PidUuidError).await),
            })
        }
    };
    let callback = match TRANSFER_CONSUMER_REPO
        .get_transfer_callbacks_by_consumer_id(consumer_pid.clone())
        .await
    {
        Ok(callback) => match callback {
            Some(callback) => callback,
            None => {
                return Err(RequestTransferResponse {
                    consumer_pid: None,
                    transfer_process: None,
                    error: Some(TransferError::from_async(TransferProcessNotFound).await),
                })
            }
        },
        Err(e) => {
            return Err(RequestTransferResponse {
                consumer_pid: None,
                transfer_process: None,
                error: Some(TransferError::from_async(NotCheckedError { inner_error: e }).await),
            })
        }
    };

    let transfer_start = TransferStartMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferStartMessage.to_string(),
        // provider_pid: convert_uuid_to_uri(&callback.provider_pid.unwrap()).unwrap(),
        provider_pid: callback.provider_pid.clone().unwrap(),
        consumer_pid: callback.consumer_pid,
        data_address: None,
    };

    let url = format!(
        "http://{}/transfers/{}/start",
        get_provider_url().unwrap(),
        callback.provider_pid.unwrap()
    );

    let req = reqwest::Client::new().post(url).json(&transfer_start).send().await;

    match req {
        Ok(res) => match res.status() {
            StatusCode::OK => Ok(RequestTransferResponse {
                consumer_pid: Some(consumer_pid.clone()),
                transfer_process: res.json().await.unwrap(),
                error: None,
            }),
            status => Err(RequestTransferResponse {
                consumer_pid: Some(consumer_pid.clone()),
                transfer_process: None,
                error: res.json().await.unwrap(),
            }),
        },
        Err(e) => Err(RequestTransferResponse {
            consumer_pid: Some(consumer_pid.clone()),
            transfer_process: None,
            error: Some(TransferError::from_async(NotCheckedError { inner_error: e.into() }).await),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteTransferRequest {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
}
pub async fn complete_transfer(
    input: CompleteTransferRequest,
) -> anyhow::Result<RequestTransferResponse, RequestTransferResponse> {
    let consumer_pid = match get_urn_from_string(&input.consumer_pid) {
        Ok(p) => p,
        Err(e) => {
            return Err(RequestTransferResponse {
                consumer_pid: None,
                transfer_process: None,
                error: Some(TransferError::from_async(PidUuidError).await),
            })
        }
    };
    let callback = match TRANSFER_CONSUMER_REPO
        .get_transfer_callbacks_by_consumer_id(consumer_pid.clone())
        .await
    {
        Ok(callback) => match callback {
            Some(callback) => callback,
            None => {
                return Err(RequestTransferResponse {
                    consumer_pid: None,
                    transfer_process: None,
                    error: Some(TransferError::from_async(TransferProcessNotFound).await),
                })
            }
        },
        Err(e) => {
            return Err(RequestTransferResponse {
                consumer_pid: None,
                transfer_process: None,
                error: Some(TransferError::from_async(NotCheckedError { inner_error: e }).await),
            })
        }
    };

    let transfer_complete = TransferCompletionMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferCompletionMessage.to_string(),
        provider_pid: callback.provider_pid.clone().unwrap(),
        consumer_pid: callback.consumer_pid,
    };

    let url = format!(
        "http://{}/transfers/{}/completion",
        get_provider_url().unwrap(),
        callback.provider_pid.unwrap()
    );

    let req = reqwest::Client::new().post(url).json(&transfer_complete).send().await;

    match req {
        Ok(res) => match res.status() {
            StatusCode::OK => Ok(RequestTransferResponse {
                consumer_pid: Some(consumer_pid.clone()),
                transfer_process: res.json().await.unwrap(),
                error: None,
            }),
            status => Err(RequestTransferResponse {
                consumer_pid: Some(consumer_pid.clone()),
                transfer_process: None,
                error: res.json().await.unwrap(),
            }),
        },
        Err(e) => Err(RequestTransferResponse {
            consumer_pid: Some(consumer_pid.clone()),
            transfer_process: None,
            error: Some(TransferError::from_async(NotCheckedError { inner_error: e.into() }).await),
        }),
    }
}
