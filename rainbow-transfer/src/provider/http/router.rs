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

use crate::common::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::common::http::middleware::{
    pids_as_urn_validation_middleware, protocol_rules_middleware, schema_validation_middleware,
};
use crate::provider::lib::control_plane::{
    get_transfer_requests_by_provider, transfer_completion, transfer_request, transfer_start,
    transfer_suspension, transfer_termination,
};
use anyhow::bail;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{middleware, Json, Router};
use clap::builder::TypedValueParser;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::FormatAction;
use rainbow_common::err::transfer_err::TransferErrorType;
use rainbow_common::err::transfer_err::TransferErrorType::{
    ConsumerNotReachableError, NotCheckedError, ProtocolBodyError, TransferProcessNotFound,
};
use rainbow_common::protocol::transfer::{
    DataAddress, TransferCompletionMessage, TransferMessageTypes, TransferMessageTypesForDb,
    TransferProcessMessage, TransferRequestMessage, TransferRoles, TransferStartMessage,
    TransferState, TransferStateForDb, TransferSuspensionMessage, TransferTerminationMessage,
    TRANSFER_CONTEXT,
};
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::transfer_provider::entities::{transfer_message, transfer_process};
use rainbow_db::transfer_provider::repo::{EditTransferProcessModel, NewTransferMessageModel, TRANSFER_PROVIDER_REPO};
use reqwest::Error;
use sea_orm::{ActiveValue, EntityTrait};
use serde_json::to_value;
use tracing::{debug, error, info};
use urn::Urn;
use utoipa::openapi::RefOr::T;
use uuid::Uuid;

pub fn router() -> Router {
    // Based on a group of middlewares
    let router_group_a = Router::new().route(
        "/transfers/:provider_pid",
        get(handle_get_transfer_by_provider),
    );

    // Based on a group of middlewares
    let router_group_b = Router::new()
        .route("/transfers/request", post(handle_transfer_request))
        .route("/transfers/start", post(handle_transfer_start))
        .route("/transfers/suspension", post(handle_transfer_suspension))
        .route("/transfers/completion", post(handle_transfer_completion))
        .route("/transfers/termination", post(handle_transfer_termination))
        .route_layer(middleware::from_fn(pids_as_urn_validation_middleware))
        .route_layer(middleware::from_fn(protocol_rules_middleware))
        .route_layer(middleware::from_fn(schema_validation_middleware));

    Router::new().merge(router_group_a).merge(router_group_b)
}

async fn handle_get_transfer_by_provider(Path(provider_pid): Path<String>) -> impl IntoResponse {
    info!("GET /transfers/{}", provider_pid);
    let id = get_urn_from_string(&provider_pid).unwrap();

    match get_transfer_requests_by_provider(id).await.unwrap() {
        Some(transfer_process) => (
            StatusCode::OK,
            Json(TransferProcessMessage {
                context: TRANSFER_CONTEXT.to_string(),
                _type: TransferMessageTypes::TransferProcessMessage.to_string(),
                provider_pid: transfer_process.provider_pid.to_string(),
                consumer_pid: transfer_process.consumer_pid.to_string(),
                state: TransferState::try_from(transfer_process.state).unwrap(),
            }),
        )
            .into_response(),
        None => TransferProcessNotFound.into_response(),
    }
}

async fn handle_transfer_request(
    result: Result<Json<TransferRequestMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /transfers/request");

    match result {
        Ok(Json(input)) => match transfer_request(input, send_transfer_start).await {
            Ok(tp) => (StatusCode::CREATED, Json(tp)).into_response(),
            Err(e) => match e.downcast::<TransferErrorType>() {
                Ok(transfer_error) => {
                    println!("{:#?}", transfer_error);
                    transfer_error.into_response()
                }
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        },
        Err(e) => match e {
            JsonRejection::JsonDataError(e_) => {
                ProtocolBodyError { message: e_.body_text() }.into_response()
            }
            _ => NotCheckedError { inner_error: e.into() }.into_response(),
        },
    }
}

async fn send_transfer_start(
    Json(input): Json<TransferRequestMessage>,
    provider_pid: Urn,
    data_address: Option<DataAddress>,
) -> anyhow::Result<()> {
    let transfer_start_message = TransferStartMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferStartMessage.to_string(),
        provider_pid: provider_pid.to_string(),
        consumer_pid: input.consumer_pid.to_string(),
        data_address,
    };

    let consumer_transfer_endpoint = format!(
        "{}/transfers/{}/start",
        input.callback_address,
        input.consumer_pid
    );

    let response = DATA_PLANE_HTTP_CLIENT
        .clone()
        .post(consumer_transfer_endpoint)
        .header("content-type", "application/json")
        .json(&transfer_start_message)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status() == StatusCode::OK {
                let _ = TRANSFER_PROVIDER_REPO.put_transfer_process(provider_pid.clone(), EditTransferProcessModel {
                    state: Some(TransferStateForDb::STARTED),
                    ..Default::default()
                }).await?;

                let _ = TRANSFER_PROVIDER_REPO.create_transfer_message(provider_pid, NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferStartMessage.to_string(),
                    from: TransferRoles::Provider,
                    to: TransferRoles::Consumer,
                    content: to_value(&transfer_start_message)?,
                }).await?;

                Ok(())
            } else {
                println!("not started...."); // TODO Error
                Ok(())
            }
        }
        Err(_) => bail!(ConsumerNotReachableError),
    }
}

async fn handle_transfer_start(
    result: Result<Json<TransferStartMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /transfers/start");

    match result {
        Ok(Json(input)) => match transfer_start(input).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<TransferErrorType>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        },
        Err(e) => match e {
            JsonRejection::JsonDataError(e_) => {
                ProtocolBodyError { message: e_.body_text() }.into_response()
            }
            _ => NotCheckedError { inner_error: e.into() }.into_response(),
        },
    }
}

async fn handle_transfer_suspension(
    result: Result<Json<TransferSuspensionMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /transfers/suspension");

    match result {
        Ok(Json(input)) => match transfer_suspension(input).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<TransferErrorType>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        },
        Err(e) => match e {
            JsonRejection::JsonDataError(e_) => {
                ProtocolBodyError { message: e_.body_text() }.into_response()
            }
            _ => NotCheckedError { inner_error: e.into() }.into_response(),
        },
    }
}

async fn handle_transfer_completion(
    result: Result<Json<TransferCompletionMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /transfers/completion");

    match result {
        Ok(Json(input)) => match transfer_completion(input).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<TransferErrorType>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        },
        Err(e) => match e {
            JsonRejection::JsonDataError(e_) => {
                ProtocolBodyError { message: e_.body_text() }.into_response()
            }
            _ => NotCheckedError { inner_error: e.into() }.into_response(),
        },
    }
}

async fn handle_transfer_termination(
    result: Result<Json<TransferTerminationMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /transfers/termination");

    match result {
        Ok(Json(input)) => match transfer_termination(input).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<TransferErrorType>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        },
        Err(e) => match e {
            JsonRejection::JsonDataError(e_) => {
                ProtocolBodyError { message: e_.body_text() }.into_response()
            }
            _ => NotCheckedError { inner_error: e.into() }.into_response(),
        },
    }
}
