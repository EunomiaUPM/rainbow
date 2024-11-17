#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod utils;

use anyhow::anyhow;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::{http, serve};
use rainbow_core::transfer::common::utils::convert_uuid_to_uri;
use rainbow_core::transfer::consumer::data::models::TransferCallbacksModel;
use rainbow_core::transfer::protocol::formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow_core::transfer::protocol::messages::{DataAddress, TransferCompletionMessage, TransferMessageTypes, TransferProcessMessage, TransferRequestMessage, TransferStartMessage, TransferSuspensionMessage, TRANSFER_CONTEXT};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command};
use tracing::{debug, error, info, trace};
use tracing_subscriber::fmt::format;
use tracing_test::traced_test;
use utils::{cleanup_test_env, load_env_file, setup_agreements_and_datasets_pull, setup_test_env};
use uuid::Uuid;

#[traced_test]
#[tokio::test]
pub async fn transfer_pull_full_case() -> anyhow::Result<()> {
    let (
        mut provider_server,
        mut consumer_server,
        client,
        agreements,
        _datasets,
        callback_address,
        consumer_pid,
        callback_id
    ) = setup_test_env().await?;

    //============================================//
    // TRANSFER REQUEST STAGE
    //============================================//

    // After Create TransferRequestMessage
    let request_data = TransferRequestMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferRequestMessage.to_string(),
        consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
        agreement_id: convert_uuid_to_uri(&agreements.get(0).unwrap().agreement_id)?,
        format: DctFormats {
            protocol: FormatProtocol::Http,
            action: FormatAction::Pull,
        },
        callback_address,
        data_address: None,
    };
    
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
    let provider_pid_ = res_body.provider_pid.clone();

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
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Consumer queries wants to know where is the endpoint
    // This info is in consumer callback database
    let res = client
        .get(format!(
            "http://localhost:1235/api/v1/callbacks/{}",
            callback_id
        ))
        .header("content-type", "application/json")
        .send()
        .await?;
    let res_body = res.json::<TransferCallbacksModel>().await.unwrap();
    println!("{:#?}", res_body);

    let callback_id = res_body.id.to_string();
    let consumer_id = res_body.consumer_pid.unwrap().to_string();
    let endpoint = format!("http://localhost:1235/{}/data/{}", callback_id, consumer_id);

    let data_plane_res = client.get(endpoint.clone()).send().await?;
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
        provider_pid: provider_pid_.clone(),
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
    let data_plane_res = client.get(endpoint.clone()).send().await?;
    println!("{:?}", &data_plane_res.status());
    // ASSERT SHOULD FAIL
    //============================================//
    // END DATA TRANSFER!!!
    //============================================//

    //============================================//
    // TRANSFER RESTART STAGE
    //============================================//
    // Consumer wants to restart
    // This info is in consumer callback database
    // Recalls endpoint
    let restart_data = TransferStartMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferStartMessage.to_string(),
        provider_pid: provider_pid_.clone(),
        consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
        data_address: None,
    };
    let res = client
        .post("http://localhost:1234/transfers/start")
        .header("content-type", "application/json")
        .json(&restart_data)
        .send()
        .await?;

    println!("{:?}", &res.status());

    //============================================//
    // BEGIN DATA TRANSFER!!! should work
    //============================================//
    // Give some time data plane to be provided
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let data_plane_res = client.get(endpoint.clone()).send().await?;
    println!("{:?}", &data_plane_res.status());
    println!("{:?}", &data_plane_res.bytes().await?);
    //============================================//
    // END DATA TRANSFER!!!
    //============================================//

    //============================================//
    // TRANSFER COMPLETION STAGE
    //============================================//
    // Consumer wants to complete
    let complete_data = TransferCompletionMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferCompletionMessage.to_string(),
        provider_pid: provider_pid_.clone(),
        consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
    };
    let res = client
        .post("http://localhost:1234/transfers/completion")
        .header("content-type", "application/json")
        .json(&complete_data)
        .send()
        .await?;

    println!("{:?}", &res.status());

    //============================================//
    // BEGIN DATA TRANSFER!!! shouldn't work
    //============================================//
    // Give some time data plane to be provided
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let data_plane_res = client.get(endpoint.clone()).send().await?;
    println!("{:?}", &data_plane_res.status());
    //============================================//
    // END DATA TRANSFER!!!
    //============================================//

    cleanup_test_env(provider_server, consumer_server).await
}
