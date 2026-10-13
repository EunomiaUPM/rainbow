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

//! The published contracts' records: a stored agreement as ODRL.

use serde_json::{json, Value};
use urn::Urn;

use crate::facades::negotiation::StoredAgreement;

const AGREEMENT: &str = "urn:agreement:tck-1";

fn stored(content: Value, target: &str) -> StoredAgreement {
    StoredAgreement {
        id: AGREEMENT.into(),
        target: target.into(),
        provider_participant_id: "did:web:provider".into(),
        consumer_participant_id: "did:web:consumer".into(),
        agreement_content: content,
    }
}

fn id() -> Urn {
    AGREEMENT.parse().unwrap()
}

/// Parties, target and id come from the stored agreement, even when its content is partial.
#[test]
fn partial_content_still_yields_the_agreement() {
    let agreement = stored(json!({"uid": AGREEMENT}), "urn:dataset:1")
        .into_odrl(&id())
        .unwrap();
    assert_eq!(agreement.id.to_string(), AGREEMENT);
    assert_eq!(agreement.target.to_string(), "urn:dataset:1");
    assert_eq!(agreement.assigner, "did:web:provider");
    assert_eq!(agreement.assignee, "did:web:consumer");
    assert!(agreement.permission.is_none());
}

/// Well-formed ODRL rules in the content are kept.
#[test]
fn odrl_rules_in_the_content_are_kept() {
    let agreement = stored(json!({"permission": [{"action": "use"}]}), "urn:dataset:1")
        .into_odrl(&id())
        .unwrap();
    assert_eq!(agreement.permission.map(|p| p.len()), Some(1));
}

/// A stored target that is not a URN cannot fix a transfer.
#[test]
fn a_target_that_is_not_a_urn_is_rejected() {
    assert!(stored(json!({}), "dataset one").into_odrl(&id()).is_err());
}

/// The agent's API JSON reads into the record, ignoring the fields consumers do not need.
#[test]
fn the_api_view_reads_into_the_record() {
    let view = json!({
        "id": AGREEMENT, "userId": "u", "target": "urn:dataset:1",
        "providerParticipantId": "did:web:provider", "consumerParticipantId": "did:web:consumer",
        "agreementContent": {}, "state": "ACTIVE"
    });
    let record: StoredAgreement = serde_json::from_value(view).unwrap();
    assert_eq!(record, stored(json!({}), "urn:dataset:1"));
}
