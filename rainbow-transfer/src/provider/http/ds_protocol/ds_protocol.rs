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


use crate::provider::core::ds_protocol::ds_protocol_err::DSProtocolTransferProviderErrors;
use crate::provider::core::ds_protocol::DSProtocolTransferProviderTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{middleware, Extension, Json, Router};
use rainbow_common::auth::header::{extract_request_info, RequestInfo};
use rainbow_common::err::transfer_err::TransferErrorType;
use rainbow_common::err::transfer_err::TransferErrorType::{
    NotCheckedError, ProtocolBodyError,
};
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_request::TransferRequestMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::utils::get_urn_from_string;
use std::sync::Arc;
use tracing::info;

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
            .route(
                "/transfers/:provider_pid",
                get(Self::handle_get_transfer_by_provider),
            )
            .layer(middleware::from_fn(extract_request_info))
            .with_state(self.transfer_service)
    }
    async fn handle_get_transfer_by_provider(
        State(transfer_service): State<Arc<T>>,
        Path(provider_pid): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /transfers/{}", provider_pid.to_string());
        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(err) => return NotCheckedError { inner_error: err }.into_response(),
        };
        match transfer_service.get_transfer_requests_by_provider(provider_pid).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<DSProtocolTransferProviderErrors>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            }
        }
    }

    async fn handle_transfer_request(
        State(transfer_service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<TransferRequestMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /transfers/request");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return ProtocolBodyError { message: e.body_text() }.into_response(),
        };
        let token = info.token.clone();

        match transfer_service.transfer_request(input, token).await {
            Ok(tp) => (StatusCode::CREATED, Json(tp)).into_response(),
            Err(e) => match e.downcast::<DSProtocolTransferProviderErrors>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }

    async fn handle_transfer_start(
        State(transfer_service): State<Arc<T>>,
        provider_pid: Path<String>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<TransferStartMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /transfers/{}/start", provider_pid.to_string());

        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(_) => {
                return TransferErrorType::PidSchemeError.into_response();
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return ProtocolBodyError { message: e.body_text() }.into_response(),
        };
        let token = info.token.clone();

        match transfer_service.transfer_start(provider_pid, input, token).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<DSProtocolTransferProviderErrors>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }

    async fn handle_transfer_suspension(
        State(transfer_service): State<Arc<T>>,
        provider_pid: Path<String>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<TransferSuspensionMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /transfers/{}/suspension", provider_pid.to_string());

        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(_) => {
                return TransferErrorType::PidSchemeError.into_response();
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return ProtocolBodyError { message: e.body_text() }.into_response(),
        };
        let token = info.token.clone();


        match transfer_service.transfer_suspension(provider_pid, input, token).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<DSProtocolTransferProviderErrors>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }

    async fn handle_transfer_completion(
        State(transfer_service): State<Arc<T>>,
        provider_pid: Path<String>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<TransferCompletionMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /transfers/{}/completion", provider_pid.to_string());

        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(_) => {
                return TransferErrorType::PidSchemeError.into_response();
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return ProtocolBodyError { message: e.body_text() }.into_response(),
        };
        let token = info.token.clone();

        match transfer_service.transfer_completion(provider_pid, input, token).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<DSProtocolTransferProviderErrors>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }

    async fn handle_transfer_termination(
        State(transfer_service): State<Arc<T>>,
        provider_pid: Path<String>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<TransferTerminationMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /transfers/{}/termination", provider_pid.to_string());

        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(_) => {
                return TransferErrorType::PidSchemeError.into_response();
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return ProtocolBodyError { message: e.body_text() }.into_response(),
        };
        let token = info.token.clone();

        match transfer_service.transfer_termination(provider_pid, input, token).await {
            Ok(tp) => (StatusCode::OK, Json(tp)).into_response(),
            Err(e) => match e.downcast::<DSProtocolTransferProviderErrors>() {
                Ok(transfer_error) => transfer_error.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
}
