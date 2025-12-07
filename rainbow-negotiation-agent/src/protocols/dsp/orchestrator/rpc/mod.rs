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
use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcNegotiationAgreementMessageDto, RpcNegotiationEventAcceptedMessageDto, RpcNegotiationEventFinalizedMessageDto,
    RpcNegotiationOfferInitMessageDto, RpcNegotiationOfferMessageDto, RpcNegotiationRequestInitMessageDto,
    RpcNegotiationRequestMessageDto, RpcNegotiationTerminationMessageDto, RpcNegotiationVerificationMessageDto,
};

pub(crate) mod rpc;
pub(crate) mod types;

#[async_trait::async_trait]
pub trait RPCOrchestratorTrait: Send + Sync + 'static {
    async fn negotiation_request_init_rpc(&self, input: &RpcNegotiationRequestInitMessageDto) -> anyhow::Result<()>;
    async fn negotiation_request_rpc(&self, input: &RpcNegotiationRequestMessageDto) -> anyhow::Result<()>;
    async fn negotiation_offer_init_rpc(&self, input: &RpcNegotiationOfferInitMessageDto) -> anyhow::Result<()>;
    async fn negotiation_offer_rpc(&self, input: &RpcNegotiationOfferMessageDto) -> anyhow::Result<()>;
    async fn negotiation_agreement_rpc(&self, input: &RpcNegotiationAgreementMessageDto) -> anyhow::Result<()>;
    async fn negotiation_agreement_verification_rpc(
        &self,
        input: &RpcNegotiationVerificationMessageDto,
    ) -> anyhow::Result<()>;
    async fn negotiation_event_accepted_rpc(&self, input: &RpcNegotiationEventAcceptedMessageDto)
    -> anyhow::Result<()>;
    async fn negotiation_event_finalized_rpc(
        &self,
        input: &RpcNegotiationEventFinalizedMessageDto,
    ) -> anyhow::Result<()>;
    async fn negotiation_termination_rpc(&self, input: &RpcNegotiationTerminationMessageDto) -> anyhow::Result<()>;
}
