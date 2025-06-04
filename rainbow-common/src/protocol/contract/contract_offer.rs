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
use crate::protocol::contract::contract_odrl::{ContractRequestMessageOfferTypes, OdrlMessageOffer, OdrlTypes};
use crate::protocol::contract::contract_protocol_trait::DSProtocolContractNegotiationMessageTrait;
use crate::protocol::contract::ContractNegotiationMessages;
use crate::utils::get_urn;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractOfferMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: ContractNegotiationMessages,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "offer")]
    pub odrl_offer: ContractRequestMessageOfferTypes,
}

impl Default for ContractOfferMessage {
    fn default() -> Self {
        ContractOfferMessage {
            context: ContextField::default(),
            _type: ContractNegotiationMessages::ContractOfferMessage,
            provider_pid: get_urn(None),
            consumer_pid: None,
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

impl DSProtocolContractNegotiationMessageTrait<'_> for ContractOfferMessage {
    fn get_message_type(&self) -> anyhow::Result<ContractNegotiationMessages> {
        Ok(self._type.clone())
    }

    fn get_consumer_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(self.consumer_pid.as_ref())
    }

    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.provider_pid))
    }

    fn get_odrl_offer(&self) -> anyhow::Result<Option<&ContractRequestMessageOfferTypes>> {
        Ok(Some(&self.odrl_offer))
    }
}
