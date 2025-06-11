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
use crate::protocol::transfer::transfer_protocol_trait::DSProtocolTransferMessageTrait;
use crate::protocol::transfer::TransferMessageTypes;
use crate::utils::get_urn;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TransferTerminationMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: TransferMessageTypes,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<String>>,
}

impl Default for TransferTerminationMessage {
    fn default() -> Self {
        Self {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferTerminationMessage,
            provider_pid: get_urn(None),
            consumer_pid: get_urn(None),
            code: None,
            reason: None,
        }
    }
}

impl DSProtocolTransferMessageTrait<'_> for TransferTerminationMessage {
    fn get_message_type(&self) -> anyhow::Result<TransferMessageTypes> {
        Ok(self._type.clone())
    }

    fn get_consumer_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.consumer_pid))
    }

    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.provider_pid))
    }
}
