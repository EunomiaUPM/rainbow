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

use crate::ssi_auth::provider::manager::Manager;
use axum::extract::{Form, Path};
use axum::http::{Method, Request, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType;
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::info;
use tracing_subscriber::fmt::format;

pub fn router() -> Router {
    Router::new()
        .route("/petition", post(handle_petition))
        .route("/pd/:state", get(pd))
        .route("/verify/:state", post(verify))
        .fallback(fallback)
}

async fn handle_petition() -> impl IntoResponse {
    info!("POST /petition");

    let uri = Manager::generate_exchange_uri().await.unwrap();
    Json(uri)
}

async fn pd(Path(state): Path<String>) -> impl IntoResponse {
    let log = format!("GET /pd/{}", state);
    info!("{}", log);

    let vpd = Manager::gererate_vp_def();
    Json(vpd)
}

#[derive(Deserialize)]
struct VerifyPayload {
    vp_token: String,
    presentation_submission: String,
}

async fn verify(
    Path(state): Path<String>,
    Form(payload): Form<VerifyPayload>,
) -> impl IntoResponse {
    let log = format!("GET /verify/{}", state);
    info!("{}", log);

    // {payload.vp_token,payload.presentation_submission}

    let manager = Manager::new();
    manager.verify(payload.vp_token).await.unwrap();




    StatusCode::OK
}

async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
    let kk = format!("{} {}", method, uri);
    info!("{}", kk);
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
