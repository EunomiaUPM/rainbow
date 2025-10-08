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

use crate::common::schemas::{TRANSFER_COMPLETION_SCHEMA, TRANSFER_ERROR_SCHEMA, TRANSFER_PROCESS_SCHEMA, TRANSFER_REQUEST_SCHEMA, TRANSFER_START_SCHEMA, TRANSFER_SUSPENSION_SCHEMA, TRANSFER_TERMINATION_SCHEMA};
use anyhow::bail;
use jsonschema::BasicOutput;
use log::error;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
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
        let e = CommonErrors::format_new(BadFormat::Received, "Message malformed in JSON Data Validation".to_string().into());
        tracing::error!("{}", e.log());
        bail!(e)
    }
    Ok(())
}