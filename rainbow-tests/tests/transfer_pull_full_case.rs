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
    TransferRequestMessage, TransferStartMessage, TransferSuspensionMessage, TRANSFER_CONTEXT,
};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::transfer_consumer::entities::transfer_callback;
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
    let request_payload = RequestTransferRequest {
        agreement_id: agreement_id.to_string(),
        format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
        callback_address: consumer_callback_address,
        data_address: None,
        consumer_pid: consumer_pid.to_string(),
    };
    let res = client
        .post("http://localhost:1235/api/v1/request-transfer")
        .header("content-type", "application/json")
        .json(&request_payload)
        .send()
        .await?;

    let res_body = res.json::<RequestTransferResponse>().await?;
    println!(
        "RequestTransferResponse \n{}",
        serde_json::to_string_pretty(&res_body)?
    );
    let provider_pid = get_urn_from_string(&res_body.transfer_process.unwrap().provider_pid)?;

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
    let res_body = res.json::<transfer_callback::Model>().await?;

    println!(
        "1.4 Consumer queries wants to know where is the endpoint: \n{}",
        serde_json::to_string_pretty(&res_body)?
    );

    let callback_id = res_body.id.to_string();
    let consumer_id = res_body.consumer_pid.to_string();
    let endpoint = format!(
        "http://localhost:1235/{}/data/pull/{}",
        callback_id, consumer_id
    );
    println!("1.4 Endpoint: \n{:?}", endpoint);

    let data_plane_res = client.get(endpoint.clone()).send().await?;
    println!("{:?}", &data_plane_res.status());
    println!("{:?}", &data_plane_res.bytes().await?);

    //============================================//
    // TRANSFER SUSPENSION STAGE
    //============================================//
    // Consumer want's to suspend temporarily the transfer
    let suspension_data = SuspendTransferRequest { consumer_pid: consumer_pid.to_string() };
    let res = client
        .post("http://localhost:1235/api/v1/suspend-transfer")
        .header("content-type", "application/json")
        .json(&suspension_data)
        .send()
        .await?;
    let status = res.status();
    let res_body = res.json::<RequestTransferResponse>().await?;

    println!(
        "RequestTransferResponse \n{}",
        serde_json::to_string_pretty(&res_body)?
    );
    println!("{:?}", status);

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
    let restart_data = RestartTransferRequest { consumer_pid: consumer_pid.to_string() };
    let res = client
        .post("http://localhost:1235/api/v1/restart-transfer")
        .header("content-type", "application/json")
        .json(&restart_data)
        .send()
        .await?;

    let status = res.status();
    let res_body = res.json::<RequestTransferResponse>().await?;

    println!(
        "RequestTransferResponse \n{}",
        serde_json::to_string_pretty(&res_body)?
    );
    println!("{:?}", status);

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
    let complete_data = CompleteTransferRequest { consumer_pid: consumer_pid.to_string() };
    let res = client
        .post("http://localhost:1235/api/v1/complete-transfer")
        .header("content-type", "application/json")
        .json(&complete_data)
        .send()
        .await?;

    let status = res.status();
    let res_body = res.json::<RequestTransferResponse>().await?;

    println!(
        "RequestTransferResponse \n{}",
        serde_json::to_string_pretty(&res_body)?
    );
    println!("{:?}", status);

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
