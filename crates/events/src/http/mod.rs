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

pub mod dlq_router;
pub mod events_router;
pub mod notification;
pub mod subscription;
pub mod subscriptions_router;

use std::sync::Arc;
use axum::Router;

pub use dlq_router::DeadLetterRouter;
pub use events_router::EventsRouter;
pub use subscriptions_router::SubscriptionsRouter;

use crate::bus::EventBus;

/// Combine all event bus HTTP sub-routers into a unified root router.
pub struct EventsHttpRouter;

impl EventsHttpRouter {
    /// Construct the unified events HTTP router nesting events, subscriptions, and DLQ.
    pub fn build(bus: Arc<EventBus>) -> Router {
        Router::new()
            .nest("/events", EventsRouter::new(bus.clone()).router())
            .nest(
                "/subscriptions",
                SubscriptionsRouter::new(bus.subscription_repo()).router(),
            )
            .nest("/dlq", DeadLetterRouter::new(bus).router())
    }
}
