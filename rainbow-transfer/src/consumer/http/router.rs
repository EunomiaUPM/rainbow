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

use crate::common::utils::{does_callback_exist, is_consumer_pid_valid};
use crate::consumer::lib::control_plane::transfer_completion;
use crate::consumer::lib::control_plane::{
    transfer_start, transfer_suspension, transfer_termination,
};
use anyhow::Error;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use log::info;
use rainbow_common::err::transfer_err::TransferErrorType;
use rainbow_common::protocol::transfer::{
    TransferCompletionMessage, TransferStartMessage, TransferSuspensionMessage,
    TransferTerminationMessage,
};
use tracing::error;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route(
            "/:callback/transfers/:consumer_pid/start",
            post(handle_transfer_start),
        )
        .route(
            "/:callback/transfers/:consumer_pid/completion",
            post(handle_transfer_completion),
        )
        .route(
            "/:callback/transfers/:consumer_pid/termination",
            post(handle_transfer_termination),
        )
        .route(
            "/:callback/transfers/:consumer_pid/suspension",
            post(handle_transfer_suspension),
        )
}

async fn handle_transfer_start(
    Path((callback, consumer_pid)): Path<(Uuid, Uuid)>,
    Json(input): Json<TransferStartMessage>,
) -> impl IntoResponse {
    info!(
        "/{}/transfers/{}/start",
        callback.to_string(),
        consumer_pid.to_string()
    );

    match transfer_start(Json(&input), callback, consumer_pid).await {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => {
                error!("Unexpected error: {:?}", e_);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
    }
}

async fn handle_transfer_completion(
    Path((callback, consumer_pid)): Path<(Uuid, Uuid)>,
    Json(input): Json<TransferCompletionMessage>,
) -> impl IntoResponse {
    info!(
        "/{}/transfers/{}/start",
        callback.to_string(),
        consumer_pid.to_string()
    );

    match transfer_completion(Json(&input), callback, consumer_pid) {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => {
                error!("Unexpected error: {:?}", e_);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
    }
}

async fn handle_transfer_termination(
    Path((callback, consumer_pid)): Path<(Uuid, Uuid)>,
    Json(input): Json<TransferTerminationMessage>,
) -> impl IntoResponse {
    info!(
        "/{}/transfers/{}/start",
        callback.to_string(),
        consumer_pid.to_string()
    );

    match transfer_termination(Json(&input), callback, consumer_pid) {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => {
                error!("Unexpected error: {:?}", e_);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
    }
}

async fn handle_transfer_suspension(
    Path((callback, consumer_pid)): Path<(Uuid, Uuid)>,
    Json(input): Json<TransferSuspensionMessage>,
) -> impl IntoResponse {
    info!(
        "/{}/transfers/{}/start",
        callback.to_string(),
        consumer_pid.to_string()
    );

    match transfer_suspension(Json(&input), callback, consumer_pid) {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => match e.downcast::<TransferErrorType>() {
            Ok(transfer_error) => transfer_error.into_response(),
            Err(e_) => {
                error!("Unexpected error: {:?}", e_);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
    }
}
