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
