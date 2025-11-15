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

use crate::core::traits::CoreGatekeeperTrait;
use crate::errors::{CustomToResponse, ErrorLogTrait, Errors};
use crate::types::gnap::{GrantRequest, RefBody};
use crate::utils::extract_gnap_token;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;
use tracing::{error, info};

pub struct GateKeeperRouter {
    gatekeeper: Arc<dyn CoreGatekeeperTrait>,
}

impl GateKeeperRouter {
    pub fn new(gatekeeper: Arc<dyn CoreGatekeeperTrait>) -> Self {
        Self { gatekeeper }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/access", post(Self::access_req))
            .route("/continue/:id", get(Self::continue_req))
            .with_state(self.gatekeeper)
    }

    async fn access_req(
        State(gatekeeper): State<Arc<dyn CoreGatekeeperTrait>>,
        payload: Result<Json<GrantRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /request/credential");

        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match gatekeeper.manage_req(payload).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn continue_req(
        State(authority): State<Arc<dyn CoreGatekeeperTrait>>,
        headers: HeaderMap,
        Path(id): Path<String>,
        payload: Result<Json<RefBody>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /request/continue");

        let token = match extract_gnap_token(headers) {
            Some(token) => token,
            None => {
                let error = Errors::unauthorized_new("Missing token");
                error!("{}", error.log());
                return error.into_response();
            }
        };

        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match authority.manage_cont_req(id, payload, token).await {
            Ok(data) => data.into_response(),
            Err(e) => e.to_response(),
        }
    }
}
