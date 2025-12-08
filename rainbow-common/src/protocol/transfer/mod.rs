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

use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::ops::Not;
use std::str::FromStr;

pub mod transfer_completion;
pub mod transfer_consumer_process;
pub mod transfer_data_address;
pub mod transfer_error;
pub mod transfer_process;
pub mod transfer_protocol_trait;
pub mod transfer_request;
pub mod transfer_start;
pub mod transfer_suspension;
pub mod transfer_termination;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferState {
    #[serde(rename = "REQUESTED")]
    REQUESTED,
    #[serde(rename = "STARTED")]
    STARTED,
    #[serde(rename = "TERMINATED")]
    TERMINATED,
    #[serde(rename = "COMPLETED")]
    COMPLETED,
    #[serde(rename = "SUSPENDED")]
    SUSPENDED,
}

impl FromStr for TransferState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REQUESTED" => Ok(Self::REQUESTED),
            "STARTED" => Ok(Self::STARTED),
            "TERMINATED" => Ok(Self::TERMINATED),
            "COMPLETED" => Ok(Self::COMPLETED),
            "SUSPENDED" => Ok(Self::SUSPENDED),
            _ => Err(anyhow!("State not recognized")),
        }
    }
}

impl fmt::Display for TransferState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferState::REQUESTED => f.write_str("REQUESTED"),
            TransferState::STARTED => f.write_str("STARTED"),
            TransferState::TERMINATED => f.write_str("TERMINATED"),
            TransferState::COMPLETED => f.write_str("COMPLETED"),
            TransferState::SUSPENDED => f.write_str("SUSPENDED"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferStateAttribute {
    #[serde(rename = "ON_REQUEST")]
    OnRequest,
    #[serde(rename = "BY_PROVIDER")]
    ByProvider,
    #[serde(rename = "BY_CONSUMER")]
    ByConsumer,
}

impl FromStr for TransferStateAttribute {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ON_REQUEST" => Ok(Self::OnRequest),
            "BY_PROVIDER" => Ok(Self::ByProvider),
            "BY_CONSUMER" => Ok(Self::ByConsumer),
            _ => Err(anyhow!("State Attribute not recognized")),
        }
    }
}

impl fmt::Display for TransferStateAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferStateAttribute::OnRequest => f.write_str("ON_REQUEST"),
            TransferStateAttribute::ByProvider => f.write_str("BY_PROVIDER"),
            TransferStateAttribute::ByConsumer => f.write_str("BY_CONSUMER"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum TransferRoles {
    Provider,
    Consumer,
}

impl Not for TransferRoles {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            TransferRoles::Provider => TransferRoles::Consumer,
            TransferRoles::Consumer => TransferRoles::Provider
        }
    }
}

impl FromStr for TransferRoles {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Provider" => Ok(Self::Provider),
            "Consumer" => Ok(Self::Consumer),
            _ => Err(anyhow!("Role not recognized")),
        }
    }
}

impl Display for TransferRoles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TransferRoles::Provider => "Provider".to_string(),
            TransferRoles::Consumer => "Consumer".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferMessageTypes {
    TransferError,
    TransferRequestMessage,
    TransferStartMessage,
    TransferSuspensionMessage,
    TransferCompletionMessage,
    TransferTerminationMessage,
    TransferProcessMessage,
}

impl fmt::Display for TransferMessageTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferMessageTypes::TransferError => f.write_str("TransferError"),
            TransferMessageTypes::TransferRequestMessage => f.write_str("TransferRequestMessage"),
            TransferMessageTypes::TransferStartMessage => f.write_str("TransferStartMessage"),
            TransferMessageTypes::TransferSuspensionMessage => f.write_str("TransferSuspensionMessage"),
            TransferMessageTypes::TransferCompletionMessage => f.write_str("TransferCompletionMessage"),
            TransferMessageTypes::TransferTerminationMessage => f.write_str("TransferTerminationMessage"),
            TransferMessageTypes::TransferProcessMessage => f.write_str("TransferProcess"),
        }
    }
}

impl FromStr for TransferMessageTypes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TransferError" => Ok(Self::TransferError),
            "TransferRequestMessage" => Ok(Self::TransferRequestMessage),
            "TransferStartMessage" => Ok(Self::TransferStartMessage),
            "TransferSuspensionMessage" => Ok(Self::TransferSuspensionMessage),
            "TransferCompletionMessage" => Ok(Self::TransferCompletionMessage),
            "TransferTerminationMessage" => Ok(Self::TransferTerminationMessage),
            "TransferProcess" => Ok(Self::TransferProcessMessage),
            _ => bail!("Invalid TransferMessageTypes value"),
        }
    }
}
