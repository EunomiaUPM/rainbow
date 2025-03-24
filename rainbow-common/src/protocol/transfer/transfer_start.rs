use crate::protocol::context_field::ContextField;
use crate::protocol::transfer::transfer_data_address::DataAddress;
use crate::protocol::transfer::TransferMessageTypes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TransferStartMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "providerPid")]
    pub provider_pid: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}

impl Default for TransferStartMessage {
    fn default() -> Self {
        Self {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferStartMessage.to_string(),
            provider_pid: "".to_string(),
            consumer_pid: "".to_string(),
            data_address: None,
        }
    }
}