use crate::protocol::context_field::ContextField;
use crate::protocol::transfer::TransferMessageTypes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TransferCompletionMessage {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "providerPid")]
    pub provider_pid: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: String,
}

impl Default for TransferCompletionMessage {
    fn default() -> Self {
        Self {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferCompletionMessage.to_string(),
            provider_pid: "".to_string(),
            consumer_pid: "".to_string(),
        }
    }
}