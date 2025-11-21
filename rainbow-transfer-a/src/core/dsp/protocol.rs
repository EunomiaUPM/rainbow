use serde::{Deserialize, Serialize};
use std::fmt::Display;
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferRequestMessageDto {
    pub agreement_id: Urn,
    pub format: String,
    pub data_address: Option<DataAddressDto>,
    pub callback_address: String,
    pub consumer_pid: Urn,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferStartMessageDto {
    pub provider_pid: Urn,
    pub consumer_pid: Urn,
    pub data_address: Option<DataAddressDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferSuspensionMessageDto {
    pub provider_pid: Urn,
    pub consumer_pid: Urn,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferCompletionMessageDto {
    pub provider_pid: Urn,
    pub consumer_pid: Urn,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferTerminationMessageDto {
    pub provider_pid: Urn,
    pub consumer_pid: Urn,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataAddressDto {
    pub endpoint_type: String,
    pub endpoint: Option<String>,
    pub endpoint_properties: Option<Vec<EndpointPropertyDto>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPropertyDto {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferProcessAckDto {
    pub consumer_pid: Urn,
    pub provider_pid: Urn,
    pub state: TransferProcessState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferErrorDto {
    pub consumer_pid: Urn,
    pub provider_pid: Urn,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransferProcessState {
    Requested,
    Started,
    Completed,
    Suspended,
    Terminated,
}

impl Display for TransferProcessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TransferProcessState::Requested => "REQUESTED".to_string(),
            TransferProcessState::Started => "STARTED".to_string(),
            TransferProcessState::Completed => "COMPLETED".to_string(),
            TransferProcessState::Suspended => "SUSPENDED".to_string(),
            TransferProcessState::Terminated => "TERMINATED".to_string(),
        };
        write!(f, "{}", str)
    }
}
