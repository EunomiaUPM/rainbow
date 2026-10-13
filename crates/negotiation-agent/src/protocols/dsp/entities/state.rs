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

//! The Contract Negotiation states (DSP 8.1), as they travel on the wire.

use common::validation::StateMachine;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use ymir::errors::Errors;

use crate::protocols::dsp::entities::message_types::NegotiationMessageType;

/// DSP contract negotiation states.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum NegotiationState {
    REQUESTED,
    OFFERED,
    ACCEPTED,
    AGREED,
    VERIFIED,
    FINALIZED,
    TERMINATED,
}

impl FromStr for NegotiationState {
    type Err = Errors;

    /// Accepts the prefixed form too (`dspace:OFFERED`), as a peer may write it.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.rsplit([':', '/']).next().unwrap_or(s) {
            "REQUESTED" => Ok(Self::REQUESTED),
            "OFFERED" => Ok(Self::OFFERED),
            "ACCEPTED" => Ok(Self::ACCEPTED),
            "AGREED" => Ok(Self::AGREED),
            "VERIFIED" => Ok(Self::VERIFIED),
            "FINALIZED" => Ok(Self::FINALIZED),
            "TERMINATED" => Ok(Self::TERMINATED),
            _ => Err(Errors::parse(
                format!("negotiation state not recognized: {s}"),
                None,
            )),
        }
    }
}

impl fmt::Display for NegotiationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            Self::REQUESTED => "REQUESTED",
            Self::OFFERED => "OFFERED",
            Self::ACCEPTED => "ACCEPTED",
            Self::AGREED => "AGREED",
            Self::VERIFIED => "VERIFIED",
            Self::FINALIZED => "FINALIZED",
            Self::TERMINATED => "TERMINATED",
        };
        f.write_str(state)
    }
}

impl NegotiationState {
    /// DSP 8.1: the states each message may arrive in, sent or received.
    pub const MACHINE: StateMachine<Self, NegotiationMessageType> = StateMachine::new(&[
        (
            NegotiationMessageType::ContractRequestMessage,
            &[None, Some(Self::OFFERED)],
        ),
        (
            NegotiationMessageType::ContractOfferMessage,
            &[None, Some(Self::REQUESTED)],
        ),
        (
            NegotiationMessageType::EventAccepted,
            &[Some(Self::OFFERED)],
        ),
        (
            NegotiationMessageType::ContractAgreementMessage,
            &[Some(Self::REQUESTED), Some(Self::ACCEPTED)],
        ),
        (
            NegotiationMessageType::ContractAgreementVerificationMessage,
            &[Some(Self::AGREED)],
        ),
        (
            NegotiationMessageType::EventFinalized,
            &[Some(Self::VERIFIED)],
        ),
        (
            NegotiationMessageType::ContractNegotiationTerminationMessage,
            &[
                Some(Self::REQUESTED),
                Some(Self::OFFERED),
                Some(Self::ACCEPTED),
                Some(Self::AGREED),
                Some(Self::VERIFIED),
            ],
        ),
    ]);

    /// The state a message leaves the process in; none for the ACK and the error.
    pub fn reached_by(message: NegotiationMessageType) -> Option<Self> {
        use NegotiationMessageType::*;
        match message {
            ContractRequestMessage => Some(Self::REQUESTED),
            ContractOfferMessage => Some(Self::OFFERED),
            EventAccepted => Some(Self::ACCEPTED),
            ContractAgreementMessage => Some(Self::AGREED),
            ContractAgreementVerificationMessage => Some(Self::VERIFIED),
            EventFinalized => Some(Self::FINALIZED),
            ContractNegotiationTerminationMessage => Some(Self::TERMINATED),
            ContractNegotiation | ContractNegotiationError => None,
        }
    }
}
