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

//! The transport side of a flow: one codec per agent and transport.

use crate::flow::{FlowError, FlowTrait};

/// Turns transport input into the flow's input, and its output or error back.
#[async_trait::async_trait]
pub trait FlowCodecTrait<F: FlowTrait>: Send + Sync {
    /// What the transport hands over (an axum request, a gRPC message, a JSON body).
    type Raw: Send;
    type Response;
    /// What an error body needs from the input (pids, ids); `Default` when decoding failed.
    type ErrorContext: Default + Send;

    /// `key` is what the route expects; checking the body against it is a shape rule.
    async fn decode(&self, key: &F::Key, raw: Self::Raw) -> Result<F::Input, FlowError>;
    fn error_context(&self, input: &F::Input) -> Self::ErrorContext;
    fn encode(&self, key: &F::Key, output: F::Output) -> Self::Response;
    fn encode_error(
        &self,
        key: &F::Key,
        context: Self::ErrorContext,
        error: FlowError,
    ) -> Self::Response;
}
