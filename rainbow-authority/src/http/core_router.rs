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

use crate::core::traits::CoreTrait;
use crate::http::{GateKeeperRouter, IssuerRouter, OpenapiRouter, VcsRouter, VerifierRouter, WalletRouter};
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{error, info, Level};
use uuid::Uuid;

pub struct RainbowAuthorityRouter {
    authority: Arc<dyn CoreTrait>,
    openapi: String,
}

impl RainbowAuthorityRouter {
    pub fn new(authority: Arc<dyn CoreTrait>) -> Self {
        let openapi = authority.config().get_openapi_json().expect("Invalid openapi path");
        Self { authority, openapi }
    }

    pub fn router(self) -> Router {
        let gatekeeper_router = GateKeeperRouter::new(self.authority.clone()).router();
        let wallet_router = WalletRouter::new(self.authority.clone()).router();
        let issuer_router = IssuerRouter::new(self.authority.clone()).router();
        let verifier_router = VerifierRouter::new(self.authority.clone()).router();
        let vcs_router = VcsRouter::new(self.authority.clone()).router();
        let openapi_router = OpenapiRouter::new(self.openapi.clone()).router();

        Router::new()
            .route(
                &format!("{}/status", self.authority.config().get_api_path()),
                get(Self::server_status),
            )
            .nest(
                &format!("{}/wallet", self.authority.config().get_api_path()),
                wallet_router,
            )
            .nest(
                &format!("{}/vc-request", self.authority.config().get_api_path()),
                vcs_router,
            )
            .nest(
                &format!("{}/gate", self.authority.config().get_api_path()),
                gatekeeper_router,
            )
            .nest(
                &format!("{}/issuer", self.authority.config().get_api_path()),
                issuer_router,
            )
            .nest(
                &format!("{}/verifier", self.authority.config().get_api_path()),
                verifier_router,
            )
            .nest(
                &format!("{}/docs", self.authority.config().get_api_path()),
                openapi_router,
            )
            .fallback(Self::fallback)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|_req: &Request<_>| tracing::info_span!("request", id = %Uuid::new_v4()))
                    .on_request(|req: &Request<_>, _span: &tracing::Span| {
                        info!("{} {}", req.method(), req.uri().path());
                    })
                    .on_response(DefaultOnResponse::new().level(Level::TRACE)),
            )
    }

    async fn server_status() -> impl IntoResponse {
        info!("Someone checked server status");
        (StatusCode::OK, "Server is Okay!").into_response()
    }
    async fn fallback() -> impl IntoResponse {
        error!("Wrong route");
        StatusCode::NOT_FOUND.into_response()
    }
}
