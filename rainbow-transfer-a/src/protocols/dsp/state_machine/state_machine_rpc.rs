use crate::entities::transfer_process::TransferAgentProcessesTrait;
use crate::errors::transfer_errors::TransferErrors;
use crate::protocols::dsp::protocol_types::{
    TransferProcessMessageTrait, TransferProcessMessageType, TransferProcessState, TransferStateAttribute,
};
use crate::protocols::dsp::state_machine::StateMachineTrait;
use log::{debug, error};
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::errors::ErrorLog;
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
        debug!("DSProtocolRPC Service: transition_validation");

        // Negotiation state
        // For RPC consumer_pid and provider_pid are always some
        let _consumer_pid = payload.get_consumer_pid().ok_or_else(|| anyhow::anyhow!("Missing consumer PID"))?;
        let provider_pid = payload.get_provider_pid().ok_or_else(|| anyhow::anyhow!("Missing provider PID"))?;
        let message_type = payload.get_message();

        // For RPC transfer process is always there
        let tp =
            self.transfer_agent_process_entities.get_transfer_process_by_key_id("providerPid", &provider_pid).await?;
        let transfer_state =
            tp.inner.state.parse::<TransferProcessState>().map_err(|_| anyhow::anyhow!("Invalid transfer state"))?;
        let transfer_state_attribute = tp
            .inner
            .state_attribute
            .unwrap_or(TransferStateAttribute::OnRequest.to_string())
            .parse::<TransferStateAttribute>()
            .map_err(|_| anyhow::anyhow!("Invalid transfer state attribute"))?;

        match message_type {
            TransferProcessMessageType::TransferStartMessage => match transfer_state {
                TransferProcessState::Requested => {}
                TransferProcessState::Started => {
                    let e = TransferErrors::protocol_new(
                        "Start message is not allowed in STARTED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    return Err(anyhow::anyhow!(e));
                }
                TransferProcessState::Suspended => {
                    // Transfer state attribute check.
                    match transfer_state_attribute {
                        // If suspended by consumer, not able to start from provider
                        TransferStateAttribute::ByConsumer => {
                            let e = TransferErrors::protocol_new(
                                "State SUSPENDED was established by Consumer, Provider is not allowed to change it"
                                    .to_string()
                                    .into(),
                            );
                            error!("{}", e.log());
                            return Err(anyhow::anyhow!(e));
                        }
                        TransferStateAttribute::OnRequest => {}
                        TransferStateAttribute::ByProvider => {}
                    }
                }
                TransferProcessState::Completed => {
                    let e = TransferErrors::protocol_new(
                        "Start message is not allowed in COMPLETED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    return Err(anyhow::anyhow!(e));
                }
                TransferProcessState::Terminated => {
                    let e = TransferErrors::protocol_new(
                        "Start message is not allowed in TERMINATED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    return Err(anyhow::anyhow!(e));
                }
            },
            // 4. Transfer suspension transition check
            TransferProcessMessageType::TransferSuspensionMessage => match transfer_state {
                TransferProcessState::Requested => {
                    let e = TransferErrors::protocol_new(
                        "Suspension message is not allowed in REQUESTED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    return Err(anyhow::anyhow!(e));
                }
                TransferProcessState::Started => {}
                TransferProcessState::Suspended => {
                    let e = TransferErrors::protocol_new("Transfer already suspended".to_string().into());
                    error!("{}", e.log());
                    return Err(anyhow::anyhow!(e));
                }
                TransferProcessState::Completed => {
                    let e = TransferErrors::protocol_new(
                        "Suspension message is not allowed in COMPLETED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    return Err(anyhow::anyhow!(e));
                }
                TransferProcessState::Terminated => {
                    let e = TransferErrors::protocol_new(
                        "Suspension message is not allowed in TERMINATED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    return Err(anyhow::anyhow!(e));
                }
            },
            // 4. Transfer completion transition check
            TransferProcessMessageType::TransferCompletionMessage => match transfer_state {
                TransferProcessState::Requested => {
                    let e = TransferErrors::protocol_new(
                        "Completion message is not allowed in REQUESTED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    return Err(anyhow::anyhow!(e));
                }
                TransferProcessState::Started => {}
                TransferProcessState::Suspended => {}
                TransferProcessState::Completed => {}
                TransferProcessState::Terminated => {
                    let e = TransferErrors::protocol_new(
                        "Completion message is not allowed in TERMINATED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    return Err(anyhow::anyhow!(e));
                }
            },
            // 4. Transfer termination transition check
            TransferProcessMessageType::TransferTerminationMessage => match transfer_state {
                TransferProcessState::Requested => {}
                TransferProcessState::Started => {}
                TransferProcessState::Suspended => {}
                TransferProcessState::Completed => {
                    let e = TransferErrors::protocol_new(
                        "Completion message is not allowed in COMPLETED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    return Err(anyhow::anyhow!(e));
                }
                TransferProcessState::Terminated => {}
            },
            // 4. Rest of messages not allowed
            _ => {
                let e = TransferErrors::protocol_new("This message type is not allowed".to_string().into());
                error!("{}", e.log());
                return Err(anyhow::anyhow!(e));
            }
        }
        Ok(())
    }
}
