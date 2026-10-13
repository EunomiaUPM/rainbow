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

//! The TCK script of negotiation: what each test makes the connector send, with no doubles.

use common::tck::{TckScriptTrait, TckStep};
use negotiation_agent::protocols::dsp::entities::message_types::NegotiationRpcMessage;
use negotiation_agent::protocols::dsp::entities::tck::{TckEvent, TckMessage};
use negotiation_agent::protocols::dsp::services::tck::service::NegotiationTckScript;

fn sends(test: &str, event: TckEvent) -> Vec<TckMessage> {
    NegotiationTckScript::script(test, event)
        .iter()
        .filter_map(|step| match step {
            TckStep::Send(message) => Some(*message),
            TckStep::Pause => None,
        })
        .collect()
}

/// CN0103 walks a provider through offer, agreement and finalization.
#[test]
fn cn0103_offers_agrees_and_finalizes() {
    assert_eq!(
        sends("CN0103", TckEvent::ProviderRequested),
        [TckMessage::Offer]
    );
    assert_eq!(
        sends("CN0103", TckEvent::ProviderAccepted),
        [TckMessage::Agreement]
    );
    assert_eq!(
        sends("CN0103", TckEvent::ProviderVerified),
        [TckMessage::Finalize]
    );
}

/// CNC0101 walks a consumer through acceptance and verification.
#[test]
fn cnc0101_accepts_and_verifies() {
    assert_eq!(
        sends("CNC0101", TckEvent::ConsumerOffered),
        [TckMessage::Accept]
    );
    assert_eq!(
        sends("CNC0101", TckEvent::ConsumerAgreed),
        [TckMessage::Verify]
    );
}

/// CN0205 and CNC0202 pause before terminating.
#[test]
fn terminations_after_a_pause() {
    assert_eq!(
        NegotiationTckScript::script("CN0205", TckEvent::ProviderRequested),
        [
            TckStep::Send(TckMessage::Offer),
            TckStep::Pause,
            TckStep::Send(TckMessage::Terminate)
        ]
    );
    assert_eq!(
        NegotiationTckScript::script("CNC0202", TckEvent::ConsumerRequested),
        [TckStep::Pause, TckStep::Send(TckMessage::Terminate)]
    );
}

/// Tests the TCK drives alone, and events a test does not continue from, play nothing.
#[test]
fn other_tests_and_events_play_nothing() {
    assert!(sends("CN0202", TckEvent::ProviderRequested).is_empty());
    assert!(sends("CNC0201", TckEvent::ConsumerOffered).is_empty());
    assert!(sends("CN0101", TckEvent::ProviderAccepted).is_empty());
    assert!(sends("TP0101", TckEvent::ProviderRequested).is_empty());
}

/// Each scenario message goes out as the RPC that sends it.
#[test]
fn scenario_messages_map_to_their_rpc() {
    assert_eq!(
        TckMessage::CounterRequest.rpc_message(),
        NegotiationRpcMessage::Request
    );
    assert_eq!(
        TckMessage::Finalize.rpc_message(),
        NegotiationRpcMessage::Finalization
    );
    assert_eq!(
        TckMessage::Accept.rpc_message(),
        NegotiationRpcMessage::Acceptance
    );
}
