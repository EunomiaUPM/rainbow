use crate::schemas::{
    TRANSFER_COMPLETION_SCHEMA, TRANSFER_REQUEST_SCHEMA, TRANSFER_START_SCHEMA,
    TRANSFER_SUSPENSION_SCHEMA, TRANSFER_TERMINATION_SCHEMA,
};
use anyhow::bail;
use jsonschema::output::BasicOutput;
use rainbow_common::err::transfer_err::TransferErrorType::{MessageTypeNotAcceptedError, NoTypeFieldError, ValidationError};
use serde_json::Value;

pub async fn schema_validation(json_value: Value) -> anyhow::Result<()> {
    let message_type = json_value.get("@type").and_then(|v| v.as_str());
    if message_type.is_none() {
        bail!(NoTypeFieldError)
    }

    let validation = match message_type.unwrap() {
        "dspace:TransferRequestMessage" => TRANSFER_REQUEST_SCHEMA.apply(&json_value).basic(),
        "dspace:TransferStartMessage" => TRANSFER_START_SCHEMA.apply(&json_value).basic(),
        "dspace:TransferSuspensionMessage" => TRANSFER_SUSPENSION_SCHEMA.apply(&json_value).basic(),
        "dspace:TransferCompletionMessage" => TRANSFER_COMPLETION_SCHEMA.apply(&json_value).basic(),
        "dspace:TransferTerminationMessage" => {
            TRANSFER_TERMINATION_SCHEMA.apply(&json_value).basic()
        }
        _ => {
            bail!(MessageTypeNotAcceptedError)
        }
    };

    if let BasicOutput::Invalid(errors) = validation {
        bail!(ValidationError { errors });
    }

    Ok(())
}
