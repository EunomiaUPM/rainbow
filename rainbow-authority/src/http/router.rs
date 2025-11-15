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
use crate::errors::{CustomToResponse, ErrorLogTrait, Errors};
use crate::http::openapi;
use crate::services::repo::RepoTrait;
use crate::types::enums::errors::BadFormat;
use crate::types::gnap::{GrantRequest, RefBody};
use crate::types::issuing::{CredentialRequest, TokenRequest};
use crate::types::vcs::VcDecisionApproval;
use crate::types::verifying::VerifyPayload;
use crate::types::wallet::{DidsInfo, KeyDefinition};
use crate::utils::{extract_bearer_token, extract_gnap_token};
use axum::body::Bytes;
use axum::extract::rejection::FormRejection;
use axum::extract::{rejection::JsonRejection, Path, Query, Request, State};
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Form, Json, Router};
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tracing::{error, info, Level};
use uuid::Uuid;
use crate::core::traits::CoreTrait;
use crate::http::gatekeeper_router::GateKeeperRouter;
use crate::http::issuer_router::IssuerRouter;
use crate::http::vcs_router::VcsRouter;
use crate::http::verifier_router::VerifierRouter;
use crate::http::wallet_router::WalletRouter;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

pub struct RainbowAuthorityRouter {
    pub authority: Arc<dyn CoreTrait>,
}

impl RainbowAuthorityRouter {
    pub fn new(authority: Arc<dyn CoreTrait>) -> Self {
        Self { authority }
    }

    pub fn router(self) -> Router {
        let gatekeeper_router = GateKeeperRouter::new(self.authority.clone()).router();
        let wallet_router = WalletRouter::new(self.authority.clone()).router();
        let issuer_router = IssuerRouter::new(self.authority.clone()).router();
        let verifier_router = VerifierRouter::new(self.authority.clone()).router();
        let vcs_router = VcsRouter::new(self.authority.clone()).router();

        Router::new()
            .route("/api/v1/status", get(server_status))
            .with_state(self.authority)
            .nest("/api/v1/wallet", wallet_router)
            .nest("/api/v1/vc-request", vcs_router)
            .nest("/api/v1/gate", gatekeeper_router)
            .nest("/api/v1", issuer_router)
            .nest("/api/v1/verifier", verifier_router)
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

async fn server_status() -> impl IntoResponse {
    info!("Someone checked server status");
    (StatusCode::OK, "Server is Okay!").into_response()
}