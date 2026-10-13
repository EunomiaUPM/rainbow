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

//! `ContractOfferMessage` received by the consumer (DSP 8.3.3, 8.3.4): opens the negotiation or counters a request.

use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_common::NegotiationProcessSlot;
use crate::protocols::dsp::entities::context_dsp::{NegotiationDspDomain, NegotiationDspOutcome};
use crate::protocols::dsp::entities::state::NegotiationState;
use crate::protocols::dsp::services::dsp_flow::handlers::InboundTransition;
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;

/// Opens the process with a new `consumerPid`, or moves it to OFFERED; keeps the offer.
pub struct OfferHandler;

impl OfferHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        persistence: &NegotiationDspPersistence,
        domain: NegotiationDspDomain,
    ) -> Outcome<NegotiationDspOutcome> {
        let offered = NegotiationState::OFFERED;
        match domain.process {
            NegotiationProcessSlot::New { .. } => {
                InboundTransition::open(persistence, &domain, offered).await
            }
            NegotiationProcessSlot::Existing(_) => {
                let offer = InboundTransition::offer(&domain)?;
                InboundTransition::apply(persistence, &domain, offered, offer).await
            }
        }
    }
}
