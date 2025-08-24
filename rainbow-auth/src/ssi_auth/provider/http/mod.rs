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

// use crate::ssi_auth::provider::manager_xxx::Manager;
// use tracing_subscriber::fmt::format;

use crate::ssi_auth::errors::{AuthErrors, CustomToResponse};
use crate::ssi_auth::provider::core::provider_trait::RainbowSSIAuthProviderManagerTrait;
use crate::ssi_auth::provider::core::Manager;
use crate::ssi_auth::provider::utils::{create_opaque_token, extract_gnap_token};
use crate::ssi_auth::types::RefBody;
use anyhow::bail;
use axum::extract::{Form, Path, State};
use axum::http::{HeaderMap, Method, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::auth::business::RainbowBusinessLoginRequest;
use rainbow_common::auth::gnap::{AccessToken, GrantRequest, GrantResponse};
use rainbow_common::errors::{CommonErrors, ErrorInfo};
use rainbow_common::ssi_wallet::RainbowSSIAuthWalletTrait;
use rainbow_db::auth_provider::entities::mates;
use rainbow_db::auth_provider::repo_factory::factory_trait::AuthRepoFactoryTrait;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub struct RainbowAuthProviderRouter<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    pub manager: Arc<Manager<T>>,
}

impl<T> RainbowAuthProviderRouter<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    pub fn new(manager: Arc<Manager<T>>) -> Self {
        Self { manager }
    }
    pub fn router(self) -> Router {
        Router::new()
            // WALLET
            .route("/api/v1/wallet/onboard", post(Self::wallet_onboard))
            .route("/api/v1/did.json", get(Self::didweb))
            // GNAP
            .route("/api/v1/access", post(Self::access_request))
            .route("/api/v1/continue/:id", post(Self::continue_request))
            // OIDC4VP
            .route("/api/v1/pd/:state", get(Self::pd))
            .route("/api/v1/verify/:state", post(Self::verify))
            // EXTRAS
            // .route("/api/v1/verify/token", post(Self::verify_token)) // TODO
            .route("/api/v1/business/login", post(Self::fast_login))
            .with_state(self.manager)
        // .fallback(Self::fallback) 2 routers cannot have 1 fallback each
    }

    async fn wallet_onboard(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/onboard");

        match manager.onboard().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn didweb(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("GET /well-known/did.json");
        Json(manager.didweb().await.unwrap())
    }

    async fn access_request(
        State(manager): State<Arc<Manager<T>>>,
        Json(payload): Json<GrantRequest>,
    ) -> impl IntoResponse {
        info!("POST /access");

        match manager.manage_access(payload).await {
            Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn continue_request(
        State(manager): State<Arc<Manager<T>>>,
        headers: HeaderMap,
        Path(id): Path<String>,
        Json(payload): Json<RefBody>,
    ) -> impl IntoResponse {
        let log = format!("POST /continue/{}", id);
        info!("{}", log);

        let token = match extract_gnap_token(headers) {
            Some(token) => token,
            None => {
                let error = CommonErrors::InvalidError {
                    info: ErrorInfo { message: "Missing token".to_string(), error_code: 1700, details: None },
                    cause: Some("Token is missing".to_string()),
                };
                error.log();
                return error.into_response();
            }
        };

        let int_model = match manager.validate_continue_request(id, payload.interact_ref, token).await {
            Ok(model) => model,
            Err(e) => return e.to_response(),
        };

        let req_model = match manager.continue_req(int_model.clone()).await {
            Ok(model) => model,
            Err(e) => return e.to_response(),
        };

        let mate = match manager.retrieve_data(req_model.clone(), int_model).await {
            Ok(mate) => mate,
            Err(e) => return e.to_response(),
        };

        match manager.save_mate(mate).await {
            Ok(_) => (),
            Err(e) => return e.to_response(),
        }

        let res = AccessToken::default(req_model.token.unwrap());

        (StatusCode::OK, Json(res)).into_response()
    }

    async fn pd(State(manager): State<Arc<Manager<T>>>, Path(state): Path<String>) -> impl IntoResponse {
        let log = format!("GET /pd/{}", state);
        info!("{}", log);

        // COMPLETAR CON REQUIREMENTS
        match manager.generate_vp_def(state).await {
            Ok(vpd) => Json(vpd).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn verify(
        State(manager): State<Arc<Manager<T>>>,
        Path(state): Path<String>,
        Form(payload): Form<VerifyPayload>,
    ) -> impl IntoResponse {
        let log = format!("GET /verify/{}", state);
        info!("{}", log);

        // {payload.vp_token,payload.presentation_submission}

        let id = match manager.verify_all(state, payload.vp_token).await {
            Ok(id) => id,
            Err(e) => return e.to_response(),
        };

        match manager.end_verification(id).await {
            Ok(Some(uri)) => (StatusCode::OK, uri).into_response(),
            Ok(None) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    // async fn verify_token(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
    //     info!("POST /verify/token");
    //
    //     let token: String;
    // }

    async fn fast_login(
        State(manager): State<Arc<Manager<T>>>,
        Json(payload): Json<RainbowBusinessLoginRequest>,
    ) -> impl IntoResponse {
        info!("POST /generate/uri");

        let uri = match manager.fast_login(payload.auth_request_id).await {
            Ok(uri) => uri,
            Err(e) => return e.to_response(),
        };

        uri.into_response()
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
