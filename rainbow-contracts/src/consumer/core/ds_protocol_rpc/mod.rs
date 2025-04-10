use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupAcceptanceRequest, SetupAcceptanceResponse, SetupRequestRequest, SetupRequestResponse,
    SetupTerminationRequest, SetupTerminationResponse, SetupVerificationRequest, SetupVerificationResponse,
};
use axum::async_trait;

pub mod ds_protocol_rpc;
pub mod ds_protocol_rpc_errors;
pub mod ds_protocol_rpc_types;

#[mockall::automock]
#[async_trait]
pub trait DSRPCContractNegotiationConsumerTrait: Send + Sync {
    async fn setup_request(&self, input: SetupRequestRequest) -> anyhow::Result<SetupRequestResponse>;
    async fn setup_acceptance(&self, input: SetupAcceptanceRequest) -> anyhow::Result<SetupAcceptanceResponse>;
    async fn setup_verification(&self, input: SetupVerificationRequest) -> anyhow::Result<SetupVerificationResponse>;
    async fn setup_termination(&self, input: SetupTerminationRequest) -> anyhow::Result<SetupTerminationResponse>;
}
