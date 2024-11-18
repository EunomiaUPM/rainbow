use crate::common::err::TransferErrorType::{CallbackClientError, ProviderAndConsumerNotMatchingError, ProviderNotReachableError};
use crate::common::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::consumer::data::entities::transfer_callback;
// use crate::setup::config::{get_provider_url, GLOBAL_CONFIG};
use rainbow_common::config::database::get_db_connection;
use anyhow::bail;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use rainbow_common::config::config::get_provider_url;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route("/:callback_id/data/:consumer_id", get(handle_data_proxy))
}


// https://hackernoon.com/effective-proxy-server-design-and-implementation
// async fn handle_data_proxy(
//     // TODO refactor this
//     Path((callback_id, consumer_id)): Path<(Uuid, Uuid)>,
// ) -> impl IntoResponse {
//     info!("Forwarding data from consumer data plane proxy");
//     info!(
//         "GET /{}/data/{}",
//         callback_id.to_string(),
//         consumer_id.to_string()
//     );
//
//     let callback = TRANSFER_CONSUMER_REPO.get_callback_by_consumer_id(consumer_id);
//     if callback.is_err() {
//         return CallbackClientError.into_response();
//     }
//     let callback = callback.unwrap();
//     if callback.is_none() {
//         return CallbackClientError.into_response();
//     }
//     let callback = callback.unwrap();
//     let data_address = callback.data_address.unwrap();
//     let forwarding_endpoint = data_address.get("dspace:endpoint").and_then(|v| v.as_str());
//
//     let res = DATA_PLANE_HTTP_CLIENT
//         .get(forwarding_endpoint.unwrap())
//         .send()
//         .await;
//
//     if res.is_err() {
//         println!("{:#?}", res.err().unwrap());
//         return ProviderNotReachableError.into_response();
//     }
//     let res = res.unwrap();
//
//     // TODO refactor this with negotiation on init between peers
//     // url should be from peer
//     let provider_url = res.url();
//     let provider_host = provider_url.host_str().unwrap();
//     let provider_port = provider_url.port().unwrap();
//     let provider_path = provider_url.path();
//     let provider_format = format!("http://{}:{}{}", provider_host, provider_port, provider_path);
//     let provider_check = format!("{}/data", get_provider_url().unwrap());
//
//     if provider_format.contains(provider_check.as_str()) == false {
//         return ProviderAndConsumerNotMatchingError.into_response();
//     }
//
//     // Forward request
//     (res.status(), res.text().await.unwrap()).into_response()
// }

async fn handle_data_proxy(
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
        .filter(transfer_callback::Column::ConsumerPid.eq(consumer_id))
        .one(db_connection)
        .await {
        Ok(Some(callback)) => callback,
        Ok(None) | Err(_) => return CallbackClientError.into_response(),
    };

    // let callback = match TRANSFER_CONSUMER_REPO.get_callback_by_consumer_id(consumer_id) {
    //     Ok(Some(callback)) => callback,
    //     Ok(None) | Err(_) => return CallbackClientError.into_response(),
    // };

    let data_address = match callback.data_address {
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
    let provider_url = res.url();
    let provider_host = provider_url.host_str().unwrap_or_default();
    let provider_port = provider_url.port().unwrap_or_default();
    let provider_path = provider_url.path();
    let provider_format = format!("http://{}:{}{}", provider_host, provider_port, provider_path);
    let provider_check = format!("{}/data", get_provider_url().unwrap_or_default());

    if !provider_format.contains(&provider_check) {
        return ProviderAndConsumerNotMatchingError.into_response();
    }

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
