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

//! The side this connector plays in a negotiation, and which side sends each message.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use ymir::errors::Errors;

use crate::protocols::dsp::entities::message_types::NegotiationMessageType;

/// Our side of a negotiation, stored on the process as `Provider` or `Consumer`.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum NegotiationRole {
    Provider,
    Consumer,
}

impl NegotiationRole {
    /// Who sends `message` (DSP 8.2, 8.3); a termination comes from either side.
    pub fn sender_of(message: NegotiationMessageType) -> Option<Self> {
        use NegotiationMessageType::*;
        match message {
            ContractRequestMessage | EventAccepted | ContractAgreementVerificationMessage => {
                Some(Self::Consumer)
            }
            ContractOfferMessage | ContractAgreementMessage | EventFinalized => {
                Some(Self::Provider)
            }
            ContractNegotiationTerminationMessage
            | ContractNegotiation
            | ContractNegotiationError => None,
        }
    }

    pub fn other(self) -> Self {
        match self {
            Self::Provider => Self::Consumer,
            Self::Consumer => Self::Provider,
        }
    }
}

impl FromStr for NegotiationRole {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "provider" => Ok(Self::Provider),
            "consumer" => Ok(Self::Consumer),
            _ => Err(Errors::parse(
                format!("negotiation role not recognized: {s}"),
                None,
            )),
        }
    }
}

impl fmt::Display for NegotiationRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Provider => "Provider",
            Self::Consumer => "Consumer",
        })
    }
}
