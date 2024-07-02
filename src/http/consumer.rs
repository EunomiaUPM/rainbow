use std::collections::HashMap;
use std::fs;

use reqwest::{Client, Response};
use tracing::{info, Level};
use tracing::log::debug;
use once_cell::sync::Lazy;

use crate::http::version::VersionResponse;
use crate::transfer::messages::TransferRequestMessage;

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

fn get_provider_url(path: &str, host: &Option<String>, url: &Option<String>) -> String {
    let host = host.clone().unwrap_or("localhost".to_owned());
    let port = url.clone().unwrap_or("1234".to_owned());
    let url = format!("http://{}:{}{}", host, port, path);
    url
}

fn get_json_file(path: &str) -> anyhow::Result<String> {
    let main_path = "./test/json-tests/";
    let file_url = format!("{}{}", main_path, path);
    let json_raw = fs::read_to_string(file_url)?;
    Ok(json_raw)
}

pub async fn start_test(host: &Option<String>, url: &Option<String>) -> anyhow::Result<()> {
    info!("Test transfer...");

    let path = get_provider_url("/version", host, url);
    let req = reqwest::get(&path)
        .await?
        .json::<VersionResponse>()
        .await?;

    println!("{req:#?}");
    Ok(())
}

pub async fn start_transfer_request(host: &Option<String>, url: &Option<String>) -> anyhow::Result<()> {
    info!("Starting transfer from consumer...");

    // config stuff
    let path = get_provider_url("/transfer/start", host, url);
    let file = get_json_file("transfer-request.json")?;
    let data = serde_json::from_str::<TransferRequestMessage>(&file)?;

    debug!("{:?}", &data);

    // request
    let res = CLIENT.post(path)
        .json(&serde_json::to_string(&data)?)
        .send()
        .await?;

    // manage response...
    println!("{:?}", res);
    Ok(())
}