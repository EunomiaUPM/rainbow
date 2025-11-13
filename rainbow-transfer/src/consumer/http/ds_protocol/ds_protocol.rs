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

use crate::common::errors::error_adapter::CustomToResponse;
use crate::consumer::core::ds_protocol::DSProtocolTransferConsumerTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{middleware, Extension, Json, Router};
use rainbow_common::auth::header::{extract_request_info, RequestInfo};
use rainbow_common::err::transfer_err::TransferErrorType::{NotCheckedError, ProtocolBodyError};
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::utils::get_urn_from_string;
use std::sync::Arc;
use tracing::{error, info};

pub struct DSProtocolTransferConsumerRouter<T> {
    transfer_service: Arc<T>,
}

impl<T> DSProtocolTransferConsumerRouter<T>
where
    T: DSProtocolTransferConsumerTrait + Send + Sync + 'static,
{
    pub fn new(transfer_service: Arc<T>) -> Self {
        Self { transfer_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/:callback/transfers/:consumer_pid/start",
                post(Self::handle_transfer_start),
            )
            .route(
                "/:callback/transfers/:consumer_pid/suspension",
                post(Self::handle_transfer_suspension),
            )
            .route(
                "/:callback/transfers/:consumer_pid/completion",
                post(Self::handle_transfer_completion),
            )
            .route(
                "/:callback/transfers/:consumer_pid/termination",
                post(Self::handle_transfer_termination),
            )
            .layer(middleware::from_fn(extract_request_info))
            .with_state(self.transfer_service)
    }
    async fn handle_transfer_start(
        State(transfer_service): State<Arc<T>>,
        Path((callback, consumer_pid)): Path<(String, String)>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<TransferStartMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /{}/transfers/{}/start", callback, consumer_pid);
        let callback = match get_urn_from_string(&callback) {
            Ok(callback) => callback,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Urn malformed. {}", err.to_string()),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let consumer_pid = match get_urn_from_string(&consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Urn malformed. {}", err.to_string()),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, &format!("{}", err.body_text()));
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let token = info.token.clone();

        match transfer_service.transfer_start(Some(callback), consumer_pid, input, token).await {
            Ok(transfer_process) => (StatusCode::OK, Json(transfer_process)).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn handle_transfer_suspension(
        State(transfer_service): State<Arc<T>>,
        Path((callback, consumer_pid)): Path<(String, String)>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<TransferSuspensionMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /{}/transfers/{}/suspension", callback, consumer_pid);
        let callback = match get_urn_from_string(&callback) {
            Ok(callback) => callback,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Urn malformed. {}", err.to_string()),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let consumer_pid = match get_urn_from_string(&consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Urn malformed. {}", err.to_string()),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, &format!("{}", err.body_text()));
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let token = info.token.clone();

        match transfer_service.transfer_suspension(Some(callback), consumer_pid, input, token).await {
            Ok(transfer_process) => (StatusCode::OK, Json(transfer_process)).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn handle_transfer_completion(
        State(transfer_service): State<Arc<T>>,
        Path((callback, consumer_pid)): Path<(String, String)>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<TransferCompletionMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /{}/transfers/{}/completion", callback, consumer_pid);
        let callback = match get_urn_from_string(&callback) {
            Ok(callback) => callback,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Urn malformed. {}", err.to_string()),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let consumer_pid = match get_urn_from_string(&consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Urn malformed. {}", err.to_string()),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, &format!("{}", err.body_text()));
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let token = info.token.clone();

        match transfer_service.transfer_completion(Some(callback), consumer_pid, input, token).await {
            Ok(transfer_process) => (StatusCode::OK, Json(transfer_process)).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn handle_transfer_termination(
        State(transfer_service): State<Arc<T>>,
        Path((callback, consumer_pid)): Path<(String, String)>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<TransferTerminationMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /{}/transfers/{}/termination", callback, consumer_pid);
        let callback = match get_urn_from_string(&callback) {
            Ok(callback) => callback,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Urn malformed. {}", err.to_string()),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let consumer_pid = match get_urn_from_string(&consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Urn malformed. {}", err.to_string()),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, &format!("{}", err.body_text()));
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let token = info.token.clone();

        match transfer_service.transfer_termination(Some(callback), consumer_pid, input, token).await {
            Ok(transfer_process) => (StatusCode::OK, Json(transfer_process)).into_response(),
            Err(err) => err.to_response(),
        }
    }
}
