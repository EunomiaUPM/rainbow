/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::bus::error::EventBusError;
use crate::bus::EventBus;
use crate::data::repo::{DeadLetterRecord, EventDeliveryRecord};

#[derive(Clone)]
pub struct DeadLetterRouter {
    bus: Arc<EventBus>,
}

#[derive(Debug, Deserialize)]
pub struct ListDlqQuery {
    pub status: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl DeadLetterRouter {
    /// Create router with reference to the shared EventBus.
    pub fn new(bus: Arc<EventBus>) -> Self {
        Self { bus }
    }

    /// Build Axum sub-router for DLQ operations.
    pub fn router(self) -> Router {
        Router::new()
            .route("/", get(Self::handle_list))
            .route("/{id}", get(Self::handle_get))
            .route("/{id}/replay", post(Self::handle_replay))
            .route("/replay-all", post(Self::handle_replay_all))
            .route("/{id}", delete(Self::handle_delete))
            .with_state(self.bus)
    }

    async fn handle_list(
        State(bus): State<Arc<EventBus>>,
        Query(query): Query<ListDlqQuery>,
    ) -> Result<Json<Vec<DeadLetterRecord>>, EventBusError> {
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0);
        let dead_letters = bus
            .dlq_repo()
            .list_dead_letters(query.status.as_deref(), limit, offset)
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?;

        Ok(Json(dead_letters))
    }

    async fn handle_get(
        State(bus): State<Arc<EventBus>>,
        Path(id): Path<String>,
    ) -> Result<Json<DeadLetterRecord>, EventBusError> {
        let dead_letter = bus
            .dlq_repo()
            .get_dead_letter(&id)
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?
            .ok_or_else(|| EventBusError::DeadLetterNotFound(uuid::Uuid::parse_str(&id).unwrap_or_default()))?;

        Ok(Json(dead_letter))
    }

    async fn handle_replay(
        State(bus): State<Arc<EventBus>>,
        Path(id): Path<String>,
    ) -> Result<Json<EventDeliveryRecord>, EventBusError> {
        let delivery = bus.replay_dead_letter(&id).await?;
        Ok(Json(delivery))
    }

    async fn handle_replay_all(
        State(bus): State<Arc<EventBus>>,
    ) -> Result<Json<serde_json::Value>, EventBusError> {
        let count = bus.replay_all_dead_letters().await?;
        Ok(Json(json!({ "replayed_count": count })))
    }

    async fn handle_delete(
        State(bus): State<Arc<EventBus>>,
        Path(id): Path<String>,
    ) -> Result<StatusCode, EventBusError> {
        bus.dlq_repo()
            .delete_dead_letter(&id)
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?;

        Ok(StatusCode::NO_CONTENT)
    }
}
