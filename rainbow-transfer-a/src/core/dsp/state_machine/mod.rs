pub(crate) mod state_machine;

use crate::core::dsp::protocol_types::TransferProcessMessageTrait;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait StateMachineTrait: Send + Sync + 'static {
    async fn validate_transition(
        &self,
        id: Option<&String>,
        payload: Arc<dyn TransferProcessMessageTrait>,
    ) -> anyhow::Result<()>;
    async fn validate_rpc_transition(
        &self,
        id: Option<&String>,
        payload: Arc<dyn TransferProcessMessageTrait>,
    ) -> anyhow::Result<()>;
}
