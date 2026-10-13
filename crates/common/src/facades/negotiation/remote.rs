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

//! The negotiation contract over the negotiation agent's management API.

use async_trait::async_trait;
use urn::Urn;
use ymir::config::types::HostType;
use ymir::errors::Outcome;
use ymir::services::client::ClientExt;
use ymir::utils::http_client;

use crate::config::types::min_known_config::MinKnownConfig;
use crate::config::types::traits::MinKnownConfigTrait;
use crate::dsp_common::odrl::OdrlAgreement;
use crate::facades::negotiation::{NegotiationFacadeTrait, StoredAgreement};

/// Reads agreements from the negotiation agent's API.
pub struct NegotiationRemoteFacade {
    agreements_url: String,
}

impl NegotiationRemoteFacade {
    /// Path segment the negotiation agent mounts its API under.
    const SERVICE: &'static str = "negotiation-agent";

    pub fn new(contracts: &MinKnownConfig) -> Self {
        Self {
            agreements_url: format!(
                "{}{}/{}/agreements",
                contracts.get_host(HostType::Http),
                contracts.get_api_version(),
                Self::SERVICE
            ),
        }
    }
}

#[async_trait]
impl NegotiationFacadeTrait for NegotiationRemoteFacade {
    #[tracing::instrument(level = "info", skip_all, err, fields(peer.service = "negotiation"))]
    async fn get_agreement(&self, agreement_id: &Urn) -> Outcome<OdrlAgreement> {
        let url = format!("{}/{agreement_id}", self.agreements_url);
        let stored: StoredAgreement = http_client().get_json(&url, None).await?;
        stored.into_odrl(agreement_id)
    }
}
