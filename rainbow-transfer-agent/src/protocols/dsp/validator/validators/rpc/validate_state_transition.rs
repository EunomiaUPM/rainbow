/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::protocols::dsp::protocol_types::{TransferProcessMessageType, TransferProcessState, TransferStateAttribute};
use crate::protocols::dsp::validator::traits::validate_state_transition::ValidateStateTransition;
use crate::protocols::dsp::validator::traits::validation_helpers::ValidationHelpers;
use anyhow::bail;
use log::error;
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;

pub struct ValidatedStateTransitionServiceForRcp {
    _helpers: Arc<dyn ValidationHelpers>,
}
impl ValidatedStateTransitionServiceForRcp {
    pub fn new(helpers: Arc<dyn ValidationHelpers>) -> Self {
        Self { _helpers: helpers }
    }
}
#[async_trait::async_trait]
impl ValidateStateTransition for ValidatedStateTransitionServiceForRcp {
    async fn validate_role_for_message(
        &self,
        role: &RoleConfig,
        message_type: &TransferProcessMessageType,
    ) -> anyhow::Result<()> {
        match (role, message_type) {
            // consumer can send all messages
            (RoleConfig::Consumer, _) => {}
            // provider cannot send TransferRequestMessage
            (RoleConfig::Provider, TransferProcessMessageType::TransferRequestMessage) => {
                let err = CommonErrors::parse_new(
                    "Only Consumer roles are allowed to send TransferProcessMessageType TransferRequestMessage",
                );
                error!("{}", err.log());
                bail!(err)
            }
            // consumer can receive all messages but TransferRequestMessage
            (RoleConfig::Provider, _) => {} // each other role should not be allowed
            _ => {}
        }
        Ok(())
    }

    async fn validate_state_transition(
        &self,
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

    async fn validate_state_attribute_transition(
        &self,
        current_state: &TransferProcessState,
        current_state_attribute: &TransferStateAttribute,
        message_type: &TransferProcessMessageType,
        role: &RoleConfig,
    ) -> anyhow::Result<()> {
        // if message is TransferStartMessage
        // if on request, ok
        // if byConsumer or byProvider, only could be changed by different role (unlike the other implementation).
        // to avoid start processes suspended by peer
        match message_type {
            TransferProcessMessageType::TransferStartMessage => match current_state_attribute {
                TransferStateAttribute::OnRequest => {}
                t => match (t, role) {
                    (TransferStateAttribute::ByConsumer, RoleConfig::Provider)
                    | (TransferStateAttribute::ByProvider, RoleConfig::Consumer) => {
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
