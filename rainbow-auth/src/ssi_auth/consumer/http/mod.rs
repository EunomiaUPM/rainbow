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

use crate::ssi_auth::consumer::core::consumer_trait::RainbowSSIAuthConsumerManagerTrait;
use crate::ssi_auth::consumer::core::Manager;
use crate::ssi_auth::errors::CustomToResponse;
use crate::ssi_auth::types::ReachProvider;
use anyhow::bail;
use axum::extract::{Path, Query, State};
use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use once_cell::sync::Lazy;
use rainbow_common::ssi_wallet::RainbowSSIAuthWalletTrait;
use rainbow_db::auth_consumer::entities::mates;
use rainbow_db::auth_consumer::repo_factory::factory_trait::AuthRepoFactoryTrait;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};
use url::Url;
use urlencoding::decode;

pub struct RainbowAuthConsumerRouter<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    pub manager: Arc<Manager<T>>,
}

impl<T> RainbowAuthConsumerRouter<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    pub fn new(manager: Arc<Manager<T>>) -> Self {
        Self { manager }
    }

    pub fn router(self) -> Router {
        Router::new()
            // WALLET
            .route("/api/v1/wallet/register", post(Self::wallet_register))
            .route("/api/v1/wallet/login", post(Self::wallet_login))
            .route("/api/v1/wallet/logout", post(Self::wallet_logout))
            .route("/api/v1/wallet/onboard", post(Self::wallet_onboard))
            .route("/api/v1/did.json", get(Self::didweb))
            // PROVIDER
            .route(
                "/api/v1/request/onboard/provider",
                post(Self::request_provider_onboard),
            )
            .route("/api/v1/callback/:id", get(Self::callback))
            // 4 MICROSERVICES
            // .route("/api/v1/retrieve/token/:id", get(Self::manual_callback))
            // AUTHORITY
            // .route("/api/v1/beg/credential", post(Self::beg4credential))
            // .route("/provider/:id/renew", post(todo!()))
            // .route("/provider/:id/finalize", post(todo!()))
            .with_state(self.manager)
        // .fallback(Self::fallback) 2 routers cannot have 1 fallback each
    }

    // WALLET ------------------------------------------------------------------------------------->

    async fn wallet_register(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/register");

        match manager.register_wallet().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn wallet_login(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/login");

        match manager.login_wallet().await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_logout(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/logout");

        match manager.logout_wallet().await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_onboard(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/onboard");

        match manager.onboard().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn didweb(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("GET /did.json");
        Json(manager.didweb().await.unwrap())
    }

    // DATASPACE PROVIDER ------------------------------------------------------------------------->

    async fn request_provider_onboard(
        State(manager): State<Arc<Manager<T>>>,
        Json(payload): Json<ReachProvider>,
    ) -> impl IntoResponse {
        info!("POST /auth/manual/ssi");

        let uri = match manager.request_onboard_provider(payload.url, payload.id, payload.slug).await {
            Ok(uri) => uri,
            Err(e) => return e.to_response(),
        };
        println!("{}", uri);
        uri.into_response()
    }

    async fn callback(
        State(manager): State<Arc<Manager<T>>>,
        Path(id): Path<String>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let log = format!("GET /callback/manual/{}", id);
        info!(log);

        let hash = match params.get("hash") {
            Some(hash) => hash,
            None => {
                error!("Unable to retrieve hash callback");
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Unable to retrieve hash from callback",
                        "error_code": 1200
                    })),
                )
                    .into_response();
            }
        };

        let interact_ref = match params.get("interact_ref") {
            Some(interact_ref) => interact_ref,
            None => {
                error!("Unable to retrieve interact reference");
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Unable to retrieve interact reference",
                        "error_code": 1200
                    })),
                )
                    .into_response();
            }
        };

        match manager.check_callback(id.clone(), interact_ref.to_string(), hash.to_string()).await {
            Ok(()) => {}
            Err(e) => return e.to_response(),
        };

        let request_model = match manager.continue_request(id, interact_ref.to_string()).await {
            Ok(res) => res,
            Err(e) => return e.to_response(),
        };

        let mate = mates::NewModel {
            participant_id: request_model.provider_id,
            participant_slug: request_model.provider_slug,
            participant_type: "Provider".to_string(),
            base_url: request_model.grant_endpoint,
            token: request_model.token,
            is_me: false,
        };

        match manager.save_mate(mate.clone()).await {
            Ok(model) => (StatusCode::CREATED, Json(model)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    // async fn beg4credential(
    //     State(manager): State<Arc<Manager<T>>>,
    //     Json(payload): Json<ReachAuthority>,
    // ) -> impl IntoResponse {
    //     info!("POST /beg/credential");
    //     match manager.beg4credential(payload.url).await {
    //         Ok(()) => {}
    //         Err(e) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    //     };
    //     StatusCode::OK.into_response()
    // }

    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        info!("{}", log);
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
}
