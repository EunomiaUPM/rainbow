/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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
use crate::protocol::contract::contract_odrl::OdrlAgreement;
use crate::protocol::contract::contract_protocol_trait::DSProtocolContractNegotiationMessageTrait;
use crate::protocol::contract::ContractNegotiationMessages;
use crate::utils::get_urn;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractAgreementMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: ContractNegotiationMessages,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "agreement")]
    pub odrl_agreement: OdrlAgreement,
}

impl Default for ContractAgreementMessage {
    fn default() -> Self {
        Self {
            context: ContextField::default(),
            _type: ContractNegotiationMessages::ContractAgreementMessage,
            provider_pid: get_urn(None),
            consumer_pid: get_urn(None),
            callback_address: "".to_string(),
            odrl_agreement: OdrlAgreement::default(),
        }
    }
}

impl DSProtocolContractNegotiationMessageTrait<'_> for ContractAgreementMessage {
    fn get_message_type(&self) -> anyhow::Result<ContractNegotiationMessages> {
        Ok(self._type)
    }

    fn get_consumer_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Option::from(&self.consumer_pid))
    }

    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.provider_pid))
    }

    fn get_odrl_agreement(&self) -> anyhow::Result<Option<&OdrlAgreement>> {
        Ok(Some(&self.odrl_agreement))
    }
}
