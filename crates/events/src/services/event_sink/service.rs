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

//! The sinks: the bus itself, an optional bus, and one that drops everything.

use std::sync::Arc;

use async_trait::async_trait;
use common::oauth::Owner;
use serde_json::Value;
use ymir::errors::Outcome;

use crate::services::event_bus::EventBus;
use crate::services::event_sink::EventSinkTrait;

/// Drops every event: for agents without a bus and for tests.
#[derive(Debug, Clone, Default)]
pub struct NoopEventSink;

impl NoopEventSink {
    /// The bus when there is one, this sink otherwise.
    pub fn or_bus(bus: Option<EventBus>) -> Arc<dyn EventSinkTrait> {
        match bus {
            Some(bus) => Arc::new(bus),
            None => Arc::new(Self),
        }
    }
}

#[async_trait]
impl EventSinkTrait for NoopEventSink {
    async fn emit_action(&self, _: &Owner, _: &str, _: &str, _: Value) -> Outcome<()> {
        Ok(())
    }
}

#[async_trait]
impl EventSinkTrait for EventBus {
    async fn emit_action(
        &self,
        owner: &Owner,
        topic: &str,
        source: &str,
        payload: Value,
    ) -> Outcome<()> {
        self.emit_payload_for(owner, topic, source, &payload)
            .await?;
        Ok(())
    }
}

/// Services not yet moved to a sink keep their optional bus.
#[async_trait]
impl<T: EventSinkTrait> EventSinkTrait for Option<T> {
    async fn emit_action(
        &self,
        owner: &Owner,
        topic: &str,
        source: &str,
        payload: Value,
    ) -> Outcome<()> {
        match self {
            Some(sink) => sink.emit_action(owner, topic, source, payload).await,
            None => Ok(()),
        }
    }
}

#[async_trait]
impl<T: EventSinkTrait + ?Sized> EventSinkTrait for &T {
    async fn emit_action(
        &self,
        owner: &Owner,
        topic: &str,
        source: &str,
        payload: Value,
    ) -> Outcome<()> {
        (**self).emit_action(owner, topic, source, payload).await
    }
}
