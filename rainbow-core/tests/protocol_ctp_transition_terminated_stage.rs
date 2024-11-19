#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use super::utils::{cleanup_test_env, setup_test_env};
use once_cell::sync::Lazy;
use rainbow_common::dcat_formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow_common::utils::convert_uuid_to_uri;
use rainbow_transfer::protocol::messages::{
    TransferCompletionMessage, TransferMessageTypes, TransferProcessMessage,
    TransferRequestMessage, TransferStartMessage, TransferSuspensionMessage,
    TransferTerminationMessage, TRANSFER_CONTEXT,
};
use std::process::{Child, Command};
use tracing_test::traced_test;
use uuid::Uuid;

#[traced_test]
#[tokio::test]
pub async fn to_requested() -> anyhow::Result<()> {
    let (
        mut provider_server,
        mut consumer_server,
        client,
        agreements,
        _datasets,
        callback_address,
        consumer_pid,
        callback_id,
    ) = setup_test_env().await?;

    // 1. NORMAL REQUEST
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            agreement_id: convert_uuid_to_uri(&agreements.get(0).unwrap().agreement_id)?,
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: callback_address.clone(), // start will trigger
            data_address: None,
        })
        .send()
        .await?;
    let res_body = res.json::<TransferProcessMessage>().await?;
    let provider_pid_ = res_body.provider_pid.clone();
    println!("provider: {}", provider_pid_);

    // 2. Set terminated
    let res = client
        .post("http://localhost:1234/transfers/termination")
        .header("content-type", "application/json")
        .json(&TransferTerminationMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            code: "".to_string(),
            reason: vec!["termination".to_string()],
        })
        .send()
        .await?;

    //============================================//
    // FROM REQUEST TO WRONG STATES
    //============================================//
    // Request with same data should fail
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            agreement_id: convert_uuid_to_uri(&agreements.get(0).unwrap().agreement_id)?,
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: callback_address.clone(), // start will trigger
            data_address: None,
        })
        .send()
        .await?;
    println!("Response status: {}", res.status());
    println!("Response body: {}", res.text().await?);

    cleanup_test_env(provider_server, consumer_server).await
}

#[traced_test]
#[tokio::test]
pub async fn to_started() -> anyhow::Result<()> {
    let (
        mut provider_server,
        mut consumer_server,
        client,
        agreements,
        _datasets,
        callback_address,
        consumer_pid,
        callback_id,
    ) = setup_test_env().await?;

    // 1. NORMAL REQUEST
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            agreement_id: convert_uuid_to_uri(&agreements.get(0).unwrap().agreement_id)?,
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: callback_address.clone(), // start will trigger
            data_address: None,
        })
        .send()
        .await?;
    let res_body = res.json::<TransferProcessMessage>().await?;
    let provider_pid_ = res_body.provider_pid.clone();
    println!("provider: {}", provider_pid_);

    // 2. Set terminated
    let res = client
        .post("http://localhost:1234/transfers/termination")
        .header("content-type", "application/json")
        .json(&TransferTerminationMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            code: "".to_string(),
            reason: vec!["termination".to_string()],
        })
        .send()
        .await?;

    //============================================//
    // FROM REQUEST TO WRONG STATES
    //============================================//
    // should fail
    let res = client
        .post("http://localhost:1234/transfers/start")
        .header("content-type", "application/json")
        .json(&TransferStartMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferStartMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            data_address: None,
        })
        .send()
        .await?;
    println!("Response status: {}", res.status());
    println!("Response body: {}", res.text().await?);

    cleanup_test_env(provider_server, consumer_server).await
}

#[traced_test]
#[tokio::test]
pub async fn to_suspended() -> anyhow::Result<()> {
    let (
        mut provider_server,
        mut consumer_server,
        client,
        agreements,
        _datasets,
        callback_address,
        consumer_pid,
        callback_id,
    ) = setup_test_env().await?;

    // 1. NORMAL REQUEST
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            agreement_id: convert_uuid_to_uri(&agreements.get(0).unwrap().agreement_id)?,
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: callback_address.clone(), // start will trigger
            data_address: None,
        })
        .send()
        .await?;
    let res_body = res.json::<TransferProcessMessage>().await?;
    let provider_pid_ = res_body.provider_pid.clone();
    println!("provider: {}", provider_pid_);

    // 2. Set terminated
    let res = client
        .post("http://localhost:1234/transfers/termination")
        .header("content-type", "application/json")
        .json(&TransferTerminationMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            code: "".to_string(),
            reason: vec!["termination".to_string()],
        })
        .send()
        .await?;

    //============================================//
    // FROM REQUEST TO WRONG STATES
    //============================================//
    // Request with same data should fail
    let res = client
        .post("http://localhost:1234/transfers/suspension")
        .header("content-type", "application/json")
        .json(&TransferSuspensionMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            code: "".to_string(),
            reason: vec![],
        })
        .send()
        .await?;
    println!("Response status: {}", res.status());
    println!("Response body: {}", res.text().await?);

    cleanup_test_env(provider_server, consumer_server).await
}

#[traced_test]
#[tokio::test]
pub async fn to_completed() -> anyhow::Result<()> {
    let (
        mut provider_server,
        mut consumer_server,
        client,
        agreements,
        _datasets,
        callback_address,
        consumer_pid,
        callback_id,
    ) = setup_test_env().await?;

    // 1. NORMAL REQUEST
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            agreement_id: convert_uuid_to_uri(&agreements.get(0).unwrap().agreement_id)?,
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: callback_address.clone(), // start will trigger
            data_address: None,
        })
        .send()
        .await?;
    let res_body = res.json::<TransferProcessMessage>().await?;
    let provider_pid_ = res_body.provider_pid.clone();
    println!("provider: {}", provider_pid_);

    // 2. Set terminated
    let res = client
        .post("http://localhost:1234/transfers/termination")
        .header("content-type", "application/json")
        .json(&TransferTerminationMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            code: "".to_string(),
            reason: vec!["termination".to_string()],
        })
        .send()
        .await?;

    //============================================//
    // FROM REQUEST TO WRONG STATES
    //============================================//
    // Ok
    let res = client
        .post("http://localhost:1234/transfers/completion")
        .header("content-type", "application/json")
        .json(&TransferCompletionMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferCompletionMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
        })
        .send()
        .await?;
    println!("Response status: {}", res.status());
    println!("Response body: {}", res.text().await?);

    cleanup_test_env(provider_server, consumer_server).await
}

#[traced_test]
#[tokio::test]
pub async fn to_terminated() -> anyhow::Result<()> {
    let (
        mut provider_server,
        mut consumer_server,
        client,
        agreements,
        _datasets,
        callback_address,
        consumer_pid,
        callback_id,
    ) = setup_test_env().await?;

    // 1. NORMAL REQUEST
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            agreement_id: convert_uuid_to_uri(&agreements.get(0).unwrap().agreement_id)?,
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: callback_address.clone(), // start will trigger
            data_address: None,
        })
        .send()
        .await?;
    let res_body = res.json::<TransferProcessMessage>().await?;
    let provider_pid_ = res_body.provider_pid.clone();
    println!("provider: {}", provider_pid_);

    // 2. Set terminated
    let res = client
        .post("http://localhost:1234/transfers/termination")
        .header("content-type", "application/json")
        .json(&TransferTerminationMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            code: "".to_string(),
            reason: vec!["termination".to_string()],
        })
        .send()
        .await?;

    //============================================//
    // FROM REQUEST TO WRONG STATES
    //============================================//
    // Should fail
    let res = client
        .post("http://localhost:1234/transfers/termination")
        .header("content-type", "application/json")
        .json(&TransferTerminationMessage {
            context: TRANSFER_CONTEXT.into(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
            code: "".to_string(),
            reason: vec!["termination".to_string()],
        })
        .send()
        .await?;
    println!("Response status: {}", res.status());
    println!("Response body: {}", res.text().await?);

    cleanup_test_env(provider_server, consumer_server).await
}
