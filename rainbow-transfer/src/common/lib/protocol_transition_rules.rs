use crate::common::err::TransferErrorType::{
    ConsumerAlreadyRegisteredError, MessageTypeNotAcceptedError, ProtocolError,
    TransferProcessAlreadySuspendedError, TransferProcessNotFound,
};
use crate::protocol::messages::TransferState;
use crate::provider::data::entities::transfer_process;
use anyhow::{anyhow, bail};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::Value;
use uuid::Uuid;
use rainbow_common::config::database::get_db_connection;

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

    let transfer_provider = transfer_process::Entity::find_by_id(provider_pid)
        .one(db_connection)
        .await?;
    println!("jelow");

    let transfer_consumer = transfer_process::Entity::find()
        .filter(transfer_process::Column::ConsumerPid.eq(consumer_pid))
        .one(db_connection)
        .await
        .map_err(|e| anyhow!(e))?;



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
            let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)
                .map_err(|e| anyhow!(e))?;
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
            let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)
                .map_err(|e| anyhow!(e))?;

            match transfer_state {
                TransferState::REQUESTED => {
                    bail!(ProtocolError {
                        state: TransferState::REQUESTED,
                        message_type: "Suspension message is not allowed in dspace:REQUESTED state"
                            .to_string(),
                    })
                }
                TransferState::STARTED => {}
                TransferState::SUSPENDED => bail!(TransferProcessAlreadySuspendedError),
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
            let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)
                .map_err(|e| anyhow!(e))?;
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
            let transfer_state = TransferState::try_from(transfer_provider.unwrap().state)
                .map_err(|e| anyhow!(e))?;
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
