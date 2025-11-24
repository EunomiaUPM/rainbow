use crate::core::dsp::protocol_types::TransferProcessMessageTrait;
use crate::core::dsp::validator::ValidatorTrait;
use std::sync::Arc;

pub struct ValidatorRpcService {}

impl ValidatorRpcService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl ValidatorTrait for ValidatorRpcService {
    async fn validate(
        &self,
        _id: Option<&String>,
        _payload: Arc<dyn TransferProcessMessageTrait>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
