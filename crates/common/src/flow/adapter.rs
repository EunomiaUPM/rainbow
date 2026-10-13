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

//! The driving side of a flow: one adapter per agent and transport extracts the input in
//! stages and turns the output or the error into the transport's response.

use crate::flow::{FlowError, FlowTrait};

/// Extracts the flow's input from what the transport hands over, and answers with its outcome.
#[async_trait::async_trait]
pub trait FlowAdapterTrait<F: FlowTrait>: Send + Sync {
    /// What the transport hands over (an axum request, a gRPC message, a JSON body).
    type Raw: Send;
    type Response;
    /// What an error body needs from the input (pids, ids); `Default` when extraction failed.
    type ErrorContext: Default + Send;

    /// The caller the auth layer resolved and the input; `key` is what the route expects, and
    /// checking the body against it is a shape rule.
    async fn extract(
        &self,
        key: &F::Key,
        raw: Self::Raw,
    ) -> Result<(F::Caller, F::Input), FlowError>;
    fn error_context(&self, input: &F::Input) -> Self::ErrorContext;
    fn respond(&self, key: &F::Key, output: F::Output) -> Self::Response;
    fn reject(&self, key: &F::Key, context: Self::ErrorContext, error: FlowError)
        -> Self::Response;
}
