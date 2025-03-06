use axum::async_trait;
use rainbow_common::protocol::transfer::{
    TransferCompletionMessage, TransferProcessMessage, TransferRequestMessage, TransferStartMessage,
    TransferSuspensionMessage, TransferTerminationMessage,
};
use std::future::Future;
use urn::Urn;

pub mod ds_protocol;

#[mockall::automock]
#[async_trait]
pub trait DSProtocolTransferProviderTrait: Send + Sync {
    async fn get_transfer_requests_by_provider(&self, provider_pid: Urn) -> anyhow::Result<TransferProcessMessage>;
    async fn transfer_request(&self, input: TransferRequestMessage) -> anyhow::Result<TransferProcessMessage>;
    async fn transfer_start(
        &self,
        provider_pid: Urn,
        input: TransferStartMessage,
    ) -> anyhow::Result<TransferProcessMessage>;
    async fn transfer_suspension(
        &self,
        provider_pid: Urn,
        input: TransferSuspensionMessage,
    ) -> anyhow::Result<TransferProcessMessage>;
    async fn transfer_completion(
        &self,
        provider_pid: Urn,
        input: TransferCompletionMessage,
    ) -> anyhow::Result<TransferProcessMessage>;
    async fn transfer_termination(
        &self,
        provider_pid: Urn,
        input: TransferTerminationMessage,
    ) -> anyhow::Result<TransferProcessMessage>;
}
