use rainbow_common::protocol::contract::contract_odrl::ContractRequestMessageOfferTypes;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
pub struct RainbowBusinessNegotiationRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: Urn,
    pub offer: ContractRequestMessageOfferTypes,
}

