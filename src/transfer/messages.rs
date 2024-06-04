use serde::{Deserialize, Serialize, Serializer};


pub enum Message {
    TransferRequestMessage(TransferRequestMessage),
    TransferStartMessage(TransferStartMessage),
    TransferCompletionMessage(TransferCompletionMessage),
    TransferSuspensionMessage(TransferSuspensionMessage),
    TransferTerminationMessage(TransferTerminationMessage),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequestMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    pub consumer_pid: String,
    pub format: String,
    pub callback_address: String,
    pub data_address: Option<DataAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferStartMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    pub provider_pid: String,
    pub consumer_pid: String,
    pub data_address: Option<DataAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSuspensionMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    pub provider_pid: String,
    pub consumer_pid: String,
    pub code: String,
    pub reason: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCompletionMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    pub provider_pid: String,
    pub consumer_pid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferTerminationMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    pub provider_pid: String,
    pub consumer_pid: String,
    pub code: String,
    pub reason: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAddress {
    #[serde(rename = "@type")]
    pub _type: String,
    pub endpoint_type: String,
    pub endpoint: String,
    pub endpoint_properties: Vec<EndpointProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointProperty {
    #[serde(rename = "@type")]
    pub _type: String,
    pub name: String,
    pub value: String,
}

