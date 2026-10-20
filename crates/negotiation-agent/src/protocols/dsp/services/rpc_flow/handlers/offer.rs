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

//! `ContractOfferMessage` sent by the provider (DSP 8.3.3, 8.3.4): opening a negotiation or countering a request.

use ymir::types::oauth::UserInfo;
use serde_json::Value;
use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_rpc::{NegotiationRpcDomain, NegotiationRpcOutcome};
use crate::protocols::dsp::entities::state::NegotiationState;
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;
use crate::protocols::dsp::services::rpc_flow::handlers::OutboundTransition;
use crate::setup::context::AppContext;

/// Stores the process the consumer accepted, or moves ours to OFFERED; keeps the offer.
pub struct OfferHandler;

impl OfferHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        ctx: &AppContext,
        persistence: &NegotiationDspPersistence,
        user: &UserInfo,
        domain: NegotiationRpcDomain,
        reply: &Value,
    ) -> Outcome<NegotiationRpcOutcome> {
        let offered = NegotiationState::OFFERED;
        if domain.process.is_none() {
            return OutboundTransition::open(ctx, persistence, user, domain, reply, offered).await;
        }
        let offer = OutboundTransition::offer(&domain)?;
        OutboundTransition::apply(ctx, persistence, domain, reply, offered, offer).await
    }
}
