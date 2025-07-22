/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use clap::builder::TypedValueParser;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_provider::entities::agreement;
use rainbow_transfer::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::DSRPCTransferConsumerRequestResponse;
use serde_json::json;
use std::collections::HashMap;
use std::process::{Child, Command};
use urn::Urn;

pub async fn setup_test_env(
    url_to_load: &str,
) -> anyhow::Result<(
    Child,
    Child,
    reqwest::Client,
    Urn,
    Urn,
    Urn,
    Urn,
    String,
    Urn,
)> {
    let cwd = "./../rainbow-core";
    let provider_envs = load_env_file(".env.provider.template");
    let mut provider_server = Command::new("cargo")
        .current_dir(cwd)
        .env_clear()
        .envs(&provider_envs)
        .env("TEST", "true")
        .args(&["run", "--", "provider", "start"])
        .spawn()
        .expect("Failed to start provider server");

    let consumer_envs = load_env_file(".env.consumer.template");
    let mut consumer_server = Command::new("cargo")
        .current_dir(cwd)
        .env_clear()
        .envs(&consumer_envs)
        .env("TEST", "true")
        .args(&["run", "--", "consumer", "start"])
        .spawn()
        .expect("Failed to start consumer server");

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // ====================================
    //  LOAD DATASERVICES AND AGREEEMENTS IN PROVIDER
    // ====================================
    let client = reqwest::Client::new();

    // catalog
    let res = client
        .post("http://localhost:1234/api/v1/catalogs")
        .json(&json!({
            "foaf:homepage": "homepage"
        }))
        .send()
        .await?;
    let res_body = res.json::<Catalog>().await?;
    let catalog_id = get_urn_from_string(&res_body.id)?;
    println!("\nCatalog Id: {:#?}\n\n", catalog_id.to_string());

    // dataservice
    let res = client
        .post(format!(
            "http://localhost:1234/api/v1/catalogs/{}/data-services",
            catalog_id.to_string()
        ))
        .json(&json!({
            "dcat:endpointURL": url_to_load
        }))
        .send()
        .await?;
    let res_body = res.json::<DataService>().await?;
    let dataservice_id = get_urn_from_string(&res_body.id)?;
    println!("\nDataservice Id: {:#?}\n\n", dataservice_id.to_string());

    // agreement
    let res = client
        .post("http://localhost:1234/api/v1/agreements")
        .json(&json!({
            "dataServiceId": dataservice_id
        }))
        .send()
        .await?;
    let res_body = res.json::<agreement::Model>().await?;
    let agreement_id = get_urn_from_string(&res_body.agreement_id)?;
    println!("\nAgreement Id: {:#?}\n\n", agreement_id.to_string());


    // ====================================
    //  CREATE CALLBACK IN CONSUMER
    // ====================================
    let res = client.post("http://localhost:1235/api/v1/transfers/rpc/setup-request").send().await?;
    let res_body = res.json::<DSRPCTransferConsumerRequestResponse>().await?;
    let consumer_pid = res_body.consumer_pid.clone();
    let consumer_callback_address = res_body.callback_address.clone();
    let callback_id = res_body.consumer_pid.clone();
    println!("\nConsumer Pid: {:#?}", consumer_pid.to_string());
    println!(
        "Consumer Callback Address: {:#?}",
        consumer_callback_address
    );
    println!("Callback Id: {:#?}\n", callback_id);

    Ok((
        provider_server,
        consumer_server,
        client,
        catalog_id,
        dataservice_id,
        agreement_id,
        consumer_pid,
        consumer_callback_address,
        callback_id,
    ))
}

// TODO refactor into same test function
pub async fn setup_test_env_push(
    url_to_load: &str,
    description_to_load: serde_json::Value,
) -> anyhow::Result<(
    Child,
    Child,
    reqwest::Client,
    Urn,
    Urn,
    Urn,
    Urn,
    String,
    Urn,
)> {
    let cwd = "./../rainbow-core";
    let provider_envs = load_env_file(".env.provider.template");
    let mut provider_server = Command::new("cargo")
        .current_dir(cwd)
        .env_clear()
        .envs(&provider_envs)
        .env("TEST", "true")
        .args(&["run", "--", "provider", "start"])
        .spawn()
        .expect("Failed to start provider server");

    let consumer_envs = load_env_file(".env.consumer.template");
    let mut consumer_server = Command::new("cargo")
        .current_dir(cwd)
        .env_clear()
        .envs(&consumer_envs)
        .env("TEST", "true")
        .args(&["run", "--", "consumer", "start"])
        .spawn()
        .expect("Failed to start consumer server");

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // ====================================
    //  LOAD DATASERVICES AND AGREEEMENTS IN PROVIDER
    // ====================================
    let client = reqwest::Client::new();

    // catalog
    let res = client
        .post("http://localhost:1234/api/v1/catalogs")
        .json(&json!({
            "foaf:homepage": "homepage"
        }))
        .send()
        .await?;
    let res_body = res.json::<Catalog>().await?;
    let catalog_id = get_urn_from_string(&res_body.id)?;
    println!("\nCatalog Id: {:#?}\n\n", catalog_id.to_string());

    // dataservice
    let json_description = serde_json::to_string_pretty(&description_to_load)?;
    let res = client
        .post(format!(
            "http://localhost:1234/api/v1/catalogs/{}/data-services",
            catalog_id
        ))
        .json(&json!({
            "dcat:endpointURL": url_to_load,
            "dcat:endpointDescription": json_description
        }))
        .send()
        .await?;
    let res_body = res.json::<DataService>().await?;
    let dataservice_id = get_urn_from_string(&res_body.id)?;
    println!("\nDataservice Id: {:#?}\n\n", dataservice_id.to_string());

    // agreement
    let res = client
        .post("http://localhost:1234/api/v1/agreements")
        .json(&json!({
            "dataServiceId": dataservice_id
        }))
        .send()
        .await?;
    let res_body = res.json::<agreement::Model>().await?;
    let agreement_id = get_urn_from_string(&res_body.agreement_id)?;
    println!("\nAgreement Id: {:#?}\n\n", agreement_id.to_string());


    // ====================================
    //  CREATE CALLBACK IN CONSUMER
    // ====================================
    let res = client.post("http://localhost:1235/api/v1/transfers/rpc/setup-request").send().await?;
    let res_body = res.json::<DSRPCTransferConsumerRequestResponse>().await?;
    let consumer_pid = res_body.consumer_pid.clone();
    let consumer_callback_address = res_body.callback_address.clone();
    let callback_id = res_body.consumer_pid;
    println!("\nConsumer Pid: {:#?}", consumer_pid.to_string());
    println!(
        "Consumer Callback Address: {:#?}",
        consumer_callback_address
    );
    println!("Callback Id: {:#?}\n", callback_id);

    Ok((
        provider_server,
        consumer_server,
        client,
        catalog_id,
        dataservice_id,
        agreement_id,
        consumer_pid,
        consumer_callback_address,
        callback_id,
    ))
}

pub async fn cleanup_test_env(
    mut provider_server: Child,
    mut consumer_server: Child,
) -> anyhow::Result<()> {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    provider_server.kill().expect("Failed to kill provider server");
    consumer_server.kill().expect("Failed to kill consumer server");
    Ok(())
}

pub fn load_env_file(env_file: &str) -> HashMap<String, String> {
    dotenvy::from_filename(env_file).ok().expect("Failed to read .env file");
    std::env::vars().collect()
}
