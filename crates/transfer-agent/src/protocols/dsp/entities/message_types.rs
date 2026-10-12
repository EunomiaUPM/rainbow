/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! The DSP transfer message types.

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use ymir::errors::{BadFormat, Errors};

/// DSP transfer messages this agent sends and receives.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum TransferDSPMessageType {
    TransferRequestMessage,
    TransferStartMessage,
    TransferCompletionMessage,
    TransferSuspensionMessage,
    TransferTerminationMessage,
    TransferProcess,
    TransferError,
}

impl Display for TransferDSPMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TransferDSPMessageType::TransferRequestMessage => "TransferRequestMessage".to_string(),
            TransferDSPMessageType::TransferStartMessage => "TransferStartMessage".to_string(),
            TransferDSPMessageType::TransferCompletionMessage => {
                "TransferCompletionMessage".to_string()
            }
            TransferDSPMessageType::TransferSuspensionMessage => {
                "TransferSuspensionMessage".to_string()
            }
            TransferDSPMessageType::TransferTerminationMessage => {
                "TransferTerminationMessage".to_string()
            }
            TransferDSPMessageType::TransferProcess => "TransferProcess".to_string(),
            TransferDSPMessageType::TransferError => "TransferError".to_string(),
        };
        write!(f, "{str}")
    }
}

impl std::str::FromStr for TransferDSPMessageType {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Accept the prefixed form too: a peer may write `dspace:TransferStartMessage`
        // and the namespace normalizer only strips prefixes it is wired for.
        let term = s.rsplit([':', '/', '#']).next().unwrap_or(s);
        match term {
            "TransferRequestMessage" => Ok(Self::TransferRequestMessage),
            "TransferStartMessage" => Ok(Self::TransferStartMessage),
            "TransferCompletionMessage" => Ok(Self::TransferCompletionMessage),
            "TransferSuspensionMessage" => Ok(Self::TransferSuspensionMessage),
            "TransferTerminationMessage" => Ok(Self::TransferTerminationMessage),
            "TransferProcess" => Ok(Self::TransferProcess),
            "TransferError" => Ok(Self::TransferError),
            other => Err(Errors::format(
                BadFormat::Received,
                format!("unknown DSP message type: {other}"),
                None,
            )),
        }
    }
}
