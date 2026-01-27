/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use crate::ssi::core::traits::CoreGaiaSelfIssuerTrait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use ymir::errors::CustomToResponse;

pub struct GaiaSelfIssuerRouter {
    self_issuer: Arc<dyn CoreGaiaSelfIssuerTrait>,
}

impl GaiaSelfIssuerRouter {
    pub fn new(self_issuer: Arc<dyn CoreGaiaSelfIssuerTrait>) -> Self {
        GaiaSelfIssuerRouter { self_issuer }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/issue", post(Self::credential))
            .route("/credentialOffer", get(Self::cred_offer))
            .route("/.well-known/openid-credential-issuer", get(Self::get_issuer))
            .route("/.well-known/oauth-authorization-server", get(Self::get_oauth_server))
            .route("/token", post(Self::get_token))
            .route("/credential", post(Self::post_credential))
            .with_state(self.self_issuer)
    }

    async fn credential(
        State(self_issuer): State<Arc<dyn CoreGaiaSelfIssuerTrait>>,
    ) -> impl IntoResponse {
        match self_issuer.generate_gaia_vcs().await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn cred_offer(
        State(self_issuer): State<Arc<dyn CoreGaiaSelfIssuerTrait>>,
    ) -> impl IntoResponse {
        let data = self_issuer.get_cred_offer_data();

        (StatusCode::OK, Json(data)).into_response()
    }

    async fn get_issuer(
        State(self_issuer): State<Arc<dyn CoreGaiaSelfIssuerTrait>>,
    ) -> impl IntoResponse {
        let data = self_issuer.get_issuer_data();
        (StatusCode::OK, Json(data)).into_response()
    }

    async fn get_oauth_server(
        State(self_issuer): State<Arc<dyn CoreGaiaSelfIssuerTrait>>,
    ) -> impl IntoResponse {
        let data = self_issuer.get_oauth_server_data();
        (StatusCode::OK, Json(data)).into_response()
    }

    async fn get_token(
        State(self_issuer): State<Arc<dyn CoreGaiaSelfIssuerTrait>>,
    ) -> impl IntoResponse {
        let data = self_issuer.get_token();
        (StatusCode::OK, Json(data)).into_response()
    }

    async fn post_credential(
        State(self_issuer): State<Arc<dyn CoreGaiaSelfIssuerTrait>>,
    ) -> impl IntoResponse {
        match self_issuer.issue_cred().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }
}
