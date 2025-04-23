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

use axum::async_trait;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement::ContractAgreementMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::ContractNegotiationEventMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_offer::ContractOfferMessage;
use urn::Urn;

pub mod ds_protocol;
pub mod ds_protocol_errors;

#[mockall::automock]
#[async_trait]
pub trait DSProtocolContractNegotiationConsumerTrait: Send + Sync {
    async fn post_offers(&self, input: ContractOfferMessage) -> anyhow::Result<ContractAckMessage>;

    async fn post_consumer_offers(
        &self,
        consumer_pid: Urn,
        input: ContractOfferMessage,
    ) -> anyhow::Result<ContractAckMessage>;

    async fn post_agreement(
        &self,
        consumer_pid: Urn,
        input: ContractAgreementMessage,
    ) -> anyhow::Result<ContractAckMessage>;

    async fn post_events(
        &self,
        consumer_pid: Urn,
        input: ContractNegotiationEventMessage,
    ) -> anyhow::Result<ContractAckMessage>;

    async fn post_termination(
        &self,
        consumer_pid: Urn,
        input: ContractTerminationMessage,
    ) -> anyhow::Result<ContractAckMessage>;
}
