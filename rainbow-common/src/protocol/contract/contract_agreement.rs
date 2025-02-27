/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::protocol::contract::contract_odrl::OdrlAgreement;
use crate::protocol::contract::{ContractNegotiationMessages, CONTEXT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractAgreementMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "dspace:agreement")]
    pub odrl_agreement: OdrlAgreement,
}

impl Default for ContractAgreementMessage {
    fn default() -> Self {
        Self {
            context: CONTEXT.to_string(),
            _type: ContractNegotiationMessages::ContractAgreementMessage.to_string(),
            provider_pid: "".to_string(),
            consumer_pid: "".to_string(),
            callback_address: "".to_string(),
            odrl_agreement: OdrlAgreement::default(),
        }
    }
}