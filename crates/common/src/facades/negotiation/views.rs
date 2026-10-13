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

//! An agreement as the negotiation agent stores it, and its ODRL form.

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use urn::Urn;
use ymir::errors::{Errors, Outcome};

use crate::dsp_common::odrl::OdrlAgreement;

/// What other agents read of a stored agreement, in the API's JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredAgreement {
    pub id: String,
    pub target: String,
    pub provider_participant_id: String,
    pub consumer_participant_id: String,
    pub agreement_content: Value,
}

impl StoredAgreement {
    /// The agreement as ODRL: the provider assigns, the consumer is assigned; the target must
    /// be a URN.
    pub fn into_odrl(self, agreement_id: &Urn) -> Outcome<OdrlAgreement> {
        let target = Urn::from_str(&self.target).map_err(|_| {
            Errors::parse(format!("agreement {agreement_id} has no URN target"), None)
        })?;
        Ok(OdrlAgreement::from_stored(
            agreement_id.clone(),
            target,
            self.provider_participant_id,
            self.consumer_participant_id,
            &self.agreement_content,
        ))
    }
}
