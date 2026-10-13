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

//! Where services emit their events: the bus when there is one, nothing otherwise.

pub mod service;

use async_trait::async_trait;
use common::oauth::Owner;
use serde_json::Value;
use ymir::errors::Outcome;

pub use service::NoopEventSink;

/// Publishes a payload about a record of `owner` under `topic`; what `emit_action!` calls.
#[mockall::automock]
#[async_trait]
pub trait EventSinkTrait: Send + Sync {
    async fn emit_action(
        &self,
        owner: &Owner,
        topic: &str,
        source: &str,
        payload: Value,
    ) -> Outcome<()>;
}
