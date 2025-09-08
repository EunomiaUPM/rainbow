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

use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_odrl::{OdrlAgreement, OdrlMessageOffer};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetupOfferRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "offer")]
    pub odrl_offer: OdrlMessageOffer,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupOfferResponse {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "offer")]
    pub odrl_offer: OdrlMessageOffer,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupAgreementRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    // #[serde(rename = "agreement")]
    // pub odrl_agreement: OdrlMessageAgreement,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupAgreementResponse {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    #[serde(rename = "agreement")]
    pub odrl_agreement: OdrlAgreement,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupFinalizationRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetupFinalizationResponse {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupTerminationRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetupTerminationResponse {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}