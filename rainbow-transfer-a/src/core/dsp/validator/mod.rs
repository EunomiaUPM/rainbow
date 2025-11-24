use crate::core::dsp::protocol_types::TransferProcessMessageTrait;
use std::sync::Arc;

pub(crate) mod validator_protocol;
pub(crate) mod validator_rpc;

#[async_trait::async_trait]
pub trait ValidatorTrait: Send + Sync + 'static {
    async fn validate(&self, id: Option<&String>, payload: Arc<dyn TransferProcessMessageTrait>) -> anyhow::Result<()>;
}
