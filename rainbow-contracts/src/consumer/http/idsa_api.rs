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

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
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
        .route("/negotiations/offers", post(handle_post_offers))
        .route(
            "/:callback_id/negotiations/:consumer_pid/offers",
            post(handle_post_consumer_offers),
        )
        .route(
            "/:callback_id/negotiations/:consumer_pid/agreement",
            post(handle_post_agreement),
        )
        .route(
            "/:callback_id/negotiations/:consumer_pid/events",
            post(handle_post_events),
        )
        .route(
            "/:callback_id/negotiations/:consumer_pid/termination",
            post(handle_post_termination),
        )
}

async fn handle_post_offers(Json(input): Json<ContractOfferMessage>) -> impl IntoResponse {
    info!("POST /negotiations/offers");
}

async fn handle_post_consumer_offers(
    Path((callback_id, consumer_pid)): Path<(String, String)>,
    Json(input): Json<ContractOfferMessage>,
) -> impl IntoResponse {
    info!(
        "POST /{}/negotiations/{}/offers",
        callback_id.to_string(),
        consumer_pid.to_string()
    );
    let callback_id = match get_urn_from_string(&callback_id) {
        Ok(callback_id) => callback_id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let consumer_pid = match get_urn_from_string(&consumer_pid) {
        Ok(consumer_pid) => consumer_pid,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    (StatusCode::OK, "Ok").into_response()
}

async fn handle_post_agreement(
    Path((callback_id, consumer_pid)): Path<(String, String)>,
    Json(input): Json<ContractAgreementMessage>,
) -> impl IntoResponse {
    info!(
        "POST /{}/negotiations/{}/agreement",
        callback_id.to_string(),
        consumer_pid.to_string()
    );
    let callback_id = match get_urn_from_string(&callback_id) {
        Ok(callback_id) => callback_id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let consumer_pid = match get_urn_from_string(&consumer_pid) {
        Ok(consumer_pid) => consumer_pid,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    (StatusCode::OK, "Ok").into_response()
}

async fn handle_post_events(
    Path((callback_id, consumer_pid)): Path<(String, String)>,
    Json(input): Json<ContractNegotiationEventMessage>,
) -> impl IntoResponse {
    info!(
        "POST /{}/negotiations/{}/events",
        callback_id.to_string(),
        consumer_pid.to_string()
    );
    let callback_id = match get_urn_from_string(&callback_id) {
        Ok(callback_id) => callback_id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let consumer_pid = match get_urn_from_string(&consumer_pid) {
        Ok(consumer_pid) => consumer_pid,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    (StatusCode::OK, "Ok").into_response()
}

async fn handle_post_termination(
    Path((callback_id, consumer_pid)): Path<(String, String)>,
    Json(input): Json<ContractTerminationMessage>,
) -> impl IntoResponse {
    info!(
        "POST /{}/negotiations/{}/termination",
        callback_id.to_string(),
        consumer_pid.to_string()
    );
    let callback_id = match get_urn_from_string(&callback_id) {
        Ok(callback_id) => callback_id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let consumer_pid = match get_urn_from_string(&consumer_pid) {
        Ok(consumer_pid) => consumer_pid,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    (StatusCode::OK, "Ok").into_response()
}
