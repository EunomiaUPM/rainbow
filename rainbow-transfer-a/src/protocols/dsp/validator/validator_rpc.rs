use crate::protocols::dsp::protocol_types::{TransferProcessMessageTrait, TransferProcessMessageType};
use crate::protocols::dsp::validator::ValidatorTrait;
use anyhow::anyhow;
use log::error;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
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
        payload: Arc<dyn TransferProcessMessageTrait>,
    ) -> anyhow::Result<()> {
        let message_type = payload.get_message();

        // Consumer PID is required for all messages
        if payload.get_consumer_pid().is_none() {
            let err = CommonErrors::format_new(BadFormat::Received, "Consumer PID is missing");
            error!("{}", err.log());
            return Err(anyhow!(err));
        }

        // Provider PID is required for all messages except TransferRequestMessage
        if message_type != TransferProcessMessageType::TransferRequestMessage && payload.get_provider_pid().is_none() {
            let err = CommonErrors::format_new(BadFormat::Received, "Provider PID is missing");
            error!("{}", err.log());
            return Err(anyhow!(err));
        }

        Ok(())
    }
}
