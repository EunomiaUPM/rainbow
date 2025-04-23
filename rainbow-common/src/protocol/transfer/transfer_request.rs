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

use crate::dcat_formats::{DctFormats, FormatAction, FormatProtocol};
use crate::protocol::context_field::ContextField;
use crate::protocol::transfer::transfer_data_address::DataAddress;
use crate::protocol::transfer::TransferMessageTypes;
use crate::protocol::ProtocolValidate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TransferRequestMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    #[serde(rename = "format")]
    pub format: DctFormats,
    #[serde(rename = "callbackAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_address: Option<String>,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}

impl Default for TransferRequestMessage {
    fn default() -> Self {
        Self {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferRequestMessage.to_string(),
            consumer_pid: "".to_string(),
            agreement_id: "".to_string(),
            format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
            callback_address: Some("".to_string()),
            data_address: None,
        }
    }
}

impl ProtocolValidate for TransferRequestMessage {
    fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
