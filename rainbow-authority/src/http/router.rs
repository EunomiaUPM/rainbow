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
use crate::core::{Authority, AuthorityTrait};
use crate::errors::{CustomToResponse, ErrorLogTrait, Errors};
use crate::http::openapi;
use crate::services::repo::RepoFactoryTrait;
use crate::types::enums::errors::BadFormat;
use crate::types::gnap::{GrantRequest, RefBody};
use crate::types::oidc::VerifyPayload;
use crate::types::vcs::VcDecisionApproval;
use crate::types::wallet::{DidsInfo, KeyDefinition};
use crate::utils::extract_gnap_token;
use axum::extract::{rejection::JsonRejection, Path, Query, State};
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Form, Json, Router};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info};

pub struct RainbowAuthorityRouter<T>
where
    T: RepoFactoryTrait + Send + Sync + 'static,
{
    pub authority: Arc<Authority<T>>,
}

impl<T> RainbowAuthorityRouter<T>
where
    T: RepoFactoryTrait + Send + Sync + 'static,
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
                post(Self::wallet_partial_onboard),
            )
            .route("/api/v1/wallet/key", post(Self::register_key))
            .route("/api/v1/wallet/did", post(Self::register_did))
            .route("/api/v1/wallet/key", delete(Self::delete_key))
            .route("/api/v1/wallet/did", delete(Self::delete_did))
            .route("/api/v1/did.json", get(Self::did_json))
            // GNAP
            .route("/api/v1/request/credential", post(Self::vc_access_request))
            .route(
                "/api/v1/request/continue/:id",
                post(Self::vc_continue_request),
            )
            // OIDC4VP
            .route("/api/v1/pd/:state", get(Self::vp_definition))
            .route("/api/v1/verify/:state", post(Self::verify))
            // OIDC4VCI
            .route("/api/v1/credentialOffer", get(Self::cred_offer))
            // ISSUER DATA
            .route(
                "/api/v1/.well-known/openid-credential-issuer",
                get(Self::get_issuer),
            )
            .route(
                "/api/v1/.well-known/oauth-authorization-server",
                get(Self::get_oauth_server),
            )
            .route("/api/v1/jwks", get(Self::get_jwks))
            .route("/api/v1/token", post(Self::get_token))
            .route("/api/v1/credential", post(Self::post_credential))
            // VC REQUESTS
            .route("/api/v1/request/all", get(Self::get_all_requests))
            .route("/api/v1/request/:id", get(Self::get_one_request))
            .route("/api/v1/request/:id", post(Self::manage_request))
            // OTHER
            // .route("/api/v1/callback/:id", post(Self::callback))
            .with_state(self.authority)
            .fallback(Self::fallback)
            .merge(openapi::route_openapi())
    }

    // WALLET ------------------------------------------------------------------------------------->
    async fn wallet_register(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/register");

        match authority.wallet_register().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_login(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/login");

        match authority.wallet_login().await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_logout(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/logout");

        match authority.wallet_logout().await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_onboard(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/onboard");

        match authority.wallet_onboard().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_partial_onboard(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/partial-onboard");

        match authority.wallet_partial_onboard().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn register_key(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/key");

        match authority.register_key().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn register_did(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/did");

        match authority.register_did().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn delete_key(
        State(authority): State<Arc<Authority<T>>>,
        payload: Result<Json<KeyDefinition>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("DELETE /wallet/key");

        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match authority.delete_key(payload).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn delete_did(
        State(authority): State<Arc<Authority<T>>>,
        payload: Result<Json<DidsInfo>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("DELETE /wallet/did");

        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match authority.delete_did(payload).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn did_json(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /did.json");

        match authority.did_json().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn vc_access_request(
        State(authority): State<Arc<Authority<T>>>,
        payload: Result<Json<GrantRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /request/credential");

        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match authority.vc_access_request(payload).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn vc_continue_request(
        State(authority): State<Arc<Authority<T>>>,
        headers: HeaderMap,
        Path(id): Path<String>,
        payload: Result<Json<RefBody>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /request/continue");

        let token = match extract_gnap_token(headers) {
            Some(token) => token,
            None => {
                let error = Errors::unauthorized_new("Missing token".to_string());
                error!("{}", error.log());
                return error.into_response();
            }
        };

        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match authority.vc_continue_request(id, payload, token).await {
            Ok(data) => data.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn vp_definition(State(authority): State<Arc<Authority<T>>>, Path(state): Path<String>) -> impl IntoResponse {
        let log = format!("GET /pd/{}", state);
        info!("{}", log);

        match authority.generate_vp_def(state).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
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

        match authority.verify(state, payload.vp_token).await {
            Ok(Some(uri)) => (StatusCode::OK, uri).into_response(),
            Ok(None) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn cred_offer(
        State(authority): State<Arc<Authority<T>>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let id = match params.get("id") {
            Some(hash) => hash.clone(),
            None => {
                info!("GET /credentialOffer");
                let error = Errors::format_new(
                    BadFormat::Received,
                    "Unable to retrieve hash from callback".to_string(),
                );
                error!("{}", error.log());
                return error.into_response();
            }
        };
        let log = format!("GET /credentialOffer/{}", id);
        info!("{}", log);

        match authority.get_cred_offer_data(id).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_issuer(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /issuer");

        (StatusCode::OK, Json(authority.issuer())).into_response()
    }

    async fn get_oauth_server(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /oauth_server");

        (StatusCode::OK, Json(authority.oauth_server())).into_response()
    }

    async fn get_jwks(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /jwks_data");

        match authority.jwks() {
            Ok(jwk) => (StatusCode::OK, Json(jwk)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_token(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /token");

        (StatusCode::OK, Json(authority.token())).into_response()
    }

    async fn post_credential(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /credential");

        (StatusCode::OK, Json(authority.credential())).into_response()
    }

    async fn get_all_requests(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /request/all");

        match authority.get_all_req().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_one_request(State(authority): State<Arc<Authority<T>>>, Path(id): Path<String>) -> impl IntoResponse {
        let log = format!("GET /request/{}", &id);
        info!("{}", log);

        match authority.get_one_req(id).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn manage_request(
        State(authority): State<Arc<Authority<T>>>,
        Path(id): Path<String>,
        payload: Result<Json<VcDecisionApproval>, JsonRejection>,
    ) -> impl IntoResponse {
        let log = format!("POST /request/{}", &id);
        info!("{}", log);

        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match authority.manage_req(id, payload).await {
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
