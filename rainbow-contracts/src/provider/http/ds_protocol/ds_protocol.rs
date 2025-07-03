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

use crate::provider::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use crate::provider::core::ds_protocol::DSProtocolContractNegotiationProviderTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{middleware, Extension, Json, Router};
use rainbow_common::auth::header::{extract_request_info, RequestInfo};
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::ContractNegotiationEventMessage;
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::utils::get_urn_from_string;
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::info;

pub struct DSProtocolContractNegotiationProviderRouter<T>
where
    T: DSProtocolContractNegotiationProviderTrait + Send + Sync + 'static,
{
    service: Arc<T>,
}

impl<T> DSProtocolContractNegotiationProviderRouter<T>
where
    T: DSProtocolContractNegotiationProviderTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/negotiations/:provider_pid",
                get(Self::handle_get_negotiations),
            )
            .route("/negotiations/request", post(Self::handle_post_request))
            .route(
                "/negotiations/:provider_pid/request",
                post(Self::handle_post_provider_request),
            )
            .route(
                "/negotiations/:provider_pid/events",
                post(Self::handle_post_provider_events),
            )
            .route(
                "/negotiations/:provider_pid/agreement/verification",
                post(Self::handle_post_provider_agreement_verification),
            )
            .route(
                "/negotiations/:provider_pid/termination",
                post(Self::handle_post_provider_termination),
            )
            .layer(middleware::from_fn(extract_request_info))
            .with_state(self.service)
    }

    async fn handle_get_negotiations(
        State(service): State<Arc<T>>,
        Path(provider_pid): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /negotiations/{}", provider_pid.to_string());
        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(err) => {
                return IdsaCNError::UUIDParseError {
                    provider_pid: Option::from(provider_pid.clone()),
                    consumer_pid: None,
                    error: err.to_string(),
                }
                    .into_response()
            }
        };
        match service.get_negotiation(provider_pid.clone()).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => IdsaCNError::NotCheckedError {
                    provider_pid: Option::from(provider_pid.clone().to_string()),
                    consumer_pid: None,
                    error: err_.to_string(),
                }
                    .into_response(),
            },
        }
    }

    async fn handle_post_request(
        State(service): State<Arc<T>>,
        headers: HeaderMap,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<ContractRequestMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /negotiations/request");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
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
        let token = info.token.clone();
        match service.post_request(input, token, client_type).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => {
                    IdsaCNError::NotCheckedError { provider_pid: None, consumer_pid: None, error: err_.to_string() }
                        .into_response()
                }
            },
        }
    }

    async fn handle_post_provider_request(
        State(service): State<Arc<T>>,
        Path(provider_pid): Path<String>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<ContractRequestMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /negotiations/{}/request", provider_pid.to_string());
        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(err) => {
                return IdsaCNError::UUIDParseError {
                    provider_pid: Option::from(provider_pid.clone()),
                    consumer_pid: None,
                    error: err.to_string(),
                }
                    .into_response()
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
        };
        let token = info.token.clone();
        match service.post_provider_request(provider_pid, input, token).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => {
                    IdsaCNError::NotCheckedError { provider_pid: None, consumer_pid: None, error: err_.to_string() }
                        .into_response()
                }
            },
        }
    }

    async fn handle_post_provider_events(
        State(service): State<Arc<T>>,
        Path(provider_pid): Path<String>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<ContractNegotiationEventMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /negotiations/{}/events", provider_pid.to_string());
        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(err) => {
                return IdsaCNError::UUIDParseError {
                    provider_pid: Option::from(provider_pid.clone()),
                    consumer_pid: None,
                    error: err.to_string(),
                }
                    .into_response()
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
        };
        let token = info.token.clone();
        match service.post_provider_events(provider_pid, input, token).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => {
                    IdsaCNError::NotCheckedError { provider_pid: None, consumer_pid: None, error: err_.to_string() }
                        .into_response()
                }
            },
        }
    }

    async fn handle_post_provider_agreement_verification(
        State(service): State<Arc<T>>,
        Path(provider_pid): Path<String>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<ContractAgreementVerificationMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "POST /negotiations/{}/agreement/verification",
            provider_pid.to_string()
        );
        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
        };
        let token = info.token.clone();
        match service.post_provider_agreement_verification(provider_pid, input, token).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => {
                    IdsaCNError::NotCheckedError { provider_pid: None, consumer_pid: None, error: err_.to_string() }
                        .into_response()
                }
            },
        }
    }

    async fn handle_post_provider_termination(
        State(service): State<Arc<T>>,
        Path(provider_pid): Path<String>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<ContractTerminationMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "POST /negotiations/{}/termination",
            provider_pid.to_string()
        );
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
        };
        let provider_pid = match get_urn_from_string(&provider_pid) {
            Ok(provider_pid) => provider_pid,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        let token = info.token.clone();
        match service.post_provider_termination(provider_pid, input, token).await {
            Ok(negotiation) => negotiation.into_response(),
            Err(err) => match err.downcast::<IdsaCNError>() {
                Ok(err_) => err_.into_response(),
                Err(err_) => {
                    IdsaCNError::NotCheckedError { provider_pid: None, consumer_pid: None, error: err_.to_string() }
                        .into_response()
                }
            },
        }
    }
}
