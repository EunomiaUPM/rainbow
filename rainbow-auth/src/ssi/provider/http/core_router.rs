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
use crate::ssi::common::http::{VcRequesterRouter, WalletRouter};
use crate::ssi::provider::core::AuthProvider;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use rainbow_common::utils::server_status;
use std::sync::Arc;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{error, info, Level};
use uuid::Uuid;
use rainbow_common::http::OpenapiRouter;
use crate::ssi::provider::core::traits::CoreProviderTrait;
use crate::ssi::provider::http::{GateKeeperRouter, VerifierRouter};

pub struct AuthProviderRouter {
    provider: Arc<AuthProvider>,
    openapi: String,
}

impl AuthProviderRouter {
    pub fn new(provider: Arc<AuthProvider>) -> Self {
        let openapi = provider.config().get_openapi_json().expect("Invalid openapi path");
        AuthProviderRouter { provider, openapi }
    }

    pub fn router(self) -> Router {
        let wallet_router = WalletRouter::new(self.provider.clone()).router();
        let vc_requester_router = VcRequesterRouter::new(self.provider.clone()).router();
        let gatekeeper_router = GateKeeperRouter::new(self.provider.clone()).router();
        let verifier_router = VerifierRouter::new(self.provider.clone()).router();
        let openapi_route = OpenapiRouter::new(self.openapi.clone()).router();

        Router::new()
            .route("/api/v1/status", get(server_status))
            .with_state(self.provider)
            .nest("/api/v1/wallet", wallet_router)
            .nest("/api/v1/vc-request", vc_requester_router)
            .nest("/api/v1/gate", gatekeeper_router)
            .nest("/api/v1/verifier", verifier_router)
            .nest("/api/v1/docs", openapi_route)
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

    async fn fallback() -> impl IntoResponse {
        error!("Wrong route");
        StatusCode::NOT_FOUND.into_response()
    }
}

