use rainbow_common::protocol::contract::contract_odrl::ContractRequestMessageOfferTypes;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
pub struct RainbowBusinessNegotiationRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    pub offer: ContractRequestMessageOfferTypes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RainbowBusinessTerminationRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RainbowBusinessAcceptanceRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
}

