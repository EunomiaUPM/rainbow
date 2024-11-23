#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use rainbow_catalog::protocol::catalog_definition::Catalog;
use rainbow_catalog::protocol::dataservice_definition::DataService;
use rainbow_transfer::consumer::lib::api::CreateCallbackResponse;
use rainbow_transfer::provider::data::entities::agreements;
use serde_json::json;
use std::collections::HashMap;
use std::process::{Child, Command};
use uuid::Uuid;

pub async fn setup_test_env(
    url_to_load: &str,
) -> anyhow::Result<(
    Child,
    Child,
    reqwest::Client,
    Uuid,
    Uuid,
    Uuid,
    String,
    String,
    Uuid
)> {
    let provider_envs = load_env_file(".env.provider.template");
    let mut provider_server = Command::new("cargo")
        .env_clear()
        .envs(&provider_envs)
        .env("TEST", "true")
        .args(&["run", "--", "provider", "start"])
        .spawn()
        .expect("Failed to start provider server");

    let consumer_envs = load_env_file(".env.consumer.template");
    let mut consumer_server = Command::new("cargo")
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
    let catalog_id = res_body.id.parse::<Uuid>()?;
    println!("\nCatalog Id: {:#?}\n\n", catalog_id);

    // dataservice
    let res = client
        .post(format!(
            "http://localhost:1234/api/v1/catalogs/{}/data-services",
            catalog_id
        ))
        .json(&json!({
            "dcat:endpointURL": url_to_load
        }))
        .send()
        .await?;
    let res_body = res.json::<DataService>().await?;
    let dataservice_id = res_body.id.parse::<Uuid>()?;
    println!("\nDataservice Id: {:#?}\n\n", dataservice_id);

    // agreement
    let res = client
        .post("http://localhost:1234/api/v1/agreements")
        .json(&json!({
            "dataServiceId": dataservice_id
        }))
        .send()
        .await?;
    let res_body = res.json::<agreements::Model>().await?;
    let agreement_id = res_body.agreement_id;
    println!("\nAgreement Id: {:#?}\n\n", agreement_id.to_string());

    // ====================================
    //  CREATE CALLBACK IN CONSUMER
    // ====================================
    let res = client.post("http://localhost:1235/api/v1/setup-transfer").send().await?;
    let res_body = res.json::<CreateCallbackResponse>().await?;
    let consumer_pid = res_body.consumer_pid.clone();
    let consumer_callback_address = res_body.callback_address.clone();
    let callback_id = res_body.callback_id.clone().parse::<Uuid>()?;
    println!("\nConsumer Pid: {:#?}", consumer_pid);
    println!("Consumer Callback Address: {:#?}", consumer_callback_address);
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
    description_to_load: serde_json::Value
) -> anyhow::Result<(
    Child,
    Child,
    reqwest::Client,
    Uuid,
    Uuid,
    Uuid,
    String,
    String,
    Uuid
)> {
    let provider_envs = load_env_file(".env.provider.template");
    let mut provider_server = Command::new("cargo")
        .env_clear()
        .envs(&provider_envs)
        .env("TEST", "true")
        .args(&["run", "--", "provider", "start"])
        .spawn()
        .expect("Failed to start provider server");

    let consumer_envs = load_env_file(".env.consumer.template");
    let mut consumer_server = Command::new("cargo")
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
    let catalog_id = res_body.id.parse::<Uuid>()?;
    println!("\nCatalog Id: {:#?}\n\n", catalog_id);

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
    let dataservice_id = res_body.id.parse::<Uuid>()?;
    println!("\nDataservice Id: {:#?}\n\n", dataservice_id);

    // agreement
    let res = client
        .post("http://localhost:1234/api/v1/agreements")
        .json(&json!({
            "dataServiceId": dataservice_id
        }))
        .send()
        .await?;
    let res_body = res.json::<agreements::Model>().await?;
    let agreement_id = res_body.agreement_id;
    println!("\nAgreement Id: {:#?}\n\n", agreement_id.to_string());

    // ====================================
    //  CREATE CALLBACK IN CONSUMER
    // ====================================
    let res = client.post("http://localhost:1235/api/v1/setup-transfer").send().await?;
    let res_body = res.json::<CreateCallbackResponse>().await?;
    let consumer_pid = res_body.consumer_pid.clone();
    let consumer_callback_address = res_body.callback_address.clone();
    let callback_id = res_body.callback_id.clone().parse::<Uuid>()?;
    println!("\nConsumer Pid: {:#?}", consumer_pid);
    println!("Consumer Callback Address: {:#?}", consumer_callback_address);
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
