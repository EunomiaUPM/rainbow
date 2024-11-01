#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use rainbow::fake_catalog::data::models::DatasetsCatalogModel;
use rainbow::fake_catalog::lib::delete_dataset;
use rainbow::fake_contracts::data::models::ContractAgreementsModel;
use rainbow::fake_contracts::data::repo::delete_agreement_repo;
use rainbow::transfer::consumer::http::api::CreateCallbackResponse;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::process::{Child, Command};
use uuid::Uuid;

pub fn get_json_file(path: &str) -> anyhow::Result<String> {
    let main_path = "./static/json-tests/";
    let file_url = format!("{}{}", main_path, path);
    let json_raw = fs::read_to_string(file_url)?;
    Ok(json_raw)
}

pub async fn setup_agreements_and_datasets() -> anyhow::Result<(Vec<DatasetsCatalogModel>, Vec<ContractAgreementsModel>)> {
    let client = reqwest::Client::new();
    let fake_parquet_url = "http://localhost:1236/data-space";
    let fake_parquet_files = vec!["/sample1.parquet"];
    let mut fake_datasets: Vec<DatasetsCatalogModel> = vec![];
    let mut agreements: Vec<ContractAgreementsModel> = vec![];

    let fake_parquet_file = fake_parquet_files
        .iter()
        .map(|f| format!("{}{}", fake_parquet_url, f))
        .collect::<Vec<_>>();

    for endpoint in fake_parquet_file {
        println!("Processing endpoint: {}", endpoint);
        let res = client
            .post("http://localhost:1234/catalogs/datasets")
            .json(&json!({
                "endpoint": endpoint
            }))
            .send()
            .await?;
        let ds: DatasetsCatalogModel = res.json().await?;
        fake_datasets.push(ds.clone());
        let res = client
            .post("http://localhost:1234/agreements")
            .json(&json!({
                "dataset": ds.dataset_id
            }))
            .send()
            .await?;
        let agreement: ContractAgreementsModel = res.json().await?;
        agreements.push(agreement);
    }

    Ok((fake_datasets, agreements))
}

pub async fn setup_test_env() -> anyhow::Result<(
    Child,
    Child,
    reqwest::Client,
    Vec<ContractAgreementsModel>,
    Vec<DatasetsCatalogModel>,
    String,
    Uuid,
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

    let client = reqwest::Client::new();
    let setup = setup_agreements_and_datasets().await?;
    let agreements = setup.1.clone();
    let datasets = setup.0.clone();

    let res = client
        .post("http://localhost:1235/api/v1/setup-transfer")
        .send()
        .await?;
    let res_json = res.json::<CreateCallbackResponse>().await?;
    let callback_id = res_json.callback_id.parse::<Uuid>()?;
    let callback_address = res_json.callback_address;
    let consumer_pid = res_json.consumer_pid.parse::<Uuid>()?;

    Ok((
        provider_server,
        consumer_server,
        client,
        agreements,
        datasets,
        callback_address,
        consumer_pid,
        callback_id
    ))
}

pub async fn cleanup_test_env(
    mut provider_server: Child,
    mut consumer_server: Child,
) -> anyhow::Result<()> {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    provider_server
        .kill()
        .expect("Failed to kill provider server");
    consumer_server
        .kill()
        .expect("Failed to kill consumer server");
    Ok(())
}

pub async fn cleanup_env(
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

pub fn load_env_file(env_file: &str) -> HashMap<String, String> {
    dotenvy::from_filename(env_file)
        .ok()
        .expect("Failed to read .env file");
    std::env::vars().collect()
}