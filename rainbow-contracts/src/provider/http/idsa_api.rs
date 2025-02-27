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

use crate::provider::core::idsa_api::*;
use crate::provider::core::idsa_api_errors::IdsaCNError;
use crate::provider::core::rainbow_cn_errors::CnErrorProvider;
use anyhow::Error;
use axum::extract::rejection::JsonRejection;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_error::ContractErrorMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::ContractNegotiationEventMessage;
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::utils::get_urn_from_string;
use reqwest::StatusCode;
use serde_json::Value;
use tracing::info;

pub fn router() -> Router {
    Router::new()
        .route("/negotiations/:provider_pid", get(handle_get_negotiations))
        .route("/negotiations/request", post(handle_post_request))
        .route(
            "/negotiations/:provider_pid/request",
            post(handle_post_provider_request),
        )
        .route(
            "/negotiations/:provider_pid/events",
            post(handle_post_provider_events),
        )
        .route(
            "/negotiations/:provider_pid/agreement/verification",
            post(handle_post_provider_agreement_verification),
        )
        .route(
            "/negotiations/:provider_pid/termination",
            post(handle_post_provider_termination),
        )
}

async fn handle_get_negotiations(Path(provider_pid): Path<String>) -> impl IntoResponse {
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
    match get_negotiation(provider_pid.clone()).await {
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
    input: Result<Json<ContractRequestMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /negotiations/request");
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    match post_request(input).await {
        Ok(negotiation) => negotiation.into_response(),
        Err(err) => match err.downcast::<IdsaCNError>() {
            Ok(err_) => err_.into_response(),
            Err(err_) => IdsaCNError::NotCheckedError {
                provider_pid: None,
                consumer_pid: None,
                error: err_.to_string(),
            }
                .into_response(),
        },
    }
}

async fn handle_post_provider_request(
    Path(provider_pid): Path<String>,
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

    match post_provider_request(provider_pid, input).await {
        Ok(negotiation) => negotiation.into_response(),
        Err(err) => match err.downcast::<IdsaCNError>() {
            Ok(err_) => err_.into_response(),
            Err(err_) => IdsaCNError::NotCheckedError {
                provider_pid: None,
                consumer_pid: None,
                error: err_.to_string(),
            }
                .into_response(),
        },
    }
}

async fn handle_post_provider_events(
    Path(provider_pid): Path<String>,
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

    match post_provider_events(&provider_pid, &input).await {
        Ok(negotiation) => negotiation.into_response(),
        Err(err) => match err.downcast::<IdsaCNError>() {
            Ok(err_) => err_.into_response(),
            Err(err_) => IdsaCNError::NotCheckedError {
                provider_pid: None,
                consumer_pid: None,
                error: err_.to_string(),
            }
                .into_response(),
        },
    }
}

async fn handle_post_provider_agreement_verification(
    Path(provider_pid): Path<String>,
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
    match post_provider_agreement_verification(provider_pid, input).await {
        Ok(negotiation) => negotiation.into_response(),
        Err(err) => match err.downcast::<IdsaCNError>() {
            Ok(err_) => err_.into_response(),
            Err(err_) => IdsaCNError::NotCheckedError {
                provider_pid: None,
                consumer_pid: None,
                error: err_.to_string(),
            }
                .into_response(),
        },
    }
}

async fn handle_post_provider_termination(
    Path(provider_pid): Path<String>,
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
    match post_provider_termination(provider_pid, input).await {
        Ok(negotiation) => negotiation.into_response(),
        Err(err) => match err.downcast::<IdsaCNError>() {
            Ok(err_) => err_.into_response(),
            Err(err_) => IdsaCNError::NotCheckedError {
                provider_pid: None,
                consumer_pid: None,
                error: err_.to_string(),
            }
                .into_response(),
        },
    }
}
