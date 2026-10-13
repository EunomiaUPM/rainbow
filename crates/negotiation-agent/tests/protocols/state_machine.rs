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

//! The DSP 8.1 state machine, the states each message reaches and who sends it, with no doubles.

use negotiation_agent::protocols::dsp::entities::message_types::NegotiationMessageType::{self, *};
use negotiation_agent::protocols::dsp::entities::role::NegotiationRole;
use negotiation_agent::protocols::dsp::entities::state::NegotiationState::{self, *};

fn allowed(current: Option<NegotiationState>, message: NegotiationMessageType) -> bool {
    NegotiationState::MACHINE
        .check(current.as_ref(), &message, "state")
        .is_ok()
}

/// A negotiation opens with a request or an offer, and with nothing else.
#[test]
fn only_a_request_or_an_offer_opens_a_negotiation() {
    assert!(allowed(None, ContractRequestMessage));
    assert!(allowed(None, ContractOfferMessage));
    for message in [
        EventAccepted,
        EventFinalized,
        ContractAgreementMessage,
        ContractAgreementVerificationMessage,
        ContractNegotiationTerminationMessage,
    ] {
        assert!(!allowed(None, message), "{message}");
    }
}

/// The happy paths of DSP 8.1: request, offer, accept, agree, verify, finalize.
#[test]
fn the_happy_paths_are_allowed() {
    assert!(allowed(Some(REQUESTED), ContractOfferMessage));
    assert!(allowed(Some(OFFERED), ContractRequestMessage));
    assert!(allowed(Some(OFFERED), EventAccepted));
    assert!(allowed(Some(REQUESTED), ContractAgreementMessage));
    assert!(allowed(Some(ACCEPTED), ContractAgreementMessage));
    assert!(allowed(Some(AGREED), ContractAgreementVerificationMessage));
    assert!(allowed(Some(VERIFIED), EventFinalized));
}

/// Out of order messages are refused, as the TCK's invalid transitions expect.
#[test]
fn out_of_order_messages_are_refused() {
    assert!(!allowed(
        Some(REQUESTED),
        ContractAgreementVerificationMessage
    ));
    assert!(!allowed(
        Some(OFFERED),
        ContractAgreementVerificationMessage
    ));
    assert!(!allowed(Some(REQUESTED), EventAccepted));
    assert!(!allowed(Some(AGREED), EventFinalized));
    assert!(!allowed(Some(REQUESTED), ContractRequestMessage));
}

/// A finished negotiation takes no termination; any open one does.
#[test]
fn termination_needs_an_open_negotiation() {
    for state in [REQUESTED, OFFERED, ACCEPTED, AGREED, VERIFIED] {
        assert!(
            allowed(Some(state), ContractNegotiationTerminationMessage),
            "{state}"
        );
    }
    for state in [FINALIZED, TERMINATED] {
        assert!(
            !allowed(Some(state), ContractNegotiationTerminationMessage),
            "{state}"
        );
    }
}

/// Each message leaves the process in its state; the ACK and the error move nothing.
#[test]
fn each_message_reaches_its_state() {
    assert_eq!(NegotiationState::reached_by(EventAccepted), Some(ACCEPTED));
    assert_eq!(
        NegotiationState::reached_by(EventFinalized),
        Some(FINALIZED)
    );
    assert_eq!(
        NegotiationState::reached_by(ContractNegotiationTerminationMessage),
        Some(TERMINATED)
    );
    assert_eq!(NegotiationState::reached_by(ContractNegotiation), None);
}

/// The consumer requests, accepts and verifies; the provider offers, agrees and finalizes.
#[test]
fn each_message_has_its_sender() {
    use NegotiationRole::{Consumer, Provider};
    for message in [
        ContractRequestMessage,
        EventAccepted,
        ContractAgreementVerificationMessage,
    ] {
        assert_eq!(
            NegotiationRole::sender_of(message),
            Some(Consumer),
            "{message}"
        );
    }
    for message in [
        ContractOfferMessage,
        ContractAgreementMessage,
        EventFinalized,
    ] {
        assert_eq!(
            NegotiationRole::sender_of(message),
            Some(Provider),
            "{message}"
        );
    }
    assert_eq!(
        NegotiationRole::sender_of(ContractNegotiationTerminationMessage),
        None
    );
}

/// Prefixed states are read, and stored roles in any case.
#[test]
fn states_and_roles_parse_leniently() {
    assert_eq!("dspace:AGREED".parse::<NegotiationState>().unwrap(), AGREED);
    assert_eq!(
        "Provider".parse::<NegotiationRole>().unwrap(),
        NegotiationRole::Provider
    );
    assert_eq!(
        "CONSUMER".parse::<NegotiationRole>().unwrap(),
        NegotiationRole::Consumer
    );
    assert!("RELAY".parse::<NegotiationRole>().is_err());
}
