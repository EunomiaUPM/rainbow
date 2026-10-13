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

//! The Contract Negotiation Protocol fields (DSP 8.2, 8.3), as extracted. The policy rules of
//! an offer or agreement are read from the compact message; here only what identifies them.

use serde::{Deserialize, Serialize};

/// Every member is optional because extraction is total: which ones a message needs is the
/// validator's call.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NegotiationProtocolFields {
    pub consumer_pid: Option<String>,
    pub provider_pid: Option<String>,
    pub callback_address: Option<String>,
    pub offer: Option<OfferFields>,
    pub agreement: Option<AgreementFields>,
    /// `ACCEPTED` or `FINALIZED`, without namespace.
    pub event_type: Option<String>,
    /// The state a `ContractNegotiation` reports, without namespace.
    pub state: Option<String>,
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<String>,
}

/// What identifies an offer: its `@id` and, when sent whole, its `target`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferFields {
    pub id: Option<String>,
    pub target: Option<String>,
}

/// What identifies an agreement and its parties.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementFields {
    pub id: Option<String>,
    pub target: Option<String>,
    pub assigner: Option<String>,
    pub assignee: Option<String>,
    pub timestamp: Option<String>,
}
