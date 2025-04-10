use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_odrl::OfferTypes;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupRequestRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "dspace:offer")]
    pub odrl_offer: OfferTypes,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupRequestResponse {
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "dspace:offer")]
    pub odrl_offer: OfferTypes,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupAcceptanceRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupAcceptanceResponse {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupVerificationRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupVerificationResponse {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupTerminationRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetupTerminationResponse {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}