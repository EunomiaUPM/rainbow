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

use std::str::FromStr;

use common::dsp_common::odrl::OdrlAgreement;
use negotiation_agent::AgreementView;
use urn::Urn;
use ymir::errors::{Errors, Outcome};

pub mod local;
pub mod remote;

/// The negotiation agent, as transfer needs it: the agreements a transfer runs under.
#[mockall::automock]
#[async_trait::async_trait]
pub trait NegotiationFacadeTrait: Send + Sync {
    /// The stored agreement by id, whoever owns it: the agreement is what fixes the transfer.
    async fn get_agreement_view(&self, agreement_id: &Urn) -> Outcome<AgreementView>;

    /// The agreement as ODRL, with the parties and target negotiation recorded.
    async fn get_agreement(&self, agreement_id: &Urn) -> Outcome<OdrlAgreement> {
        let stored = self.get_agreement_view(agreement_id).await?.inner;
        let target = Urn::from_str(&stored.target).map_err(|_| {
            Errors::parse(format!("agreement {agreement_id} has no URN target"), None)
        })?;
        Ok(OdrlAgreement::from_stored(
            agreement_id.clone(),
            target,
            stored.provider_participant_id,
            stored.consumer_participant_id,
            &stored.agreement_content,
        ))
    }
}
