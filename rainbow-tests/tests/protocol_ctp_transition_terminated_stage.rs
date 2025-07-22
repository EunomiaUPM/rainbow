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

use crate::utils::cleanup_test_env;
use clap::builder::TypedValueParser;
use once_cell::sync::Lazy;
use rainbow_common::dcat_formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow_common::protocol::context_field::ContextField;
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_request::TransferRequestMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::protocol::transfer::TransferMessageTypes;
use rainbow_common::utils::get_urn;
use std::process::{Child, Command};
use tracing_test::traced_test;
use urn::Urn;
use uuid::Uuid;

#[path = "utils.rs"]
mod utils;

#[traced_test]
#[tokio::test]
pub async fn to_requested() -> anyhow::Result<()> {
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
    ) = utils::setup_test_env("a").await?;

    // 1. NORMAL REQUEST
    // 1. NORMAL REQUEST
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: consumer_pid.clone().to_string(),
            agreement_id: get_urn(Some(agreement_id.parse::<Urn>()?)).to_string(),
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: consumer_callback_address.clone(), // start will trigger
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
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: consumer_pid.clone(),
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
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: consumer_pid.clone(),
            agreement_id: get_urn(Some(agreement_id.parse::<Urn>()?)).to_string(),
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: consumer_callback_address.clone(), // start will trigger
            data_address: None,
        })
        .send()
        .await?;
    println!("Response status: {}", res.status());
    println!("Response body: {}", res.text().await?);

    utils::cleanup_test_env(provider_server, consumer_server).await?;
    Ok(())
}

#[traced_test]
#[tokio::test]
pub async fn to_started() -> anyhow::Result<()> {
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
    ) = utils::setup_test_env("a").await?;

    // 1. NORMAL REQUEST
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: consumer_pid.clone().to_string(),
            agreement_id: get_urn(Some(agreement_id.parse::<Urn>()?)).to_string(),
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: consumer_callback_address.clone(), // start will trigger
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
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: consumer_pid.clone(),
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
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferStartMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: consumer_pid.clone(),
            data_address: None,
        })
        .send()
        .await?;
    println!("Response status: {}", res.status());
    println!("Response body: {}", res.text().await?);

    utils::cleanup_test_env(provider_server, consumer_server).await?;
    Ok(())
}

#[traced_test]
#[tokio::test]
pub async fn to_suspended() -> anyhow::Result<()> {
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
    ) = utils::setup_test_env("a").await?;

    // 1. NORMAL REQUEST
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: consumer_pid.clone().to_string(),
            agreement_id: get_urn(Some(agreement_id.parse::<Urn>()?)).to_string(),
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: consumer_callback_address.clone(), // start will trigger
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
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: consumer_pid.clone(),
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
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: consumer_pid.clone(),
            code: "".to_string(),
            reason: vec![],
        })
        .send()
        .await?;
    println!("Response status: {}", res.status());
    println!("Response body: {}", res.text().await?);

    utils::cleanup_test_env(provider_server, consumer_server).await?;
    Ok(())
}

#[traced_test]
#[tokio::test]
pub async fn to_completed() -> anyhow::Result<()> {
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
    ) = utils::setup_test_env("a").await?;

    // 1. NORMAL REQUEST
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: consumer_pid.clone().to_string(),
            agreement_id: get_urn(Some(agreement_id.parse::<Urn>()?)).to_string(),
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: consumer_callback_address.clone(), // start will trigger
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
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: consumer_pid.clone(),
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
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferCompletionMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: consumer_pid.clone(),
        })
        .send()
        .await?;
    println!("Response status: {}", res.status());
    println!("Response body: {}", res.text().await?);

    utils::cleanup_test_env(provider_server, consumer_server).await?;
    Ok(())
}

#[traced_test]
#[tokio::test]
pub async fn to_terminated() -> anyhow::Result<()> {
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
    ) = utils::setup_test_env("a").await?;

    // 1. NORMAL REQUEST
    let res = client
        .post("http://localhost:1234/transfers/request")
        .header("content-type", "application/json")
        .json(&TransferRequestMessage {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: consumer_pid.clone().to_string(),
            agreement_id: get_urn(Some(agreement_id.parse::<Urn>()?)).to_string(),
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: consumer_callback_address.clone(), // start will trigger
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
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: consumer_pid.clone(),
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
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferTerminationMessage.to_string(),
            provider_pid: provider_pid_.clone(),
            consumer_pid: consumer_pid.clone(),
            code: "".to_string(),
            reason: vec!["termination".to_string()],
        })
        .send()
        .await?;
    println!("Response status: {}", res.status());
    println!("Response body: {}", res.text().await?);

    utils::cleanup_test_env(provider_server, consumer_server).await?;
    Ok(())
}
