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

use crate::ssi_auth::consumer::core::{consumer_vc_request, ConsumerSSIVCRequest};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType;
use reqwest::StatusCode;
use tracing::info;

pub fn router() -> Router {
    Router::new()
        .route("/ssi-auth/vc-request", post(handle_consumer_vc_request))
        .route("/ssi-auth/wf-exchange", post(handle_consumer_wf_exchange))
}

async fn handle_consumer_vc_request(Json(input): Json<ConsumerSSIVCRequest>) -> impl IntoResponse {
    info!("POST /ssi-auth/vc-request");

    match consumer_vc_request(input).await {
        Ok(_) => (StatusCode::CREATED, "OK").into_response(),
        Err(e) => TransferErrorType::NotCheckedError { inner_error: e }.into_response(),
    }
}

async fn handle_consumer_wf_exchange() -> impl IntoResponse {
    info!("POST /ssi-auth/wf-exchange");
    // match consumer_vc_request().await {
    //     Ok(_) => (StatusCode::CREATED, "OK").into_response(),
    //     Err(e) => TransferErrorType::NotCheckedError { inner_error: e }.into_response(),
    // }
}
