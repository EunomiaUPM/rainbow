use crate::protocols::dsp::protocol_types::TransferProcessMessageType;
use anyhow::{bail, Result};
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::transfer::TransferState;

pub fn validate_role_for_message(role: Option<&str>, message_type: &TransferProcessMessageType) -> Result<()> {
    match (role, message_type) {
        (Some("Provider"), TransferProcessMessageType::TransferStartMessage) => {
            let err = CommonErrors::parse_new(
                "Only Consumer roles are allowed to receive TransferProcessMessageType TransferStartMessage",
            );
            error!("{}", err.log());
            bail!(err)
        }
        (Some("Provider"), _) => {}
        (None, TransferProcessMessageType::TransferRequestMessage) => {}
        (Some("Consumer"), TransferProcessMessageType::TransferRequestMessage) => {
            let err = CommonErrors::parse_new(
                "Only Provider roles are allowed to receive TransferProcessMessageType TransferRequestMessage",
            );
            error!("{}", err.log());
            bail!(err)
        }
        (Some("Consumer"), _) => {}
        (Some(_), _) => {
            let err = CommonErrors::parse_new(
                "Only Provider or Consumer roles are allowed to participate in transfer process",
            );
            error!("{}", err.log());
            bail!(err)
        }
        _ => {}
    }
    Ok(())
}

pub fn validate_state_transition(
    current_state: Option<TransferState>,
    message_type: &TransferProcessMessageType,
) -> Result<()> {
    match message_type {
        TransferProcessMessageType::TransferRequestMessage => {
            if current_state.is_some() {
                // This case needs context about the process ID, so we might handle it in the caller
                // or pass the ID here. But for generic state transition, if state exists, Request is invalid?
                // The original code checked if process exists. Here we check if state exists.
                // If state exists, it means process exists.
                let err = CommonErrors::parse_new(
                    "TransferProcessMessageType TransferRequestMessage is not allowed here. Process already exists",
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
            match current_state.unwrap() {
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
            match current_state.unwrap() {
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
            match current_state.unwrap() {
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
            match current_state.unwrap() {
                TransferState::REQUESTED => {}
                TransferState::STARTED => {}
                TransferState::TERMINATED => {
                    let err = CommonErrors::parse_new(
                        "TransferProcessMessageType TransferTerminationMessage is not allowed here. Current state is already TERMINATED",
                    );
                    error!("{}", err.log());
                    bail!(err)
                }
                TransferState::COMPLETED => {
                    let err = CommonErrors::parse_new(
                        "TransferProcessMessageType TransferTerminationMessage is not allowed here. Current state is COMPLETED",
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
