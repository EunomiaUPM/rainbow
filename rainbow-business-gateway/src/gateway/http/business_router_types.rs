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

use rainbow_common::protocol::contract::contract_odrl::ContractRequestMessageOfferTypes;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
pub struct RainbowBusinessNegotiationRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    pub offer: ContractRequestMessageOfferTypes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RainbowBusinessTerminationRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RainbowBusinessAcceptanceRequest {
    #[serde(rename = "consumerParticipantId")]
    pub consumer_participant_id: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
}

