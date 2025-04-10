// pub mod idsa_api;
// pub mod idsa_api_errors;

use axum::async_trait;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement::ContractAgreementMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::ContractNegotiationEventMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_offer::ContractOfferMessage;
use urn::Urn;

pub mod ds_protocol;
pub mod ds_protocol_errors;

#[mockall::automock]
#[async_trait]
pub trait DSProtocolContractNegotiationConsumerTrait: Send + Sync {
    async fn post_offers(&self, input: ContractOfferMessage) -> anyhow::Result<ContractAckMessage>;

    async fn post_consumer_offers(
        &self,
        consumer_pid: Urn,
        input: ContractOfferMessage,
    ) -> anyhow::Result<ContractAckMessage>;

    async fn post_agreement(
        &self,
        consumer_pid: Urn,
        input: ContractAgreementMessage,
    ) -> anyhow::Result<ContractAckMessage>;

    async fn post_events(
        &self,
        consumer_pid: Urn,
        input: ContractNegotiationEventMessage,
    ) -> anyhow::Result<ContractAckMessage>;

    async fn post_termination(
        &self,
        consumer_pid: Urn,
        input: ContractTerminationMessage,
    ) -> anyhow::Result<ContractAckMessage>;
}
