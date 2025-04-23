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

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// mod utils;

use anyhow::anyhow;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::{http, serve};
use rainbow_common::dcat_formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow_common::protocol::transfer::{
    DataAddress, TransferCompletionMessage, TransferMessageTypes, TransferProcessMessage,
    TransferRequestMessage, TransferStartMessage, TransferSuspensionMessage,
};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::transfer_consumer::entities::transfer_callback;
use rainbow_db::transfer_provider::entities::transfer_process;
use rainbow_transfer::consumer::lib::api::{
    CompleteTransferRequest, RequestTransferRequest, RequestTransferResponse,
    RestartTransferRequest, SuspendTransferRequest,
};
use serde_json::{json, Value};
use std::io::BufRead;
use tracing::{debug, error, info, trace};
use tracing_subscriber::fmt::format;
use tracing_test::traced_test;
use urn::Urn;
use uuid::Uuid;

#[path = "utils.rs"]
mod utils;

#[traced_test]
#[tokio::test]
pub async fn transfer_pull_full_case() -> anyhow::Result<()> {
    let pull_url = "http://localhost:1236/data-space/sample1.parquet";
    let (
        mut provider_server,
        mut consumer_server,
        client,
        catalog_id,
        dataservice_id,
        agreement_id,
        consumer_pid,
        consumer_callback_address,
        callback_id,
    ) = utils::setup_test_env(pull_url).await?;


    //============================================//
    // TRANSFER REQUEST STAGE
    //============================================//

    // Consumer creates TransferRequestMessage
    let request_data = TransferRequestMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferRequestMessage.to_string(),
        consumer_pid: consumer_pid.to_string(),
        agreement_id: agreement_id.to_string(),
        format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
        callback_address: consumer_callback_address,
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
    let res_body = res.json::<TransferProcessMessage>().await?;
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

    // Client queries wants to know where is the endpoint
    // This info is in transfers
    let res = client
        .get(format!(
            "http://localhost:1234/api/v1/transfers/{}",
            provider_pid_
        ))
        .header("content-type", "application/json")
        .send()
        .await?;
    let res_body = res.json::<transfer_process::Model>().await?;

    println!("{}", serde_json::to_string_pretty(&res_body)?);

    // TODO address should go with data_plane_id and not with provider in address
    // or totally different endpoint...
    let data_plane_id = res_body.data_plane_id.clone().unwrap();
    let endpoint = format!("http://localhost:1234/data/pull/{}", provider_pid_);
    println!("1.4 Endpoint: \n{:?}", endpoint);

    let data_plane_res = client.get(endpoint.clone()).send().await?;
    println!("{:?}", &data_plane_res.status());
    println!("{:?}", &data_plane_res.bytes().await?);

    // ASSERT TRANSFER!!
    //============================================//
    // END DATA TRANSFER!!!
    //============================================//
    //
    //============================================//
    // TRANSFER SUSPENSION STAGE
    //============================================//
    // Consumer want's to suspend temporarily the transfer
    let suspension_data = TransferSuspensionMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
        provider_pid: provider_pid_.clone(),
        consumer_pid: consumer_pid.to_string(),
        code: "A".to_string(),           // TODO DEFINE ALL THIS!!!
        reason: vec!["bla".to_string()], // TODO DEFINE REASONS
    };
    let res = client
        .post(format!("http://localhost:1234/transfers/{}/suspension", provider_pid_.clone()))
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
        consumer_pid: consumer_pid.to_string(),
        data_address: None,
    };
    let res = client
        .post(format!("http://localhost:1234/transfers/{}/start", provider_pid_.clone()))
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
        consumer_pid: consumer_pid.to_string(),
    };
    let res = client
        .post(format!("http://localhost:1234/transfers/{}/completion", provider_pid_.clone()))
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

    utils::cleanup_test_env(provider_server, consumer_server).await?;
    Ok(())
}
