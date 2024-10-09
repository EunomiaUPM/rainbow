use anyhow::anyhow;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::{http, serve};
use http_body_util::BodyExt;
use rainbow::fake_catalog::data::models::DatasetsCatalogModel;
use rainbow::fake_catalog::data::repo::delete_dataset_repo;
use rainbow::fake_catalog::lib::{create_dataset, delete_dataset};
use rainbow::fake_contracts::data::models::ContractAgreementsModel;
use rainbow::fake_contracts::data::repo::delete_agreement_repo;
use rainbow::fake_contracts::lib::create_agreement;
use rainbow::transfer::common::utils::convert_uri_to_uuid;
use rainbow::transfer::consumer::http::server::{
    create_consumer_router, start_consumer_server, start_consumer_server_with_listener,
};
use rainbow::transfer::consumer::lib::callbacks_controller::create_new_callback;
use rainbow::transfer::protocol::messages::{
    TransferMessageTypes, TransferProcessMessage, TransferRequestMessage, TransferStartMessage,
    TransferState,
};
use rainbow::transfer::provider::data::repo::{
    get_transfer_process_by_provider_pid, update_transfer_process_by_provider_pid,
};
use rainbow::transfer::provider::http::server::{
    create_provider_router, start_provider_server, start_provider_server_with_listener,
};
use rainbow::transfer::provider::lib::control_plane::get_transfer_requests_by_provider;
use std::fs;
use tower::ServiceExt;
use tracing::{debug, error, info, trace};
use tracing_subscriber::fmt::format;
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

async fn setup_env() -> anyhow::Result<(Vec<DatasetsCatalogModel>, Vec<ContractAgreementsModel>)> {
    let fake_parquet_url = "http://localhost:1236/data-space";
    let fake_parquet_files = vec!["/sample1.parquet", "/sample2.parquet", "/sample3.parquet"];
    let mut fake_datasets: Vec<DatasetsCatalogModel> = vec![];
    let mut agreements: Vec<ContractAgreementsModel> = vec![];

    let fake_parquet_file = fake_parquet_files
        .iter()
        .map(|f| format!("{}{}", fake_parquet_url, f))
        .collect::<Vec<_>>();

    for endpoint in fake_parquet_file {
        let ds = create_dataset(endpoint)?;
        let agreement = create_agreement(ds.dataset_id.clone())?;
        fake_datasets.push(ds);
        agreements.push(agreement);
    }

    Ok((fake_datasets, agreements))
}

async fn cleanup_env(
    setup: (Vec<DatasetsCatalogModel>, Vec<ContractAgreementsModel>),
) -> anyhow::Result<()> {
    let (fake_datasets, agreements) = setup;
    for ds in fake_datasets {
        delete_dataset(ds.dataset_id)?;
    }
    for agg in agreements {
        delete_agreement_repo(agg.agreement_id)?;
    }
    Ok(())
}

#[traced_test]
#[tokio::test]
pub async fn transfer_all_provider() -> anyhow::Result<()> {
    //============================================//
    // ON INIT (LOAD AGREEMENTS AND START SERVERS)
    //============================================//

    let setup = setup_env().await?;
    let agreements = setup.1.clone();
    let datasets = setup.0.clone();

    //

    tokio::spawn(async move {
        let request_to_provider = create_provider_router().await;
        let tcp_provider = start_provider_server_with_listener(&None, &None).await.expect("provider failed");
        serve(tcp_provider, request_to_provider).await.expect("server failed");
    });
    tokio::spawn(async move {
        let request_to_consumer = create_consumer_router().await;
        let tcp_consumer = start_consumer_server_with_listener(&None, &None).await.expect("consumer failed");
        serve(tcp_consumer, request_to_consumer).await.expect("server failed");
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    //============================================//
    // TRANSFER REQUEST STAGE
    //============================================//

    // 1.
    // Hi, i'm a consumer and going to start the protocol
    // I'm going to do a HTTP_PUSH type
    let transfer_request_message = get_json_file("transfer-request.json")?;
    let mut data = serde_json::from_str::<TransferRequestMessage>(&transfer_request_message)?;
    let consumer_pid = Uuid::new_v4();
    let provider_pid: Uuid;
    data.consumer_pid = format!("urn:uuid:{}", consumer_pid.to_string());
    data.agreement_id = agreements.get(0).unwrap().agreement_id.to_string();

    // Register callback in consumer
    let callback_id = create_new_callback()?;
    data.callback_address = callback_id.to_string();

    println!("{}", serde_json::to_string_pretty(&data)?);

    let body = reqwest::get("http://localhost:1234/transfers/1").await?;

    println!("body = {body:?}");


    //
    // // Do request to provider from consumer
    // let response = request_to_provider
    //     .oneshot(
    //         Request::builder()
    //             .method(http::Method::POST)
    //             .header("Content-Type", "application/json")
    //             .uri("/transfers/request")
    //             .body(Body::from(serde_json::to_string(&data)?))
    //             .unwrap(),
    //     )
    //     .await?;
    //
    // println!("{:?}", response.status());
    //
    // // 2.
    // // Hi, I'm the provider returning a TransferProcessMessage
    // // body.provider_pid not none
    // // body.consumer_pid == consumer_pid
    // // body.state == TransferState::REQUESTED
    // match extract_body::<TransferProcessMessage>(response.into_body()).await {
    //     Ok(body) => {
    //         println!("{}", serde_json::to_string_pretty(&body)?);
    //         assert_eq!(
    //             body._type,
    //             TransferMessageTypes::TransferProcessMessage.to_string()
    //         );
    //         assert_ne!(body.provider_pid.to_string(), "");
    //         assert_eq!(
    //             body.consumer_pid.to_string(),
    //             format!("urn:uuid:{}", consumer_pid.to_string())
    //         );
    //         assert_eq!(body.state.to_string(), TransferState::REQUESTED.to_string());
    //         provider_pid = convert_uri_to_uuid(&body.provider_pid)?;
    //     }
    //     Err(err) => {
    //         error!("ERROR: {:?}", err);
    //         return Err(anyhow!(err));
    //     }
    // }
    //
    // //============================================//
    // // TRANSFER START STAGE
    // //============================================//
    //
    // // 3.
    // // Hi, I'm the provider again.
    // // I negotiated succesfully with the data-space the transfer
    // // update_transfer_process_by_provider_pid(&provider_pid, TransferState::STARTED)?;
    //
    // // So I'm sending dataAdress and send a TransferStartMessage to consumer
    // let consumer_cb_uri = format!(
    //     "/{}/transfers/{}/start",
    //     callback_id.to_string(),
    //     consumer_pid.to_string()
    // );
    // let transfer_start_message = get_json_file("transfer-start.json")?;
    // let mut data = serde_json::from_str::<TransferStartMessage>(&transfer_start_message)?;
    // println!("{}", consumer_cb_uri);
    // println!("{}", serde_json::to_string_pretty(&data)?);
    //
    // let response = request_to_consumer
    //     .oneshot(
    //         Request::builder()
    //             .method(http::Method::POST)
    //             .header("Content-Type", "application/json")
    //             .uri(consumer_cb_uri)
    //             .body(Body::from(serde_json::to_string(&data)?))
    //             .unwrap(),
    //     )
    //     .await?;
    //
    // // 4.
    // // Consumer again, sending back OK
    // assert_eq!(response.status().to_string(), StatusCode::OK.to_string());
    //
    // //============================================//
    // // DATA PLANE TRANSFER
    // //============================================//
    //
    // // 5.
    // // Provider resolves endpoint from Contract and Catalog (by now is all faked)
    // get_transfer_process_by_provider_pid(provider_pid)?;

    //============================================//
    // CLEANUP
    //============================================//
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
    cleanup_env(setup).await?;
    Ok(())
}
