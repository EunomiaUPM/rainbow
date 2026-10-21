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

//! `ContractNegotiationTerminationMessage` sent (DSP 8.2.6, 8.3.7), by either side.

use serde_json::Value;
use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_rpc::{NegotiationRpcDomain, NegotiationRpcOutcome};
use crate::protocols::dsp::entities::state::NegotiationState;
use crate::protocols::dsp::entities::state_change::NegotiationAttachment;
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;
use crate::protocols::dsp::services::rpc_flow::handlers::OutboundTransition;

/// Moves our process to TERMINATED, keeping our code and reasons.
pub struct TerminationHandler;

impl TerminationHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        persistence: &NegotiationDspPersistence,
        domain: NegotiationRpcDomain,
        reply: &Value,
    ) -> Outcome<NegotiationRpcOutcome> {
        let target = NegotiationState::TERMINATED;
        let nothing = NegotiationAttachment::Nothing;
        OutboundTransition::apply(persistence, domain, reply, target, nothing).await
    }
}
