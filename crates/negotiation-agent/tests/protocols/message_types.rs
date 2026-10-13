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

//! NegotiationMessageType: wire types, the event split and lenient parsing, with no doubles.

use negotiation_agent::protocols::dsp::entities::message_types::NegotiationMessageType;
use negotiation_agent::protocols::dsp::entities::message_types::NegotiationMessageType::*;

const ALL: [NegotiationMessageType; 9] = [
    ContractRequestMessage,
    ContractOfferMessage,
    EventAccepted,
    EventFinalized,
    ContractAgreementMessage,
    ContractAgreementVerificationMessage,
    ContractNegotiationTerminationMessage,
    ContractNegotiation,
    ContractNegotiationError,
];

/// Every type reads back from its wire `@type` and event.
#[test]
fn every_type_round_trips_through_the_wire() {
    for message in ALL {
        let read = NegotiationMessageType::from_wire(message.wire_type(), message.event_type());
        assert_eq!(read, Some(message), "{message}");
    }
}

/// Both events share the event message type and differ by `eventType`.
#[test]
fn events_share_the_wire_type() {
    assert_eq!(EventAccepted.wire_type(), "ContractNegotiationEventMessage");
    assert_eq!(
        EventFinalized.wire_type(),
        "ContractNegotiationEventMessage"
    );
    assert_ne!(EventAccepted.to_string(), EventFinalized.to_string());
}

/// Prefixed terms are read as the bare term.
#[test]
fn prefixed_terms_are_read() {
    assert_eq!(
        NegotiationMessageType::from_wire("dspace:ContractOfferMessage", None),
        Some(ContractOfferMessage)
    );
    assert_eq!(
        NegotiationMessageType::from_wire(
            "ContractNegotiationEventMessage",
            Some("https://w3id.org/dspace/2025/1/FINALIZED")
        ),
        Some(EventFinalized)
    );
}

/// An event message without a known event, or an unknown type, names no message.
#[test]
fn unknown_events_and_types_are_none() {
    assert_eq!(
        NegotiationMessageType::from_wire("ContractNegotiationEventMessage", None),
        None
    );
    assert_eq!(
        NegotiationMessageType::from_wire("ContractNegotiationEventMessage", Some("REJECTED")),
        None
    );
    assert_eq!(
        NegotiationMessageType::from_wire("TransferStartMessage", None),
        None
    );
}
