pub mod cli;
pub mod data;
pub mod grpc;
pub mod http;
pub mod lib;

use std::fs;

use once_cell::sync::Lazy;
use reqwest::Client;
use tracing::info;
use tracing::log::debug;
use uuid::Uuid;

use crate::transfer::common::misc_router::VersionResponse;
use crate::transfer::protocol::messages::*;

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

fn get_provider_url(path: &str, host: &Option<String>, url: &Option<String>) -> String {
    let host = host.clone().unwrap_or("localhost".to_owned());
    let port = url.clone().unwrap_or("1234".to_owned());
    let url = format!("http://{}:{}{}", host, port, path);
    url
}

fn get_json_file(path: &str) -> anyhow::Result<String> {
    let main_path = "./static/json-tests/";
    let file_url = format!("{}{}", main_path, path);
    let json_raw = fs::read_to_string(file_url)?;
    Ok(json_raw)
}

pub async fn start_test(host: &Option<String>, url: &Option<String>) -> anyhow::Result<()> {
    info!("Test transfer...");

    let path = get_provider_url("/version", host, url);
    let res = reqwest::get(&path).await?.json::<VersionResponse>().await?;

    debug!("{:#?}", res);
    Ok(())
}

pub async fn start_transfer_request(
    host: &Option<String>,
    url: &Option<String>,
) -> anyhow::Result<()> {
    info!("Starting transfer from consumer...");

    // config stuff
    let path = get_provider_url("/transfers/request", host, url);
    let file = get_json_file("transfer-request.json")?;
    let mut data = serde_json::from_str::<TransferRequestMessage>(&file)?;
    let uuid = Uuid::new_v4();
    data.consumer_pid = format!("urn:uuid:{}", uuid.to_string());

    debug!("{}", serde_json::to_string_pretty(&data)?);

    // request
    let res = CLIENT
        .post(path)
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;
    debug!("{}", &res.text().await?);

    Ok(())
}

pub async fn start_transfer_start(
    host: &Option<String>,
    url: &Option<String>,
) -> anyhow::Result<()> {
    info!("Starting transfer...");

    // config stuff
    let path = get_provider_url("/transfers/start", host, url);
    let file = get_json_file("transfer-start.json")?;
    let data = serde_json::from_str::<TransferStartMessage>(&file)?;

    debug!("{:#?}", &data);

    // request
    let res = CLIENT.post(path).json(&data).send().await?;
    // manage response...
    debug!("{}", &res.text().await?);
    Ok(())
}

pub async fn start_transfer_suspension(
    host: &Option<String>,
    url: &Option<String>,
) -> anyhow::Result<()> {
    info!("Starting transfer suspension...");

    // config stuff
    let path = get_provider_url("/transfers/suspension", host, url);
    let file = get_json_file("transfer-suspension.json")?;
    let data = serde_json::from_str::<TransferSuspensionMessage>(&file)?;

    debug!("{:#?}", &data);

    // request
    let res = CLIENT.post(path).json(&data).send().await?;
    // manage response...
    debug!("{}", &res.text().await?);
    Ok(())
}

pub async fn start_transfer_completion(
    host: &Option<String>,
    url: &Option<String>,
) -> anyhow::Result<()> {
    info!("Starting transfer completion...");

    // config stuff
    let path = get_provider_url("/transfers/completion", host, url);
    let file = get_json_file("transfer-completion.json")?;
    let data = serde_json::from_str::<TransferCompletionMessage>(&file)?;

    debug!("{:#?}", &data);

    // request
    let res = CLIENT.post(path).json(&data).send().await?;
    // manage response...
    debug!("{}", &res.text().await?);
    Ok(())
}

pub async fn start_transfer_termination(
    host: &Option<String>,
    url: &Option<String>,
) -> anyhow::Result<()> {
    info!("Starting transfer termination...");

    // config stuff
    let path = get_provider_url("/transfers/termination", host, url);
    let file = get_json_file("transfer-termination.json")?;
    let data = serde_json::from_str::<TransferTerminationMessage>(&file)?;

    debug!("{:#?}", &data);

    // request
    let res = CLIENT.post(path).json(&data).send().await?;
    // manage response...
    debug!("{}", &res.text().await?);
    Ok(())
}
