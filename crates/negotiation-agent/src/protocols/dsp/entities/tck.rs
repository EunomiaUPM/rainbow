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

//! The messages a DSP TCK scenario makes the connector send, and the events it continues from.

use crate::protocols::dsp::entities::message_types::NegotiationRpcMessage;

/// The messages a scenario makes the connector send.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TckMessage {
    Offer,
    Agreement,
    Finalize,
    Terminate,
    Accept,
    Verify,
    CounterRequest,
}

impl TckMessage {
    pub fn rpc_message(self) -> NegotiationRpcMessage {
        match self {
            Self::Offer => NegotiationRpcMessage::Offer,
            Self::Agreement => NegotiationRpcMessage::Agreement,
            Self::Finalize => NegotiationRpcMessage::Finalization,
            Self::Terminate => NegotiationRpcMessage::Termination,
            Self::Accept => NegotiationRpcMessage::Acceptance,
            Self::Verify => NegotiationRpcMessage::Verification,
            Self::CounterRequest => NegotiationRpcMessage::Request,
        }
    }
}

/// What just happened to a negotiation, from which a scenario continues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TckEvent {
    /// We provide and a request opened the negotiation.
    ProviderRequested,
    /// We provide and the consumer countered our offer.
    ProviderCounterRequested,
    /// We provide and the consumer accepted our offer.
    ProviderAccepted,
    /// We provide and the consumer verified our agreement.
    ProviderVerified,
    /// We consume and our request opened the negotiation.
    ConsumerRequested,
    /// We consume and the provider offered.
    ConsumerOffered,
    /// We consume and the provider agreed.
    ConsumerAgreed,
}
