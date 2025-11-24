use crate::entities::transfer_process::TransferAgentProcessesTrait;
use crate::protocols::dsp::protocol_types::{
    TransferProcessMessageTrait, TransferProcessState, TransferStateAttribute,
};
use crate::protocols::dsp::state_machine::{helpers, StateMachineTrait};
use log::error;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::transfer::TransferRoles;
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
    async fn validate_transition(&self, payload: Arc<dyn TransferProcessMessageTrait>) -> anyhow::Result<()> {
        // get stuff from params
        let message_type = payload.get_message();
        let consumer_pid = payload.get_consumer_pid().unwrap(); // consumerPid always exists
                                                                // retrieve current state
        let current_state_process = self
            .transfer_agent_process_entities
            .get_transfer_process_by_key_id("consumerPid", &consumer_pid)
            .await
            .map(Some)
            .or_else(|e| {
                if let Some(common_err) = e.downcast_ref::<CommonErrors>() {
                    if matches!(common_err, CommonErrors::MissingResourceError { .. }) {
                        return Ok(None);
                    }
                }
                Err(e)
            })?;
        // current role should be always exist, since in on_transfer_request is not called
        let role = current_state_process.clone().unwrap().inner.role.parse::<TransferRoles>().map_err(|_e| {
            let err = CommonErrors::parse_new("Something is wrong. Seems this process' role is not protocol compliant");
            error!("{}", err.log());
            err
        })?;
        // validate role for message type
        // provider can receive: [request, start, suspension, completion, termination]
        // consumer can receive: [start, suspension, completion, termination]
        helpers::validate_role_for_message(&role, &message_type)?;

        // current state should be always exist, since in on_transfer_request is not called
        let current_state =
            current_state_process.clone().unwrap().inner.state.parse::<TransferProcessState>().map_err(|_e| {
                let err =
                    CommonErrors::parse_new("Something is wrong. Seems this process' state is not protocol compliant");
                error!("{}", err.log());
                err
            })?;

        // validate state transition from state a to b
        helpers::validate_state_transition(&current_state, &message_type)?;

        // current state attribute
        // logical semaphore for avoiding consumer to start provider's suspension and viceversa
        let current_state_attribute = current_state_process
            .clone()
            .unwrap()
            .inner
            .state_attribute
            .unwrap_or(TransferStateAttribute::OnRequest.to_string())
            .parse::<TransferStateAttribute>()?;
        helpers::validate_state_attribute_transition(
            &current_state,
            &current_state_attribute,
            &message_type,
            &role,
        )?;

        Ok(())
    }
}
