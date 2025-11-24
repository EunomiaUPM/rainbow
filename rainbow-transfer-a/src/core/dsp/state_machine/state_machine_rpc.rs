use crate::core::dsp::protocol_types::{TransferProcessMessageTrait, TransferProcessMessageType};
use crate::core::dsp::state_machine::StateMachineTrait;
use crate::entities::transfer_process::TransferAgentProcessesTrait;
use anyhow::bail;
use log::error;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::transfer::TransferState;
use std::sync::Arc;

pub struct StateMachineForRpcService {
    transfer_agent_process_entities: Arc<dyn TransferAgentProcessesTrait>,
    _config: Arc<ApplicationProviderConfig>,
}

impl StateMachineForRpcService {
    pub fn new(
        transfer_agent_process_entities: Arc<dyn TransferAgentProcessesTrait>,
        config: Arc<ApplicationProviderConfig>,
    ) -> Self {
        Self { transfer_agent_process_entities, _config: config }
    }
}

#[async_trait::async_trait]
impl StateMachineTrait for StateMachineForRpcService {
    async fn validate_transition(
        &self,
        _id: Option<&String>,
        payload: Arc<dyn TransferProcessMessageTrait>,
    ) -> anyhow::Result<()> {
       Ok(())
    }
}
