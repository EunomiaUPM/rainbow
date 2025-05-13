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

use super::ContractNegotiationMessages;
use crate::protocol::context_field::ContextField;
use crate::protocol::contract::contract_protocol_trait::DSProtocolContractNegotiationMessageTrait;
use crate::utils::get_urn;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractAgreementVerificationMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: ContractNegotiationMessages,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
}

impl Default for ContractAgreementVerificationMessage {
    fn default() -> Self {
        ContractAgreementVerificationMessage {
            context: ContextField::default(),
            _type: ContractNegotiationMessages::ContractAgreementVerificationMessage,
            provider_pid: get_urn(None),
            consumer_pid: get_urn(None),
        }
    }
}

impl DSProtocolContractNegotiationMessageTrait<'_> for ContractAgreementVerificationMessage {
    fn get_message_type(&self) -> anyhow::Result<ContractNegotiationMessages> {
        Ok(self._type.clone())
    }

    fn get_consumer_pid(&self) -> anyhow::Result<&Urn> {
        Ok(&self.consumer_pid)
    }

    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.provider_pid))
    }
}
