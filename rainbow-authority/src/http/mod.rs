/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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
mod openapi;

use crate::core::traits::{AuthorityTrait, RainbowSSIAuthWalletTrait};
use crate::core::Authority;
use crate::data::repo_factory::factory_trait::AuthRepoFactoryTrait;
use crate::errors::{CustomToResponse, ErrorLog, Errors};
use crate::types::gnap::{GrantRequest, RefBody};
use crate::types::manager::VcManager;
use crate::types::oidc::VerifyPayload;
use crate::types::wallet::{DidsInfo, KeyDefinition};
use crate::utils::extract_gnap_token;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Form, Json, Router};
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info};

pub struct RainbowAuthorityRouter<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + 'static,
{
    pub authority: Arc<Authority<T>>,
}

impl<T> RainbowAuthorityRouter<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + 'static,
{
    pub fn new(authority: Arc<Authority<T>>) -> Self {
        Self { authority }
    }

    pub fn router(self) -> Router {
        Router::new()
            // WALLET
            .route("/api/v1/wallet/register", post(Self::wallet_register))
            .route("/api/v1/wallet/login", post(Self::wallet_login))
            .route("/api/v1/wallet/logout", post(Self::wallet_logout))
            .route("/api/v1/wallet/onboard", post(Self::wallet_onboard))
            .route(
                "/api/v1/wallet/partial-onboard",
                post(Self::partial_onboard),
            )
            .route("/api/v1/wallet/key", post(Self::register_key))
            .route("/api/v1/wallet/did", post(Self::register_did))
            .route("/api/v1/wallet/key", delete(Self::delete_key))
            .route("/api/v1/wallet/did", delete(Self::delete_did))
            .route("/api/v1/did.json", get(Self::didweb))
            // GNAP
            .route("/api/v1/request/credential", post(Self::access_request))
            .route("/api/v1/continue/:id", post(Self::continue_request))
            // OIDC4VP
            .route("/api/v1/pd/:state", get(Self::pd))
            .route("/api/v1/verify/:state", post(Self::verify))
            // VC REQUESTS
            .route("/api/v1/request/all", get(Self::get_all_requests))
            .route("/api/v1/request/:id", get(Self::get_one))
            .route("/api/v1/request/:id", post(Self::manage_request))
            // OTHER
            .route("/api/v1/callback/:id", post(Self::callback))
            .with_state(self.authority)
            .fallback(Self::fallback)
            .merge(openapi::route_openapi())
    }

    // WALLET ------------------------------------------------------------------------------------->

    async fn wallet_register(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/register");
        info!("POST /wallet/register");

        match manager.register_wallet().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn wallet_login(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/login");

        match manager.login_wallet().await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_logout(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/logout");

        match manager.logout_wallet().await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_onboard(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/onboard");

        match manager.onboard_wallet().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn partial_onboard(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/partial-onboard");

        match manager.partial_onboard().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn register_key(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/key");

        match manager.register_key().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn register_did(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/did");

        match manager.register_did().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn delete_key(
        State(manager): State<Arc<Authority<T>>>,
        Json(payload): Json<KeyDefinition>,
    ) -> impl IntoResponse {
        info!("DELETE /wallet/key");

        match manager.delete_key(payload).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn delete_did(State(manager): State<Arc<Authority<T>>>, Json(payload): Json<DidsInfo>) -> impl IntoResponse {
        info!("DELETE /wallet/did");

        match manager.delete_did(payload).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn didweb(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /did.json");
        match manager.get_did_doc().await {
            Ok(did) => Json(did).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn access_request(
        State(authority): State<Arc<Authority<T>>>,
        Json(payload): Json<GrantRequest>,
    ) -> impl IntoResponse {
        info!("POST /access");

        match authority.manage_access(payload).await {
            Ok(response) => (StatusCode::OK, Json(response)).into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn continue_request(
        State(authority): State<Arc<Authority<T>>>,
        headers: HeaderMap,
        Path(id): Path<String>,
        Json(payload): Json<RefBody>,
    ) -> impl IntoResponse {
        let log = format!("POST /continue/{}", id);
        info!("{}", log);

        let token = match extract_gnap_token(headers) {
            Some(token) => token,
            None => {
                let error = Errors::unauthorized_new(Some("Missing token".to_string()));
                error!("{}", error.log());
                return error.into_response();
            }
        };

        let int_model = match authority.validate_continue_request(id, payload.interact_ref, token).await {
            Ok(model) => model,
            Err(e) => return e.to_response(),
        };

        let req_model = match authority.continue_req(int_model.clone()).await {
            Ok(model) => model,
            Err(e) => return e.to_response(),
        };

        let uri = req_model.vc_uri.unwrap(); // EXPECTED ALWAYS
        uri.into_response()
    }
    async fn pd(State(authority): State<Arc<Authority<T>>>, Path(state): Path<String>) -> impl IntoResponse {
        let log = format!("GET /pd/{}", state);
        info!("{}", log);

        match authority.generate_vp_def(state).await {
            Ok(vpd) => Json(vpd).into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn verify(
        State(authority): State<Arc<Authority<T>>>,
        Path(state): Path<String>,
        Form(payload): Form<VerifyPayload>,
    ) -> impl IntoResponse {
        let log = format!("POST /verify/{}", state);
        info!("{}", log);

        // {payload.vp_token,payload.presentation_submission}

        let id = match authority.verify_all(state, payload.vp_token).await {
            Ok(id) => id,
            Err(e) => return e.to_response(),
        };

        match authority.end_verification(id).await {
            Ok(Some(uri)) => (StatusCode::OK, uri).into_response(),
            Ok(None) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_all_requests(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /request/all");

        match authority.repo.request().get_all(None, None).await {
            Ok(data) => {
                let res = serde_json::to_value(data).unwrap(); // EXPECTED ALWAYS
                (StatusCode::OK, Json(res)).into_response()
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                error.into_response()
            }
        }
    }

    async fn get_one(State(authority): State<Arc<Authority<T>>>, Path(id): Path<String>) -> impl IntoResponse {
        let log = format!("GET /request/{}", &id);
        info!("{}", log);

        match authority.repo.request().get_by_id(id.as_str()).await {
            Ok(Some(data)) => {
                let res = serde_json::to_value(data).unwrap();
                (StatusCode::OK, Json(res)).into_response()
            }
            Ok(None) => {
                let error = Errors::missing_resource_new(id.clone(), Some(format!("Missing request with id: {}", id)));
                error!("{}", error.log());
                error.into_response()
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                error.into_response()
            }
        }
    }

    async fn manage_request(
        State(authority): State<Arc<Authority<T>>>,
        Path(id): Path<String>,
        Json(payload): Json<VcManager>,
    ) -> impl IntoResponse {
        let log = format!("POST /request/{}", &id);
        info!("{}", log);

        match authority.manage_vc_request(id, payload).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn callback(
        State(authority): State<Arc<Authority<T>>>,
        Path(id): Path<String>,
        Json(payload): Json<Value>,
    ) -> impl IntoResponse {
        let log = format!("POST /callback/{}", &id);
        info!("{}", log);

        match authority.manage_callback(id, payload).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        error!("Unexpected route");
        error!("{}", log);
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
}
