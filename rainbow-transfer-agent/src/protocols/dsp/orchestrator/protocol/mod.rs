use crate::protocols::dsp::protocol_types::{
    TransferCompletionMessageDto, TransferProcessAckDto, TransferProcessMessageWrapper, TransferRequestMessageDto,
    TransferStartMessageDto, TransferSuspensionMessageDto, TransferTerminationMessageDto,
};
pub(crate) mod protocol;

#[async_trait::async_trait]
pub trait ProtocolOrchestratorTrait: Send + Sync + 'static {
    async fn on_get_transfer_process(
        &self,
        id: &String,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>>;
    async fn on_transfer_request(
        &self,
        input: &TransferProcessMessageWrapper<TransferRequestMessageDto>,
    ) -> anyhow::Result<(TransferProcessMessageWrapper<TransferProcessAckDto>, bool)>;
    async fn on_transfer_start(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferStartMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>>;
    async fn on_transfer_suspension(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferSuspensionMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>>;
    async fn on_transfer_completion(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferCompletionMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>>;
    async fn on_transfer_termination(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferTerminationMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>>;
}
