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

//! Agreements read through the negotiation port.

use std::str::FromStr;
use std::sync::Arc;

use common::dsp_common::odrl::OdrlAgreement;
use urn::Urn;
use ymir::errors::{Errors, Outcome};

use crate::protocols::dsp::facades::negotiation_facade::NegotiationFacadeTrait;
use crate::protocols::dsp::services::dsp_domain_loader::DspDomainLoaderTrait;

/// Reads the agreement from the negotiation agent and parses its ODRL content.
pub struct NegotiationAgreementResolver {
    negotiation: Arc<dyn NegotiationFacadeTrait>,
}

impl NegotiationAgreementResolver {
    pub fn new(negotiation: Arc<dyn NegotiationFacadeTrait>) -> Self {
        Self { negotiation }
    }
}

#[async_trait::async_trait]
impl DspDomainLoaderTrait for NegotiationAgreementResolver {
    /// The parties, target and id come from the stored agreement; its ODRL content only adds
    /// the rules, and is read leniently because peers and seeders store partial content.
    #[tracing::instrument(level = "info", skip_all, err, fields(agreement = %agreement_id))]
    async fn resolve_agreement(&self, agreement_id: &Urn) -> Outcome<OdrlAgreement> {
        let agreement = self.negotiation.get_agreement(agreement_id).await?.inner;
        let target = Urn::from_str(&agreement.target).map_err(|_| {
            Errors::parse(format!("agreement {agreement_id} has no URN target"), None)
        })?;
        let content = &agreement.agreement_content;
        Ok(OdrlAgreement {
            id: agreement_id.clone(),
            target,
            assigner: agreement.provider_participant_id,
            assignee: agreement.consumer_participant_id,
            permission: Self::rules(content, "permission"),
            obligation: Self::rules(content, "obligation"),
            prohibition: Self::rules(content, "prohibition"),
            ..OdrlAgreement::default()
        })
    }
}

impl NegotiationAgreementResolver {
    /// A rule list from the content, when it is there and well formed.
    fn rules<T: serde::de::DeserializeOwned>(content: &serde_json::Value, key: &str) -> Option<T> {
        content
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}
