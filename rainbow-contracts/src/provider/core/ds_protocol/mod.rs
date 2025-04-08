use axum::async_trait;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::ContractNegotiationEventMessage;
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use urn::Urn;

// pub mod ds_protocol_xxx;
pub mod ds_protocol_errors;
pub mod ds_protocol_types;
pub mod ds_protocol;

#[mockall::automock]
#[async_trait]
pub trait DSProtocolContractNegotiationProviderTrait: Send + Sync {
    async fn get_negotiation(
        &self, provider_pid: Urn) -> anyhow::Result<ContractAckMessage>;
    async fn post_request(
        &self, input: ContractRequestMessage) -> anyhow::Result<ContractAckMessage>;
    async fn post_provider_request(
        &self,
        provider_pid: Urn,
        input: ContractRequestMessage,
    ) -> anyhow::Result<ContractAckMessage>;
    async fn post_provider_events(
        &self,
        provider_pid: Urn,
        input: ContractNegotiationEventMessage,
    ) -> anyhow::Result<ContractAckMessage>;
    async fn post_provider_agreement_verification(
        &self,
        provider_id: Urn,
        input: ContractAgreementVerificationMessage,
    ) -> anyhow::Result<ContractAckMessage>;

    async fn post_provider_termination(
        &self,
        provider_id: Urn,
        input: ContractTerminationMessage,
    ) -> anyhow::Result<ContractAckMessage>;
}