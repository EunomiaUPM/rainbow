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

//! The DSP message shapes and the pure negotiation rules, with no doubles.

use common::facades::catalog::CatalogOffer;
use negotiation_agent::protocols::dsp::entities::message_types::NegotiationMessageType;
use negotiation_agent::protocols::dsp::entities::protocol_fields::{
    AgreementFields, NegotiationProtocolFields, OfferFields,
};
use negotiation_agent::protocols::dsp::entities::role::NegotiationRole;
use negotiation_agent::protocols::dsp::services::validator::{
    NegotiationDspRules, NegotiationRules,
};
use serde_json::json;

fn shape_ok(message: NegotiationMessageType, fields: &NegotiationProtocolFields) -> bool {
    NegotiationDspRules::message_shape(message)
        .validate(fields)
        .is_ok()
}

fn offer(id: &str, target: Option<&str>) -> Option<OfferFields> {
    Some(OfferFields {
        id: Some(id.into()),
        target: target.map(Into::into),
    })
}

/// A request opening a negotiation needs its consumerPid, an offer and a callbackAddress.
#[test]
fn an_opening_request_needs_pid_offer_and_callback() {
    let mut fields = NegotiationProtocolFields {
        consumer_pid: Some("c".into()),
        offer: offer("urn:offer:1", None),
        callback_address: Some("http://cb".into()),
        ..Default::default()
    };
    assert!(shape_ok(
        NegotiationMessageType::ContractRequestMessage,
        &fields
    ));
    fields.callback_address = None;
    assert!(!shape_ok(
        NegotiationMessageType::ContractRequestMessage,
        &fields
    ));
    fields.provider_pid = Some("p".into());
    assert!(shape_ok(
        NegotiationMessageType::ContractRequestMessage,
        &fields
    ));
    fields.offer = None;
    assert!(!shape_ok(
        NegotiationMessageType::ContractRequestMessage,
        &fields
    ));
}

/// An agreement needs an id, a URN target and both parties.
#[test]
fn an_agreement_needs_an_id_a_urn_target_and_parties() {
    let mut fields = NegotiationProtocolFields {
        consumer_pid: Some("c".into()),
        provider_pid: Some("p".into()),
        agreement: Some(AgreementFields {
            id: Some("urn:agreement:1".into()),
            target: Some("urn:dataset:1".into()),
            assigner: Some("did:web:p".into()),
            assignee: Some("did:web:c".into()),
            timestamp: None,
        }),
        ..Default::default()
    };
    assert!(shape_ok(
        NegotiationMessageType::ContractAgreementMessage,
        &fields
    ));
    fields.agreement.as_mut().unwrap().id = Some("6c89f109-3c06-4586".into());
    assert!(shape_ok(
        NegotiationMessageType::ContractAgreementMessage,
        &fields
    ));
    fields.agreement.as_mut().unwrap().target = Some("dataset-1".into());
    assert!(!shape_ok(
        NegotiationMessageType::ContractAgreementMessage,
        &fields
    ));
}

/// Every other message needs both pids.
#[test]
fn state_changes_need_both_pids() {
    let fields = NegotiationProtocolFields {
        consumer_pid: Some("c".into()),
        ..Default::default()
    };
    for message in [
        NegotiationMessageType::EventAccepted,
        NegotiationMessageType::ContractAgreementVerificationMessage,
        NegotiationMessageType::ContractNegotiationTerminationMessage,
    ] {
        assert!(!shape_ok(message, &fields), "{message}");
    }
}

/// Each side sends its own messages, and only receives the other side's.
#[test]
fn roles_send_and_receive_their_messages() {
    use NegotiationMessageType::*;
    use NegotiationRole::{Consumer, Provider};
    assert!(NegotiationRules::may_send(Consumer, ContractRequestMessage, "role").is_ok());
    assert!(NegotiationRules::may_send(Provider, ContractRequestMessage, "role").is_err());
    assert!(NegotiationRules::may_receive(Provider, ContractRequestMessage, "role").is_ok());
    assert!(
        NegotiationRules::may_receive(Consumer, ContractAgreementVerificationMessage, "role")
            .is_err()
    );
    assert!(
        NegotiationRules::may_send(Provider, ContractNegotiationTerminationMessage, "role").is_ok()
    );
}

/// A published offer passes on its own target, and fails on another or when missing.
#[test]
fn a_requested_offer_must_be_published_on_its_target() {
    let published = CatalogOffer {
        id: "urn:offer:1".into(),
        entity: "urn:dataset:1".into(),
        entity_type: "Dataset".into(),
        odrl_offer: json!({}),
    };
    let on = |target: &str| offer("urn:offer:1", Some(target));
    assert!(NegotiationRules::offer_published(
        Some(&published),
        on("urn:dataset:1").as_ref(),
        "offer"
    )
    .is_ok());
    assert!(NegotiationRules::offer_published(
        Some(&published),
        offer("urn:offer:1", None).as_ref(),
        "offer"
    )
    .is_ok());
    assert!(NegotiationRules::offer_published(
        Some(&published),
        on("urn:dataset:2").as_ref(),
        "offer"
    )
    .is_err());
    assert!(
        NegotiationRules::offer_published(None, on("urn:dataset:1").as_ref(), "offer").is_err()
    );
}
