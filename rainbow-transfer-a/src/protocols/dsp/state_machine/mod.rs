pub(crate) mod helpers;
pub(crate) mod state_machine_protocol;
pub(crate) mod state_machine_rpc;

use crate::protocols::dsp::protocol_types::TransferProcessMessageTrait;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait StateMachineTrait: Send + Sync + 'static {
    async fn validate_transition(
        &self,
        id: Option<&String>,
        payload: Arc<dyn TransferProcessMessageTrait>,
    ) -> anyhow::Result<()>;
}
