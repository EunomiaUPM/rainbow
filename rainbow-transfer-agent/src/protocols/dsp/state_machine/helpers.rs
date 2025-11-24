use crate::protocols::dsp::protocol_types::{TransferProcessMessageType, TransferProcessState, TransferStateAttribute};
use anyhow::bail;
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::transfer::TransferRoles;

pub fn validate_role_for_message(
    role: &TransferRoles,
    message_type: &TransferProcessMessageType,
) -> anyhow::Result<()> {
    match (role, message_type) {
        // provider can receive all messages
        (TransferRoles::Provider, _) => {}
        // consumer cannot receive TransferRequestMessage
        (TransferRoles::Consumer, TransferProcessMessageType::TransferRequestMessage) => {
            let err = CommonErrors::parse_new(
                "Only Provider roles are allowed to receive TransferProcessMessageType TransferRequestMessage",
            );
            error!("{}", err.log());
            bail!(err)
        }
        // consumer can receive all messages but TransferRequestMessage
        (TransferRoles::Consumer, _) => {} // each other role should not be allowed
    }
    Ok(())
}

pub fn validate_state_transition(
    current_state: &TransferProcessState,
    message_type: &TransferProcessMessageType,
) -> anyhow::Result<()> {
    match message_type {
        TransferProcessMessageType::TransferRequestMessage => {
            // is not validated since there's no transition
        }
        TransferProcessMessageType::TransferStartMessage => {
            match current_state {
                TransferProcessState::Requested => {}
                TransferProcessState::Started => {
                    validate_state_transition_error_helper(&current_state, message_type)?;
                }
                TransferProcessState::Terminated => {
                    validate_state_transition_error_helper(&current_state, message_type)?;
                }
                TransferProcessState::Completed => {
                    validate_state_transition_error_helper(&current_state, message_type)?;
                }
                TransferProcessState::Suspended => {
                    // TODO check if startable if was suspended by same role
                }
            }
        }
        TransferProcessMessageType::TransferCompletionMessage => match current_state {
            TransferProcessState::Requested => {
                validate_state_transition_error_helper(&current_state, message_type)?;
            }
            TransferProcessState::Started => {}
            TransferProcessState::Terminated => {
                validate_state_transition_error_helper(&current_state, message_type)?;
            }
            TransferProcessState::Completed => {
                validate_state_transition_error_helper(&current_state, message_type)?;
            }
            TransferProcessState::Suspended => {}
        },
        TransferProcessMessageType::TransferSuspensionMessage => {
            match current_state {
                TransferProcessState::Requested => {
                    validate_state_transition_error_helper(&current_state, message_type)?;
                }
                TransferProcessState::Started => {
                    // TODO check if suspendable if was started by same role
                }
                TransferProcessState::Terminated => {
                    validate_state_transition_error_helper(&current_state, message_type)?;
                }
                TransferProcessState::Completed => {
                    validate_state_transition_error_helper(&current_state, message_type)?;
                }
                TransferProcessState::Suspended => {
                    validate_state_transition_error_helper(&current_state, message_type)?;
                }
            }
        }
        TransferProcessMessageType::TransferTerminationMessage => match current_state {
            TransferProcessState::Requested => {}
            TransferProcessState::Started => {}
            TransferProcessState::Terminated => {
                validate_state_transition_error_helper(&current_state, message_type)?;
            }
            TransferProcessState::Completed => {
                validate_state_transition_error_helper(&current_state, message_type)?;
            }
            TransferProcessState::Suspended => {}
        },
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

pub fn validate_state_attribute_transition(
    current_state: &TransferProcessState,
    current_state_attribute: &TransferStateAttribute,
    message_type: &TransferProcessMessageType,
    role: &TransferRoles,
) -> anyhow::Result<()> {
    // if message is TransferStartMessage
    // if on request, ok
    // if byConsumer or byProvider, only could be changed by same role.
    // to avoid start processes suspended by peer
    match message_type {
        TransferProcessMessageType::TransferStartMessage => match current_state_attribute {
            TransferStateAttribute::OnRequest => {}
            t => match (t, role) {
                (TransferStateAttribute::ByConsumer, TransferRoles::Consumer)
                | (TransferStateAttribute::ByProvider, TransferRoles::Provider) => {
                    let err = CommonErrors::parse_new(
                        format!(
                            "TransferProcessMessageType {} is not allowed here. Current state is {} {}",
                            message_type.to_string(),
                            current_state.to_string(),
                            current_state_attribute.to_string()
                        )
                        .as_str(),
                    );
                    error!("{}", err.log());
                    bail!(err);
                }
                _ => {}
            },
        },
        _ => {}
    };
    // sorry by the arrow matching...
    Ok(())
}

fn validate_state_transition_error_helper(
    current_state: &TransferProcessState,
    message_type: &TransferProcessMessageType,
) -> anyhow::Result<()> {
    let err = CommonErrors::parse_new(
        format!(
            "TransferProcessMessageType {} is not allowed here. Current state is {}",
            message_type.to_string(),
            current_state.to_string()
        )
        .as_str(),
    );
    error!("{}", err.log());
    bail!(err)
}
