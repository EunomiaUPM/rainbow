#![allow(unused)]
use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcTransferCompletionMessageDto, RpcTransferRequestMessageDto, RpcTransferStartMessageDto,
    RpcTransferSuspensionMessageDto, RpcTransferTerminationMessageDto,
};

#[async_trait::async_trait]
pub trait ValidationRpcSteps: Send + Sync + 'static {
    async fn transfer_request_rpc(&self, input: &RpcTransferRequestMessageDto) -> anyhow::Result<()>;
    async fn transfer_start_rpc(&self, input: &RpcTransferStartMessageDto) -> anyhow::Result<()>;
    async fn transfer_completion_rpc(&self, input: &RpcTransferCompletionMessageDto) -> anyhow::Result<()>;
    async fn transfer_suspension_rpc(&self, input: &RpcTransferSuspensionMessageDto) -> anyhow::Result<()>;
    async fn transfer_termination_rpc(&self, input: &RpcTransferTerminationMessageDto) -> anyhow::Result<()>;
}
