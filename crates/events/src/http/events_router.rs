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

use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use urn::Urn;

use crate::bus::envelope::{EventEnvelope, Topic};
use crate::bus::error::EventBusError;
use crate::bus::{EventBus, EventBusTrait};

#[derive(Clone)]
pub struct EventsRouter {
    bus: Arc<EventBus>,
}

#[derive(Debug, Deserialize)]
pub struct PublishEventRequest {
    pub topic: String,
    pub source_crate: Option<String>,
    pub schema_version: Option<u32>,
    pub correlation_id: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ListEventsQuery {
    pub topic: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl EventsRouter {
    /// Create router with reference to the shared EventBus.
    pub fn new(bus: Arc<EventBus>) -> Self {
        Self { bus }
    }

    /// Build Axum sub-router for event operations.
    pub fn router(self) -> Router {
        Router::new()
            .route("/publish", post(Self::handle_publish))
            .route("/", get(Self::handle_list_events))
            .route("/{id}", get(Self::handle_get_event))
            .route("/{id}/deliveries", get(Self::handle_get_deliveries))
            .with_state(self.bus)
    }

    async fn handle_publish(
        State(bus): State<Arc<EventBus>>,
        Json(req): Json<PublishEventRequest>,
    ) -> Result<(StatusCode, Json<EventEnvelope>), EventBusError> {
        let topic = Topic::new(req.topic).map_err(EventBusError::InvalidTopic)?;
        let correlation_id = match req.correlation_id {
            Some(ref s) => Some(Urn::from_str(s).map_err(|e| EventBusError::InvalidTopic(e.to_string()))?),
            None => None,
        };

        let envelope = EventEnvelope::new(
            topic,
            req.source_crate.unwrap_or_else(|| "events".to_string()),
            req.schema_version.unwrap_or(1),
            correlation_id,
            req.payload,
        );

        let published = bus.publish(envelope).await?;
        Ok((StatusCode::CREATED, Json(published)))
    }

    async fn handle_list_events(
        State(bus): State<Arc<EventBus>>,
        Query(query): Query<ListEventsQuery>,
    ) -> Result<Json<Vec<EventEnvelope>>, EventBusError> {
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0);
        let events = bus
            .event_repo()
            .list_events(query.topic.as_deref(), limit, offset)
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?;

        Ok(Json(events))
    }

    async fn handle_get_event(
        State(bus): State<Arc<EventBus>>,
        Path(id): Path<String>,
    ) -> Result<Json<EventEnvelope>, EventBusError> {
        let urn = Urn::from_str(&id)
            .or_else(|_| Urn::from_str(&format!("urn:uuid:{id}")))
            .map_err(|e| EventBusError::InvalidTopic(e.to_string()))?;

        let event = bus
            .event_repo()
            .get_event_by_id(&urn)
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?
            .ok_or_else(|| EventBusError::EventNotFound(uuid::Uuid::parse_str(&id).unwrap_or_default()))?;

        Ok(Json(event))
    }

    async fn handle_get_deliveries(
        State(bus): State<Arc<EventBus>>,
        Path(id): Path<String>,
    ) -> Result<Json<Vec<crate::data::repo::EventDeliveryRecord>>, EventBusError> {
        let deliveries = bus
            .delivery_repo()
            .list_by_event(&id)
            .await
            .map_err(|e| EventBusError::Database(format!("{e:?}")))?;

        Ok(Json(deliveries))
    }
}
