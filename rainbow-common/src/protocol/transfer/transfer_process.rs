use crate::protocol::context_field::ContextField;
use crate::protocol::transfer::{TransferMessageTypes, TransferState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TransferProcessMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "providerPid")]
    pub provider_pid: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "state")]
    pub state: TransferState,
}

impl Default for TransferProcessMessage {
    fn default() -> Self {
        TransferProcessMessage {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferProcessMessage.to_string(),
            provider_pid: "".to_string(),
            consumer_pid: "".to_string(),
            state: TransferState::REQUESTED,
        }
    }
}