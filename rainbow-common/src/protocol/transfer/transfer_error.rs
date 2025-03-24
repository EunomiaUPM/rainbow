use crate::err::transfer_err::TransferErrorType;
use crate::protocol::context_field::ContextField;
use crate::protocol::transfer::TransferMessageTypes;
use axum::body::to_bytes;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferError {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<String>,
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<String>,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "reason")]
    pub reason: Vec<String>,
}

impl Default for TransferError {
    fn default() -> Self {
        Self {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferError.to_string(),
            provider_pid: None,
            consumer_pid: None,
            code: "".to_string(),
            reason: vec![],
        }
    }
}

impl TransferError {
    pub async fn from_async(value: TransferErrorType) -> Self {
        let response = value.into_response();
        let response_data = to_bytes(response.into_parts().1, 2048).await.unwrap();
        serde_json::from_slice::<TransferError>(&response_data).unwrap()
    }
}