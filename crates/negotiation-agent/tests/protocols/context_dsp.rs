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

//! The inbound DSP stages: request, JSON, RDF canonicalization, typed fields and the
//! idempotency key, with no doubles.

use axum::extract::Request;
use common::facades::grants_facade::VerifiedPeer;
use common::oauth::{RolePath, Visibility};
use negotiation_agent::protocols::dsp::entities::context_dsp::{
    NegotiationDspParsed, NegotiationDspRaw, NegotiationDspRdf, NegotiationDspTyped,
};
use negotiation_agent::protocols::dsp::entities::message_types::NegotiationMessageType;
use ymir::errors::Outcome;

const VERIFICATION: &str = r#"{"@context":"https://w3id.org/dspace/2025/1/context.jsonld","@type":"ContractAgreementVerificationMessage","providerPid":"urn:uuid:p","consumerPid":"urn:uuid:c"}"#;
const VERIFICATION_REORDERED: &str = r#"{"consumerPid":"urn:uuid:c","@type":"ContractAgreementVerificationMessage","providerPid":"urn:uuid:p","@context":"https://w3id.org/dspace/2025/1/context.jsonld"}"#;

fn peer(id: &str) -> VerifiedPeer {
    VerifiedPeer {
        participant_id: id.into(),
        role: RolePath::root(),
        visibility: Visibility::Public,
    }
}

async fn typed(
    peer_id: &str,
    body: &'static str,
    route: NegotiationMessageType,
) -> Outcome<NegotiationDspTyped> {
    let request = Request::builder()
        .method("POST")
        .uri("/negotiations/urn:uuid:p/agreement/verification")
        .body(axum::body::Body::from(body))
        .unwrap();
    let raw = NegotiationDspRaw::from_request(peer(peer_id), request).await?;
    let parsed = NegotiationDspParsed::from_raw(raw, route)?;
    NegotiationDspTyped::from_rdf(NegotiationDspRdf::from_parsed(parsed).await?)
}

/// The stages keep the resolved peer, the route and the body's own type.
#[tokio::test]
async fn stages_keep_peer_route_and_type() {
    let typed = typed(
        "did:web:consumer",
        VERIFICATION,
        NegotiationMessageType::ContractAgreementVerificationMessage,
    )
    .await
    .unwrap();
    assert_eq!(typed.peer().participant_id, "did:web:consumer");
    assert_eq!(
        typed.route(),
        NegotiationMessageType::ContractAgreementVerificationMessage
    );
    assert_eq!(
        typed.message,
        NegotiationMessageType::ContractAgreementVerificationMessage
    );
    assert_eq!(typed.fields.consumer_pid.as_deref(), Some("urn:uuid:c"));
}

/// Key order does not change the canonical hash nor the idempotency key.
#[tokio::test]
async fn hash_and_key_ignore_how_the_body_is_written() {
    let route = NegotiationMessageType::ContractAgreementVerificationMessage;
    let a = typed("did:web:consumer", VERIFICATION, route)
        .await
        .unwrap();
    let b = typed("did:web:consumer", VERIFICATION_REORDERED, route)
        .await
        .unwrap();
    assert_eq!(a.rdf.canonical_hash, b.rdf.canonical_hash);
    assert_eq!(a.idempotency_key, b.idempotency_key);
}

/// The same message from another peer is another key.
#[tokio::test]
async fn the_key_depends_on_the_peer() {
    let route = NegotiationMessageType::ContractAgreementVerificationMessage;
    let a = typed("did:web:consumer", VERIFICATION, route)
        .await
        .unwrap();
    let b = typed("did:web:other", VERIFICATION, route).await.unwrap();
    assert_ne!(a.idempotency_key, b.idempotency_key);
}

/// A body that is not JSON fails before RDF.
#[tokio::test]
async fn a_body_that_is_not_json_fails() {
    let err = typed(
        "did:web:consumer",
        "not json",
        NegotiationMessageType::ContractAgreementVerificationMessage,
    )
    .await
    .unwrap_err();
    assert!(err.reason().contains("not JSON"), "{}", err.reason());
}

/// The offer is kept as sent, rules included, next to its extracted identity.
#[tokio::test]
async fn the_offer_is_kept_as_sent() {
    let body = r#"{"@context":"https://w3id.org/dspace/2025/1/context.jsonld","@type":"ContractRequestMessage","consumerPid":"urn:uuid:c","offer":{"@id":"urn:offer:1","@type":"Offer","target":"urn:dataset:1","permission":[{"action":"use"}]},"callbackAddress":"http://cb"}"#;
    let typed = typed(
        "did:web:consumer",
        body,
        NegotiationMessageType::ContractRequestMessage,
    )
    .await
    .unwrap();
    assert_eq!(
        typed.offer_content().unwrap()["permission"][0]["action"],
        "use"
    );
    assert_eq!(
        typed.fields.offer.unwrap().target.as_deref(),
        Some("urn:dataset:1")
    );
}
