/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use crate::consumer::core::bypass_service::ByPassTrait;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use rainbow_common::utils::get_urn_from_string;
use reqwest::{Client, StatusCode};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, info};

pub struct CatalogBypassRouter<T>
where
    T: ByPassTrait + Send + Sync + 'static,
{
    service: Arc<T>,
}

impl<T> CatalogBypassRouter<T>
where
    T: ByPassTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/catalog-bypass/:participant_id/*extra",
                get(Self::forward_to_catalog),
            )
            .with_state(self.service)
    }
    async fn forward_to_catalog(
        State(service): State<Arc<T>>,
        Path((participant_id, extra)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/catalog-bypass/{}/{}", participant_id, extra);
        match service.bypass(participant_id, extra).await {
            Ok(value) => (StatusCode::OK, Json(value)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}
