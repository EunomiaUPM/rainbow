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

use axum::async_trait;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::ContractNegotiationEventMessage;
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use urn::Urn;

pub mod ds_protocol;
pub mod ds_protocol_errors;
pub mod ds_protocol_types;

#[mockall::automock]
#[async_trait]
pub trait DSProtocolContractNegotiationProviderTrait: Send + Sync {
    async fn get_negotiation(&self, provider_pid: Urn) -> anyhow::Result<ContractAckMessage>;
    async fn post_request(
        &self,
        input: ContractRequestMessage,
        token: String,
        client_type: String,
    ) -> anyhow::Result<ContractAckMessage>;
    async fn post_provider_request(
        &self,
        provider_pid: Urn,
        input: ContractRequestMessage,
        token: String,
    ) -> anyhow::Result<ContractAckMessage>;
    async fn post_provider_events(
        &self,
        provider_pid: Urn,
        input: ContractNegotiationEventMessage,
        token: String,
    ) -> anyhow::Result<ContractAckMessage>;
    async fn post_provider_agreement_verification(
        &self,
        provider_id: Urn,
        input: ContractAgreementVerificationMessage,
        token: String,
    ) -> anyhow::Result<ContractAckMessage>;

    async fn post_provider_termination(
        &self,
        provider_id: Urn,
        input: ContractTerminationMessage,
        token: String,
    ) -> anyhow::Result<ContractAckMessage>;
}
