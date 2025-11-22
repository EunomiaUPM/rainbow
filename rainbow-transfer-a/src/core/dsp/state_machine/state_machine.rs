use crate::core::dsp::protocol_types::{TransferProcessMessageTrait, TransferProcessMessageType};
use crate::core::dsp::state_machine::StateMachineTrait;
use crate::entities::transfer_process::TransferAgentProcessesTrait;
use anyhow::bail;
use log::error;
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use rainbow_common::config::ConfigRoles;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::transfer::TransferState;
use std::sync::Arc;

pub struct StateMachineForDspService {
    transfer_agent_process_entities: Arc<dyn TransferAgentProcessesTrait>,
    config: Arc<ApplicationProviderConfig>,
}

impl StateMachineForDspService {
    pub fn new(
        transfer_agent_process_entities: Arc<dyn TransferAgentProcessesTrait>,
        config: Arc<ApplicationProviderConfig>,
    ) -> Self {
        Self { transfer_agent_process_entities, config }
    }
}

#[async_trait::async_trait]
impl StateMachineTrait for StateMachineForDspService {
    async fn validate_transition(
        &self,
        id: Option<&String>,
        payload: Arc<dyn TransferProcessMessageTrait>,
    ) -> anyhow::Result<()> {
        let role = self.config.get_role();
        let message_type = payload.get_message();
        let consumer_pid = payload.get_consumer_pid().unwrap(); // consumerPid always exists
        // only use consumerPid because all have consumerPid defined
        let current_state_process = self
            .transfer_agent_process_entities
            .get_transfer_process_by_key_id("consumerPid", &consumer_pid)
            .await
            .map(Some)
            .or_else(|e| match e.downcast::<CommonErrors>().unwrap() {
                CommonErrors::MissingResourceError { .. } => Ok(None),
                e => Err(e),
            })?;
        let current_state = current_state_process.as_ref().map(|c| c.inner.state.clone());
        // match role to message type
        match (&role, &message_type) {
            (ConfigRoles::Provider, TransferProcessMessageType::TransferStartMessage) => {
                let err = CommonErrors::parse_new(
                    "Only Consumer roles are allowed to receive TransferProcessMessageType TransferStartMessage",
                );
                error!("{}", err.log());
                bail!(err)
            }
            (ConfigRoles::Provider, _) => {}
            (ConfigRoles::Consumer, TransferProcessMessageType::TransferRequestMessage) => {
                let err = CommonErrors::parse_new(
                    "Only Provider roles are allowed to receive TransferProcessMessageType TransferRequestMessage",
                );
                error!("{}", err.log());
                bail!(err)
            }
            (ConfigRoles::Consumer, _) => {}
            (_, _) => {
                let err = CommonErrors::parse_new(
                    "Only Provider or Consumer roles are allowed to participate in transfer process",
                );
                error!("{}", err.log());
                bail!(err)
            }
        }

        // match Message type to current state
        match message_type {
            TransferProcessMessageType::TransferRequestMessage => {
                if current_state_process.is_some() {
                    let current_state = current_state_process.unwrap();
                    let err = CommonErrors::parse_new(
                        format!(
                            "TransferProcessMessageType TransferRequestMessage is not allowed here. Process {} with consumerPid {} exists",
                            current_state.inner.id,
                            current_state.identifiers.get("consumerPid").unwrap(),
                        ).as_str(),
                    );
                    error!("{}", err.log());
                    bail!(err)
                }
            }
            TransferProcessMessageType::TransferStartMessage => {
                if current_state.is_none() {
                    let err = CommonErrors::parse_new("Something is wrong. Seems this process has no state");
                    error!("{}", err.log());
                    bail!(err)
                }
                let current_state = current_state.unwrap().parse::<TransferState>().map_err(|e| {
                    let err = CommonErrors::parse_new(
                        "Something is wrong. Seems this process' state is not protocol compliant",
                    );
                    error!("{}", err.log());
                    err
                })?;
                match current_state {
                    TransferState::REQUESTED => {}
                    TransferState::STARTED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferStartMessage is not allowed here. Current state is already STARTED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::TERMINATED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferStartMessage is not allowed here. Current state is TERMINATED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::COMPLETED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferStartMessage is not allowed here. Current state is COMPLETED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::SUSPENDED => {
                        // TODO check if startable if was suspended by same role
                    }
                }
            }
            TransferProcessMessageType::TransferCompletionMessage => {
                if current_state.is_none() {
                    let err = CommonErrors::parse_new("Something is wrong. Seems this process has no state");
                    error!("{}", err.log());
                    bail!(err)
                }
                let current_state = current_state.unwrap().parse::<TransferState>().map_err(|e| {
                    let err = CommonErrors::parse_new(
                        "Something is wrong. Seems this process' state is not protocol compliant",
                    );
                    error!("{}", err.log());
                    err
                })?;
                match current_state {
                    TransferState::REQUESTED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferCompletionMessage is not allowed here. Current state is REQUESTED. Please terminate instead.",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::STARTED => {}
                    TransferState::TERMINATED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferCompletionMessage is not allowed here. Current state is TERMINATED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::COMPLETED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferCompletionMessage is not allowed here. Current state is already COMPLETED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::SUSPENDED => {}
                }
            }
            TransferProcessMessageType::TransferSuspensionMessage => {
                if current_state.is_none() {
                    let err = CommonErrors::parse_new("Something is wrong. Seems this process has no state");
                    error!("{}", err.log());
                    bail!(err)
                }
                let current_state = current_state.unwrap().parse::<TransferState>().map_err(|e| {
                    let err = CommonErrors::parse_new(
                        "Something is wrong. Seems this process' state is not protocol compliant",
                    );
                    error!("{}", err.log());
                    err
                })?;
                match current_state {
                    TransferState::REQUESTED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferSuspensionMessage is not allowed here. Current state is REQUESTED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::STARTED => {
                        // TODO check if suspendable if was started by same role
                    }
                    TransferState::TERMINATED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferSuspensionMessage is not allowed here. Current state is TERMINATED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::COMPLETED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferSuspensionMessage is not allowed here. Current state is COMPLETED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::SUSPENDED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferSuspensionMessage is not allowed here. Current state is already SUSPENDED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                }
            }
            TransferProcessMessageType::TransferTerminationMessage => {
                if current_state.is_none() {
                    let err = CommonErrors::parse_new("Something is wrong. Seems this process has no state");
                    error!("{}", err.log());
                    bail!(err)
                }
                let current_state = current_state.unwrap().parse::<TransferState>().map_err(|e| {
                    let err = CommonErrors::parse_new(
                        "Something is wrong. Seems this process' state is not protocol compliant",
                    );
                    error!("{}", err.log());
                    err
                })?;
                match current_state {
                    TransferState::REQUESTED => {}
                    TransferState::STARTED => {}
                    TransferState::TERMINATED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferSuspensionMessage is not allowed here. Current state is already TERMINATED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::COMPLETED => {
                        let err = CommonErrors::parse_new(
                            "TransferProcessMessageType TransferSuspensionMessage is not allowed here. Current state is COMPLETED",
                        );
                        error!("{}", err.log());
                        bail!(err)
                    }
                    TransferState::SUSPENDED => {}
                }
            }
            TransferProcessMessageType::TransferProcess => {
                let err = CommonErrors::parse_new("TransferProcessMessageType TransferProcess is not allowed here");
                error!("{}", err.log());
                bail!(err)
            }
            TransferProcessMessageType::TransferError => {
                let err = CommonErrors::parse_new("TransferProcessMessageType TransferProcess is not allowed here");
                error!("{}", err.log());
                bail!(err)
            }
        }

        Ok(())
    }
}
