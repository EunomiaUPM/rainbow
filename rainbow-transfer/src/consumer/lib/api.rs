use anyhow::anyhow;
use axum::http::StatusCode;
use rainbow_common::config::config::{get_consumer_url, get_provider_url};
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::DctFormats;
// use crate::setup::config::{get_consumer_url, get_provider_url};
use rainbow_common::err::transfer_err::TransferErrorType::{
    CallbackClientError, NotCheckedError, PidUuidError, TransferProcessNotFound,
};
use rainbow_common::protocol::transfer::{
    DataAddress, TransferCompletionMessage, TransferError, TransferMessageTypes,
    TransferProcessMessage, TransferRequestMessage, TransferStartMessage,
    TransferSuspensionMessage, TRANSFER_CONTEXT,
};
use rainbow_common::utils::{convert_uri_to_uuid, convert_uuid_to_uri};
use rainbow_dataplane::{
    bootstrap_data_plane_in_consumer, get_data_plane_peer, set_data_plane_next_hop,
};
use rainbow_db::transfer_consumer::entities::transfer_callback;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;

pub async fn get_all_callbacks() -> anyhow::Result<Vec<transfer_callback::Model>> {
    let db_connection = get_db_connection().await;
    let callbacks = transfer_callback::Entity::find().all(db_connection).await?;
    Ok(callbacks)
}

pub async fn get_callback_by_id(
    callback_id: Uuid,
) -> anyhow::Result<Option<transfer_callback::Model>> {
    let db_connection = get_db_connection().await;
    let callbacks = transfer_callback::Entity::find_by_id(callback_id).one(db_connection).await?;
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
    let db_connection = get_db_connection().await;
    let callback = transfer_callback::Entity::insert(transfer_callback::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        consumer_pid: ActiveValue::Set(Uuid::new_v4()),
        provider_pid: ActiveValue::Set(None),
        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        updated_at: ActiveValue::Set(None),
        data_plane_id: ActiveValue::Set(None),
    })
    .exec_with_returning(db_connection)
    .await?;

    let callback_url = format!("http://{}/{}", get_consumer_url()?, callback.id,);
    let response = CreateCallbackResponse {
        callback_id: callback.id.to_string(),
        callback_address: callback_url,
        consumer_pid: convert_uuid_to_uri(&callback.consumer_pid)?,
        data_address: None,
    };

    Ok(response)
}

pub async fn create_new_callback_with_address(
    data_address: DataAddress,
) -> anyhow::Result<CreateCallbackResponse> {
    let db_connection = get_db_connection().await;
    let callback_id = Uuid::new_v4();
    let consumer_pid = Uuid::new_v4();

    let callback_url = format!("http://{}/{}", get_consumer_url()?, callback_id,);

    let data_plane_address = format!("{}/data/push/{}", callback_url, consumer_pid);

    let callback = transfer_callback::Entity::insert(transfer_callback::ActiveModel {
        id: ActiveValue::Set(callback_id),
        consumer_pid: ActiveValue::Set(consumer_pid),
        provider_pid: ActiveValue::Set(None),
        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        updated_at: ActiveValue::Set(None),
        data_plane_id: ActiveValue::Set(None),
    })
    .exec_with_returning(db_connection)
    .await?;

    let response = CreateCallbackResponse {
        callback_id: callback.id.to_string(),
        callback_address: callback_url,
        consumer_pid: convert_uuid_to_uri(&callback.consumer_pid)?,
        data_address: None,
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
    let db_connection = get_db_connection().await;
    let consumer_pid = Uuid::from_str(&request.consumer_pid);
    let request_format = request.format;
    let request_callback_address = request.callback_address;
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
        format: request_format.clone(),
        callback_address: request_callback_address.clone(),
        data_address: request.data_address,
    };

    println!("{:#?}", transfer_request);
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
                let provider_uuid = convert_uri_to_uuid(&provider_pid).unwrap();
                let consumer_uuid = convert_uri_to_uuid(&consumer_pid).unwrap();
                let data_plane_peer =
                    bootstrap_data_plane_in_consumer(transfer_request).await.unwrap();
                let data_plane_id = data_plane_peer.id;
                set_data_plane_next_hop(data_plane_peer, provider_uuid, consumer_uuid)
                    .await
                    .unwrap();

                // persist data plane in transfer
                let tp = transfer_callback::Entity::find()
                    .filter(transfer_callback::Column::ConsumerPid.eq(consumer_uuid))
                    .one(db_connection)
                    .await
                    .unwrap();
                if tp.is_none() {
                    return Err(RequestTransferResponse {
                        consumer_pid: Some(consumer_pid.clone()),
                        transfer_process: None,
                        error: Some(
                            TransferError::from_async(NotCheckedError {
                                inner_error: anyhow!("Db error"),
                            })
                            .await,
                        ),
                    });
                }
                let tp = tp.unwrap();
                transfer_callback::Entity::update(transfer_callback::ActiveModel {
                    id: ActiveValue::Set(tp.id),
                    consumer_pid: ActiveValue::Set(tp.consumer_pid),
                    provider_pid: ActiveValue::Set(tp.provider_pid),
                    created_at: ActiveValue::Set(tp.created_at),
                    updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
                    data_plane_id: ActiveValue::Set(Some(data_plane_id)),
                })
                .exec(db_connection)
                .await
                .unwrap();

                // return to client
                Ok(RequestTransferResponse {
                    consumer_pid: Some(consumer_pid),
                    transfer_process: Some(transfer_process),
                    error: None,
                })
            }
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
            error: Some(TransferError::from_async(NotCheckedError { inner_error: e.into() }).await),
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
    let db_connection = get_db_connection().await;
    let callback = transfer_callback::Entity::find()
        .filter(transfer_callback::Column::ConsumerPid.eq(consumer_pid))
        .one(db_connection)
        .await
        .unwrap();

    if callback.is_none() {
        return Err(RequestTransferResponse {
            consumer_pid: Some(convert_uuid_to_uri(&consumer_pid).unwrap()),
            transfer_process: None,
            error: Some(TransferError::from_async(CallbackClientError).await),
        });
    }

    let callback = callback.unwrap();
    let dataplane = get_data_plane_peer(callback.data_plane_id.unwrap()).await.unwrap();

    Ok(DataPlaneAddressResponse { data_plane_address: dataplane.local_address.unwrap() })
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

    let db_connection = get_db_connection().await;
    let callback = transfer_callback::Entity::find()
        .filter(transfer_callback::Column::ConsumerPid.eq(consumer_pid))
        .one(db_connection)
        .await
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
        consumer_pid: convert_uuid_to_uri(&callback.consumer_pid).unwrap(),
        code: "TRANSFER_SUSPENSION_FROM_CONSUMER_CLIENT".to_string(),
        reason: vec![],
    };
    let url = format!(
        "http://{}/{}",
        get_provider_url().unwrap(),
        "transfers/suspension"
    );

    let req = reqwest::Client::new().post(url).json(&transfer_suspend).send().await;

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
            error: Some(TransferError::from_async(NotCheckedError { inner_error: e.into() }).await),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RestartTransferRequest {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
    // TODO create new data address
}
pub async fn restart_transfer(
    input: RestartTransferRequest,
) -> anyhow::Result<RequestTransferResponse, RequestTransferResponse> {
    let consumer_pid = Uuid::from_str(&input.consumer_pid)
        .map_err(|_| return (StatusCode::BAD_REQUEST, "invalid consumer_pid"))
        .unwrap();

    let db_connection = get_db_connection().await;
    let callback = transfer_callback::Entity::find()
        .filter(transfer_callback::Column::ConsumerPid.eq(consumer_pid))
        .one(db_connection)
        .await
        .unwrap()
        .unwrap();

    let transfer_start = TransferStartMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferStartMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&callback.provider_pid.unwrap()).unwrap(),
        consumer_pid: convert_uuid_to_uri(&callback.consumer_pid).unwrap(),
        data_address: None,
    };

    let url = format!(
        "http://{}/{}",
        get_provider_url().unwrap(),
        "transfers/start"
    );

    let req = reqwest::Client::new().post(url).json(&transfer_start).send().await;

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
            error: Some(TransferError::from_async(NotCheckedError { inner_error: e.into() }).await),
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

    let db_connection = get_db_connection().await;
    let callback = transfer_callback::Entity::find()
        .filter(transfer_callback::Column::ConsumerPid.eq(consumer_pid))
        .one(db_connection)
        .await
        .unwrap()
        .unwrap();

    let transfer_complete = TransferCompletionMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferCompletionMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&callback.provider_pid.unwrap()).unwrap(),
        consumer_pid: convert_uuid_to_uri(&callback.consumer_pid).unwrap(),
    };

    let url = format!(
        "http://{}/{}",
        get_provider_url().unwrap(),
        "transfers/completion"
    );

    let req = reqwest::Client::new().post(url).json(&transfer_complete).send().await;

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
            error: Some(TransferError::from_async(NotCheckedError { inner_error: e.into() }).await),
        }),
    }
}
