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
use crate::ssi::common::errors::CustomToResponse;
use crate::ssi::common::types::gnap::{GrantRequest, RefBody};
use crate::ssi::provider::core::traits::CoreGateKeeperTrait;
use crate::ssi::provider::utils::extract_gnap_token;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use tracing::error;

pub struct GateKeeperRouter {
    gatekeeper: Arc<dyn CoreGateKeeperTrait>,
}

impl GateKeeperRouter {
    pub fn new(gatekeeper: Arc<dyn CoreGateKeeperTrait>) -> Self {
        GateKeeperRouter { gatekeeper }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/access", post(Self::manage_req))
            .route("/continue/:id", post(Self::continue_req))
            .with_state(self.gatekeeper)
    }

    async fn manage_req(
        State(gatekeeper): State<Arc<dyn CoreGateKeeperTrait>>,
        payload: Result<Json<GrantRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => {
                error!("{:#?}", e);
                return e.into_response();
            }
        };

        match gatekeeper.manage_req(payload).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn continue_req(
        State(gatekeeper): State<Arc<dyn CoreGateKeeperTrait>>,
        headers: HeaderMap,
        Path(id): Path<String>,
        Json(payload): Json<RefBody>,
    ) -> impl IntoResponse {
        let token = match extract_gnap_token(headers) {
            Some(token) => token,
            None => {
                let error = CommonErrors::unauthorized_new("Missing token");
                error!("{}", error.log());
                return error.into_response();
            }
        };

        match gatekeeper.continue_req(id, payload, token).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }
}
