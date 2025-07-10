use crate::common::schemas::{TRANSFER_COMPLETION_SCHEMA, TRANSFER_ERROR_SCHEMA, TRANSFER_PROCESS_SCHEMA, TRANSFER_REQUEST_SCHEMA, TRANSFER_START_SCHEMA, TRANSFER_SUSPENSION_SCHEMA, TRANSFER_TERMINATION_SCHEMA};
use anyhow::bail;
use jsonschema::BasicOutput;
use log::error;
use rainbow_common::protocol::transfer::transfer_protocol_trait::DSProtocolTransferMessageTrait;
use rainbow_common::protocol::transfer::TransferMessageTypes;
use serde_json::to_value;

pub fn validate_payload_schema<'a, M: DSProtocolTransferMessageTrait<'a>>(message: &M) -> anyhow::Result<()> {
    let validation = match message.get_message_type()? {
        TransferMessageTypes::TransferError => {
            TRANSFER_ERROR_SCHEMA.apply(&to_value(message)?).basic()
        }
        TransferMessageTypes::TransferRequestMessage => {
            TRANSFER_REQUEST_SCHEMA.apply(&to_value(message)?).basic()
        }
        TransferMessageTypes::TransferStartMessage => {
            TRANSFER_START_SCHEMA.apply(&to_value(message)?).basic()
        }
        TransferMessageTypes::TransferSuspensionMessage => {
            TRANSFER_SUSPENSION_SCHEMA.apply(&to_value(message)?).basic()
        }
        TransferMessageTypes::TransferCompletionMessage => {
            TRANSFER_COMPLETION_SCHEMA.apply(&to_value(message)?).basic()
        }
        TransferMessageTypes::TransferTerminationMessage => {
            TRANSFER_TERMINATION_SCHEMA.apply(&to_value(message)?).basic()
        }
        TransferMessageTypes::TransferProcessMessage => {
            TRANSFER_PROCESS_SCHEMA.apply(&to_value(message)?).basic()
        }
    };
    if let BasicOutput::Invalid(errors) = validation {
        for error in errors {
            error!("{}", error.instance_location());
            error!("{}", error.error_description());
        }
        bail!("Message malformed in JSON Data Validation");
    }
    Ok(())
}