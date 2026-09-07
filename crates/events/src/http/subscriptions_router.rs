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

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};

use crate::bus::error::EventBusError;
use crate::data::repo::{
    CreateSubscriptionDto, EventSubscriptionRepo, SubscriptionRecord, UpdateSubscriptionDto,
};

#[derive(Clone)]
pub struct SubscriptionsRouter {
    repo: Arc<dyn EventSubscriptionRepo>,
}

impl SubscriptionsRouter {
    /// Create router backed by the subscription repository.
    pub fn new(repo: Arc<dyn EventSubscriptionRepo>) -> Self {
        Self { repo }
    }

    /// Build Axum sub-router for subscription CRUD endpoints.
    pub fn router(self) -> Router {
        Router::new()
            .route("/", post(Self::handle_create))
            .route("/", get(Self::handle_list))
            .route("/{id}", get(Self::handle_get))
            .route("/{id}", put(Self::handle_update))
            .route("/{id}", delete(Self::handle_delete))
            .with_state(self.repo)
    }

    async fn handle_create(
        State(repo): State<Arc<dyn EventSubscriptionRepo>>,
        Json(dto): Json<CreateSubscriptionDto>,
    ) -> Result<(StatusCode, Json<SubscriptionRecord>), EventBusError> {
        let sub = repo
            .create_subscription(dto)
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?;

        Ok((StatusCode::CREATED, Json(sub)))
    }

    async fn handle_list(
        State(repo): State<Arc<dyn EventSubscriptionRepo>>,
    ) -> Result<Json<Vec<SubscriptionRecord>>, EventBusError> {
        let subs = repo
            .list_subscriptions()
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?;

        Ok(Json(subs))
    }

    async fn handle_get(
        State(repo): State<Arc<dyn EventSubscriptionRepo>>,
        Path(id): Path<String>,
    ) -> Result<Json<SubscriptionRecord>, EventBusError> {
        let sub = repo
            .get_subscription(&id)
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?
            .ok_or_else(|| EventBusError::SubscriptionNotFound(uuid::Uuid::parse_str(&id).unwrap_or_default()))?;

        Ok(Json(sub))
    }

    async fn handle_update(
        State(repo): State<Arc<dyn EventSubscriptionRepo>>,
        Path(id): Path<String>,
        Json(dto): Json<UpdateSubscriptionDto>,
    ) -> Result<Json<SubscriptionRecord>, EventBusError> {
        let sub = repo
            .update_subscription(&id, dto)
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?;

        Ok(Json(sub))
    }

    async fn handle_delete(
        State(repo): State<Arc<dyn EventSubscriptionRepo>>,
        Path(id): Path<String>,
    ) -> Result<StatusCode, EventBusError> {
        repo.delete_subscription(&id)
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?;

        Ok(StatusCode::NO_CONTENT)
    }
}
