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

use crate::data_plane_info::DataPlaneInfoTrait;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType;
use rainbow_common::utils::get_urn_from_string;
use std::sync::Arc;
use tracing::info;

pub struct DataPlaneRouter<T>
where
    T: DataPlaneInfoTrait + Send + Sync,
{
    data_plane_info_service: Arc<T>,
}
impl<T> DataPlaneRouter<T>
where
    T: DataPlaneInfoTrait + Send + Sync + 'static,
{
    pub fn new(data_plane_info_service: Arc<T>) -> Self {
        Self { data_plane_info_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/dataplane/:data_plane_id",
                get(Self::handle_get_data_plane_by_id),
            )
            .with_state(self.data_plane_info_service)
    }
    async fn handle_get_data_plane_by_id(
        State(data_plane_info_service): State<Arc<T>>,
        Path(data_plane_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/dataplane/{}", data_plane_id);
        let data_plane_id = match get_urn_from_string(&data_plane_id) {
            Ok(data_plane_id) => data_plane_id,
            Err(_) => {
                return TransferErrorType::PidSchemeError.into_response();
            }
        };
        match data_plane_info_service.get_data_plane_info_by_session_id(data_plane_id).await {
            Ok(dataplane_session) => (StatusCode::OK, Json(dataplane_session)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }
}
