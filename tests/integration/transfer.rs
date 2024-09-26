use anyhow::anyhow;
use axum::body::Body;
use axum::http;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use rainbow::transfer::common::utils::convert_uri_to_uuid;
use rainbow::transfer::consumer::http::server::create_consumer_router;
use rainbow::transfer::consumer::lib::callbacks_controller::create_new_callback;
use rainbow::transfer::protocol::messages::{
    TransferMessageTypes, TransferProcessMessage, TransferRequestMessage, TransferStartMessage,
    TransferState,
};
use rainbow::transfer::provider::data::repo::update_transfer_process_by_provider_pid;
use rainbow::transfer::provider::http::server::create_provider_router;
use std::fs;
use tower::ServiceExt;
use tracing::{debug, error, info, trace};
use tracing_test::traced_test;
use uuid::Uuid;

fn get_json_file(path: &str) -> anyhow::Result<String> {
    let main_path = "./static/json-tests/";
    let file_url = format!("{}{}", main_path, path);
    let json_raw = fs::read_to_string(file_url)?;
    Ok(json_raw)
}

async fn extract_body<T>(body: Body) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let body = body.collect().await?.to_bytes();
    let body = serde_json::from_slice::<T>(&body)?;
    Ok(body)
}

#[traced_test]
#[tokio::test]
pub async fn transfer_all_provider() -> anyhow::Result<()> {
    let request_to_provider = create_provider_router().await;
    let request_to_consumer = create_consumer_router().await;

    // 1.
    // Hi, i'm a consumer and going to start the protocol
    // I'm going to do a HTTP_PUSH type
    let transfer_request_message = get_json_file("transfer-request.json")?;
    let mut data = serde_json::from_str::<TransferRequestMessage>(&transfer_request_message)?;
    let consumer_pid = Uuid::new_v4();
    let provider_pid: Uuid;
    data.consumer_pid = format!("urn:uuid:{}", consumer_pid.to_string());

    // Register callback in consumer
    let callback_id = create_new_callback()?;
    data.callback_address = callback_id.to_string();

    println!("{}", serde_json::to_string_pretty(&data)?);

    // Do request to provider from consumer
    let response = request_to_provider
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .header("Content-Type", "application/json")
                .uri("/transfers/request")
                .body(Body::from(serde_json::to_string(&data)?))
                .unwrap(),
        )
        .await?;

    println!("{:?}", response.status());

    // 2.
    // Hi, I'm the provider returning a TransferProcessMessage
    // body.provider_pid not none
    // body.consumer_pid == consumer_pid
    // body.state == TransferState::REQUESTED
    match extract_body::<TransferProcessMessage>(response.into_body()).await {
        Ok(body) => {
            println!("{}", serde_json::to_string_pretty(&body)?);
            assert_eq!(
                body._type,
                TransferMessageTypes::TransferProcessMessage.to_string()
            );
            assert_ne!(body.provider_pid.to_string(), "");
            assert_eq!(
                body.consumer_pid.to_string(),
                format!("urn:uuid:{}", consumer_pid.to_string())
            );
            assert_eq!(body.state.to_string(), TransferState::REQUESTED.to_string());
            provider_pid = convert_uri_to_uuid(&body.provider_pid)?;
        }
        Err(err) => {
            error!("ERROR: {:?}", err);
            return Err(anyhow!(err));
        }
    }

    // 3.
    // Hi, I'm the provider again.
    // I negotiated succesfully with the data-space the transfer
    update_transfer_process_by_provider_pid(&provider_pid, TransferState::STARTED)?;

    // So I'm sending dataAdress and send a TransferStartMessage to consumer
    let consumer_cb_uri = format!(
        "/{}/transfers/{}/start",
        callback_id.to_string(),
        consumer_pid.to_string()
    );
    let transfer_start_message = get_json_file("transfer-start.json")?;
    let mut data = serde_json::from_str::<TransferStartMessage>(&transfer_start_message)?;
    println!("{}", consumer_cb_uri);
    println!("{}", serde_json::to_string_pretty(&data)?);

    let response = request_to_consumer
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .header("Content-Type", "application/json")
                .uri(consumer_cb_uri)
                .body(Body::from(serde_json::to_string(&data)?))
                .unwrap(),
        )
        .await?;

    // 4.
    // Consumer again, sending back OK
    assert_eq!(response.status().to_string(), StatusCode::OK.to_string());

    // 5.
    // Provider data plane sends data to dataAddress

    Ok(())
}
