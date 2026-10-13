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

//! How an agent stores a DSP process: the same three operations for transfer and negotiation.

use ymir::errors::Outcome;

/// A process moving to `target`, the peer's code and reason if any, and the message that moved it.
#[derive(Debug, Clone)]
pub struct StateChange<S, M> {
    pub target: S,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
    pub message: M,
}

/// A message kept in a process history, with the states it left the process between.
#[derive(Debug, Clone)]
pub struct RecordedMessage<S, M> {
    pub message: M,
    pub from: S,
    pub to: S,
}

/// Persistence of one agent's DSP processes; `Message` is how the agent records a message.
#[async_trait::async_trait]
pub trait DspPersistenceTrait: Send + Sync {
    type Process: Send + Sync;
    type State: Send;
    type Message: Send;

    /// The process by one of its pids, ours or the peer's, whoever asks.
    async fn hydrate_process(&self, pid: &str) -> Outcome<Option<Self::Process>>;

    /// Moves the process to the change's state and records the message that caused it.
    async fn change_process_state(
        &self,
        process: &Self::Process,
        change: StateChange<Self::State, Self::Message>,
    ) -> Outcome<Self::Process>;

    /// Adds a message to the process history.
    async fn record_message(
        &self,
        process: &Self::Process,
        message: RecordedMessage<Self::State, Self::Message>,
    ) -> Outcome<()>;
}
