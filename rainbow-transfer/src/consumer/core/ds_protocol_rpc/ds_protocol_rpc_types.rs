use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::transfer::transfer_data_address::DataAddress;
use rainbow_common::protocol::transfer::transfer_error::TransferError;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerRequestRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    #[serde(rename = "format")]
    pub format: DctFormats,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerRequestResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    #[serde(rename = "callbackAddress")]
    pub callback_address: String,
    #[serde(rename = "format")]
    pub format: DctFormats,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
    pub message: TransferProcessMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerStartRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerStartResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
    pub message: TransferProcessMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerSuspensionRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
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

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerSuspensionResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    pub message: TransferProcessMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerCompletionRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerCompletionResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    pub message: TransferProcessMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerTerminationRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
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

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerTerminationResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    pub message: TransferProcessMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DSRPCTransferConsumerErrorResponse {
    #[serde(rename = "providerPid")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Option<Urn>,
    pub error: TransferError,
}