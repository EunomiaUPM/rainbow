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

//! Inbound negotiation messages read into their DTOs, with no doubles.

use negotiation_agent::protocols::dsp::protocol_types::{
    NegotiationProcessMessageTrait, NegotiationProcessMessageWrapper,
    NegotiationRequestInitMessageDto,
};
use serde_json::json;

/// A contract request as the DSP TCK sends it: its own `@id` and a bare UUID `consumerPid`.
#[test]
fn a_request_with_a_message_id_and_a_bare_uuid_pid_is_read() {
    let body = json!({
        "@id": "6c89f109-3c06-4586-b4d5-0b2dc924c967",
        "@type": "ContractRequestMessage",
        "consumerPid": "8e290e24-def3-4e24-8a6c-24a1a5c1e6af",
        "offer": {
            "@id": "urn:odrl-policy:1962ff6f-4940-4958-9d33-5b83522e827f",
            "target": "urn:dataset:aa58c407-47a3-4ff5-bec0-a84540c4ad9d",
            "@type": "Offer",
            "permission": [{"action": "use"}]
        },
        "callbackAddress": "http://127.0.0.1:18899",
        "@context": ["https://w3id.org/dspace/2025/1/context.jsonld"]
    });
    let message: NegotiationProcessMessageWrapper<NegotiationRequestInitMessageDto> =
        serde_json::from_value(body).unwrap();

    assert_eq!(message.id.as_deref(), Some("6c89f109-3c06-4586-b4d5-0b2dc924c967"));
    assert_eq!(
        message.dto.get_consumer_pid().as_deref(),
        Some("8e290e24-def3-4e24-8a6c-24a1a5c1e6af")
    );
}

/// A message without `@id` still writes none back.
#[test]
fn a_message_without_an_id_writes_none() {
    let body = json!({
        "@type": "ContractRequestMessage",
        "consumerPid": "urn:uuid:8e290e24-def3-4e24-8a6c-24a1a5c1e6af",
        "offer": {"@id": "urn:odrl-policy:1", "target": "urn:dataset:1", "@type": "Offer"},
        "@context": ["https://w3id.org/dspace/2025/1/context.jsonld"]
    });
    let message: NegotiationProcessMessageWrapper<NegotiationRequestInitMessageDto> =
        serde_json::from_value(body).unwrap();
    assert!(serde_json::to_value(&message).unwrap().get("@id").is_none());
}
