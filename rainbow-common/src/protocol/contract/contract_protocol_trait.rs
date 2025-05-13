use crate::protocol::contract::contract_negotiation_event::NegotiationEventType;
use crate::protocol::contract::contract_odrl::{ContractRequestMessageOfferTypes, OdrlAgreement};
use crate::protocol::contract::ContractNegotiationMessages;
use serde::{Deserialize, Serialize};
use urn::Urn;

pub trait DSProtocolContractNegotiationMessageTrait<'a>: Serialize + Deserialize<'a> + Clone {
    fn get_message_type(&self) -> anyhow::Result<ContractNegotiationMessages>;
    fn get_consumer_pid(&self) -> anyhow::Result<&Urn>;
    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(None)
    }
    fn get_negotiation_event_type(&self) -> anyhow::Result<Option<NegotiationEventType>> {
        Ok(None)
    }
    fn get_odrl_offer(&self) -> anyhow::Result<Option<&ContractRequestMessageOfferTypes>> {
        Ok(None)
    }
    fn get_odrl_agreement(&self) -> anyhow::Result<Option<&OdrlAgreement>> {
        Ok(None)
    }
}
