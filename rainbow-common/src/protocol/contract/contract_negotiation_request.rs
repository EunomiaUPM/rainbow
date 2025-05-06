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

use super::contract_odrl::{ContractRequestMessageOfferTypes, OdrlMessageOffer, OdrlTypes};
use crate::protocol::context_field::ContextField;
use crate::protocol::contract::ContractNegotiationMessages;
use crate::protocol::ProtocolValidate;
use crate::utils::get_urn;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractRequestMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: ContractNegotiationMessages,
    #[serde(rename = "providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "offer")]
    pub odrl_offer: ContractRequestMessageOfferTypes,
}

impl Default for ContractRequestMessage {
    fn default() -> Self {
        Self {
            context: ContextField::default(),
            _type: ContractNegotiationMessages::ContractRequestMessage,
            provider_pid: Default::default(),
            consumer_pid: get_urn(None),
            callback_address: Default::default(),
            odrl_offer: ContractRequestMessageOfferTypes::OfferMessage(OdrlMessageOffer {
                id: get_urn(None),
                profile: None,
                permission: None,
                obligation: None,
                _type: OdrlTypes::Offer,
                prohibition: None,
                target: get_urn(None),
            }),
        }
    }
}

impl ContractRequestMessage {
    pub fn validate(&self) -> anyhow::Result<&Self> {
        // Syntactic JSON validation through serde_json
        // Lacks of semantic validation
        // Correct @context
        self.context.validate()?;
        // Offer is validated in Offer struct implementation
        // match &self.odrl_offer {
        //     OfferTypes::Offer(o) => o.validate(),
        //     OfferTypes::MessageOffer(o) => o.validate(),
        //     _ => bail!("Invalid offer type. Only Offer and MessageOffer are allowed"),
        // }?;
        Ok(&self)
    }
}
