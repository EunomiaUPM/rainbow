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
// use crate::common::http::client::DATA_PLANE_HTTP_CLIENT;
// use crate::common::http::middleware::{
//     pids_as_urn_validation_middleware, protocol_rules_middleware, schema_validation_middleware,
// };
use crate::provider::core::ds_protocol::DSProtocolTransferProviderTrait;
use anyhow::bail;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Request, State};
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
    DataAddress, TransferCompletionMessage, TransferMessageTypes, TransferMessageTypesForDb, TransferProcessMessage,
    TransferRequestMessage, TransferRoles, TransferStartMessage, TransferState, TransferStateForDb,
    TransferSuspensionMessage, TransferTerminationMessage, TRANSFER_CONTEXT,
};
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::transfer_provider::entities::{transfer_message, transfer_process};
use rainbow_db::transfer_provider::repo::{EditTransferProcessModel, NewTransferMessageModel};
use reqwest::Error;
use sea_orm::{ActiveValue, EntityTrait};
use serde_json::to_value;
use std::sync::Arc;
use tracing::{debug, error, info};
use urn::Urn;
use uuid::Uuid;

pub struct DSProtocolTransferProviderRouter<T> {
    transfer_service: Arc<T>,
}

impl<T> DSProtocolTransferProviderRouter<T>
where
    T: DSProtocolTransferProviderTrait + Send + Sync + 'static,
{
    pub fn new(transfer_service: Arc<T>) -> Self {
        Self { transfer_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/transfers/request", post(Self::handle_transfer_request))
            .route(
                "/transfers/:provider_pid/start",
                post(Self::handle_transfer_start),
            )
            .route(
                "/transfers/:provider_pid/suspension",
                post(Self::handle_transfer_suspension),
            )
            .route(
                "/transfers/:provider_pid/completion",
                post(Self::handle_transfer_completion),
            )
            .route(
                "/transfers/:provider_pid/termination",
                post(Self::handle_transfer_termination),
            )
            // .route_layer(middleware::from_fn(pids_as_urn_validation_middleware))
            // .route_layer(middleware::from_fn_with_state(service, protocol_rules_middleware))
            // .route_layer(middleware::from_fn(schema_validation_middleware))
            .route(
                "/transfers/:provider_pid",
                get(Self::handle_get_transfer_by_provider),
            )
            .with_state(self.transfer_service)
    }
    async fn handle_get_transfer_by_provider(
        State(transfer_service): State<Arc<T>>,
        Path(provider_pid): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /transfers/{}", provider_pid.to_string());
        let id = get_urn_from_string(&provider_pid).unwrap();
        match transfer_service.get_transfer_requests_by_provider(id).await {
            Ok(transfer_process) => (
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
            Err(_) => TransferProcessNotFound.into_response(),
        }
    }

    async fn handle_transfer_request(
        State(transfer_service): State<Arc<T>>,
        result: Result<Json<TransferRequestMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /transfers/request");

        match result {
            Ok(Json(input)) => match transfer_service.transfer_request(input).await {
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
                JsonRejection::JsonDataError(e_) => ProtocolBodyError { message: e_.body_text() }.into_response(),
                _ => NotCheckedError { inner_error: e.into() }.into_response(),
            },
        }
    }

    async fn send_transfer_start(
        State(transfer_service): State<Arc<T>>,
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
            input.callback_address, input.consumer_pid
        );

        // let response = DATA_PLANE_HTTP_CLIENT
        //     .clone()
        //     .post(consumer_transfer_endpoint)
        //     .header("content-type", "application/json")
        //     .json(&transfer_start_message)
        //     .send()
        //     .await;
        //
        // match response {
        //     Ok(res) => {
        //         if res.status() == StatusCode::OK {
        //             // let _ = transfer_service.put_transfer_process(provider_pid.clone(), EditTransferProcessModel {
        //             //     state: Some(TransferStateForDb::STARTED),
        //             //     ..Default::default()
        //             // }).await?;
        //             //
        //             // let _ = transfer_service.TRANSFER_PROVIDER_REPO.create_transfer_message(provider_pid, NewTransferMessageModel {
        //             //     message_type: TransferMessageTypes::TransferStartMessage.to_string(),
        //             //     from: TransferRoles::Provider,
        //             //     to: TransferRoles::Consumer,
        //             //     content: to_value(&transfer_start_message)?,
        //             // }).await?;
        //
        //             Ok(())
        //         } else {
        //             println!("not started...."); // TODO Error
        //             Ok(())
        //         }
        //     }
        //     Err(_) => bail!(ConsumerNotReachableError),
        // }
        Ok(())
    }

    async fn handle_transfer_start(
        State(transfer_service): State<Arc<T>>,
        provider_pid: Path<String>,
        result: Result<Json<TransferStartMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /transfers/{}/start", provider_pid.to_string());

        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(_) => {
                return TransferErrorType::PidSchemeError.into_response();
            }
        };

        match result {
            Ok(Json(input)) => match transfer_service.transfer_start(provider_pid, input).await {
                Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
                Err(e) => match e.downcast::<TransferErrorType>() {
                    Ok(transfer_error) => transfer_error.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                },
            },
            Err(e) => match e {
                JsonRejection::JsonDataError(e_) => ProtocolBodyError { message: e_.body_text() }.into_response(),
                _ => NotCheckedError { inner_error: e.into() }.into_response(),
            },
        }
    }

    async fn handle_transfer_suspension(
        State(transfer_service): State<Arc<T>>,
        provider_pid: Path<String>,
        result: Result<Json<TransferSuspensionMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /transfers/{}/suspension", provider_pid.to_string());

        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(_) => {
                return TransferErrorType::PidSchemeError.into_response();
            }
        };

        match result {
            Ok(Json(input)) => match transfer_service.transfer_suspension(provider_pid, input).await {
                Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
                Err(e) => match e.downcast::<TransferErrorType>() {
                    Ok(transfer_error) => transfer_error.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                },
            },
            Err(e) => match e {
                JsonRejection::JsonDataError(e_) => ProtocolBodyError { message: e_.body_text() }.into_response(),
                _ => NotCheckedError { inner_error: e.into() }.into_response(),
            },
        }
    }

    async fn handle_transfer_completion(
        State(transfer_service): State<Arc<T>>,
        provider_pid: Path<String>,
        result: Result<Json<TransferCompletionMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /transfers/{}/completion", provider_pid.to_string());

        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(_) => {
                return TransferErrorType::PidSchemeError.into_response();
            }
        };

        match result {
            Ok(Json(input)) => match transfer_service.transfer_completion(provider_pid, input).await {
                Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
                Err(e) => match e.downcast::<TransferErrorType>() {
                    Ok(transfer_error) => transfer_error.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                },
            },
            Err(e) => match e {
                JsonRejection::JsonDataError(e_) => ProtocolBodyError { message: e_.body_text() }.into_response(),
                _ => NotCheckedError { inner_error: e.into() }.into_response(),
            },
        }
    }

    async fn handle_transfer_termination(
        State(transfer_service): State<Arc<T>>,
        provider_pid: Path<String>,
        result: Result<Json<TransferTerminationMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /transfers/{}/termination", provider_pid.to_string());

        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(_) => {
                return TransferErrorType::PidSchemeError.into_response();
            }
        };

        match result {
            Ok(Json(input)) => match transfer_service.transfer_termination(provider_pid, input).await {
                Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
                Err(e) => match e.downcast::<TransferErrorType>() {
                    Ok(transfer_error) => transfer_error.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                },
            },
            Err(e) => match e {
                JsonRejection::JsonDataError(e_) => ProtocolBodyError { message: e_.body_text() }.into_response(),
                _ => NotCheckedError { inner_error: e.into() }.into_response(),
            },
        }
    }
}
