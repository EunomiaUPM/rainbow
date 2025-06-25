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

// use crate::ssi_auth::consumer::core::{consumer_vc_request, ConsumerSSIVCRequest};
// use anyhow::bail;
// use rainbow_common::err::transfer_err::TransferErrorType;

use crate::ssi_auth::consumer::core::manager::{RainbowSSIAuthConsumerManagerTrait, RainbowSSIAuthConsumerWalletTrait};
use crate::ssi_auth::consumer::core::types::{CallbackResponse, ReachAuthority, ReachProvider};
use crate::ssi_auth::consumer::core::Manager;
use anyhow::bail;
use axum::extract::{Path, Query, State};
use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use once_cell::sync::Lazy;
use rainbow_db::auth_consumer::repo::AuthConsumerRepoTrait;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};
use url::Url;
use urlencoding::decode;

pub struct RainbowAuthConsumerRouter<T>
where
    T: AuthConsumerRepoTrait + Send + Sync + Clone + 'static,
{
    pub manager: Arc<Mutex<Manager<T>>>,
}

impl<T> RainbowAuthConsumerRouter<T>
where
    T: AuthConsumerRepoTrait + Send + Sync + Clone + 'static,
{
    pub fn new(manager: Arc<Mutex<Manager<T>>>) -> Self {
        Self { manager }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/wallet/register", post(Self::wallet_register))
            .route("/api/v1/wallet/login", post(Self::wallet_login))
            .route("/api/v1/wallet/logout", post(Self::wallet_logout))
            .route("/api/v1/wallet/onboard", post(Self::wallet_oboard))
            .route("/api/v1/auth/ssi", post(Self::auth_ssi))
            .route("/api/v1/callback/:id", post(Self::callback))
            .route("/api/v1/auth/manual/ssi", post(Self::manual_auth_ssi))
            .route("/api/v1/callback/manual/:id", get(Self::manual_callback))
            .route("/api/v1/retrieve/token/:id", get(Self::manual_callback))
            .route("/api/v1/beg/credential", post(Self::beg4credential))
            .route("/api/v1/.well-known/did.json", get(Self::didweb)) // TODO
            // .route("/provider/:id/renew", post(todo!()))
            // .route("/provider/:id/finalize", post(todo!()))
            .with_state(self.manager)
        // .fallback(Self::fallback) 2 routers cannot have 1 fallback each
    }

    async fn wallet_register(State(manager): State<Arc<Mutex<Manager<T>>>>) -> impl IntoResponse {
        info!("POST /wallet/register");

        let mut manager = manager.lock().await;
        match manager.register_wallet().await {
            Ok(()) => StatusCode::CREATED,
            Err(e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    async fn wallet_login(State(manager): State<Arc<Mutex<Manager<T>>>>) -> impl IntoResponse {
        info!("POST /wallet/login");

        let mut manager = manager.lock().await;
        match manager.login_wallet().await {
            Ok(()) => StatusCode::OK,
            Err(e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    async fn wallet_logout(State(manager): State<Arc<Mutex<Manager<T>>>>) -> impl IntoResponse {
        info!("POST /wallet/logout");

        let mut manager = manager.lock().await;
        match manager.logout_wallet().await {
            Ok(()) => StatusCode::OK,
            Err(e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    async fn wallet_oboard(State(manager): State<Arc<Mutex<Manager<T>>>>) -> impl IntoResponse {
        info!("POST /wallet/onboard");

        let mut manager = manager.lock().await;
        match manager.onboard().await {
            Ok(()) => StatusCode::CREATED,
            Err(e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    async fn auth_ssi(
        State(manager): State<Arc<Mutex<Manager<T>>>>,
        Json(payload): Json<ReachProvider>,
    ) -> impl IntoResponse {
        info!("POST /auth/ssi");

        let mut manager = manager.lock().await;

        match manager.onboard().await {
            Ok(()) => {}
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Wallet Onboarding Failed"})),
                    )
                        .into_response()
                }
            }
        }

        let mut auth_ver;
        match manager.request_access(payload.url, payload.id, payload.slug, payload.actions).await {
            // TODO Carlos pasame did:web
            Ok(auth_ver_model) => auth_ver = auth_ver_model,
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Error contacting the provider"})),
                    )
                        .into_response()
                }
            }
        }

        let vpd_as_string = match manager.join_exchange(auth_ver.uri).await {
            Ok(texto) => texto,
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Retrieving the Presentation Definition"})),
                    )
                        .into_response()
                }
            }
        };

        let vpd = match manager.parse_vpd(vpd_as_string.clone()).await {
            Ok(json) => json,
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Parsing the Presentation Definition"})),
                    )
                        .into_response()
                }
            }
        };

        let vcs = match manager.match_vc4vp(vpd).await {
            Ok(vcs) => vcs,
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Error retrieving credentials matching the vp"})),
                    )
                        .into_response()
                }
            }
        };

        let mut creds = Vec::new();
        for vc in vcs {
            creds.push(vc.id);
        }

        let res = match manager.present_vp(vpd_as_string, creds).await {
            Ok(vcs) => (
                StatusCode::OK,
                Json(json!({
                    "TODO CORRECTO": "ASI ES"
                })),
            )
                .into_response(),
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Error presenting credentials"})),
                    )
                        .into_response()
                }
            }
        };
        res // TODO HABLAR CON CARLOS
    }


    async fn callback(
        State(manager): State<Arc<Mutex<Manager<T>>>>,
        Path(id): Path<String>,
        Json(payload): Json<CallbackResponse>,
    ) -> impl IntoResponse {
        let log = format!("GET /callback/{}", id);
        info!(log);

        let hash = payload.hash;
        let interact_ref = payload.interact_ref;

        let mut manager = manager.lock().await;

        let uri = match manager.check_callback(id.clone(), interact_ref.to_string(), hash.to_string()).await {
            Ok(uri) => uri,
            Err(e) => return StatusCode::BAD_REQUEST.into_response(),
        };

        let res = match manager.continue_request(id, interact_ref.to_string(), uri).await {
            Ok(res) => res,
            Err(e) => return StatusCode::BAD_REQUEST.into_response(),
        };

        res.token.unwrap().into_response() // TODO hablar a carlos
    }

    async fn manual_auth_ssi(
        State(manager): State<Arc<Mutex<Manager<T>>>>,
        Json(payload): Json<ReachProvider>,
    ) -> impl IntoResponse {
        info!("POST /auth/manual/ssi");

        let mut manager = manager.lock().await;

        let mut auth_ver;
        match manager.manual_request_access(payload.url, payload.id, payload.slug, payload.actions).await {
            // TODO Carlos pasame did:web
            Ok(auth_ver_model) => auth_ver = auth_ver_model,
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Error contacting the provider"})),
                    )
                        .into_response()
                }
            }
        }
        auth_ver.uri.into_response()
    }

    async fn manual_callback(
        State(manager): State<Arc<Mutex<Manager<T>>>>,
        Path(id): Path<String>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let log = format!("GET /callback/manual/{}", id);
        info!(log);

        let hash = match params.get("hash") {
            Some(hash) => hash,
            None => return (StatusCode::BAD_REQUEST, "hash not available").into_response(),
        };

        let interact_ref = match params.get("interact_ref") {
            Some(interact_ref) => interact_ref,
            None => return (StatusCode::BAD_REQUEST, "interact ref not available").into_response(),
        };

        let mut manager = manager.lock().await;

        let uri = match manager.check_callback(id.clone(), interact_ref.to_string(), hash.to_string()).await {
            Ok(uri) => uri,
            Err(e) => return (StatusCode::BAD_REQUEST, format!("check callback failed: {}", e.to_string())).into_response(),
        };

        let res = match manager.continue_request(id, interact_ref.to_string(), uri).await {
            Ok(res) => res,
            Err(e) => return (StatusCode::BAD_REQUEST, format!("continue request failed: {}", e.to_string())).into_response(),
        };

        let grant_endpoint = res.grant_endpoint.replace("/api/v1/access", "");
        match manager.save_mate(Some(res.provider_id), res.provider_slug, grant_endpoint, res.token.unwrap(), res.actions).await {
            Ok(a) => (StatusCode::CREATED, Json(a.json::<Value>().await.unwrap())).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("mate not saved: {}", e.to_string())).into_response(),
        }
    }

    async fn get_token(State(manager): State<Arc<Mutex<Manager<T>>>>, Path(id): Path<String>) -> impl IntoResponse {
        let log = format!("GET /callback/manual/{}", id);
        info!(log);

        let mut manager = manager.lock().await;
        let token = match manager.auth_repo.get_auth_by_id(id).await {
            Ok(model) => model.token,
            Err(e) => return StatusCode::BAD_REQUEST.into_response(),
        };

        match token {
            Some(token) => token.into_response(),
            None => StatusCode::BAD_REQUEST.into_response(),
        }
    }

    async fn didweb(State(manager): State<Arc<Mutex<Manager<T>>>>) -> impl IntoResponse {
        let mut manager = manager.lock().await;
        Json(manager.didweb().await.unwrap())
    }

    async fn beg4credential(State(manager): State<Arc<Mutex<Manager<T>>>>, Json(payload): Json<ReachAuthority>,) -> impl IntoResponse {
        let mut manager = manager.lock().await;
        match manager.beg4credential(payload.url).await{
            Ok(()) => {},
            Err(e) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
        StatusCode::OK.into_response()
    }
    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        info!("{}", log);
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }


}


