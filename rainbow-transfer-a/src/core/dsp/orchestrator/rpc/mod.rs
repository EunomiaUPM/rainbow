use crate::core::dsp::orchestrator::rpc::types::{
    RpcTransferCompletionMessageDto, RpcTransferErrorDto, RpcTransferMessageDto, RpcTransferRequestMessageDto,
    RpcTransferStartMessageDto, RpcTransferSuspensionMessageDto, RpcTransferTerminationMessageDto,
};

pub(crate) mod rpc;
pub(crate) mod types;

#[async_trait::async_trait]
pub trait RPCOrchestratorTrait: Send + Sync + 'static {
    async fn setup_transfer_request(
        &self,
        input: &RpcTransferRequestMessageDto,
    ) -> anyhow::Result<
        RpcTransferMessageDto<RpcTransferRequestMessageDto>,
        RpcTransferErrorDto<RpcTransferRequestMessageDto>,
    >;
    async fn setup_transfer_start(
        &self,
        input: &RpcTransferStartMessageDto,
    ) -> anyhow::Result<
        RpcTransferMessageDto<RpcTransferStartMessageDto>,
        RpcTransferErrorDto<RpcTransferStartMessageDto>,
    >;
    async fn setup_transfer_suspension(
        &self,
        input: &RpcTransferSuspensionMessageDto,
    ) -> anyhow::Result<
        RpcTransferMessageDto<RpcTransferSuspensionMessageDto>,
        RpcTransferErrorDto<RpcTransferSuspensionMessageDto>,
    >;
    async fn setup_transfer_completion(
        &self,
        input: &RpcTransferCompletionMessageDto,
    ) -> anyhow::Result<
        RpcTransferMessageDto<RpcTransferCompletionMessageDto>,
        RpcTransferErrorDto<RpcTransferCompletionMessageDto>,
    >;
    async fn setup_transfer_termination(
        &self,
        input: &RpcTransferTerminationMessageDto,
    ) -> anyhow::Result<
        RpcTransferMessageDto<RpcTransferTerminationMessageDto>,
        RpcTransferErrorDto<RpcTransferTerminationMessageDto>,
    >;
}
