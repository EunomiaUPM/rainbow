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

use rainbow_common::protocol::transfer::transfer_data_address::DataAddress;
use rainbow_common::protocol::transfer::transfer_error::TransferError;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_protocol_trait::DSProtocolTransferMessageTrait;
use rainbow_common::protocol::transfer::TransferMessageTypes;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferProviderStartRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerCallbackAddress")]
    pub consumer_callback: Option<String>,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}

impl DSProtocolTransferMessageTrait<'_> for DSRPCTransferProviderStartRequest {
    fn get_message_type(&self) -> anyhow::Result<TransferMessageTypes> {
        Ok(TransferMessageTypes::TransferStartMessage)
    }

    fn get_consumer_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.consumer_pid))
    }

    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.provider_pid))
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferProviderStartResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
    pub message: TransferProcessMessage,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferProviderSuspensionRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerCallbackAddress")]
    pub consumer_callback: Option<String>,
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

impl DSProtocolTransferMessageTrait<'_> for DSRPCTransferProviderSuspensionRequest {
    fn get_message_type(&self) -> anyhow::Result<TransferMessageTypes> {
        Ok(TransferMessageTypes::TransferSuspensionMessage)
    }

    fn get_consumer_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.consumer_pid))
    }

    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.provider_pid))
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferProviderSuspensionResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    pub message: TransferProcessMessage,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferProviderCompletionRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerCallbackAddress")]
    pub consumer_callback: Option<String>,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
}

impl DSProtocolTransferMessageTrait<'_> for DSRPCTransferProviderCompletionRequest {
    fn get_message_type(&self) -> anyhow::Result<TransferMessageTypes> {
        Ok(TransferMessageTypes::TransferCompletionMessage)
    }

    fn get_consumer_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.consumer_pid))
    }

    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.provider_pid))
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferProviderCompletionResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    pub message: TransferProcessMessage,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferProviderTerminationRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerCallbackAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_callback: Option<String>,
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

impl DSProtocolTransferMessageTrait<'_> for DSRPCTransferProviderTerminationRequest {
    fn get_message_type(&self) -> anyhow::Result<TransferMessageTypes> {
        Ok(TransferMessageTypes::TransferTerminationMessage)
    }
    fn get_consumer_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.consumer_pid))
    }
    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(Some(&self.provider_pid))
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferProviderTerminationResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    pub message: TransferProcessMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferProviderErrorResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Option<Urn>,
    pub error: TransferError,
}
