use crate::setup::config::{get_provider_url, GLOBAL_CONFIG};
use crate::transfer::common::err::TransferErrorType::{CallbackClientError, ProviderAndConsumerNotMatchingError, ProviderNotReachableError};
use crate::transfer::common::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::transfer::consumer::data::repo::TRANSFER_CONSUMER_REPO;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route("/:callback_id/data/:consumer_id", get(handle_data_proxy))
}


// https://hackernoon.com/effective-proxy-server-design-and-implementation
async fn handle_data_proxy(
    // TODO refactor this
    Path((callback_id, consumer_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    info!("Forwarding data from consumer data plane proxy");
    info!(
        "GET /{}/data/{}",
        callback_id.to_string(),
        consumer_id.to_string()
    );

    let callback = TRANSFER_CONSUMER_REPO.get_callback_by_consumer_id(consumer_id);
    if callback.is_err() {
        return CallbackClientError.into_response();
    }
    let callback = callback.unwrap();
    if callback.is_none() {
        return CallbackClientError.into_response();
    }
    let callback = callback.unwrap();
    let data_address = callback.data_address.unwrap();
    let forwarding_endpoint = data_address.get("dspace:endpoint").and_then(|v| v.as_str());

    let res = DATA_PLANE_HTTP_CLIENT
        .get(forwarding_endpoint.unwrap())
        .send()
        .await;

    if res.is_err() {
        println!("{:#?}", res.err().unwrap());
        return ProviderNotReachableError.into_response();
    }
    let res = res.unwrap();

    // TODO refactor this with negotiation on init between peers 
    // url should be from peer
    let provider_url = res.url();
    let provider_host = provider_url.host_str().unwrap();
    let provider_port = provider_url.port().unwrap();
    let provider_path = provider_url.path();
    let provider_format = format!("http://{}:{}{}", provider_host, provider_port, provider_path);
    let provider_check = format!("{}/data", get_provider_url().unwrap());

    if provider_format.contains(provider_check.as_str()) == false {
        return ProviderAndConsumerNotMatchingError.into_response();
    }

    // Forward request
    (res.status(), res.text().await.unwrap()).into_response()
}
