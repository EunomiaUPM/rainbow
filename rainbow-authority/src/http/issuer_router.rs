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
use crate::core::traits::CoreIssuerTrait;
use crate::errors::{CustomToResponse, ErrorLogTrait, Errors};
use crate::types::enums::errors::BadFormat;
use crate::types::gnap::{GrantRequest, RefBody};
use crate::types::issuing::{CredentialRequest, TokenRequest};
use crate::utils::{extract_bearer_token, extract_gnap_token};
use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Form, Json, Router};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info};

pub struct IssuerRouter {
    issuer: Arc<dyn CoreIssuerTrait>,
}

impl IssuerRouter {
    pub fn new(issuer: Arc<dyn CoreIssuerTrait>) -> Self {
        Self { issuer }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/credentialOffer", get(Self::cred_offer))
            .route(
                "/.well-known/openid-credential-issuer",
                get(Self::get_issuer),
            )
            .route(
                "/.well-known/oauth-authorization-server",
                get(Self::get_oauth_server),
            )
            .route("/jwks", get(Self::get_jwks))
            .route("/token", post(Self::get_token))
            .route("/credential", post(Self::post_credential))
            .with_state(self.issuer)
    }

    async fn cred_offer(
        State(issuer): State<Arc<dyn CoreIssuerTrait>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let id = match params.get("id") {
            Some(hash) => hash.clone(),
            None => {
                info!("GET /credentialOffer");
                let error = Errors::format_new(BadFormat::Received, "Unable to retrieve hash from callback");
                error!("{}", error.log());
                return error.into_response();
            }
        };
        let log = format!("GET /credentialOffer/{}", id);
        info!("{}", log);

        match issuer.get_cred_offer_data(id).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_issuer(State(issuer): State<Arc<dyn CoreIssuerTrait>>) -> impl IntoResponse {
        info!("GET /issuer");

        (StatusCode::OK, Json(issuer.issuer_metadata())).into_response()
    }

    async fn get_oauth_server(State(issuer): State<Arc<dyn CoreIssuerTrait>>) -> impl IntoResponse {
        info!("GET /oauth_server");

        (StatusCode::OK, Json(issuer.oauth_server_metadata())).into_response()
    }

    async fn get_jwks(State(issuer): State<Arc<dyn CoreIssuerTrait>>) -> impl IntoResponse {
        info!("GET /jwks_data");

        match issuer.jwks() {
            Ok(jwk) => (StatusCode::OK, Json(jwk)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_token(
        State(issuer): State<Arc<dyn CoreIssuerTrait>>,
        payload: Result<Form<TokenRequest>, FormRejection>,
    ) -> impl IntoResponse {
        info!("GET /token");

        let payload = match payload {
            Ok(Form(data)) => data,
            Err(e) => return e.into_response(),
        };

        match issuer.get_token(payload).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn post_credential(
        State(authority): State<Arc<dyn CoreIssuerTrait>>,
        headers: HeaderMap,
        payload: Result<Json<CredentialRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /credential");

        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        let token = match extract_bearer_token(headers) {
            Some(token) => token,
            None => {
                let error = Errors::unauthorized_new("Missing token");
                error!("{}", error.log());
                return error.into_response();
            }
        };

        match authority.get_credential(payload, token).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }
}
