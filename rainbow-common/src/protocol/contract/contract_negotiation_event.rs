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

use crate::protocol::contract::{ContractNegotiationMessages, ContractNegotiationState, CONTEXT};
use crate::utils::get_urn;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractNegotiationEventMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:eventType")]
    pub event_type: NegotiationEventType,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum NegotiationEventType {
    #[serde(rename = "dspace:ACCEPTED")]
    Accepted,
    #[serde(rename = "dspace:FINALIZED")]
    Finalized,
}

impl Default for ContractNegotiationEventMessage {
    fn default() -> Self {
        Self {
            context: CONTEXT.to_string(),
            _type: ContractNegotiationMessages::ContractNegotiationEventMessage.to_string(),
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
            NegotiationEventType::Finalized => ContractNegotiationState::Finalized
        }
    }
}