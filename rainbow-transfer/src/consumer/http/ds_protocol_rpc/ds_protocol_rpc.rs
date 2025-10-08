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
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    DSRPCTransferConsumerCompletionRequest, DSRPCTransferConsumerRequestRequest, DSRPCTransferConsumerStartRequest,
    DSRPCTransferConsumerSuspensionRequest, DSRPCTransferConsumerTerminationRequest,
};
use crate::consumer::core::ds_protocol_rpc::DSRPCTransferConsumerTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use log::info;
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::error;

pub struct DSRPCTransferConsumerRouter<T> {
    transfer_rpc_service: Arc<T>,
}

impl<T> DSRPCTransferConsumerRouter<T>
where
    T: DSRPCTransferConsumerTrait + Send + Sync + 'static,
{
    pub fn new(transfer_rpc_service: Arc<T>) -> Self {
        Self { transfer_rpc_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/transfers/rpc/setup-request",
                post(Self::setup_request),
            )
            .route("/api/v1/transfers/rpc/setup-start", post(Self::setup_start))
            .route(
                "/api/v1/transfers/rpc/setup-suspension",
                post(Self::setup_suspension),
            )
            .route(
                "/api/v1/transfers/rpc/setup-completion",
                post(Self::setup_completion),
            )
            .route(
                "/api/v1/transfers/rpc/setup-termination",
                post(Self::setup_termination),
            )
            .with_state(self.transfer_rpc_service)
    }
    async fn setup_request(
        State(transfer_rpc_service): State<Arc<T>>,
        input: Result<Json<DSRPCTransferConsumerRequestRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/rpc/setup-request");
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, format!("{}", err.body_text()).into());
                error!("{}", e.log());
                return e.into_response();
            }
        };
        match transfer_rpc_service.setup_request(input).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn setup_start(
        State(transfer_rpc_service): State<Arc<T>>,
        input: Result<Json<DSRPCTransferConsumerStartRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/rpc/setup-start");
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, format!("{}", err.body_text()).into());
                error!("{}", e.log());
                return e.into_response();
            }
        };
        match transfer_rpc_service.setup_start(input).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn setup_suspension(
        State(transfer_rpc_service): State<Arc<T>>,
        input: Result<Json<DSRPCTransferConsumerSuspensionRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/rpc/setup-suspension");
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, format!("{}", err.body_text()).into());
                error!("{}", e.log());
                return e.into_response();
            }
        };
        match transfer_rpc_service.setup_suspension(input).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn setup_completion(
        State(transfer_rpc_service): State<Arc<T>>,
        input: Result<Json<DSRPCTransferConsumerCompletionRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/rpc/setup-completion");
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, format!("{}", err.body_text()).into());
                error!("{}", e.log());
                return e.into_response();
            }
        };
        match transfer_rpc_service.setup_completion(input).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn setup_termination(
        State(transfer_rpc_service): State<Arc<T>>,
        input: Result<Json<DSRPCTransferConsumerTerminationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/rpc/setup-termination");
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, format!("{}", err.body_text()).into());
                error!("{}", e.log());
                return e.into_response();
            }
        };
        match transfer_rpc_service.setup_termination(input).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => err.to_response(),
        }
    }
}
