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

use crate::protocol::context_field::ContextField;
use crate::protocol::contract::contract_odrl::OdrlMessageOffer;
use crate::protocol::contract::ContractNegotiationMessages;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractOfferMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:callbackAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_address: Option<String>,
    #[serde(rename = "dspace:offer")]
    pub odrl_offer: OdrlMessageOffer,
}

impl Default for ContractOfferMessage {
    fn default() -> Self {
        ContractOfferMessage {
            context: ContextField::default(),
            _type: ContractNegotiationMessages::ContractOfferMessage.to_string(),
            provider_pid: "".to_string(),
            callback_address: None,
            odrl_offer: OdrlMessageOffer::default(),
        }
    }
}