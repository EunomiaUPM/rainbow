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

use rainbow_common::protocol::contract::contract_odrl::OdrlAgreement;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_consumer::repo::NewContractNegotiationProcess;
use rainbow_db::contracts_consumer::repo::{EditAgreement, EditContractNegotiationMessage, EditContractNegotiationOffer, EditContractNegotiationProcess, NewAgreement, NewContractNegotiationMessage, NewContractNegotiationOffer};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewContractNegotiationRequest {
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
}

impl Into<NewContractNegotiationProcess> for NewContractNegotiationRequest {
    fn into(self) -> NewContractNegotiationProcess {
        NewContractNegotiationProcess {
            provider_id: self.provider_id.map(|id| get_urn_from_string(&id).unwrap()),
            consumer_id: None,
            associated_provider: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditContractNegotiationRequest {}

impl Into<EditContractNegotiationProcess> for EditContractNegotiationRequest {
    fn into(self) -> EditContractNegotiationProcess {
        EditContractNegotiationProcess {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewContractNegotiationMessageRequest {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "subtype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "content")] // TODO
    pub content: serde_json::Value,
}

impl Into<NewContractNegotiationMessage> for NewContractNegotiationMessageRequest {
    fn into(self) -> NewContractNegotiationMessage {
        NewContractNegotiationMessage {
            _type: self._type,
            subtype: self.subtype,
            from: self.from,
            to: self.to,
            content: self.content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditContractNegotiationMessageRequest {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "subtype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "content")] // TODO
    pub content: serde_json::Value,
}

impl Into<EditContractNegotiationMessage> for EditContractNegotiationMessageRequest {
    fn into(self) -> EditContractNegotiationMessage {
        EditContractNegotiationMessage {
            _type: Option::from(self._type),
            subtype: self.subtype,
            from: Option::from(self.from),
            to: Option::from(self.to),
            content: Option::from(self.content),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewContractNegotiationOfferRequest {
    #[serde(rename = "offer")] // TODO
    pub offer_id: Urn,
    pub offer_content: serde_json::Value,
}

impl Into<NewContractNegotiationOffer> for NewContractNegotiationOfferRequest {
    fn into(self) -> NewContractNegotiationOffer {
        NewContractNegotiationOffer {
            offer_id: Option::from(self.offer_id),
            offer_content: self.offer_content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditContractNegotiationOfferRequest {}

impl Into<EditContractNegotiationOffer> for EditContractNegotiationOfferRequest {
    fn into(self) -> EditContractNegotiationOffer {
        EditContractNegotiationOffer {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewAgreementRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "providerParticipantId")]
    pub provider_participant_id: String,
    #[serde(rename = "agreement")]
    pub agreement_content: OdrlAgreement,
}

impl Into<NewAgreement> for NewAgreementRequest {
    fn into(self) -> NewAgreement {
        NewAgreement {
            agreement_id: None,
            consumer_participant_id: self.consumer_participant_id,
            provider_participant_id: self.provider_participant_id,
            agreement_content: self.agreement_content,
            active: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditAgreementRequest {
    #[serde(rename = "agreementStatusActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl Into<EditAgreement> for EditAgreementRequest {
    fn into(self) -> EditAgreement {
        EditAgreement {
            active: self.active
        }
    }
}