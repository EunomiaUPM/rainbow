use std::fmt;
use std::fmt::Write;

use crate::transfer::protocol::formats::DctFormats;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub static TRANSFER_CONTEXT: &str = "https://w3id.org/dspace/2024/1/context.json";

#[derive(Debug, Serialize, Deserialize)]
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
    pub data_address: Option<DataAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub reason: Vec<Value>,
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
    pub reason: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAddress {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:endpointType")]
    pub endpoint_type: String,
    #[serde(rename = "dspace:endpoint")]
    pub endpoint: String,
    #[serde(rename = "dspace:endpointProperties")]
    pub endpoint_properties: Vec<EndpointProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointProperty {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:name")]
    pub name: String,
    #[serde(rename = "dspace:value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferProcess {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferKickOff {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "provider_url")]
    pub provider_url: String,
    #[serde(rename = "dct:format")]
    pub format: DctFormats,
    #[serde(rename = "dspace:dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferError {
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
pub enum TransferMessageTypes {
    TransferError,
    TransferRequestMessage,
    TransferStartMessage,
    TransferSuspensionMessage,
    TransferCompletionMessage,
    TransferTerminationMessage,
    TransferProcess,
    TransferKickOffMessage,
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
            TransferMessageTypes::TransferProcess => f.write_str("dspace:TransferProcess"),
            TransferMessageTypes::TransferKickOffMessage => {
                f.write_str("dspace:TransferKickOffMessage")
            }
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
