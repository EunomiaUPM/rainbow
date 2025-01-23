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

use crate::dcat_formats::DctFormats;
use crate::err::transfer_err::TransferErrorType;
use anyhow::bail;
use axum::body::to_bytes;
use axum::response::IntoResponse;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use std::fmt;

pub static TRANSFER_CONTEXT: &str = "https://w3id.org/dspace/2024/1/context.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferRequestMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:agreementId")]
    pub agreement_id: String,
    #[serde(rename = "dct:format")]
    pub format: DctFormats,
    #[serde(rename = "dspace:callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "dspace:dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferStartMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferSuspensionMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:code")]
    pub code: String,
    #[serde(rename = "dspace:reason")]
    pub reason: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCompletionMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferTerminationMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:code")]
    pub code: String,
    #[serde(rename = "dspace:reason")]
    pub reason: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct DataAddress {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:endpointType")] // TODO define this
    pub endpoint_type: String,
    #[serde(rename = "dspace:endpoint")]
    pub endpoint: String,
    #[serde(rename = "dspace:endpointProperties")]
    pub endpoint_properties: Vec<EndpointProperty>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct EndpointProperty {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:name")]
    pub name: String,
    #[serde(rename = "dspace:value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferProcessMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:state")]
    pub state: TransferState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferState {
    #[serde(rename = "dspace:REQUESTED")]
    REQUESTED,
    #[serde(rename = "dspace:STARTED")]
    STARTED,
    #[serde(rename = "dspace:TERMINATED")]
    TERMINATED,
    #[serde(rename = "dspace:COMPLETED")]
    COMPLETED,
    #[serde(rename = "dspace:SUSPENDED")]
    SUSPENDED,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "transfer_state")]
pub enum TransferStateForDb {
    #[sea_orm(string_value = "dspace:REQUESTED")]
    REQUESTED,
    #[sea_orm(string_value = "dspace:STARTED")]
    STARTED,
    #[sea_orm(string_value = "dspace:TERMINATED")]
    TERMINATED,
    #[sea_orm(string_value = "dspace:COMPLETED")]
    COMPLETED,
    #[sea_orm(string_value = "dspace:SUSPENDED")]
    SUSPENDED,
}

impl From<TransferStateForDb> for TransferState {
    fn from(state: TransferStateForDb) -> Self {
        match state {
            TransferStateForDb::REQUESTED => TransferState::REQUESTED,
            TransferStateForDb::STARTED => TransferState::STARTED,
            TransferStateForDb::TERMINATED => TransferState::TERMINATED,
            TransferStateForDb::COMPLETED => TransferState::COMPLETED,
            TransferStateForDb::SUSPENDED => TransferState::SUSPENDED,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferError {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")] // IMPROVEMENT should be optional
    pub provider_pid: Option<String>,
    #[serde(rename = "dspace:consumerPid")] // IMPROVEMENT should be optional
    pub consumer_pid: String,
    #[serde(rename = "dspace:code")]
    pub code: String,
    #[serde(rename = "dspace:reason")]
    pub reason: Vec<String>,
}

impl TransferError {
    pub async fn from_async(value: TransferErrorType) -> Self {
        let response = value.into_response();
        let response_data = to_bytes(response.into_parts().1, 2048).await.unwrap();
        serde_json::from_slice::<TransferError>(&response_data).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransferMessageTypes {
    TransferError,
    TransferRequestMessage,
    TransferStartMessage,
    TransferSuspensionMessage,
    TransferCompletionMessage,
    TransferTerminationMessage,
    TransferProcessMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "transfer_message_type")]
pub enum TransferMessageTypesForDb {
    #[sea_orm(string_value = "dspace:TransferRequestMessage")]
    TransferRequestMessage,
    #[sea_orm(string_value = "dspace:TransferStartMessage")]
    TransferStartMessage,
    #[sea_orm(string_value = "dspace:TransferSuspensionMessage")]
    TransferSuspensionMessage,
    #[sea_orm(string_value = "dspace:TransferCompletionMessage")]
    TransferCompletionMessage,
    #[sea_orm(string_value = "dspace:TransferTerminationMessage")]
    TransferTerminationMessage,
}

impl TryFrom<String> for TransferMessageTypesForDb {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "dspace:TransferRequestMessage" => {
                Ok(TransferMessageTypesForDb::TransferRequestMessage)
            }
            "dspace:TransferStartMessage" => Ok(TransferMessageTypesForDb::TransferStartMessage),
            "dspace:TransferSuspensionMessage" => {
                Ok(TransferMessageTypesForDb::TransferSuspensionMessage)
            }
            "dspace:TransferCompletionMessage" => {
                Ok(TransferMessageTypesForDb::TransferCompletionMessage)
            }
            "dspace:TransferTerminationMessage" => {
                Ok(TransferMessageTypesForDb::TransferTerminationMessage)
            }
            _ => bail!("Invalid transfer message type"),
        }
    }
}

impl From<TransferMessageTypesForDb> for TransferMessageTypes {
    fn from(state: TransferMessageTypesForDb) -> Self {
        match state {
            TransferMessageTypesForDb::TransferRequestMessage => {
                TransferMessageTypes::TransferRequestMessage
            }
            TransferMessageTypesForDb::TransferStartMessage => {
                TransferMessageTypes::TransferStartMessage
            }
            TransferMessageTypesForDb::TransferSuspensionMessage => {
                TransferMessageTypes::TransferSuspensionMessage
            }
            TransferMessageTypesForDb::TransferCompletionMessage => {
                TransferMessageTypes::TransferCompletionMessage
            }
            TransferMessageTypesForDb::TransferTerminationMessage => {
                TransferMessageTypes::TransferTerminationMessage
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "transfer_roles")]
pub enum TransferRoles {
    #[sea_orm(string_value = "provider")]
    Provider,
    #[sea_orm(string_value = "consumer")]
    Consumer,
}

impl fmt::Display for TransferMessageTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferMessageTypes::TransferError => f.write_str("dspace:TransferError"),
            TransferMessageTypes::TransferRequestMessage => {
                f.write_str("dspace:TransferRequestMessage")
            }
            TransferMessageTypes::TransferStartMessage => {
                f.write_str("dspace:TransferStartMessage")
            }
            TransferMessageTypes::TransferSuspensionMessage => {
                f.write_str("dspace:TransferSuspensionMessage")
            }
            TransferMessageTypes::TransferCompletionMessage => {
                f.write_str("dspace:TransferCompletionMessage")
            }
            TransferMessageTypes::TransferTerminationMessage => {
                f.write_str("dspace:TransferTerminationMessage")
            }
            TransferMessageTypes::TransferProcessMessage => f.write_str("dspace:TransferProcess"),
        }
    }
}

impl fmt::Display for TransferState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferState::REQUESTED => f.write_str("dspace:REQUESTED"),
            TransferState::STARTED => f.write_str("dspace:STARTED"),
            TransferState::TERMINATED => f.write_str("dspace:TERMINATED"),
            TransferState::COMPLETED => f.write_str("dspace:COMPLETED"),
            TransferState::SUSPENDED => f.write_str("dspace:SUSPENDED"),
        }
    }
}

impl TryFrom<String> for TransferState {
    type Error = anyhow::Error;

    fn try_from(value: String) -> anyhow::Result<Self, Self::Error> {
        match value.as_str() {
            "dspace:REQUESTED" => Ok(TransferState::REQUESTED),
            "dspace:STARTED" => Ok(TransferState::STARTED),
            "dspace:TERMINATED" => Ok(TransferState::TERMINATED),
            "dspace:COMPLETED" => Ok(TransferState::COMPLETED),
            "dspace:SUSPENDED" => Ok(TransferState::SUSPENDED),
            // _ => Err(Error::from(TransferErrorType::UnknownTransferState)),
            _ => bail!("Invalid TransferState value"),
        }
    }
}
