use crate::common::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::consumer::data::entities::transfer_callback;
use anyhow::bail;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use rainbow_common::config::config::get_provider_url;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::err::transfer_err::TransferErrorType::{CallbackClientError, ProviderAndConsumerNotMatchingError, ProviderNotReachableError};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route("/:callback_id/data/pull/:consumer_id", get(handle_data_pull_proxy))
        .route("/:callback_id/data/push/:consumer_id", post(handle_data_push_proxy))
        .route("/:callback_id/data/push/:consumer_id", put(handle_data_push_proxy))
}

async fn handle_data_push_proxy(
    // TODO refactor this
    Path((callback_id, consumer_id)): Path<(Uuid, Uuid)>,
    Json(input): Json<serde_json::Value>,
) -> impl IntoResponse {
    info!("Forwarding data from consumer data plane proxy");
    info!(
        "POST /{}/data/{}",
        callback_id,
        consumer_id
    );

    // Fetch the callback information
    let db_connection = get_db_connection().await;
    let callback = match transfer_callback::Entity::find()
        .filter(transfer_callback::Column::Id.eq(callback_id))
        .one(db_connection)
        .await {
        Ok(Some(callback)) => callback,
        Ok(None) | Err(_) => return CallbackClientError.into_response(),
    };

    let data_address = match callback.next_hop_address {
        Some(addr) => addr,
        None => return CallbackClientError.into_response(),
    };

    let forwarding_endpoint = match data_address.get("dspace:endpoint").and_then(|v| v.as_str()) {
        Some(endpoint) => endpoint,
        None => return CallbackClientError.into_response(),
    };

    // Send the request to the upstream server
    let res = match DATA_PLANE_HTTP_CLIENT
        .post(forwarding_endpoint)
        .json(&input)
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            error!("Error sending request: {:?}", err);
            return ProviderNotReachableError.into_response();
        }
    };

    let status = res.status();
    let headers = res.headers().clone();
    let body_bytes = match res.bytes().await {
        Ok(bytes) => bytes,
        Err(err) => {
            error!("Error reading response body: {:?}", err);
            return (
                StatusCode::BAD_GATEWAY,
                "Error reading upstream response".to_string(),
            )
                .into_response();
        }
    };

    // Build a new response including the headers
    let mut response = Response::builder()
        .status(status)
        .body(Body::from(body_bytes))
        .unwrap_or_else(|err| {
            error!("Error building response: {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal server error"))
                .unwrap()
        });

    // Copy headers from the upstream response
    *response.headers_mut() = headers;

    response.into_response()
}

// https://hackernoon.com/effective-proxy-server-design-and-implementation
async fn handle_data_pull_proxy(
    // TODO refactor this
    Path((callback_id, consumer_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    info!("Forwarding data from consumer data plane proxy");
    info!(
        "GET /{}/data/{}",
        callback_id,
        consumer_id
    );

    // Fetch the callback information
    let db_connection = get_db_connection().await;
    let callback = match transfer_callback::Entity::find()
        .filter(transfer_callback::Column::Id.eq(callback_id))
        .one(db_connection)
        .await {
        Ok(Some(callback)) => callback,
        Ok(None) | Err(_) => return CallbackClientError.into_response(),
    };

    let data_address = match callback.next_hop_address {
        Some(addr) => addr,
        None => return CallbackClientError.into_response(),
    };

    let forwarding_endpoint = match data_address.get("dspace:endpoint").and_then(|v| v.as_str()) {
        Some(endpoint) => endpoint,
        None => return CallbackClientError.into_response(),
    };

    // Send the request to the upstream server
    let res = match DATA_PLANE_HTTP_CLIENT
        .get(forwarding_endpoint)
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            error!("Error sending request: {:?}", err);
            return ProviderNotReachableError.into_response();
        }
    };

    // Verify the provider URL
    // let provider_url = res.url();
    // let provider_host = provider_url.host_str().unwrap_or_default();
    // let provider_port = provider_url.port().unwrap_or_default();
    // let provider_path = provider_url.path();
    // let provider_format = format!("http://{}:{}{}", provider_host, provider_port, provider_path);
    // let provider_check = format!("{}/data", get_provider_url().unwrap_or_default());
    // 
    // println!("{}", provider_format);
    // println!("{}", provider_check);
    // 
    // 
    // if !provider_format.contains(&provider_check) {
    //     return ProviderAndConsumerNotMatchingError.into_response();
    // }

    // Extract status, headers, and body
    let status = res.status();
    let headers = res.headers().clone();
    let body_bytes = match res.bytes().await {
        Ok(bytes) => bytes,
        Err(err) => {
            error!("Error reading response body: {:?}", err);
            return (
                StatusCode::BAD_GATEWAY,
                "Error reading upstream response".to_string(),
            )
                .into_response();
        }
    };

    // Build a new response including the headers
    let mut response = Response::builder()
        .status(status)
        .body(Body::from(body_bytes))
        .unwrap_or_else(|err| {
            error!("Error building response: {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal server error"))
                .unwrap()
        });

    // Copy headers from the upstream response
    *response.headers_mut() = headers;

    response.into_response()
}
