use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupAgreementRequest, SetupAgreementResponse, SetupFinalizationRequest, SetupFinalizationResponse,
    SetupOfferRequest, SetupOfferResponse, SetupTerminationRequest, SetupTerminationResponse,
};
use axum::async_trait;

pub mod ds_protocol_rpc;
pub mod ds_protocol_rpc_errors;
pub mod ds_protocol_rpc_types;

#[mockall::automock]
#[async_trait]
pub trait DSRPCContractNegotiationProviderTrait: Send + Sync {
    async fn setup_offer(&self, input: SetupOfferRequest) -> anyhow::Result<SetupOfferResponse>;
    async fn setup_agreement(&self, input: SetupAgreementRequest) -> anyhow::Result<SetupAgreementResponse>;
    async fn setup_finalization(&self, input: SetupFinalizationRequest) -> anyhow::Result<SetupFinalizationResponse>;
    async fn setup_termination(&self, input: SetupTerminationRequest) -> anyhow::Result<SetupTerminationResponse>;
}
