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

use crate::common::http::middleware::{authentication_middleware, authorization_middleware};
use crate::consumer::lib::api::{
    complete_transfer, create_new_callback, create_new_callback_with_address, get_all_callbacks,
    get_callback_by_id, get_data_address_by_consumer_pid, request_transfer, restart_transfer,
    suspend_transfer, CompleteTransferRequest, RequestTransferRequest, RequestTransferResponse,
    RestartTransferRequest, SuspendTransferRequest,
};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{middleware, Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType::{CallbackClientError, NotCheckedError};
use rainbow_common::protocol::transfer::DataAddress;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::info;
use uuid::Uuid;

pub fn router() -> Router {
    let group_1 = Router::new()
        .route("/api/v1/callbacks", get(handle_get_all_callbacks))
        .route(
            "/api/v1/callbacks/:callback_id",
            get(handle_get_callback_by_id),
        )
        .route("/api/v1/callbacks", post(handle_create_callback));

    let group_2 = Router::new()
        .route("/api/v1/setup-transfer", post(handle_create_callback))
        .route("/api/v1/request-transfer", post(handle_request_transfer))
        .route(
            "/api/v1/data-address/:consumer_pid",
            get(handle_get_data_address_by_consumer_pid),
        )
        .route("/api/v1/suspend-transfer", post(handle_suspend_transfer))
        .route("/api/v1/restart-transfer", post(handle_restart_transfer))
        .route("/api/v1/complete-transfer", post(handle_complete_transfer));

    Router::new()
        .merge(group_1)
        .merge(group_2)
        .layer(middleware::from_fn(authentication_middleware))
        .layer(middleware::from_fn(authorization_middleware))
}

#[utoipa::path(
    get,
    path = "/api/v1/callbacks",
    operation_id = "getExample",
    tag = "Example",
    description = "This endpoint retrieves an example response for demonstration purposes.",
    params(),
    responses(
        (status = 200, description = "Get todo success", body = String)
    ),
)]
pub async fn handle_get_all_callbacks() -> impl IntoResponse {
    info!("GET /api/v1/callbacks");

    match get_all_callbacks().await {
        Ok(callbacks) => (StatusCode::OK, Json(callbacks)).into_response(),
        Err(e) => NotCheckedError { inner_error: e }.into_response(),
    }
}

pub async fn handle_get_callback_by_id(Path(callback_id): Path<Uuid>) -> impl IntoResponse {
    info!("GET /api/v1/callbacks/{}", callback_id.to_string());

    match get_callback_by_id(callback_id).await {
        Ok(callback) => match callback {
            Some(callback_) => (StatusCode::OK, Json(callback_)).into_response(),
            None => CallbackClientError.into_response(),
        },
        Err(e) => NotCheckedError { inner_error: e }.into_response(),
    }
}

pub async fn handle_create_callback(data_address: Option<Json<DataAddress>>) -> impl IntoResponse {
    info!("POST /api/v1/setup-transfer");

    match data_address {
        Some(Json(address)) => match create_new_callback_with_address(address).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(e) => NotCheckedError { inner_error: e }.into_response(),
        },
        None => match create_new_callback().await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(e) => NotCheckedError { inner_error: e }.into_response(),
        },
    }
}

pub async fn handle_request_transfer(Json(input): Json<RequestTransferRequest>) -> impl IntoResponse {
    info!("POST /api/v1/request-transfer");

    match request_transfer(input).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(res) => (StatusCode::BAD_REQUEST, Json(res)).into_response(),
    }
}

pub async fn handle_get_data_address_by_consumer_pid(
    Path(consumer_pid): Path<Uuid>,
) -> impl IntoResponse {
    info!("GET /api/v1/data-address/{}", consumer_pid.to_string());

    match get_data_address_by_consumer_pid(consumer_pid).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(res) => (StatusCode::BAD_REQUEST, Json(res)).into_response(),
    }
}

pub async fn handle_suspend_transfer(Json(input): Json<SuspendTransferRequest>) -> impl IntoResponse {
    info!("POST /api/v1/suspend-transfer");

    match suspend_transfer(input).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(res) => (StatusCode::BAD_REQUEST, Json(res)).into_response(),
    }
}

pub async fn handle_restart_transfer(Json(input): Json<RestartTransferRequest>) -> impl IntoResponse {
    info!("POST /api/v1/restart-transfer");

    match restart_transfer(input).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(res) => (StatusCode::BAD_REQUEST, Json(res)).into_response(),
    }
}

pub async fn handle_complete_transfer(Json(input): Json<CompleteTransferRequest>) -> impl IntoResponse {
    info!("POST /api/v1/complete-transfer");

    match complete_transfer(input).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(res) => (StatusCode::BAD_REQUEST, Json(res)).into_response(),
    }
}
