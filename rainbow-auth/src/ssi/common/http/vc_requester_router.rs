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
use crate::ssi::common::types::entities::{ReachAuthority, ReachMethod};
use crate::ssi::common::types::gnap::CallbackBody;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::error;
use crate::ssi::common::core::traits::CoreVcRequesterTrait;

pub struct VcRequesterRouter {
    requester: Arc<dyn CoreVcRequesterTrait>,
}

impl VcRequesterRouter {
    pub fn new(requester: Arc<dyn CoreVcRequesterTrait>) -> Self {
        VcRequesterRouter { requester }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/beg/cross-user", post(Self::beg_cross_user))
            .route("/beg/oidc", post(Self::beg_oidc))
            .route("/all", get(Self::get_all))
            .route("/:id", get(Self::get_one))
            .route("/callback/:id", get(Self::get_callback))
            .route("/callback/:id", post(Self::post_callback))
            .with_state(self.requester)
    }

    async fn beg_cross_user(
        State(requester): State<Arc<dyn CoreVcRequesterTrait>>,
        payload: Result<Json<ReachAuthority>, JsonRejection>,
    ) -> impl IntoResponse {
        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => {
                error!("{:#?}", e);
                return e.into_response();
            }
        };

        match requester.beg_vc(payload, ReachMethod::CrossUser).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn beg_oidc(
        State(requester): State<Arc<dyn CoreVcRequesterTrait>>,
        payload: Result<Json<ReachAuthority>, JsonRejection>,
    ) -> impl IntoResponse {
        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => {
                error!("{:#?}", e);
                return e.into_response();
            }
        };

        match requester.beg_vc(payload, ReachMethod::Oidc).await {
            Ok(Some(data)) => data.into_response(),
            Ok(None) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_all(State(requester): State<Arc<dyn CoreVcRequesterTrait>>) -> impl IntoResponse {
        match requester.get_all().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_one(
        State(requester): State<Arc<dyn CoreVcRequesterTrait>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        match requester.get_by_id(id).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn get_callback(
        State(requester): State<Arc<dyn CoreVcRequesterTrait>>,
        Path(id): Path<String>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let hash = match params.get("hash") {
            Some(hash) => hash.clone(),
            None => {
                let error = CommonErrors::format_new(BadFormat::Received, "Unable to retrieve hash from callback");
                error!("{}", error.log());
                return error.into_response();
            }
        };

        let interact_ref = match params.get("interact_ref") {
            Some(interact_ref) => interact_ref.clone(),
            None => {
                let error = CommonErrors::format_new(BadFormat::Received, "Unable to retrieve interact reference");
                error!("{}", error.log());
                return error.into_response();
            }
        };

        let payload = CallbackBody { interact_ref, hash };
        match requester.continue_req(id, payload).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn post_callback(
        State(requester): State<Arc<dyn CoreVcRequesterTrait>>,
        Path(id): Path<String>,
        payload: Result<Json<CallbackBody>, JsonRejection>,
    ) -> impl IntoResponse {
        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match requester.continue_req(id, payload).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }
}
