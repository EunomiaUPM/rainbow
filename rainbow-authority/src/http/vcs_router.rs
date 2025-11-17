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

use crate::core::traits::CoreVcsTrait;
use crate::errors::CustomToResponse;
use crate::types::vcs::VcDecisionApproval;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;

pub struct VcsRouter {
    approver: Arc<dyn CoreVcsTrait>,
}

impl VcsRouter {
    pub fn new(approver: Arc<dyn CoreVcsTrait>) -> Self {
        Self { approver }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/all", get(Self::get_all_requests))
            .route("/:id", get(Self::get_one_request))
            .route("/:id", post(Self::manage_request))
            .with_state(self.approver)
    }

    async fn get_all_requests(State(approver): State<Arc<dyn CoreVcsTrait>>) -> impl IntoResponse {
        match approver.get_all().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_one_request(
        State(approver): State<Arc<dyn CoreVcsTrait>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        match approver.get_by_id(id).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn manage_request(
        State(approver): State<Arc<dyn CoreVcsTrait>>,
        Path(id): Path<String>,
        payload: Result<Json<VcDecisionApproval>, JsonRejection>,
    ) -> impl IntoResponse {
        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match approver.manage_req(id, payload).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }
}
