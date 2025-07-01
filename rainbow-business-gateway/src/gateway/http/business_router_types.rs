use rainbow_common::protocol::contract::contract_odrl::ContractRequestMessageOfferTypes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RainbowBusinessNegotiationRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    pub offer: ContractRequestMessageOfferTypes,
}

