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

use crate::common::schemas::{
    TRANSFER_COMPLETION_SCHEMA, TRANSFER_REQUEST_SCHEMA, TRANSFER_START_SCHEMA,
    TRANSFER_SUSPENSION_SCHEMA, TRANSFER_TERMINATION_SCHEMA,
};
use anyhow::bail;
use jsonschema::output::BasicOutput;
use rainbow_common::err::transfer_err::TransferErrorType::{
    MessageTypeNotAcceptedError, NoTypeFieldError, ValidationError,
};
use serde_json::Value;

pub async fn schema_validation(json_value: Value) -> anyhow::Result<()> {
    let message_type = json_value.get("@type").and_then(|v| v.as_str());
    if message_type.is_none() {
        bail!(NoTypeFieldError)
    }

    let validation = match message_type.unwrap() {
        "TransferRequestMessage" => TRANSFER_REQUEST_SCHEMA.apply(&json_value).basic(),
        "TransferStartMessage" => TRANSFER_START_SCHEMA.apply(&json_value).basic(),
        "TransferSuspensionMessage" => TRANSFER_SUSPENSION_SCHEMA.apply(&json_value).basic(),
        "TransferCompletionMessage" => TRANSFER_COMPLETION_SCHEMA.apply(&json_value).basic(),
        "TransferTerminationMessage" => {
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
