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

//! The DSP negotiation message types; the event message is split by the event it carries.

use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// DSP negotiation messages this agent sends and receives (DSP 8.2, 8.3).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NegotiationMessageType {
    ContractRequestMessage,
    ContractOfferMessage,
    /// `ContractNegotiationEventMessage` with `eventType` ACCEPTED.
    EventAccepted,
    /// `ContractNegotiationEventMessage` with `eventType` FINALIZED.
    EventFinalized,
    ContractAgreementMessage,
    ContractAgreementVerificationMessage,
    ContractNegotiationTerminationMessage,
    ContractNegotiation,
    ContractNegotiationError,
}

impl NegotiationMessageType {
    pub const EVENT_MESSAGE: &'static str = "ContractNegotiationEventMessage";
    pub const ACCEPTED: &'static str = "ACCEPTED";
    pub const FINALIZED: &'static str = "FINALIZED";

    /// The `@type` the message has on the wire.
    pub fn wire_type(self) -> &'static str {
        match self {
            Self::ContractRequestMessage => "ContractRequestMessage",
            Self::ContractOfferMessage => "ContractOfferMessage",
            Self::EventAccepted | Self::EventFinalized => Self::EVENT_MESSAGE,
            Self::ContractAgreementMessage => "ContractAgreementMessage",
            Self::ContractAgreementVerificationMessage => "ContractAgreementVerificationMessage",
            Self::ContractNegotiationTerminationMessage => "ContractNegotiationTerminationMessage",
            Self::ContractNegotiation => "ContractNegotiation",
            Self::ContractNegotiationError => "ContractNegotiationError",
        }
    }

    /// The `eventType` an event message carries.
    pub fn event_type(self) -> Option<&'static str> {
        match self {
            Self::EventAccepted => Some(Self::ACCEPTED),
            Self::EventFinalized => Some(Self::FINALIZED),
            _ => None,
        }
    }

    /// The message a wire `@type` names; an event message needs its `eventType` as well.
    pub fn from_wire(term: &str, event_type: Option<&str>) -> Option<Self> {
        let term = Self::last_segment(term);
        Some(match term {
            "ContractRequestMessage" => Self::ContractRequestMessage,
            "ContractOfferMessage" => Self::ContractOfferMessage,
            Self::EVENT_MESSAGE => match event_type.map(Self::last_segment) {
                Some(Self::ACCEPTED) => Self::EventAccepted,
                Some(Self::FINALIZED) => Self::EventFinalized,
                _ => return None,
            },
            "ContractAgreementMessage" => Self::ContractAgreementMessage,
            "ContractAgreementVerificationMessage" => Self::ContractAgreementVerificationMessage,
            "ContractNegotiationTerminationMessage" => Self::ContractNegotiationTerminationMessage,
            "ContractNegotiation" => Self::ContractNegotiation,
            "ContractNegotiationError" => Self::ContractNegotiationError,
            _ => return None,
        })
    }

    /// A term without the prefix or namespace a peer may write it with.
    fn last_segment(term: &str) -> &str {
        term.rsplit([':', '/', '#']).next().unwrap_or(term)
    }
}

impl Display for NegotiationMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.event_type() {
            Some(event) => write!(f, "{}({event})", self.wire_type()),
            None => f.write_str(self.wire_type()),
        }
    }
}

/// The RPC messages a local user sends to make the agent send a DSP message; an opening
/// request or offer is told apart from a counter one.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NegotiationRpcMessage {
    RequestInit,
    Request,
    OfferInit,
    Offer,
    Acceptance,
    Agreement,
    Verification,
    Finalization,
    Termination,
}

impl NegotiationRpcMessage {
    pub const ALL: [Self; 9] = [
        Self::RequestInit,
        Self::Request,
        Self::OfferInit,
        Self::Offer,
        Self::Acceptance,
        Self::Agreement,
        Self::Verification,
        Self::Finalization,
        Self::Termination,
    ];

    /// The DSP message it sends.
    pub fn message(self) -> NegotiationMessageType {
        use NegotiationMessageType::*;
        match self {
            Self::RequestInit | Self::Request => ContractRequestMessage,
            Self::OfferInit | Self::Offer => ContractOfferMessage,
            Self::Acceptance => EventAccepted,
            Self::Agreement => ContractAgreementMessage,
            Self::Verification => ContractAgreementVerificationMessage,
            Self::Finalization => EventFinalized,
            Self::Termination => ContractNegotiationTerminationMessage,
        }
    }

    /// Whether it opens a negotiation rather than moving one.
    pub fn opens(self) -> bool {
        matches!(self, Self::RequestInit | Self::OfferInit)
    }

    /// The name inside its eunomia type, `RpcNegotiation{name}Message`.
    pub fn name(self) -> &'static str {
        match self {
            Self::RequestInit => "RequestInit",
            Self::Request => "Request",
            Self::OfferInit => "OfferInit",
            Self::Offer => "Offer",
            Self::Acceptance => "Acceptance",
            Self::Agreement => "Agreement",
            Self::Verification => "Verification",
            Self::Finalization => "Finalization",
            Self::Termination => "Termination",
        }
    }

    /// The message whose eunomia type is `RpcNegotiation{name}Message`.
    pub fn from_type(term: &str) -> Option<Self> {
        let name = term
            .strip_prefix("RpcNegotiation")?
            .strip_suffix("Message")?;
        Self::ALL.into_iter().find(|m| m.name() == name)
    }
}

impl Display for NegotiationRpcMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RpcNegotiation{}Message", self.name())
    }
}
