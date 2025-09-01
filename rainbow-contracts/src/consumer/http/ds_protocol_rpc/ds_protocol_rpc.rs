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

use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_errors::DSRPCContractNegotiationConsumerErrors;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupAcceptanceRequest, SetupRequestRequest, SetupTerminationRequest, SetupVerificationRequest,
};
use crate::consumer::core::ds_protocol_rpc::DSRPCContractNegotiationConsumerTrait;
use crate::consumer::core::rainbow_entities::rainbow_entities_errors::CnErrorConsumer;
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use std::sync::Arc;
use tracing::info;

pub struct DSRPCContractNegotiationConsumerRouter<T>
where
    T: DSRPCContractNegotiationConsumerTrait + Send + Sync + 'static,
{
    service: Arc<T>,
}

impl<T> DSRPCContractNegotiationConsumerRouter<T>
where
    T: DSRPCContractNegotiationConsumerTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/negotiations/rpc/setup-request",
                post(Self::handle_setup_request),
            )
            .route(
                "/api/v1/negotiations/rpc/setup-acceptance",
                post(Self::handle_setup_acceptance),
            )
            .route(
                "/api/v1/negotiations/rpc/setup-verification",
                post(Self::handle_setup_verification),
            )
            .route(
                "/api/v1/negotiations/rpc/setup-termination",
                post(Self::handle_setup_termination),
            )
            .with_state(self.service)
    }
    async fn handle_setup_request(
        State(service): State<Arc<T>>,
        headers: HeaderMap,
        input: Result<Json<SetupRequestRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/negotiations/rpc/setup-request");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorConsumer::JsonRejection(e).into_response(),
        };

        let client_type = match headers.get("rainbow-client-type") {
            Some(header_value) => {
                match header_value.to_str() {
                    Ok(s) => s,
                    Err(e) => {
                        return NotCheckedError { inner_error: e.into() }.into_response();
                    }
                }
            }
            None => "standard",
        }.to_string();

        let is_rerequest = input.provider_pid.clone().is_some() && input.consumer_pid.clone().is_some();
        match is_rerequest {
            false => match service.setup_request(input, client_type).await {
                Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
                Err(err) => match err.downcast::<CnErrorConsumer>() {
                    Ok(e_) => e_.into_response(),
                    Err(e_) => match e_.downcast::<DSRPCContractNegotiationConsumerErrors>() {
                        Ok(e__) => e__.into_response(),
                        Err(e__) => NotCheckedError { inner_error: e__ }.into_response(),
                    },
                },
            }
            true => match service.setup_rerequest(input).await {
                Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
                Err(err) => match err.downcast::<CnErrorConsumer>() {
                    Ok(e_) => e_.into_response(),
                    Err(e_) => match e_.downcast::<DSRPCContractNegotiationConsumerErrors>() {
                        Ok(e__) => e__.into_response(),
                        Err(e__) => NotCheckedError { inner_error: e__ }.into_response(),
                    },
                },
            }
        }
    }
    async fn handle_setup_acceptance(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupAcceptanceRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/negotiations/rpc/setup-acceptance");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorConsumer::JsonRejection(e).into_response(),
        };
        match service.setup_acceptance(input).await {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => match err.downcast::<CnErrorConsumer>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => match e_.downcast::<DSRPCContractNegotiationConsumerErrors>() {
                    Ok(e__) => e__.into_response(),
                    Err(e__) => NotCheckedError { inner_error: e__ }.into_response(),
                },
            },
        }
    }
    async fn handle_setup_verification(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupVerificationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/negotiations/rpc/setup-verification");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorConsumer::JsonRejection(e).into_response(),
        };
        match service.setup_verification(input).await {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => match err.downcast::<CnErrorConsumer>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => match e_.downcast::<DSRPCContractNegotiationConsumerErrors>() {
                    Ok(e__) => e__.into_response(),
                    Err(e__) => NotCheckedError { inner_error: e__ }.into_response(),
                },
            },
        }
    }
    async fn handle_setup_termination(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupTerminationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/negotiations/rpc/setup-termination");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorConsumer::JsonRejection(e).into_response(),
        };
        match service.setup_termination(input).await {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => match err.downcast::<CnErrorConsumer>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => match e_.downcast::<DSRPCContractNegotiationConsumerErrors>() {
                    Ok(e__) => e__.into_response(),
                    Err(e__) => NotCheckedError { inner_error: e__ }.into_response(),
                },
            },
        }
    }
}
