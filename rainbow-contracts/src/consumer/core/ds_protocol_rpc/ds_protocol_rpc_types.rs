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

use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_odrl::ContractRequestMessageOfferTypes;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupRequestRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "offer")]
    pub odrl_offer: ContractRequestMessageOfferTypes,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupRequestResponse {
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "offer")]
    pub odrl_offer: ContractRequestMessageOfferTypes,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupAcceptanceRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupAcceptanceResponse {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupVerificationRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SetupVerificationResponse {
    #[serde(rename = "consumerPid")]
    pub consumer_pid: Urn,
    #[serde(rename = "providerPid")]
    pub provider_pid: Urn,
    pub message: ContractAckMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SetupTerminationRequest {
    #[serde(rename = "providerAddress")]
    pub provider_address: String,
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