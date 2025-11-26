use crate::protocols::dsp::protocol_types::{
    TransferCompletionMessageDto, TransferProcessMessageWrapper, TransferRequestMessageDto, TransferStartMessageDto,
    TransferSuspensionMessageDto, TransferTerminationMessageDto,
};

#[async_trait::async_trait]
pub trait ValidationDspSteps: Send + Sync + 'static {
    async fn on_transfer_request(
        &self,
        input: &TransferProcessMessageWrapper<TransferRequestMessageDto>,
    ) -> anyhow::Result<()>;
    async fn on_transfer_start(
        &self,
        uri_id: &String,
        input: &TransferProcessMessageWrapper<TransferStartMessageDto>,
    ) -> anyhow::Result<()>;
    async fn on_transfer_completion(
        &self,
        uri_id: &String,
        input: &TransferProcessMessageWrapper<TransferCompletionMessageDto>,
    ) -> anyhow::Result<()>;
    async fn on_transfer_suspension(
        &self,
        uri_id: &String,
        input: &TransferProcessMessageWrapper<TransferSuspensionMessageDto>,
    ) -> anyhow::Result<()>;
    async fn on_transfer_termination(
        &self,
        uri_id: &String,
        input: &TransferProcessMessageWrapper<TransferTerminationMessageDto>,
    ) -> anyhow::Result<()>;
}
