use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_odrl::{OdrlAgreement, OdrlOffer};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupOfferRequest {
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "dspace:offer")]
    pub odrl_offer: OdrlOffer,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupOfferResponse {
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "dspace:offer")]
    pub odrl_offer: OdrlOffer,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupAgreementRequest {
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "dspace:agreement")]
    pub odrl_agreement: OdrlAgreement,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupAgreementResponse {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "dspace:agreement")]
    pub odrl_agreement: OdrlAgreement,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupFinalizationRequest {
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetupFinalizationResponse {
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupTerminationRequest {
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
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