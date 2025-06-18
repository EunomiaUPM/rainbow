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
use crate::protocol::contract::contract_protocol_trait::DSProtocolContractNegotiationMessageTrait;
use crate::protocol::contract::{ContractNegotiationMessages, ContractNegotiationState};
use crate::utils::get_urn;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractNegotiationEventMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: ContractNegotiationMessages,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "eventType")]
    pub event_type: NegotiationEventType,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum NegotiationEventType {
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "FINALIZED")]
    Finalized,
}

impl Default for ContractNegotiationEventMessage {
    fn default() -> Self {
        Self {
            context: ContextField::default(),
            _type: ContractNegotiationMessages::ContractNegotiationEventMessage,
            provider_pid: get_urn(None),
            consumer_pid: get_urn(None),
            event_type: NegotiationEventType::Accepted,
        }
    }
}

impl Into<ContractNegotiationState> for NegotiationEventType {
    fn into(self) -> ContractNegotiationState {
        match self {
            NegotiationEventType::Accepted => ContractNegotiationState::Accepted,
            NegotiationEventType::Finalized => ContractNegotiationState::Finalized,
        }
    }
}

impl DSProtocolContractNegotiationMessageTrait<'_> for ContractNegotiationEventMessage {
    fn get_message_type(&self) -> anyhow::Result<ContractNegotiationMessages> {
        Ok(self._type)
    }

    fn get_consumer_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Option::from(&self.consumer_pid))
    }
    
    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.provider_pid))
    }

    fn get_negotiation_event_type(&self) -> anyhow::Result<Option<NegotiationEventType>> {
        Ok(Some(self.event_type))
    }
}
