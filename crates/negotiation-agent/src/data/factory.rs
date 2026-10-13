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

//! Repository factory port.

use std::sync::Arc;

use crate::data::repo::agreement::AgreementRepoTrait;
use crate::data::repo::negotiation_message::NegotiationMessageRepoTrait;
use crate::data::repo::negotiation_process::NegotiationProcessRepoTrait;
use crate::data::repo::negotiation_process_identifier::NegotiationIdentifierRepoTrait;
use crate::data::repo::offer::OfferRepoTrait;

/// Hands out the negotiation repositories.
pub trait DataFactory: Send + Sync {
    fn negotiation_process_repo(&self) -> Arc<dyn NegotiationProcessRepoTrait>;
    fn negotiation_message_repo(&self) -> Arc<dyn NegotiationMessageRepoTrait>;
    fn negotiation_identifier_repo(&self) -> Arc<dyn NegotiationIdentifierRepoTrait>;
    fn offer_repo(&self) -> Arc<dyn OfferRepoTrait>;
    fn agreement_repo(&self) -> Arc<dyn AgreementRepoTrait>;
}
