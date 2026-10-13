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

//! The negotiation agent's published contract: the agreements a transfer runs under.

pub mod remote;
pub mod views;

use async_trait::async_trait;
use urn::Urn;
use ymir::errors::Outcome;

use crate::dsp_common::odrl::OdrlAgreement;

pub use views::StoredAgreement;

/// Agreements by id, whoever owns them: the agreement is what fixes a transfer.
#[mockall::automock]
#[async_trait]
pub trait NegotiationFacadeTrait: Send + Sync {
    /// The agreement as ODRL, with the parties and target the negotiation recorded.
    async fn get_agreement(&self, agreement_id: &Urn) -> Outcome<OdrlAgreement>;
}
