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

//! What the outbound negotiation RPC answers, as a eunomia `RpcNegotiationResult`.

use common::eunomia::EUNOMIA_CONTEXT_URL;
use serde::Serialize;
use serde_json::Value;

use crate::protocols::dsp::entities::context_rpc::NegotiationRpcOutcome;
use crate::services::negotiation_process::views::NegotiationProcessView;

/// What was asked, what the peer said and the process as it stands.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NegotiationRpcResult {
    #[serde(rename = "@context")]
    pub context: &'static str,
    #[serde(rename = "@type")]
    pub kind: &'static str,
    pub request: Value,
    pub response: Value,
    pub negotiation_agent_model: NegotiationProcessView,
}

impl NegotiationRpcResult {
    pub const TYPE: &'static str = "RpcNegotiationResult";

    pub fn assemble(outcome: NegotiationRpcOutcome) -> Self {
        let identifiers = outcome.process.identifiers();
        Self {
            context: EUNOMIA_CONTEXT_URL,
            kind: Self::TYPE,
            request: outcome.request,
            response: outcome.response,
            negotiation_agent_model: NegotiationProcessView::assemble(
                outcome.process.model,
                identifiers,
            ),
        }
    }
}
