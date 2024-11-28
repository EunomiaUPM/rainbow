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

use anyhow::anyhow;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::{http, serve};
use rainbow_common::dcat_formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow_common::protocol::transfer::{
    DataAddress, TransferCompletionMessage, TransferMessageTypes, TransferProcessMessage,
    TransferRequestMessage, TransferStartMessage, TransferSuspensionMessage, TRANSFER_CONTEXT,
};
use rainbow_common::utils::convert_uuid_to_uri;
use rainbow_db::transfer_consumer::entities::transfer_callback;
use serde_json::{json, Value};
use std::io::BufRead;
use tracing::{debug, error, info, trace};
use tracing_subscriber::fmt::format;
use tracing_test::traced_test;
use uuid::Uuid;

#[path = "utils.rs"]
mod utils;

#[traced_test]
#[tokio::test]
pub async fn transfer_push_full_case() -> anyhow::Result<()> {
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
    ) = utils::setup_test_env_push(
        "http://localhost:1026/v2/subscriptions",
        serde_json::json!({
          "description": "$data_description",
          "subject": {
            "entities": [
              {
                "idPattern": ".*",
                "type": "AirQualityUnit"
              }
            ],
            "condition": {
              "attrs": ["CO"]
            }
          },
          "notification": {
            "http": {
              "url": "$data_url"
            },
            "attrs": ["CO"]
          },
        }),
    )
        .await?;

    // 1. Kickoff from client with DataAddress
    // 2. I create a TransferRequest
    let request_data = TransferRequestMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferRequestMessage.to_string(),
        consumer_pid: consumer_pid.clone(),
        agreement_id: convert_uuid_to_uri(&agreement_id)?,
        format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Push },
        callback_address: consumer_callback_address,
        data_address: Some(DataAddress {
            _type: "dspace:DataAddress".to_string(),
            endpoint_type: "HTTP".to_string(),
            endpoint: "http://localhost:1237/data-client".to_string(),
            endpoint_properties: vec![],
        }), // <- this data address should be created
    };
    println!("2.\n Creating TransferRequest: \n{:?}", request_data);

    // 3. Transfer request to provider
    let res =
        client.post("http://localhost:1234/transfers/request").json(&request_data).send().await?;

    let res_body = res.json::<TransferProcessMessage>().await?;
    let provider_pid_ = res_body.provider_pid.clone();
    println!("3.\n Provider says: \n{:?}", res_body);

    // 4. Transfer start is happening under the hood. check logs

    // 5. begin data transfer

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    utils::cleanup_test_env(provider_server, consumer_server).await?;
    Ok(())
}
