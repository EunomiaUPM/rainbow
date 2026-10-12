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

//! NegotiationAgreementResolver over a mocked negotiation port.

use chrono::Utc;
use common::oauth::{RolePath, Visibility};
use negotiation_agent::data::entities::agreement::Model;
use negotiation_agent::AgreementView;
use serde_json::{json, Value};
use std::sync::Arc;
use transfer_agent::protocols::dsp::facades::negotiation_facade::MockNegotiationFacadeTrait;
use transfer_agent::protocols::dsp::services::dsp_domain_loader::agreement_resolver::NegotiationAgreementResolver;
use transfer_agent::protocols::dsp::services::dsp_domain_loader::DspDomainLoaderTrait;
use urn::Urn;

const AGREEMENT: &str = "urn:agreement:tck-1";

fn resolver(content: Value) -> NegotiationAgreementResolver {
    let mut negotiation = MockNegotiationFacadeTrait::new();
    negotiation.expect_get_agreement().returning(move |id| {
        Ok(AgreementView::assemble(Model {
            id: id.to_string(),
            user_id: "provider".into(),
            user_role: RolePath::root(),
            visibility: Visibility::Private,
            negotiation_agent_process_id: "urn:negotiation-process:1".into(),
            negotiation_agent_message_id: "urn:negotiation-message:1".into(),
            consumer_participant_id: "did:web:consumer".into(),
            provider_participant_id: "did:web:provider".into(),
            agreement_content: content.clone(),
            target: "urn:dataset:1".into(),
            state: "ACTIVE".into(),
            created_at: Utc::now().into(),
            updated_at: None,
        }))
    });
    NegotiationAgreementResolver::new(Arc::new(negotiation))
}

/// Parties, target and id come from the stored agreement, even when its content is partial.
#[tokio::test]
async fn partial_content_still_yields_the_agreement() {
    let agreement = resolver(json!({"uid": AGREEMENT, "target": "urn:dataset:1"}))
        .resolve_agreement(&AGREEMENT.parse::<Urn>().unwrap())
        .await
        .unwrap();
    assert_eq!(agreement.id.to_string(), AGREEMENT);
    assert_eq!(agreement.target.to_string(), "urn:dataset:1");
    assert_eq!(agreement.assigner, "did:web:provider");
    assert_eq!(agreement.assignee, "did:web:consumer");
    assert!(agreement.permission.is_none());
}

/// Well-formed ODRL rules in the content are kept.
#[tokio::test]
async fn odrl_rules_in_the_content_are_kept() {
    let agreement = resolver(json!({"permission": [{"action": "use"}]}))
        .resolve_agreement(&AGREEMENT.parse::<Urn>().unwrap())
        .await
        .unwrap();
    assert_eq!(agreement.permission.map(|p| p.len()), Some(1));
}
