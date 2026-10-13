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

//! `ContractNegotiationTerminationMessage` received (DSP 8.2.6, 8.3.7), from either side.

use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_dsp::{NegotiationDspDomain, NegotiationDspOutcome};
use crate::protocols::dsp::entities::state::NegotiationState;
use crate::protocols::dsp::entities::state_change::NegotiationAttachment;
use crate::protocols::dsp::services::dsp_flow::handlers::InboundTransition;
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;

/// Moves the process to TERMINATED, keeping the peer's code and reasons.
pub struct TerminationHandler;

impl TerminationHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        persistence: &NegotiationDspPersistence,
        domain: NegotiationDspDomain,
    ) -> Outcome<NegotiationDspOutcome> {
        InboundTransition::apply(
            persistence,
            &domain,
            NegotiationState::TERMINATED,
            NegotiationAttachment::Nothing,
        )
        .await
    }
}
