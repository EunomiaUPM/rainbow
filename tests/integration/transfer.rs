#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

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
use rainbow::transfer::common::utils::{convert_uri_to_uuid, convert_uuid_to_uri};
use rainbow::transfer::consumer::data::repo::{get_callback_by_consumer_id, get_callback_by_id};
use rainbow::transfer::consumer::http::server::{
    create_consumer_router, start_consumer_server, start_consumer_server_with_listener,
};
use rainbow::transfer::consumer::lib::callbacks_controller::create_new_callback;
use rainbow::transfer::protocol::formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow::transfer::protocol::messages::{
    DataAddress, TransferMessageTypes, TransferProcessMessage, TransferRequestMessage,
    TransferStartMessage, TransferState, TransferSuspensionMessage, TRANSFER_CONTEXT,
};
use rainbow::transfer::provider::data::repo::{
    get_transfer_process_by_provider_pid, update_transfer_process_by_provider_pid,
};
use rainbow::transfer::provider::http::server::{
    create_provider_router, start_provider_server, start_provider_server_with_listener,
};
use rainbow::transfer::provider::lib::control_plane::get_transfer_requests_by_provider;
use rainbow::transfer::provider::lib::data_plane::resolve_endpoint_from_agreement;
use serde_json::Value;
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

    // LOAD AGREEMENTS AND ENDPOINTS
    let setup = setup_env().await?;
    let agreements = setup.1.clone();
    let datasets = setup.0.clone();
    let client = reqwest::Client::new();

    // CREATE SERVERS
    tokio::spawn(async move {
        let request_to_provider = create_provider_router().await;
        let tcp_provider = start_provider_server_with_listener(&None, &None)
            .await
            .expect("provider failed");
        serve(tcp_provider, request_to_provider)
            .await
            .expect("server failed");
    });
    tokio::spawn(async move {
        let request_to_consumer = create_consumer_router().await;
        let tcp_consumer = start_consumer_server_with_listener(&None, &None)
            .await
            .expect("consumer failed");
        serve(tcp_consumer, request_to_consumer)
            .await
            .expect("server failed");
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    //============================================//
    // TRANSFER REQUEST STAGE
    //============================================//

    // 1.
    // Hi, i'm a consumer and going to start the protocol
    // First i create a callback
    let callback_id = create_new_callback()?;
    let callback_address = format!("http://localhost:1235/{}", callback_id.to_string());
    println!("1.1 Creating Callback Address: \n{}", callback_address);

    // TODO - create kickoff function
    // After Create TransferRequestMessage
    let transfer_request_message = get_json_file("transfer-request.json")?;
    let mut request_data =
        serde_json::from_str::<TransferRequestMessage>(&transfer_request_message)?;
    let consumer_pid = Uuid::new_v4();
    request_data.format = DctFormats {
        protocol: FormatProtocol::Http,
        action: FormatAction::Pull,
    };
    request_data.consumer_pid = convert_uuid_to_uri(&consumer_pid)?;
    request_data.agreement_id = convert_uuid_to_uri(&agreements.get(0).unwrap().agreement_id)?;
    request_data.callback_address = callback_address;
    request_data.data_address = None;
    //
    println!(
        "1.2 Create TransferRequest \n{}",
        serde_json::to_string_pretty(&request_data)?
    );

    // And I send it to Provider
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&request_data)
        .send()
        .await?;
    let res_body = &res.json::<TransferProcessMessage>().await?;
    let provider_pid = res_body.provider_pid.clone();

    println!(
        "1.3 Send data to provider and status is: \n{}",
        serde_json::to_string_pretty(&res_body)?
    );

    //============================================//
    // TRANSFER START STAGE IS HAPPENING UNDER THE HOOD (async) - Check the logs
    //============================================//
    // Assert transfer start
    // Assert endpoint

    //============================================//
    // BEGIN DATA TRANSFER!!!
    //============================================//
    // Give some time data plane to be provided
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Consumer queries wants to know where is the endpoint
    // This info is consumer callback database
    // let agreement = convert_uuid_to_uri(&agreements.get(0).unwrap().agreement_id)?;
    // let endpoint = resolve_endpoint_from_agreement(agreement.parse()?).await?;
    let callback = get_callback_by_consumer_id(consumer_pid)?.unwrap();
    let callback_data_address =
        serde_json::from_value::<DataAddress>(Value::from(callback.data_address))?;
    let endpoint = callback_data_address.endpoint;

    let data_plane_res = client.get(&endpoint).send().await?;

    println!("{:?}", &data_plane_res.status());
    println!("{:?}", &data_plane_res.bytes().await?);
    // ASSERT TRANSFER!!

    //============================================//
    // END DATA TRANSFER!!!
    //============================================//

    //============================================//
    // TRANSFER SUSPENSION STAGE
    //============================================//
    // Consumer want's to suspend temporarily the transfer
    let suspension_data = TransferSuspensionMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
        provider_pid,
        consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
        code: "A".to_string(),           // TODO DEFINE ALL THIS!!!
        reason: vec!["bla".to_string()], // TODO DEFINE REASONS
    };
    let res = client
        .post("http://localhost:1234/transfers/suspension")
        .header("content-type", "application/json")
        .json(&suspension_data)
        .send()
        .await?;

    println!("{:?}", &res.status());
    // ASSERT

    //============================================//
    // BEGIN DATA TRANSFER!!! should fail
    //============================================//
    // Give some time data plane to be unprovided
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let data_plane_res = client.get(&endpoint).send().await?;

    println!("{:?}", &data_plane_res.status()); // <---- revisar esto bien, que no cuadra...
                                                // ASSERT TRANSFER!!
                                                // ASSERT SHOULD FAIL

    //============================================//
    // END DATA TRANSFER!!!
    //============================================//

    //============================================//
    // CLEANUP
    //============================================//
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    // cleanup_env(setup).await?;
    Ok(())
}
