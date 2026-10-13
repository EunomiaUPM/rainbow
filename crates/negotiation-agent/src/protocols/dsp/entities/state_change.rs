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

//! How the negotiation flows record a message, and the persistence shapes built on it.

use std::fmt;

use common::dsp_common::persistence::{RecordedMessage, StateChange};
use serde_json::Value;
use urn::Urn;

use crate::protocols::dsp::entities::message_types::NegotiationMessageType;
use crate::protocols::dsp::entities::state::NegotiationState;

/// Whether a message came from the peer or went to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageDirection {
    Inbound,
    Outbound,
}

impl fmt::Display for MessageDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Inbound => "INBOUND",
            Self::Outbound => "OUTBOUND",
        })
    }
}

/// A message as kept in the process history: which one, which way and its JSON-LD.
#[derive(Debug, Clone)]
pub struct NegotiationMessageRecord {
    pub message: NegotiationMessageType,
    pub direction: MessageDirection,
    pub payload: Value,
}

pub type NegotiationStateChange = StateChange<NegotiationState, NegotiationMessageRecord>;
pub type NegotiationRecordedMessage = RecordedMessage<NegotiationState, NegotiationMessageRecord>;

/// What a message leaves besides the state change: an offer, an agreement or its activation.
#[derive(Debug, Clone)]
pub enum NegotiationAttachment {
    Nothing,
    Offer(OfferRecord),
    Agreement(AgreementRecord),
    /// The agreement already stored for the process becomes active (FINALIZED).
    ActivateAgreement,
}

/// An offer as stored: its id as sent and its content.
#[derive(Debug, Clone)]
pub struct OfferRecord {
    pub offer_id: String,
    pub content: Value,
}

/// An agreement as stored: its id, target, both parties and its content.
#[derive(Debug, Clone)]
pub struct AgreementRecord {
    pub id: Urn,
    pub target: Urn,
    pub provider_id: String,
    pub consumer_id: String,
    pub content: Value,
}
