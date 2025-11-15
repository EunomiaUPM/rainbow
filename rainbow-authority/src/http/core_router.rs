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
use crate::http::gatekeeper_router::GateKeeperRouter;
use crate::http::issuer_router::IssuerRouter;
use crate::http::vcs_router::VcsRouter;
use crate::http::verifier_router::VerifierRouter;
use crate::http::wallet_router::WalletRouter;
use crate::http::OpenapiRouter;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
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
            .route("/api/v1/status", get(Self::server_status))
            .nest("/api/v1/wallet", wallet_router)
            .nest("/api/v1/vc-request", vcs_router)
            .nest("/api/v1/gate", gatekeeper_router)
            .nest("/api/v1/issuer", issuer_router)
            .nest("/api/v1/verifier", verifier_router)
            .nest("/api/v1/docs", openapi_router)
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
}
