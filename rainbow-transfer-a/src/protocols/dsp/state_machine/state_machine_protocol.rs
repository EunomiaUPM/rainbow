use crate::entities::transfer_process::TransferAgentProcessesTrait;
use crate::protocols::dsp::protocol_types::TransferProcessMessageTrait;
use crate::protocols::dsp::state_machine::{helpers, StateMachineTrait};
use log::error;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::transfer::TransferState;
use std::sync::Arc;

pub struct StateMachineForProtocolService {
    transfer_agent_process_entities: Arc<dyn TransferAgentProcessesTrait>,
}

impl StateMachineForProtocolService {
    pub fn new(
        transfer_agent_process_entities: Arc<dyn TransferAgentProcessesTrait>,
        _config: Arc<ApplicationProviderConfig>,
    ) -> Self {
        Self { transfer_agent_process_entities }
    }
}

#[async_trait::async_trait]
impl StateMachineTrait for StateMachineForProtocolService {
    async fn validate_transition(
        &self,
        _id: Option<&String>,
        payload: Arc<dyn TransferProcessMessageTrait>,
    ) -> anyhow::Result<()> {
        let message_type = payload.get_message();
        let consumer_pid = payload.get_consumer_pid().unwrap(); // consumerPid always exists
        let current_state_process = self
            .transfer_agent_process_entities
            .get_transfer_process_by_key_id("consumerPid", &consumer_pid)
            .await
            .map(Some)
            .or_else(|e| match e.downcast::<CommonErrors>().unwrap() {
                CommonErrors::MissingResourceError { .. } => Ok(None),
                e => Err(e),
            })?;
        let role = current_state_process.as_ref().map(|t| t.inner.role.as_str());

        helpers::validate_role_for_message(role, &message_type)?;

        let current_state = current_state_process.as_ref().map(|c| c.inner.state.clone());
        let current_state_enum = match current_state {
            Some(s) => Some(s.parse::<TransferState>().map_err(|_e| {
                let err =
                    CommonErrors::parse_new("Something is wrong. Seems this process' state is not protocol compliant");
                error!("{}", err.log());
                err
            })?),
            None => None,
        };

        helpers::validate_state_transition(current_state_enum, &message_type)?;

        Ok(())
    }
}
