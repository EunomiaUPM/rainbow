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

use crate::core::traits::{AuthorityTrait, RainbowSSIAuthWalletTrait};
use crate::core::Authority;
use crate::data::repo_factory::factory_trait::AuthRepoFactoryTrait;
use crate::errors::CustomToResponse;
use crate::types::gnap::{GrantRequest, RefBody};
use crate::types::oidc::VerifyPayload;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use std::sync::Arc;
use tracing::info;

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
            .route("/api/v1/did.json", get(Self::didweb))
            // GNAP
            .route("/api/v1/credential/request", post(Self::access_request))
            .route("/api/v1/continue/:id", post(Self::continue_request))
            // OIDC4VP
            .route("/api/v1/pd/:state", get(Self::pd))
            .route("/api/v1/verify/:state", post(Self::verify))
            .with_state(self.authority)
            .fallback(Self::fallback)
    }

    async fn wallet_register(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/register");

        match authority.register_wallet().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn wallet_login(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/login");

        match authority.login_wallet().await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn wallet_logout(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/logout");

        match authority.logout_wallet().await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn wallet_onboard(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/onboard");

        match authority.onboard().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn didweb(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /did.json");

        match authority.didweb().await {
            Ok(_) => StatusCode::OK.into_response(),
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
    }
    async fn pd(State(authority): State<Arc<Authority<T>>>, Path(state): Path<String>) -> impl IntoResponse {
        let log = format!("GET /pd/{}", state);
        info!("{}", log);
    }
    async fn verify(
        State(authority): State<Arc<Authority<T>>>,
        Path(state): Path<String>,
        Form(payload): Form<VerifyPayload>,
    ) -> impl IntoResponse {
        let log = format!("POST /verify/{}", state);
        info!("{}", log);
    }

    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        info!("{}", log);
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
}
