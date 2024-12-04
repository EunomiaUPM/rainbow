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

use anyhow::bail;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::err::transfer_err::TransferErrorType::{
    ConsumerAlreadyRegisteredError, MessageTypeNotAcceptedError, ProtocolError,
    TransferProcessAlreadySuspendedError, TransferProcessNotFound,
};
use rainbow_common::protocol::transfer::TransferState;
use rainbow_db::transfer_consumer::repo::TRANSFER_CONSUMER_REPO;
use rainbow_db::transfer_provider::repo::TRANSFER_PROVIDER_REPO;
use serde_json::Value;
use uuid::Uuid;

pub async fn protocol_transition_rules(json_value: Value) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;

    let provider_pid = json_value
        .get("dspace:providerPid")
        .and_then(|v| v.as_str())
        .and_then(|v| Uuid::parse_str(v).ok())
        .unwrap_or_default();

    let consumer_pid = json_value
        .get("dspace:consumerPid")
        .and_then(|v| v.as_str())
        .and_then(|v| Uuid::parse_str(v).ok())
        .unwrap_or_default();

    let message_type = json_value.get("@type").and_then(|v| v.as_str()).unwrap();

    let transfer_provider =
        TRANSFER_PROVIDER_REPO.get_transfer_process_by_provider(provider_pid).await?;
    let transfer_consumer =
        TRANSFER_CONSUMER_REPO.get_transfer_callbacks_by_consumer_id(consumer_pid).await
            .unwrap_or_default();

    match message_type {
        "dspace:TransferRequestMessage" => {
            if transfer_consumer.is_some() {
                bail!(ConsumerAlreadyRegisteredError);
            }
        }
        "dspace:TransferStartMessage" => {
            if transfer_provider.is_none() {
                bail!(TransferProcessNotFound);
            }
            let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)?;
            match transfer_state {
                TransferState::REQUESTED => {}
                TransferState::STARTED => {
                    bail!(ProtocolError {
                        state: TransferState::STARTED,
                        message_type: "Start message is not allowed in dspace:STARTED state"
                            .to_string(),
                    })
                }
                TransferState::SUSPENDED => {}
                TransferState::COMPLETED => {
                    bail!(ProtocolError {
                        state: TransferState::COMPLETED,
                        message_type: "Start message is not allowed in dspace:COMPLETED state"
                            .to_string(),
                    })
                }
                TransferState::TERMINATED => {
                    bail!(ProtocolError {
                        state: TransferState::TERMINATED,
                        message_type: "Start message is not allowed in dspace:TERMINATED state"
                            .to_string(),
                    })
                }
            }
        }
        "dspace:TransferSuspensionMessage" => {
            if transfer_provider.is_none() {
                bail!(TransferProcessNotFound);
            }
            let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)?;
            match transfer_state {
                TransferState::REQUESTED => {
                    bail!(ProtocolError {
                        state: TransferState::REQUESTED,
                        message_type: "Suspension message is not allowed in dspace:REQUESTED state"
                            .to_string(),
                    })
                }
                TransferState::STARTED => {}
                TransferState::SUSPENDED => {
                    bail!(TransferProcessAlreadySuspendedError)
                }
                TransferState::COMPLETED => {
                    bail!(ProtocolError {
                        state: TransferState::COMPLETED,
                        message_type: "Suspension message is not allowed in dspace:COMPLETED state"
                            .to_string(),
                    })
                }
                TransferState::TERMINATED => {
                    bail!(ProtocolError {
                        state: TransferState::TERMINATED,
                        message_type:
                            "Suspension message is not allowed in dspace:TERMINATED state"
                                .to_string(),
                    })
                }
            }
        }
        "dspace:TransferCompletionMessage" => {
            if transfer_provider.is_none() {
                bail!(TransferProcessNotFound);
            }
            let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)?;
            match transfer_state {
                TransferState::REQUESTED => {
                    bail!(ProtocolError {
                        state: TransferState::REQUESTED,
                        message_type: "Completion message is not allowed in dspace:REQUESTED state"
                            .to_string(),
                    })
                }
                TransferState::STARTED => {}
                TransferState::SUSPENDED => {}
                TransferState::COMPLETED => {}
                TransferState::TERMINATED => {
                    bail!(ProtocolError {
                        state: TransferState::TERMINATED,
                        message_type:
                            "Completion message is not allowed in dspace:TERMINATED state"
                                .to_string(),
                    })
                }
            }
        }
        "dspace:TransferTerminationMessage" => {
            if transfer_provider.is_none() {
                bail!(TransferProcessNotFound);
            }
            let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)?;
            match transfer_state {
                TransferState::REQUESTED => {}
                TransferState::STARTED => {}
                TransferState::SUSPENDED => {}
                TransferState::COMPLETED => {
                    bail!(ProtocolError {
                        state: TransferState::COMPLETED,
                        message_type:
                            "Termination message is not allowed in dspace:COMPLETED state"
                                .to_string(),
                    })
                }
                TransferState::TERMINATED => {}
            }
        }
        _ => {
            bail!(MessageTypeNotAcceptedError);
        }
    }

    Ok(())
}
