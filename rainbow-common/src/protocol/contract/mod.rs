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

use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod cn_consumer_process;
pub mod contract_ack;
pub mod contract_agreement;
pub mod contract_agreement_verification;
pub mod contract_error;
pub mod contract_negotiation_event;
pub mod contract_negotiation_request;
pub mod contract_negotiation_termination;
pub mod contract_odrl;
pub mod contract_offer;
pub mod contract_protocol_trait;
pub mod odrloffer_wrapper;

#[derive(Debug, Serialize, Deserialize)]
pub enum ContractNegotiationState {
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "OFFERED")]
    Offered,
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "AGREED")]
    Agreed,
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "FINALIZED")]
    Finalized,
    #[serde(rename = "TERMINATED")]
    Terminated,
}

impl Display for ContractNegotiationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractNegotiationState::Requested => {
                write!(f, "REQUESTED")
            }
            ContractNegotiationState::Offered => {
                write!(f, "OFFERED")
            }
            ContractNegotiationState::Accepted => {
                write!(f, "ACCEPTED")
            }
            ContractNegotiationState::Agreed => {
                write!(f, "AGREED")
            }
            ContractNegotiationState::Verified => {
                write!(f, "VERIFIED")
            }
            ContractNegotiationState::Finalized => {
                write!(f, "FINALIZED")
            }
            ContractNegotiationState::Terminated => {
                write!(f, "TERMINATED")
            }
        }
    }
}

impl FromStr for ContractNegotiationState {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dspace:REQUESTED" => Ok(Self::Requested),
            "REQUESTED" => Ok(Self::Requested),
            "dspace:OFFERED" => Ok(Self::Offered),
            "OFFERED" => Ok(Self::Offered),
            "dspace:ACCEPTED" => Ok(Self::Accepted),
            "ACCEPTED" => Ok(Self::Accepted),
            "dspace:AGREED" => Ok(Self::Agreed),
            "AGREED" => Ok(Self::Agreed),
            "dspace:VERIFIED" => Ok(Self::Verified),
            "VERIFIED" => Ok(Self::Verified),
            "dspace:FINALIZED" => Ok(Self::Finalized),
            "FINALIZED" => Ok(Self::Finalized),
            "dspace:TERMINATED" => Ok(Self::Terminated),
            "TERMINATED" => Ok(Self::Terminated),
            &_ => Err(anyhow!("Invalid ContractNegotiationState".to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ContractNegotiationMessages {
    #[serde(rename = "ContractRequestMessage")]
    ContractRequestMessage,
    #[serde(rename = "ContractOfferMessage")]
    ContractOfferMessage,
    #[serde(rename = "ContractAgreementMessage")]
    ContractAgreementMessage,
    #[serde(rename = "ContractAgreementVerificationMessage")]
    ContractAgreementVerificationMessage,
    #[serde(rename = "ContractNegotiationEventMessage")]
    ContractNegotiationEventMessage,
    #[serde(rename = "ContractNegotiationTerminationMessage")]
    ContractNegotiationTerminationMessage,
    #[serde(rename = "ContractNegotiation")]
    ContractNegotiationAck,
    #[serde(rename = "ContractNegotiationError")]
    ContractNegotiationError,
}

impl Display for ContractNegotiationMessages {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractNegotiationMessages::ContractRequestMessage => {
                write!(f, "ContractRequestMessage")
            }
            ContractNegotiationMessages::ContractOfferMessage => {
                write!(f, "ContractOfferMessage")
            }
            ContractNegotiationMessages::ContractAgreementMessage => {
                write!(f, "ContractAgreementMessage")
            }
            ContractNegotiationMessages::ContractAgreementVerificationMessage => {
                write!(f, "ContractAgreementVerificationMessage")
            }
            ContractNegotiationMessages::ContractNegotiationEventMessage => {
                write!(f, "ContractNegotiationEventMessage")
            }
            ContractNegotiationMessages::ContractNegotiationTerminationMessage => {
                write!(f, "ContractNegotiationTerminationMessage")
            }
            ContractNegotiationMessages::ContractNegotiationAck => {
                write!(f, "ContractNegotiation")
            }
            ContractNegotiationMessages::ContractNegotiationError => {
                write!(f, "ContractNegotiationError")
            }
        }
    }
}

impl FromStr for ContractNegotiationMessages {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ContractRequestMessage" => Ok(ContractNegotiationMessages::ContractRequestMessage),
            "ContractOfferMessage" => Ok(ContractNegotiationMessages::ContractOfferMessage),
            "ContractAgreementMessage" => Ok(ContractNegotiationMessages::ContractAgreementMessage),
            "ContractAgreementVerificationMessage" => Ok(ContractNegotiationMessages::ContractAgreementVerificationMessage),
            "ContractNegotiationEventMessage" => Ok(ContractNegotiationMessages::ContractNegotiationEventMessage),
            "ContractNegotiationTerminationMessage" => Ok(ContractNegotiationMessages::ContractNegotiationTerminationMessage),
            _ => bail!("Contract negotiation message not recognized")
        }
    }
}
