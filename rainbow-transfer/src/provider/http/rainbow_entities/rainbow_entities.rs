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

use crate::common::errors::error_adapter::CustomToResponse;
use crate::provider::core::rainbow_entities::RainbowTransferProviderServiceTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use log::info;
use rainbow_common::batch_requests::BatchRequests;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::utils::get_urn_from_string;
use std::sync::Arc;
use tracing::error;

pub struct RainbowTransferProviderEntitiesRouter<T> {
    transfer_service: Arc<T>,
}

impl<T> RainbowTransferProviderEntitiesRouter<T>
where
    T: RainbowTransferProviderServiceTrait + Send + Sync + 'static,
{
    pub fn new(transfer_service: Arc<T>) -> Self {
        Self { transfer_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/transfers", get(Self::handle_get_all_transfers))
            .route(
                "/api/v1/transfers/batch",
                post(Self::handle_get_batch_transfers),
            )
            .route(
                "/api/v1/transfers/:id",
                get(Self::handle_get_transfer_by_id),
            )
            .route(
                "/api/v1/transfers/:id/messages",
                get(Self::handle_get_messages_by_transfer),
            )
            .route(
                "/api/v1/transfers/:id/messages/:mid",
                get(Self::handle_get_messages_by_id),
            )
            .with_state(self.transfer_service)
    }

    async fn handle_get_all_transfers(State(transfer_service): State<Arc<T>>) -> impl IntoResponse {
        info!("GET /api/v1/transfers");

        match transfer_service.get_all_transfers().await {
            Ok(transfer_processes) => (StatusCode::OK, Json(transfer_processes)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    async fn handle_get_batch_transfers(
        State(transfer_service): State<Arc<T>>,
        input: Result<Json<BatchRequests>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/batch");

        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, format!("{}", err.body_text()).into());
                error!("{}", e.log());
                return e.into_response();
            }
        };

        match transfer_service.get_batch_transfers(&input.ids).await {
            Ok(transfer_processes) => (StatusCode::OK, Json(transfer_processes)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    async fn handle_get_transfer_by_id(
        State(transfer_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/transfers/{}", id);
        let id = match get_urn_from_string(&id) {
            Ok(process_id) => process_id,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    format!("Urn malformed. {}", err.to_string()).into(),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        match transfer_service.get_transfer_by_id(id).await {
            Ok(transfer_processes) => (StatusCode::OK, Json(transfer_processes)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    async fn handle_get_messages_by_transfer(
        State(transfer_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/transfers/{}/messages", id);
        let id = match get_urn_from_string(&id) {
            Ok(process_id) => process_id,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    format!("Urn malformed. {}", err.to_string()).into(),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        match transfer_service.get_messages_by_transfer(id).await {
            Ok(transfer_processes) => (StatusCode::OK, Json(transfer_processes)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    async fn handle_get_messages_by_id(
        State(transfer_service): State<Arc<T>>,
        Path((id, mid)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/transfers/{}/messages/{}", id, mid);
        let id = match get_urn_from_string(&id) {
            Ok(process_id) => process_id,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    format!("Urn malformed. {}", err.to_string()).into(),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };
        let mid = match get_urn_from_string(&mid) {
            Ok(process_id) => process_id,
            Err(err) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    format!("Urn malformed. {}", err.to_string()).into(),
                );
                error!("{}", e.log());
                return e.into_response();
            }
        };

        match transfer_service.get_messages_by_id(id, mid).await {
            Ok(transfer_processes) => (StatusCode::OK, Json(transfer_processes)).into_response(),
            Err(err) => err.to_response(),
        }
    }
}
