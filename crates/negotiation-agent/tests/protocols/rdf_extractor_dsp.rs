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

//! DspNegotiation: the DSP negotiation fields read off the expanded message, with no doubles.

use common::rdf::RdfMessage;
use negotiation_agent::protocols::dsp::entities::message_types::NegotiationMessageType;
use negotiation_agent::protocols::dsp::entities::rdf_extractor_dsp::DspNegotiation;
use serde_json::json;

async fn read(message: serde_json::Value) -> RdfMessage<DspNegotiation> {
    RdfMessage::<DspNegotiation>::read(message)
        .await
        .expect("readable")
}

/// A contract request as the DSP TCK sends it: bare UUID pid, offer with id and target.
#[tokio::test]
async fn a_tck_request_reads_pids_offer_and_callback() {
    let read = read(json!({
        "@context": ["https://w3id.org/dspace/2025/1/context.jsonld"],
        "@id": "6c89f109-3c06-4586-b4d5-0b2dc924c967",
        "@type": "ContractRequestMessage",
        "consumerPid": "8e290e24-def3-4e24-8a6c-24a1a5c1e6af",
        "offer": {
            "@id": "urn:odrl-policy:1962ff6f",
            "@type": "Offer",
            "target": "urn:dataset:aa58c407",
            "permission": [{"action": "use"}]
        },
        "callbackAddress": "http://127.0.0.1:18899"
    }))
    .await;
    assert_eq!(read.message, NegotiationMessageType::ContractRequestMessage);
    let fields = read.fields;
    assert_eq!(
        fields.consumer_pid.as_deref(),
        Some("8e290e24-def3-4e24-8a6c-24a1a5c1e6af")
    );
    assert_eq!(fields.provider_pid, None);
    assert_eq!(
        fields.callback_address.as_deref(),
        Some("http://127.0.0.1:18899")
    );
    let offer = fields.offer.expect("offer");
    assert_eq!(offer.id.as_deref(), Some("urn:odrl-policy:1962ff6f"));
    assert_eq!(offer.target.as_deref(), Some("urn:dataset:aa58c407"));
}

/// An offer by id only has no target.
#[tokio::test]
async fn an_offer_by_id_has_no_target() {
    let read = read(json!({
        "@context": "https://w3id.org/dspace/2025/1/context.jsonld",
        "@type": "ContractRequestMessage",
        "consumerPid": "urn:uuid:c",
        "providerPid": "urn:uuid:p",
        "offer": {"@id": "urn:offer:1"}
    }))
    .await;
    let offer = read.fields.offer.expect("offer");
    assert_eq!(offer.id.as_deref(), Some("urn:offer:1"));
    assert_eq!(offer.target, None);
}

/// An agreement message reads the agreement's id, target and parties.
#[tokio::test]
async fn an_agreement_reads_its_parties() {
    let read = read(json!({
        "@context": "https://w3id.org/dspace/2025/1/context.jsonld",
        "@type": "ContractAgreementMessage",
        "consumerPid": "urn:uuid:c",
        "providerPid": "urn:uuid:p",
        "agreement": {
            "@id": "urn:agreement:1",
            "@type": "Agreement",
            "target": "urn:dataset:1",
            "assigner": "did:web:provider",
            "assignee": "did:web:consumer",
            "timestamp": "2026-10-13T10:00:00Z",
            "permission": [{"action": "use"}]
        }
    }))
    .await;
    let agreement = read.fields.agreement.expect("agreement");
    assert_eq!(agreement.id.as_deref(), Some("urn:agreement:1"));
    assert_eq!(agreement.target.as_deref(), Some("urn:dataset:1"));
    assert_eq!(agreement.assigner.as_deref(), Some("did:web:provider"));
    assert_eq!(agreement.assignee.as_deref(), Some("did:web:consumer"));
    assert_eq!(agreement.timestamp.as_deref(), Some("2026-10-13T10:00:00Z"));
}

/// The event type decides the message, prefixed or not.
#[tokio::test]
async fn events_are_typed_by_their_event() {
    for (event, expected) in [
        ("ACCEPTED", NegotiationMessageType::EventAccepted),
        ("dspace:FINALIZED", NegotiationMessageType::EventFinalized),
    ] {
        let read = read(json!({
            "@context": "https://w3id.org/dspace/2025/1/context.jsonld",
            "@type": "ContractNegotiationEventMessage",
            "consumerPid": "urn:uuid:c",
            "providerPid": "urn:uuid:p",
            "eventType": event
        }))
        .await;
        assert_eq!(read.message, expected, "{event}");
    }
}

/// A termination keeps its code and every reason.
#[tokio::test]
async fn a_termination_keeps_code_and_reasons() {
    let read = read(json!({
        "@context": "https://w3id.org/dspace/2025/1/context.jsonld",
        "@type": "ContractNegotiationTerminationMessage",
        "consumerPid": "urn:uuid:c",
        "providerPid": "urn:uuid:p",
        "code": "rejected",
        "reason": ["one", "two"]
    }))
    .await;
    assert_eq!(read.fields.code.as_deref(), Some("rejected"));
    let mut reasons = read.fields.reason;
    reasons.sort();
    assert_eq!(reasons, ["one", "two"]);
}

/// A peer's `ContractNegotiation` gives the pids and the state as a bare term.
#[tokio::test]
async fn an_ack_reads_its_state() {
    let read = read(json!({
        "@context": "https://w3id.org/dspace/2025/1/context.jsonld",
        "@type": "ContractNegotiation",
        "consumerPid": "urn:uuid:c",
        "providerPid": "urn:uuid:p",
        "state": "REQUESTED"
    }))
    .await;
    assert_eq!(read.message, NegotiationMessageType::ContractNegotiation);
    assert_eq!(read.fields.provider_pid.as_deref(), Some("urn:uuid:p"));
    assert_eq!(read.fields.state.as_deref(), Some("REQUESTED"));
}

/// Messages built on this side carry the event type of an event message.
#[tokio::test]
async fn built_events_carry_their_event_type() {
    let members = DspNegotiation::members(
        NegotiationMessageType::EventFinalized,
        serde_json::Map::from_iter([
            ("consumerPid".to_string(), json!("urn:uuid:c")),
            ("providerPid".to_string(), json!("urn:uuid:p")),
        ]),
    );
    let built =
        RdfMessage::<DspNegotiation>::build(&NegotiationMessageType::EventFinalized, members)
            .await
            .unwrap();
    assert_eq!(built.json["eventType"], "FINALIZED");
    assert_eq!(built.message, NegotiationMessageType::EventFinalized);
}
