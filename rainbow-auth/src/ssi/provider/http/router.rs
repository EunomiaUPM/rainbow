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
use tracing::{info, Level};
use uuid::Uuid;

pub struct AuthProviderRouter {
    pub provider: Arc<AuthProvider>,
}

impl AuthProviderRouter {
    pub fn new(provider: Arc<AuthProvider>) -> Self {
        AuthProviderRouter { provider }
    }

    pub fn router(self) -> Router {
        let wallet_router = WalletRouter::new(self.provider.clone()).router();
        let vc_requester_router = VcRequesterRouter::new(self.provider.clone()).router();

        Router::new()
            .route("/api/v1/status", get(server_status))
            .with_state(self.provider)
            .nest("/api/v1/wallet", wallet_router)
            .nest("/api/v1/vc-request", vc_requester_router)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|_req: &Request<_>| tracing::info_span!("request", id = %Uuid::new_v4()))
                    .on_request(|req: &Request<_>, _span: &tracing::Span| {
                        info!("{} {}", req.method(), req.uri().path());
                    })
                    .on_response(DefaultOnResponse::new().level(Level::TRACE)),
            )
    }
}

