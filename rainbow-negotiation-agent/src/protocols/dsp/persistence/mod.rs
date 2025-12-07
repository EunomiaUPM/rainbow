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

pub(crate) mod persistence_protocol;
pub(crate) mod persistence_rpc;

use crate::entities::agreement::NegotiationAgentAgreementsTrait;
use crate::entities::negotiation_message::NegotiationAgentMessagesTrait;
use crate::entities::negotiation_process::{NegotiationAgentProcessesTrait, NegotiationProcessDto};
use crate::entities::offer::NegotiationAgentOffersTrait;
use crate::protocols::dsp::protocol_types::NegotiationProcessMessageTrait;
use std::sync::Arc;
use urn::Urn;

#[async_trait::async_trait]
#[allow(unused)]
pub trait NegotiationPersistenceTrait: Send + Sync {
    async fn get_negotiation_process_service(&self) -> anyhow::Result<Arc<dyn NegotiationAgentProcessesTrait>>;
    async fn get_negotiation_message_service(&self) -> anyhow::Result<Arc<dyn NegotiationAgentMessagesTrait>>;
    async fn get_negotiation_offer_service(&self) -> anyhow::Result<Arc<dyn NegotiationAgentOffersTrait>>;
    async fn get_negotiation_agreement_service(&self) -> anyhow::Result<Arc<dyn NegotiationAgentAgreementsTrait>>;
    async fn fetch_process(&self, id: &str) -> anyhow::Result<NegotiationProcessDto>;
    async fn create_process(
        &self,
        protocol: &str,
        direction: &str,
        peer_address: Option<String>,
        payload_dto: Arc<dyn NegotiationProcessMessageTrait>,
        payload_value: serde_json::Value,
    ) -> anyhow::Result<NegotiationProcessDto>;
    async fn update_process(
        &self,
        id: &str,
        payload_dto: Arc<dyn NegotiationProcessMessageTrait>,
        payload_value: serde_json::Value,
    ) -> anyhow::Result<NegotiationProcessDto>;
}
