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

use super::super::core::idsa_api::post_offers;
use crate::consumer::core::idsa_api::{post_agreement, post_consumer_offers, post_events, post_termination};
use crate::consumer::core::idsa_api_errors::IdsaCNError;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{middleware, Extension, Json, Router};
use rainbow_common::protocol::contract::contract_agreement::ContractAgreementMessage;
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::ContractNegotiationEventMessage;
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_offer::ContractOfferMessage;
use rainbow_common::utils::get_urn_from_string;
use reqwest::StatusCode;
use serde_json::Value;
use tracing::info;

pub fn router() -> Router {
    Router::new()
        .route(
            "/:callback_id/negotiations/:consumer_pid/offers",
            post(handle_post_consumer_offers),
        )
        .route(
            "/negotiations/:consumer_pid/offers",
            post(handle_post_consumer_offers),
        )
        .route(
            "/:callback_id/negotiations/:consumer_pid/agreement",
            post(handle_post_agreement),
        )
        .route(
            "/negotiations/:consumer_pid/agreement",
            post(handle_post_agreement),
        )
        .route(
            "/:callback_id/negotiations/:consumer_pid/events",
            post(handle_post_events),
        )
        .route(
            "/negotiations/:consumer_pid/events",
            post(handle_post_events),
        )
        .route(
            "/:callback_id/negotiations/:consumer_pid/termination",
            post(handle_post_termination),
        )
        .route(
            "/negotiations/:consumer_pid/termination",
            post(handle_post_termination),
        )
        .route_layer(middleware::from_fn(extract_params))
        .route("/negotiations/offers", post(handle_post_offers))
}

///
/// # Extraction of params feature
/// This feature is used to extract the parameters from the URL and add them to the request extensions.
#[derive(Clone)]
struct ExtractedParams {
    callback_id: Option<String>,
    consumer_pid: String,
}

async fn extract_params(req: Request, next: Next) -> Response {
    let uri = req.uri().path();
    let parts: Vec<&str> = uri.split('/').filter(|s| !s.is_empty()).collect();
    let (callback_id, consumer_pid) = if parts.len() == 3 {
        (None, parts[1].to_string())
    } else {
        (Some(parts[0].to_string()), parts[2].to_string())
    };
    let mut req = req;
    req.extensions_mut().insert(ExtractedParams { callback_id, consumer_pid });
    next.run(req).await
}

async fn handle_post_offers(
    input: Result<Json<ContractOfferMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /negotiations/offers");
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    match post_offers(input).await {
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

async fn handle_post_consumer_offers(
    Extension(params): Extension<ExtractedParams>,
    input: Result<Json<ContractOfferMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!(
        "POST /{}/negotiations/{}/offers",
        params.callback_id.unwrap_or("".to_string()),
        params.consumer_pid.to_string()
    );
    let consumer_pid = match get_urn_from_string(&params.consumer_pid) {
        Ok(consumer_pid) => consumer_pid,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    match post_consumer_offers(consumer_pid, input).await {
        Ok(negotiation) => negotiation.into_response(),
        Err(err) => match err.downcast::<IdsaCNError>() {
            Ok(err_) => err_.into_response(),
            Err(err_) => IdsaCNError::NotCheckedError {
                provider_pid: None,
                consumer_pid: None,
                error: err_.to_string(),
            }
                .into_response(),
        }
    }
}

async fn handle_post_agreement(
    Extension(params): Extension<ExtractedParams>,
    input: Result<Json<ContractAgreementMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!(
        "POST /{}/negotiations/{}/agreement",
        params.callback_id.unwrap_or("".to_string()),
        params.consumer_pid.to_string()
    );
    let consumer_pid = match get_urn_from_string(&params.consumer_pid) {
        Ok(consumer_pid) => consumer_pid,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    match post_agreement(consumer_pid, input).await {
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

async fn handle_post_events(
    Extension(params): Extension<ExtractedParams>,
    input: Result<Json<ContractNegotiationEventMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!(
        "POST /{}/negotiations/{}/events",
        params.callback_id.unwrap_or("".to_string()),
        params.consumer_pid.to_string()
    );
    let consumer_pid = match get_urn_from_string(&params.consumer_pid) {
        Ok(consumer_pid) => consumer_pid,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    match post_events(consumer_pid, input).await {
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

async fn handle_post_termination(
    Extension(params): Extension<ExtractedParams>,
    input: Result<Json<ContractTerminationMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!(
        "POST /{}/negotiations/{}/termination",
        params.callback_id.unwrap_or("".to_string()),
        params.consumer_pid.to_string()
    );
    let consumer_pid = match get_urn_from_string(&params.consumer_pid) {
        Ok(consumer_pid) => consumer_pid,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let input = match input {
        Ok(input) => input.0,
        Err(e) => return IdsaCNError::JsonRejection(e).into_response(),
    };
    match post_termination(consumer_pid, input).await {
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
