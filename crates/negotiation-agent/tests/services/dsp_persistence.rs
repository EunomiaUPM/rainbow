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

//! NegotiationDspPersistence: hydration over mocked repositories, writes over mocked services.

use std::collections::HashMap;

use common::dsp_common::persistence::{DspPersistenceTrait, StateChange};
use negotiation_agent::data::repo::negotiation_process::MockNegotiationProcessRepoTrait;
use negotiation_agent::data::repo::negotiation_process_identifier::MockNegotiationIdentifierRepoTrait;
use negotiation_agent::protocols::dsp::entities::message_types::NegotiationMessageType;
use negotiation_agent::protocols::dsp::entities::process::NegotiationProcess;
use negotiation_agent::protocols::dsp::entities::role::NegotiationRole;
use negotiation_agent::protocols::dsp::entities::state::NegotiationState;
use negotiation_agent::protocols::dsp::entities::state_change::{
    AgreementRecord, MessageDirection, NegotiationAttachment, NegotiationMessageRecord, OfferRecord,
};
use negotiation_agent::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;
use negotiation_agent::services::agreement::views::AgreementView;
use negotiation_agent::services::agreement::MockAgreementServiceTrait;
use negotiation_agent::services::negotiation_message::views::NegotiationMessageView;
use negotiation_agent::services::negotiation_message::MockNegotiationMessageServiceTrait;
use negotiation_agent::services::negotiation_process::views::NegotiationProcessView;
use negotiation_agent::services::negotiation_process::MockNegotiationProcessServiceTrait;
use negotiation_agent::services::offer::views::OfferView;
use negotiation_agent::services::offer::MockOfferServiceTrait;
use common::dsp_common::persistence::INITIAL_STATE;
use serde_json::json;

use crate::support::context::TestContext;
use crate::support::fixtures::{
    agreement_row, identifier_row, message_row, offer_row, process_row, urn,
};

const CONSUMER_PID: &str = "urn:uuid:consumer-pid";
const PROVIDER_PID: &str = "urn:uuid:provider-pid";

/// Repos that hydrate process 1 as `role` in `state` under both pids.
fn stored(role: &'static str, state: &'static str) -> TestContext {
    let mut processes = MockNegotiationProcessRepoTrait::new();
    processes
        .expect_get_negotiation_process_by_key_value()
        .returning(move |_, _| Ok(Some(process_row(1, role, state))));
    let mut identifiers = MockNegotiationIdentifierRepoTrait::new();
    identifiers
        .expect_get_identifiers_by_process_id()
        .returning(|_| {
            Ok(vec![
                identifier_row(1, NegotiationProcess::CONSUMER_PID, CONSUMER_PID),
                identifier_row(1, NegotiationProcess::PROVIDER_PID, PROVIDER_PID),
            ])
        });
    TestContext::new()
        .process_repo(processes)
        .identifier_repo(identifiers)
}

fn record(message: NegotiationMessageType) -> NegotiationMessageRecord {
    NegotiationMessageRecord {
        message,
        direction: MessageDirection::Inbound,
        payload: json!({"@type": message.wire_type()}),
    }
}

/// A message service that stores any message as row 200.
fn messages(
    expected_type: &'static str,
    from: &'static str,
    to: &'static str,
) -> MockNegotiationMessageServiceTrait {
    let mut messages = MockNegotiationMessageServiceTrait::new();
    messages
        .expect_create()
        .withf(move |_, cmd| {
            cmd.message_type == expected_type
                && cmd.state_transition_from == from
                && cmd.state_transition_to == to
                && cmd.direction == "INBOUND"
        })
        .times(1)
        .returning(|_, _| {
            Ok(NegotiationMessageView::assemble(message_row(200)))
        });
    messages
}

/// A process service that accepts one edit to `state`.
fn edits_to(state: &'static str) -> MockNegotiationProcessServiceTrait {
    let mut processes = MockNegotiationProcessServiceTrait::new();
    processes
        .expect_edit()
        .withf(move |_, _, edit| edit.state.as_deref() == Some(state))
        .times(1)
        .returning(|_, _, _| {
            Ok(NegotiationProcessView::assemble(
                process_row(1, "Provider", "X"),
                HashMap::new(),
            ))
        });
    processes
}

/// A stored process hydrates with its pids, role and state parsed.
#[tokio::test]
async fn a_stored_process_hydrates_parsed() {
    let ctx = stored("Provider", "OFFERED").build();
    let process = NegotiationDspPersistence::new(ctx)
        .hydrate_process(CONSUMER_PID)
        .await
        .unwrap()
        .expect("found");
    assert_eq!(process.role, NegotiationRole::Provider);
    assert_eq!(process.state, NegotiationState::OFFERED);
    assert_eq!(process.own_pid(), Some(PROVIDER_PID));
    assert_eq!(process.peer_pid(), Some(CONSUMER_PID));
}

/// An unknown pid hydrates to nothing.
#[tokio::test]
async fn an_unknown_pid_is_none() {
    let mut processes = MockNegotiationProcessRepoTrait::new();
    processes
        .expect_get_negotiation_process_by_key_value()
        .returning(|_, _| Ok(None));
    let ctx = TestContext::new().process_repo(processes).build();
    let found = NegotiationDspPersistence::new(ctx)
        .hydrate_process("urn:uuid:unknown")
        .await
        .unwrap();
    assert!(found.is_none());
}

/// A role outside DSP is a broken record, not a process.
#[tokio::test]
async fn a_broken_role_fails_hydration() {
    let ctx = stored("Relay", "OFFERED").build();
    let result = NegotiationDspPersistence::new(ctx)
        .hydrate_process(CONSUMER_PID)
        .await;
    assert!(result.is_err());
}

/// Opening a process stores it, its first message (from INITIAL) and the offer it brought.
#[tokio::test]
async fn opening_stores_process_message_and_offer() {
    let mut processes = MockNegotiationProcessServiceTrait::new();
    processes.expect_create().times(1).returning(|_, cmd| {
        Ok(NegotiationProcessView::assemble(
            process_row(1, &cmd.role, &cmd.state),
            cmd.identifiers.clone().unwrap_or_default(),
        ))
    });
    let mut offers = MockOfferServiceTrait::new();
    offers
        .expect_create()
        .withf(|_, cmd| {
            cmd.offer_id == "urn:offer:1"
                && cmd.negotiation_agent_message_id.to_string() == urn(200)
        })
        .times(1)
        .returning(|_, _| Ok(OfferView::assemble(offer_row(1))));
    // The stored process is read back by its pid after the write.
    let ctx = stored("Provider", "REQUESTED")
        .processes(processes)
        .messages(messages("ContractRequestMessage", INITIAL_STATE, "REQUESTED"))
        .offers(offers)
        .build();
    let cmd = negotiation_agent::entities::negotiation_process::NewNegotiationProcessDto {
        id: None,
        visibility: None,
        owner: None,
        state: "REQUESTED".into(),
        state_attribute: None,
        associated_agent_peer: "did:web:peer".into(),
        protocol: "DSP".into(),
        callback_address: None,
        role: "Provider".into(),
        properties: None,
        identifiers: Some(HashMap::from([
            ("consumerPid".to_string(), CONSUMER_PID.to_string()),
            ("providerPid".to_string(), PROVIDER_PID.to_string()),
        ])),
    };
    let process = NegotiationDspPersistence::new(ctx)
        .open_process(
            &NegotiationDspPersistence::actor(),
            &cmd,
            record(NegotiationMessageType::ContractRequestMessage),
            NegotiationAttachment::Offer(OfferRecord {
                offer_id: "urn:offer:1".into(),
                content: json!({"@id": "urn:offer:1"}),
            }),
        )
        .await
        .unwrap();
    assert_eq!(process.own_pid(), Some(PROVIDER_PID));
}

/// An agreement moves the process to AGREED, keeps the message and stores the agreement.
#[tokio::test]
async fn an_agreement_is_stored_with_its_message() {
    let mut agreements = MockAgreementServiceTrait::new();
    agreements
        .expect_create()
        .withf(|_, cmd| {
            cmd.id.as_ref().map(ToString::to_string).as_deref() == Some("urn:agreement:1")
                && cmd.provider_participant_id == "did:web:provider"
        })
        .times(1)
        .returning(|_, _| Ok(AgreementView::assemble(agreement_row(1))));
    let ctx = stored("Consumer", "REQUESTED")
        .processes(edits_to("AGREED"))
        .messages(messages("ContractAgreementMessage", "REQUESTED", "AGREED"))
        .agreements(agreements)
        .build();
    let persistence = NegotiationDspPersistence::new(ctx);
    let process = persistence
        .hydrate_process(CONSUMER_PID)
        .await
        .unwrap()
        .unwrap();
    let change = StateChange {
        target: NegotiationState::AGREED,
        code: None,
        reason: None,
        message: record(NegotiationMessageType::ContractAgreementMessage),
    };
    let agreement = AgreementRecord {
        id: "urn:agreement:1".parse().unwrap(),
        target: "urn:dataset:1".parse().unwrap(),
        provider_id: "did:web:provider".into(),
        consumer_id: "did:web:consumer".into(),
        content: json!({}),
    };
    persistence
        .change_with(
            &process,
            change,
            NegotiationAttachment::Agreement(agreement),
        )
        .await
        .unwrap();
}

/// Finalizing activates the agreement stored for the process.
#[tokio::test]
async fn finalizing_activates_the_agreement() {
    let mut agreements = MockAgreementServiceTrait::new();
    agreements
        .expect_get_by_process()
        .times(1)
        .returning(|_, _| Ok(AgreementView::assemble(agreement_row(7))));
    agreements
        .expect_edit()
        .withf(|_, id, edit| id.to_string() == urn(7) && edit.state.as_deref() == Some("ACTIVE"))
        .times(1)
        .returning(|_, _, _| Ok(AgreementView::assemble(agreement_row(7))));
    let ctx = stored("Consumer", "VERIFIED")
        .processes(edits_to("FINALIZED"))
        .messages(messages(
            "ContractNegotiationEventMessage",
            "VERIFIED",
            "FINALIZED",
        ))
        .agreements(agreements)
        .build();
    let persistence = NegotiationDspPersistence::new(ctx);
    let process = persistence
        .hydrate_process(PROVIDER_PID)
        .await
        .unwrap()
        .unwrap();
    let change = StateChange {
        target: NegotiationState::FINALIZED,
        code: None,
        reason: None,
        message: record(NegotiationMessageType::EventFinalized),
    };
    persistence
        .change_with(&process, change, NegotiationAttachment::ActivateAgreement)
        .await
        .unwrap();
}

/// A termination keeps the peer's code and reasons as the process error details.
#[tokio::test]
async fn a_termination_keeps_code_and_reason() {
    let mut processes = MockNegotiationProcessServiceTrait::new();
    processes
        .expect_edit()
        .withf(|_, _, edit| {
            edit.state.as_deref() == Some("TERMINATED")
                && edit.error_details == Some(json!({"code": "c1", "reason": ["why"]}))
        })
        .times(1)
        .returning(|_, _, _| {
            Ok(NegotiationProcessView::assemble(
                process_row(1, "Provider", "X"),
                HashMap::new(),
            ))
        });
    let ctx = stored("Provider", "OFFERED")
        .processes(processes)
        .messages(messages(
            "ContractNegotiationTerminationMessage",
            "OFFERED",
            "TERMINATED",
        ))
        .build();
    let persistence = NegotiationDspPersistence::new(ctx);
    let process = persistence
        .hydrate_process(CONSUMER_PID)
        .await
        .unwrap()
        .unwrap();
    let change = StateChange {
        target: NegotiationState::TERMINATED,
        code: Some("c1".into()),
        reason: Some(vec!["why".into()]),
        message: record(NegotiationMessageType::ContractNegotiationTerminationMessage),
    };
    persistence
        .change_process_state(&process, change)
        .await
        .unwrap();
}
