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

//! Agreements read straight from the agreement service, as the system user (any agreement).

use std::sync::Arc;

use common::dsp_common::odrl::OdrlAgreement;
use common::facades::negotiation::{NegotiationFacadeTrait, StoredAgreement};
use ymir::types::oauth::UserInfo;
use urn::Urn;
use ymir::errors::Outcome;

use crate::services::agreement::AgreementServiceTrait;

/// The negotiation agent's agreements for the agents sharing its process.
pub struct NegotiationLocalFacade {
    agreements: Arc<dyn AgreementServiceTrait>,
}

impl NegotiationLocalFacade {
    pub fn new(agreements: Arc<dyn AgreementServiceTrait>) -> Self {
        Self { agreements }
    }
}

#[async_trait::async_trait]
impl NegotiationFacadeTrait for NegotiationLocalFacade {
    #[tracing::instrument(level = "info", skip_all, err, fields(peer.service = "negotiation"))]
    async fn get_agreement(&self, agreement_id: &Urn) -> Outcome<OdrlAgreement> {
        let stored = self
            .agreements
            .get_one(&UserInfo::system(), agreement_id)
            .await?
            .inner;
        StoredAgreement {
            id: stored.id,
            target: stored.target,
            provider_participant_id: stored.provider_participant_id,
            consumer_participant_id: stored.consumer_participant_id,
            agreement_content: stored.agreement_content,
        }
        .into_odrl(agreement_id)
    }
}
