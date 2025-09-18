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

use crate::ssi_auth::consumer::core::traits::consumer_trait::RainbowSSIAuthConsumerManagerTrait;
use crate::ssi_auth::consumer::core::Manager;
use crate::ssi_auth::errors::CustomToResponse;
use crate::ssi_auth::types::entities::{ReachAuthority, ReachProvider};
use crate::ssi_auth::types::gnap::CallbackBody;
use axum::extract::{Path, Query, State};
use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::ssi_wallet::RainbowSSIAuthWalletTrait;
use rainbow_db::auth_consumer::repo_factory::factory_trait::AuthRepoFactoryTrait;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info};

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
            .route("/api/v1/callback/:id", get(Self::get_callback))
            .route("/api/v1/callback/:id", post(Self::post_callback))
            // 4 MICROSERVICES
            // .route("/api/v1/retrieve/token/:id", get(Self::manual_callback))
            // AUTHORITY
            .route("/api/v1/authority/beg", post(Self::beg4credential))
            .route("/api/v1/authority/beg/oidc", post(Self::beg4credential))
            .route(
                "/api/v1/authority/request/all",
                get(Self::get_all_authority),
            )
            .route(
                "/api/v1/authority/request/:id",
                get(Self::get_one_authority),
            )
            // .route("/provider/:id/renew", post(todo!()))
            // .route("/provider/:id/finalize", post(todo!()))
            .with_state(self.manager)
            .fallback(Self::fallback) // 2 routers cannot have 1 fallback each
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
        match manager.didweb().await {
            Ok(did) => Json(did).into_response(),
            Err(e) => e.to_response(),
        }
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
        info!("{}", uri);
        uri.into_response()
    }

    async fn get_callback(
        State(manager): State<Arc<Manager<T>>>,
        Path(id): Path<String>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let log = format!("POST /callback/manual/{}", id);
        info!(log);

        let hash = match params.get("hash") {
            Some(hash) => hash,
            None => {
                let error = CommonErrors::format_new(
                    BadFormat::Received,
                    Some("Unable to retrieve hash from callback".to_string()),
                );
                error!("{}", error.log());
                return error.into_response();
            }
        };

        let interact_ref = match params.get("interact_ref") {
            Some(interact_ref) => interact_ref,
            None => {
                let error = CommonErrors::format_new(
                    BadFormat::Received,
                    Some("Unable to retrieve interact reference".to_string()),
                );
                error!("{}", error.log());
                return error.into_response();
            }
        };

        match manager.check_callback(id.clone(), interact_ref.to_string(), hash.to_string()).await {
            Ok(()) => {}
            Err(e) => return e.to_response(),
        };

        match manager.continue_request(id, interact_ref.to_string()).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn post_callback(
        State(manager): State<Arc<Manager<T>>>,
        Path(id): Path<String>,
        Json(payload): Json<CallbackBody>,
    ) -> impl IntoResponse {
        let log = format!("POST /callback/{}", id);
        info!(log);

        match manager
            .check_callback(
                id.clone(),
                payload.interact_ref.to_string(),
                payload.hash.to_string(),
            )
            .await
        {
            Ok(()) => {}
            Err(e) => return e.to_response(),
        };

        match manager.continue_request(id, payload.interact_ref.to_string()).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn beg4credential(
        State(manager): State<Arc<Manager<T>>>,
        Json(payload): Json<ReachAuthority>,
    ) -> impl IntoResponse {
        info!("POST /beg/credential");
        match manager.beg_credential(payload.id, payload.slug, payload.url).await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
        // TODO RES
    }

    async fn get_all_authority(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("GET /authority/request/all");

        match manager.repo.authority().get_all(None, None).await {
            Ok(data) => {
                let res = serde_json::to_value(data).unwrap(); // EXPECTED ALWAYS
                (StatusCode::OK, Json(res)).into_response()
            }
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                error.into_response()
            }
        }
    }

    async fn get_one_authority(State(authority): State<Arc<Manager<T>>>, Path(id): Path<String>) -> impl IntoResponse {
        let log = format!("GET /authority/request/{}", &id);
        info!("{}", log);

        match authority.repo.authority().get_by_id(id.as_str()).await {
            Ok(Some(data)) => {
                let res = serde_json::to_value(data).unwrap();
                (StatusCode::OK, Json(res)).into_response()
            }
            Ok(None) => {
                let error =
                    CommonErrors::missing_resource_new(id.clone(), Some(format!("Missing request with id: {}", id)));
                error!("{}", error.log());
                error.into_response()
            }
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                error.into_response()
            }
        }
    }

    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        info!("{}", log);
        error!("Unexpected route");
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
}
