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
use crate::ssi::common::http::WalletRouter;
use crate::ssi::consumer::core::AuthConsumer;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use rainbow_common::utils::server_status;
use std::sync::Arc;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use uuid::Uuid;

pub struct AuthConsumerRouter {
    pub consumer: Arc<AuthConsumer>,
}

impl AuthConsumerRouter {
    pub fn new(consumer: Arc<AuthConsumer>) -> Self {
        AuthConsumerRouter { consumer }
    }

    pub fn router(self) -> Router {
        let wallet_router = WalletRouter::new(self.consumer.clone()).router();

        Router::new()
            .with_state(self.consumer)
            .route("/api/v1/status", get(server_status))
            .nest("/api/v1/wallet", wallet_router)
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
