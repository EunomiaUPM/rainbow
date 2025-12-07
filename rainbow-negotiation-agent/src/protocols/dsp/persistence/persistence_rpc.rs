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

use crate::entities::agreement::NegotiationAgentAgreementsTrait;
use crate::entities::negotiation_message::NegotiationAgentMessagesTrait;
use crate::entities::negotiation_process::{NegotiationAgentProcessesTrait, NegotiationProcessDto};
use crate::entities::offer::NegotiationAgentOffersTrait;
use crate::protocols::dsp::persistence::NegotiationPersistenceTrait;
use crate::protocols::dsp::protocol_types::NegotiationProcessMessageTrait;
use serde_json::Value;
use std::sync::Arc;
use urn::Urn;

pub struct NegotiationPersistenceForRpcService {
    pub negotiation_process_service: Arc<dyn NegotiationAgentProcessesTrait>,
    pub negotiation_messages_service: Arc<dyn NegotiationAgentMessagesTrait>,
    pub offer_service: Arc<dyn NegotiationAgentOffersTrait>,
    pub agreement_service: Arc<dyn NegotiationAgentAgreementsTrait>,
}

impl NegotiationPersistenceForRpcService {
    pub fn new(
        negotiation_process_service: Arc<dyn NegotiationAgentProcessesTrait>,
        negotiation_messages_service: Arc<dyn NegotiationAgentMessagesTrait>,
        offer_service: Arc<dyn NegotiationAgentOffersTrait>,
        agreement_service: Arc<dyn NegotiationAgentAgreementsTrait>,
    ) -> Self {
        Self { negotiation_process_service, negotiation_messages_service, offer_service, agreement_service }
    }
}

#[async_trait::async_trait]
impl NegotiationPersistenceTrait for NegotiationPersistenceForRpcService {
    async fn get_negotiation_process_service(&self) -> anyhow::Result<Arc<dyn NegotiationAgentProcessesTrait>> {
        todo!()
    }

    async fn get_negotiation_message_service(&self) -> anyhow::Result<Arc<dyn NegotiationAgentMessagesTrait>> {
        todo!()
    }

    async fn get_negotiation_offer_service(&self) -> anyhow::Result<Arc<dyn NegotiationAgentOffersTrait>> {
        todo!()
    }

    async fn get_negotiation_agreement_service(&self) -> anyhow::Result<Arc<dyn NegotiationAgentAgreementsTrait>> {
        todo!()
    }

    async fn fetch_process(&self, id: &str) -> anyhow::Result<NegotiationProcessDto> {
        todo!()
    }

    async fn create_process(
        &self,
        protocol: &str,
        direction: &str,
        peer_address: Option<String>,
        payload_dto: Arc<dyn NegotiationProcessMessageTrait>,
        payload_value: Value,
    ) -> anyhow::Result<NegotiationProcessDto> {
        todo!()
    }

    async fn update_process(
        &self,
        id: &str,
        payload_dto: Arc<dyn NegotiationProcessMessageTrait>,
        payload_value: Value,
    ) -> anyhow::Result<NegotiationProcessDto> {
        todo!()
    }
}
