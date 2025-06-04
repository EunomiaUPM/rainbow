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

use crate::ssi_auth::provider::core::manager::RainbowSSIAuthProviderManagerTrait;
use crate::ssi_auth::provider::core::types::RefBody;
use axum::extract::{Form, Path, State};
use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::auth::gnap::{AccessToken, GrantRequest, GrantResponse};
use rainbow_db::auth_provider::repo::AuthProviderRepoTrait;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use anyhow::bail;
use tracing::info;

pub struct RainbowAuthProviderRouter<T>
where
    T: RainbowSSIAuthProviderManagerTrait + Send + Sync + 'static,
{
    pub manager: Arc<T>,
}

impl<T> RainbowAuthProviderRouter<T>
where
    T: RainbowSSIAuthProviderManagerTrait + Send + Sync + 'static,
{
    pub fn new(manager: Arc<T>) -> Self {
        Self { manager }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/access", post(Self::access_request))
            .route("/pd/:state", get(Self::pd))
            .route("/verify/:state", post(Self::verify))
            .route("/continue", post(Self::continue_request))
            .route("/verify/token", post(Self::verify_token))
            .with_state(self.manager)
            // .fallback(Self::fallback) 2 routers cannot have 1 fallback each
    }

    async fn access_request(State(manager): State<Arc<T>>, Json(payload): Json<GrantRequest>) -> impl IntoResponse {
        info!("POST /access");

        let exchange = manager.generate_exchange_uri(payload).await;

        let res = match exchange {
            Ok((client_id, oidc4vp_uri, consumer_nonce)) => {
                GrantResponse::default4oidc4vp(client_id, oidc4vp_uri, consumer_nonce)
            }
            Err(e) => GrantResponse::error(e.to_string()),
        };

        Json(res)
    }

    async fn pd(State(manager): State<Arc<T>>, Path(state): Path<String>) -> impl IntoResponse {
        let log = format!("GET /pd/{}", state);
        info!("{}", log);

        // COMPLETAR CON REQUIREMENTS
        match manager.generate_vp_def(state).await {
            Ok(vpd) => Json(vpd).into_response(),
            Err(e) => {
                let body = Json(json!({"error": e.to_string()}));
                (StatusCode::BAD_REQUEST, body).into_response()
            }
        }
    }

    async fn verify(
        State(manager): State<Arc<T>>,
        Path(state): Path<String>,
        Form(payload): Form<VerifyPayload>,
    ) -> impl IntoResponse {
        let log = format!("GET /verify/{}", state);
        info!("{}", log);

        // {payload.vp_token,payload.presentation_submission}

        match manager.verify_all(state, payload.vp_token).await {
            Ok(Some(uri)) => (StatusCode::OK, uri).into_response(),
            Ok(None) => StatusCode::OK.into_response(),
            Err(e) => StatusCode::BAD_REQUEST.into_response(),
        }
    }

    async fn continue_request(State(manager): State<Arc<T>>, Json(payload): Json<RefBody>) -> impl IntoResponse {
        info!("POST /continue");

        let model = match manager.continue_req(payload.interact_ref).await {
            Ok(model) => model,
            Err(e) => {
                let error = json!({"error": "error"});

                return (StatusCode::BAD_GATEWAY, Json(error)).into_response();
            }
        };

        let id = model["consumer"].as_str().unwrap().to_string();
        let token = model["token"].as_str().unwrap().to_string();
        let actions = model["actions"].as_str().unwrap().to_string();
        match manager.save_mate(id, token.clone(), actions).await {
            Ok(_) => (),
            Err(e) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }

        let json = AccessToken::default(token);


        (StatusCode::OK, Json(json)).into_response()
    }

    async fn verify_token(State(manager): State<Arc<T>>) -> impl IntoResponse {
        info!("POST /verify/token");

        let token: String;


    }

    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        info!("{}", log);
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
}

#[derive(Deserialize)]
struct VerifyPayload {
    vp_token: String,
    presentation_submission: String,
}

