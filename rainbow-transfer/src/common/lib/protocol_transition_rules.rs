/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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
use crate::provider::core::ds_protocol::DSProtocolTransferProviderTrait;
use anyhow::bail;
use rainbow_common::err::transfer_err::TransferErrorType::{
    ConsumerAlreadyRegisteredError, MessageTypeNotAcceptedError, ProtocolError, TransferProcessAlreadySuspendedError,
};
use rainbow_common::protocol::transfer::{TransferMessageTypes, TransferState};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use serde_json::Value;
use std::sync::Arc;

pub async fn protocol_transition_rules<T>(transfer_service: Arc<T>, json_value: Value) -> anyhow::Result<()>
where
    T: DSProtocolTransferProviderTrait + Send + Sync + 'static,
{
    let provider_pid = json_value
        .get("providerPid")
        .and_then(|v| v.as_str())
        .and_then(|v| get_urn_from_string(&v.to_string()).ok())
        .unwrap_or_else(|| get_urn(None));

    let consumer_pid = json_value
        .get("consumerPid")
        .and_then(|v| v.as_str())
        .and_then(|v| get_urn_from_string(&v.to_string()).ok())
        .unwrap_or_else(|| get_urn(None));

    let message_type = json_value.get("@type").and_then(|v| v.as_str()).unwrap();
    let transfer_provider = transfer_service.get_transfer_requests_by_provider(provider_pid).await;
    let transfer_consumer = transfer_service.get_transfer_requests_by_consumer(consumer_pid).await?;

    match message_type.parse::<TransferMessageTypes>() {
        Ok(message) => match message {
            TransferMessageTypes::TransferRequestMessage => {
                if transfer_consumer.is_some() {
                    bail!(ConsumerAlreadyRegisteredError)
                }
            }
            TransferMessageTypes::TransferStartMessage => {
                let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)?;
                match transfer_state {
                    TransferState::REQUESTED => {}
                    TransferState::STARTED => {
                        bail!(ProtocolError {
                            state: TransferState::STARTED,
                            message_type: "Start message is not allowed in STARTED state".to_string(),
                        })
                    }
                    TransferState::SUSPENDED => {}
                    TransferState::COMPLETED => {
                        bail!(ProtocolError {
                            state: TransferState::COMPLETED,
                            message_type: "Start message is not allowed in COMPLETED state".to_string(),
                        })
                    }
                    TransferState::TERMINATED => {
                        bail!(ProtocolError {
                            state: TransferState::TERMINATED,
                            message_type: "Start message is not allowed in TERMINATED state".to_string(),
                        })
                    }
                }
            }
            TransferMessageTypes::TransferSuspensionMessage => {
                let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)?;
                match transfer_state {
                    TransferState::REQUESTED => {
                        bail!(ProtocolError {
                            state: TransferState::REQUESTED,
                            message_type: "Suspension message is not allowed in REQUESTED state".to_string(),
                        })
                    }
                    TransferState::STARTED => {}
                    TransferState::SUSPENDED => {
                        bail!(TransferProcessAlreadySuspendedError)
                    }
                    TransferState::COMPLETED => {
                        bail!(ProtocolError {
                            state: TransferState::COMPLETED,
                            message_type: "Suspension message is not allowed in COMPLETED state".to_string(),
                        })
                    }
                    TransferState::TERMINATED => {
                        bail!(ProtocolError {
                            state: TransferState::TERMINATED,
                            message_type: "Suspension message is not allowed in TERMINATED state".to_string(),
                        })
                    }
                }
            }
            TransferMessageTypes::TransferCompletionMessage => {
                let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)?;
                match transfer_state {
                    TransferState::REQUESTED => {
                        bail!(ProtocolError {
                            state: TransferState::REQUESTED,
                            message_type: "Completion message is not allowed in REQUESTED state".to_string(),
                        })
                    }
                    TransferState::STARTED => {}
                    TransferState::SUSPENDED => {}
                    TransferState::COMPLETED => {}
                    TransferState::TERMINATED => {
                        bail!(ProtocolError {
                            state: TransferState::TERMINATED,
                            message_type: "Completion message is not allowed in TERMINATED state".to_string(),
                        })
                    }
                }
            }
            TransferMessageTypes::TransferTerminationMessage => {
                let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)?;
                match transfer_state {
                    TransferState::REQUESTED => {}
                    TransferState::STARTED => {}
                    TransferState::SUSPENDED => {}
                    TransferState::COMPLETED => {
                        bail!(ProtocolError {
                            state: TransferState::COMPLETED,
                            message_type: "Termination message is not allowed in COMPLETED state".to_string(),
                        })
                    }
                    TransferState::TERMINATED => {}
                }
            }
            _ => {
                bail!(MessageTypeNotAcceptedError);
            }
        },
        Err(_) => bail!(MessageTypeNotAcceptedError),
    }

    Ok(())
}
