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

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod contract_ack;
pub mod contract_agreement;
pub mod contract_agreement_verification;
pub mod contract_error;
pub mod contract_negotiation_event;
pub mod contract_negotiation_request;
pub mod contract_negotiation_termination;
pub mod contract_odrl;
pub mod contract_offer;

static CONTEXT: &str = "https://w3id.org/dspace/2025/1/context.jsonld";

#[derive(Debug, Serialize, Deserialize)]
pub enum ContractNegotiationState {
    #[serde(rename = "dspace:REQUESTED")]
    Requested,
    #[serde(rename = "dspace:OFFERED")]
    Offered,
    #[serde(rename = "dspace:ACCEPTED")]
    Accepted,
    #[serde(rename = "dspace:AGREED")]
    Agreed,
    #[serde(rename = "dspace:VERIFIED")]
    Verified,
    #[serde(rename = "dspace:FINALIZED")]
    Finalized,
    #[serde(rename = "dspace:TERMINATED")]
    Terminated,
}

impl Display for ContractNegotiationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractNegotiationState::Requested => {
                write!(f, "dspace:REQUESTED")
            }
            ContractNegotiationState::Offered => {
                write!(f, "dspace:OFFERED")
            }
            ContractNegotiationState::Accepted => {
                write!(f, "dspace:ACCEPTED")
            }
            ContractNegotiationState::Agreed => {
                write!(f, "dspace:AGREED")
            }
            ContractNegotiationState::Verified => {
                write!(f, "dspace:VERIFIED")
            }
            ContractNegotiationState::Finalized => {
                write!(f, "dspace:FINALIZED")
            }
            ContractNegotiationState::Terminated => {
                write!(f, "dspace:TERMINATED")
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

#[derive(Debug, Serialize, Deserialize)]
pub enum ContractNegotiationMessages {
    #[serde(rename = "dspace:ContractRequestMessage")]
    ContractRequestMessage,
    #[serde(rename = "dspace:ContractOfferMessage")]
    ContractOfferMessage,
    #[serde(rename = "dspace:ContractAgreementMessage")]
    ContractAgreementMessage,
    #[serde(rename = "dspace:ContractAgreementVerificationMessage")]
    ContractAgreementVerificationMessage,
    #[serde(rename = "dspace:ContractNegotiationEventMessage")]
    ContractNegotiationEventMessage,
    #[serde(rename = "dspace:ContractNegotiationTerminationMessage")]
    ContractNegotiationTerminationMessage,
    #[serde(rename = "dspace:ContractNegotiation")]
    ContractNegotiationAck,
    #[serde(rename = "dspace:ContractNegotiationError")]
    ContractNegotiationError,
}

impl Display for ContractNegotiationMessages {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractNegotiationMessages::ContractRequestMessage => {
                write!(f, "dspace:ContractRequestMessage")
            }
            ContractNegotiationMessages::ContractOfferMessage => {
                write!(f, "dspace:ContractOfferMessage")
            }
            ContractNegotiationMessages::ContractAgreementMessage => {
                write!(f, "dspace:ContractAgreementMessage")
            }
            ContractNegotiationMessages::ContractAgreementVerificationMessage => {
                write!(f, "dspace:ContractAgreementVerificationMessage")
            }
            ContractNegotiationMessages::ContractNegotiationEventMessage => {
                write!(f, "dspace:ContractNegotiationEventMessage")
            }
            ContractNegotiationMessages::ContractNegotiationTerminationMessage => {
                write!(f, "dspace:ContractNegotiationTerminationMessage")
            }
            ContractNegotiationMessages::ContractNegotiationAck => {
                write!(f, "dspace:ContractNegotiation")
            }
            ContractNegotiationMessages::ContractNegotiationError => {
                write!(f, "dspace:ContractNegotiationError")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextField {
    Single(String),
    Multiple(Vec<String>),
}

impl Display for ContextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(CONTEXT)
    }
}

impl Default for ContextField {
    fn default() -> Self {
        ContextField::Single(
            CONTEXT.to_string()
        )
    }
}

impl CNValidate for ContextField {
    fn validate(&self) -> anyhow::Result<()> {
        match self {
            ContextField::Single(s) => {
                if s == CONTEXT {
                    Ok(())
                } else {
                    Err(anyhow!("Invalid @context value"))
                }
            }
            ContextField::Multiple(v) => {
                if v.iter().any(|s| s == CONTEXT) {
                    Ok(())
                } else {
                    Err(anyhow!("Invalid @context value"))
                }
            }
        }
    }
}

pub trait CNValidate {
    fn validate(&self) -> anyhow::Result<()>;
}