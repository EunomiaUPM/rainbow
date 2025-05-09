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

use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupAgreementRequest, SetupFinalizationRequest, SetupOfferRequest, SetupTerminationRequest,
};
use crate::provider::core::ds_protocol_rpc::DSRPCContractNegotiationProviderTrait;
use crate::provider::core::rainbow_entities::rainbow_entities_errors::CnErrorProvider;
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use std::sync::Arc;
use tracing::info;

pub struct DSRPCContractNegotiationProviderRouter<T>
where
    T: DSRPCContractNegotiationProviderTrait + Send + Sync + 'static,
{
    service: Arc<T>,
}

impl<T> DSRPCContractNegotiationProviderRouter<T>
where
    T: DSRPCContractNegotiationProviderTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/negotiations/rpc/setup-offer",
                post(Self::handle_setup_offer),
            )
            .route(
                "/api/v1/negotiations/rpc/setup-agreement",
                post(Self::handle_setup_agreement),
            )
            .route(
                "/api/v1/negotiations/rpc/setup-finalization",
                post(Self::handle_setup_finalization),
            )
            .route(
                "/api/v1/negotiations/rpc/setup-termination",
                post(Self::handle_setup_termination),
            )
            .with_state(self.service)
    }

    async fn handle_setup_offer(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupOfferRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/contract-negotiation/rpc/setup-offer");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
        };
        let is_reoffer = input.provider_pid.clone().is_some() && input.consumer_pid.clone().is_some();
        match is_reoffer {
            false => match service.setup_offer(input).await {
                Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
                Err(err) => match err.downcast::<CnErrorProvider>() {
                    Ok(e_) => e_.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                },
            },
            true => match service.setup_reoffer(input).await {
                Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
                Err(err) => match err.downcast::<CnErrorProvider>() {
                    Ok(e_) => e_.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                },
            },
        }
    }

    async fn handle_setup_agreement(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupAgreementRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/contract-negotiation/rpc/setup-agreement");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
        };
        match service.setup_agreement(input).await {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }

    async fn handle_setup_finalization(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupFinalizationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/contract-negotiation/rpc/setup-finalization");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
        };
        match service.setup_finalization(input).await {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }

    async fn handle_setup_termination(
        State(service): State<Arc<T>>,
        input: Result<Json<SetupTerminationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/contract-negotiation/rpc/setup-termination");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CnErrorProvider::JsonRejection(e).into_response(),
        };
        match service.setup_termination(input).await {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => match err.downcast::<CnErrorProvider>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
            },
        }
    }
}
