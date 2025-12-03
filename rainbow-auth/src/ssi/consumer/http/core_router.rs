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

use super::OnboarderConsumerRouter;
use crate::ssi::common::http::{MateRouter, VcRequesterRouter, WalletRouter};
use crate::ssi::consumer::core::traits::CoreConsumerTrait;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use rainbow_common::config::traits::ApiConfigTrait;
use rainbow_common::http::OpenapiRouter;
use rainbow_common::utils::server_status;
use std::sync::Arc;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{error, info, Level};
use uuid::Uuid;

pub struct AuthConsumerRouter {
    consumer: Arc<dyn CoreConsumerTrait>,
    openapi: String,
}

impl AuthConsumerRouter {
    pub fn new(consumer: Arc<dyn CoreConsumerTrait>) -> Self {
        let openapi = consumer.config().get_openapi().expect("Invalid openapi path");
        AuthConsumerRouter { consumer, openapi }
    }

    pub fn router(self) -> Router {
        // SERVICES ROUTERS
        let wallet_router = WalletRouter::new(self.consumer.clone()).router();
        let vc_requester_router = VcRequesterRouter::new(self.consumer.clone()).router();
        let mate_router = MateRouter::new(self.consumer.clone()).router();
        let onboarder_router = OnboarderConsumerRouter::new(self.consumer.clone()).router();
        let openapi_route = OpenapiRouter::new(self.openapi.clone()).router();

        let api_path = self.consumer.config().get_api_path();

        Router::new()
            .route(&format!("{}/status", api_path), get(server_status))
            .nest(&format!("{}/wallet", api_path), wallet_router)
            .nest(&format!("{}/vc-request", api_path), vc_requester_router)
            .nest(&format!("{}/mates", api_path), mate_router)
            .nest(&format!("{}/onboard", api_path), onboarder_router)
            .nest(&format!("{}/docs", api_path), openapi_route)
            .fallback(Self::fallback)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|_req: &Request<_>| tracing::info_span!("C-Auth-request", id = %Uuid::new_v4()))
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
