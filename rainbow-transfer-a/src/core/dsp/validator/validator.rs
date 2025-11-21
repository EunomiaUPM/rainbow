use crate::core::dsp::protocol_types::TransferProcessMessageTrait;
use crate::core::dsp::validator::ValidatorTrait;
use std::sync::Arc;

pub struct DspValidatorService {}

impl DspValidatorService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl ValidatorTrait for DspValidatorService {
    async fn validate(&self, id: Option<&String>, payload: Arc<dyn TransferProcessMessageTrait>) -> anyhow::Result<()> {
        Ok(())
    }
}
