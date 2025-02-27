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
use super::{ContextField, ContractNegotiationMessages, CONTEXT};
use crate::utils::get_urn;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractTerminationMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "dspace:reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<String>>,
}

impl Default for ContractTerminationMessage {
    fn default() -> Self {
        Self {
            context: ContextField::Single(CONTEXT.to_string()),
            _type: ContractNegotiationMessages::ContractNegotiationTerminationMessage.to_string(),
            provider_pid: get_urn(None),
            consumer_pid: get_urn(None),
            code: None,
            reason: None,
        }
    }
}