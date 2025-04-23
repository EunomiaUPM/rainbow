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

// use crate::ssi_auth::provider::manager::Manager;
// use tracing_subscriber::fmt::format;

use crate::ssi_auth::provider::manager::Manager;
use axum::extract::{Form, Path};
use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::auth::{GrantRequest, GrantRequestResponse};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use tracing::info;

pub fn router() -> Router {
    Router::new()
        .route("/access", post(access_request))
        .route("/pd/:state", get(pd))
        .route("/verify/:state", post(verify))
        .fallback(fallback)
}

async fn access_request(Json(payload): Json<GrantRequest>) -> impl IntoResponse {
    info!("POST /access");

    let manager = Manager::new();
    let exchange = manager.generate_exchange_uri(payload).await;

    let res = match exchange {
        Ok((client_id, oidc4vp_uri, consumer_nonce)) => {
            GrantRequestResponse::default4oidc4vp(client_id, oidc4vp_uri, consumer_nonce)
        }
        Err(e) => GrantRequestResponse::error(e.to_string()),
    };

    Json(res)
}

async fn pd(Path(state): Path<String>) -> impl IntoResponse {
    let log = format!("GET /pd/{}", state);
    info!("{}", log);

    // COMPLETAR CON REQUIREMENTS
    match Manager::generate_vp_def(state).await {
        Ok(vpd) => Json(vpd).into_response(),
        Err(e) => {
            let body = Json(json!({"error": e.to_string()}));
            (StatusCode::BAD_REQUEST, body).into_response()
        }
    }
}

async fn verify(
    Path(state): Path<String>,
    Form(payload): Form<VerifyPayload>,
) -> impl IntoResponse {
    let log = format!("GET /verify/{}", state);
    info!("{}", log);

    // {payload.vp_token,payload.presentation_submission}

    let manager = Manager::new();
    match manager.verifyAll(state, payload.vp_token).await {
        Ok(vpd) => {}
        Err(e) => {}
    }

    StatusCode::OK
}

async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
    let log = format!("{} {}", method, uri);
    info!("{}", log);
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

// ----------------------------------------------------------------->
//
// async fn handle_petition() -> impl IntoResponse {
//     info!("POST /petition");
//
//     let uri = Manager::generate_exchange_uri().await.unwrap();
//     Json(uri)
// }
//

#[derive(Deserialize)]
struct VerifyPayload {
    vp_token: String,
    presentation_submission: String,
}
