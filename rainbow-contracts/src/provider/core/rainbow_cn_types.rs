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

use rainbow_common::protocol::contract::ContractNegotiationState;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_provider::repo::{EditAgreement, EditContractNegotiationMessage, EditContractNegotiationOffer, EditContractNegotiationProcess, EditParticipant, NewAgreement, NewContractNegotiationMessage, NewContractNegotiationOffer, NewContractNegotiationProcess, NewParticipant};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewContractNegotiationRequest {
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<String>,
    #[serde(rename = "dspace:state")]
    pub state: ContractNegotiationState,
}

impl Into<NewContractNegotiationProcess> for NewContractNegotiationRequest {
    fn into(self) -> NewContractNegotiationProcess {
        NewContractNegotiationProcess {
            provider_id: self.provider_id.map(|id| get_urn_from_string(&id).unwrap()),
            consumer_id: self.consumer_id.map(|id| get_urn_from_string(&id).unwrap()),
            state: self.state,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditContractNegotiationRequest {
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<String>,
    #[serde(rename = "dspace:state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ContractNegotiationState>,
}

impl Into<EditContractNegotiationProcess> for EditContractNegotiationRequest {
    fn into(self) -> EditContractNegotiationProcess {
        EditContractNegotiationProcess {
            provider_id: self.provider_id.map(|id| get_urn_from_string(&id).unwrap()),
            consumer_id: self.consumer_id.map(|id| get_urn_from_string(&id).unwrap()),
            state: self.state,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewContractNegotiationMessageRequest {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:from")]
    pub from: String,
    #[serde(rename = "dspace:to")]
    pub to: String,
    #[serde(rename = "dspace:content")] // TODO
    pub content: serde_json::Value,
}

impl Into<NewContractNegotiationMessage> for NewContractNegotiationMessageRequest {
    fn into(self) -> NewContractNegotiationMessage {
        NewContractNegotiationMessage {
            _type: self._type,
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
    #[serde(rename = "dspace:from")]
    pub from: String,
    #[serde(rename = "dspace:to")]
    pub to: String,
    #[serde(rename = "dspace:content")] // TODO
    pub content: serde_json::Value,
}

impl Into<EditContractNegotiationMessage> for EditContractNegotiationMessageRequest {
    fn into(self) -> EditContractNegotiationMessage {
        EditContractNegotiationMessage {
            _type: self._type,
            from: self.from,
            to: self.to,
            content: self.content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewContractNegotiationOfferRequest {
    #[serde(rename = "odrl:offer")] // TODO
    pub offer_content: serde_json::Value,
}

impl Into<NewContractNegotiationOffer> for NewContractNegotiationOfferRequest {
    fn into(self) -> NewContractNegotiationOffer {
        NewContractNegotiationOffer {
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
    #[serde(rename = "dspace:consumerParticipantId")]
    pub consumer_participant_id: Urn,
    #[serde(rename = "dspace:providerParticipantId")]
    pub provider_participant_id: Urn,
    #[serde(rename = "odrl:agreement")]
    pub agreement_content: serde_json::Value,
}

impl Into<NewAgreement> for NewAgreementRequest {
    fn into(self) -> NewAgreement {
        NewAgreement {
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
    #[serde(rename = "dspace:agreementStatusActive")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewParticipantRequest {
    #[serde(rename = "dspace:participantType")]
    pub _type: String,
    pub extra_fields: serde_json::Value,
}

impl Into<NewParticipant> for NewParticipantRequest {
    fn into(self) -> NewParticipant {
        NewParticipant {
            identity_token: None,
            _type: self._type,
            extra_fields: self.extra_fields,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditParticipantRequest {
    pub extra_fields: Option<serde_json::Value>,
}

impl Into<EditParticipant> for EditParticipantRequest {
    fn into(self) -> EditParticipant {
        EditParticipant {
            identity_token: None,
            extra_fields: self.extra_fields,
        }
    }
}

