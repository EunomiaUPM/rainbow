/*
 *
 * * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 * *
 * * This program is free software: you can redistribute it and/or modify
 * * it under the terms of the GNU General Public License as published by
 * * the Free Software Foundation, either version 3 of the License, or
 * * (at your option) any later version.
 * *
 * * This program is distributed in the hope that it will be useful,
 * * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * * GNU General Public License for more details.
 * *
 * * You should have received a copy of the GNU General Public License
 * * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

pub(crate) mod protocol;

use crate::protocols::dsp::protocol_types::{
    NegotiationAckMessageDto, NegotiationAgreementMessageDto, NegotiationEventMessageDto,
    NegotiationOfferInitMessageDto, NegotiationOfferMessageDto, NegotiationProcessMessageWrapper,
    NegotiationRequestInitMessageDto, NegotiationRequestMessageDto, NegotiationTerminationMessageDto,
    NegotiationVerificationMessageDto,
};

#[async_trait::async_trait]
pub trait ProtocolOrchestratorTrait: Send + Sync + 'static {
    // ============================================================================================
    //  SHARED OPERATIONS (DSP 8.2.1 / 8.3.2)
    // ============================================================================================

    /// Retrieves the current state of a negotiation process.
    /// Maps to GET /negotiations/:id
    async fn on_get_negotiation(
        &self,
        id: &String,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>>;

    // ============================================================================================
    //  PROVIDER ROLE OPERATIONS (Handling requests from a Consumer) - DSP 8.2
    // ============================================================================================

    /// Handles the initiation of a negotiation by a Consumer.
    /// Maps to POST /negotiations/request
    /// Returns: (Response DTO, bool) where bool is true if resource already existed (idempotency).
    async fn on_initial_contract_request(
        &self,
        input: &NegotiationProcessMessageWrapper<NegotiationRequestInitMessageDto>,
    ) -> anyhow::Result<(
        NegotiationProcessMessageWrapper<NegotiationAckMessageDto>,
        bool,
    )>;

    /// Handles a counter-offer or update request from a Consumer on an existing negotiation.
    /// Maps to POST /negotiations/:providerPid/request
    async fn on_consumer_request(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationRequestMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>>;

    /// Handles the verification of an Agreement signed by the Consumer.
    /// Maps to POST /negotiations/:providerPid/agreement/verification
    async fn on_agreement_verification(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationVerificationMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>>;

    // ============================================================================================
    //  CONSUMER ROLE OPERATIONS (Handling callbacks from a Provider) - DSP 8.3
    // ============================================================================================

    /// Handles the initiation of a negotiation via an Offer from a Provider.
    /// Maps to POST /negotiations/offers
    /// Returns: (Response DTO, bool) where bool is true if resource already existed.
    async fn on_initial_provider_offer(
        &self,
        input: &NegotiationProcessMessageWrapper<NegotiationOfferInitMessageDto>,
    ) -> anyhow::Result<(
        NegotiationProcessMessageWrapper<NegotiationAckMessageDto>,
        bool,
    )>;

    /// Handles a counter-offer from a Provider on an existing negotiation.
    /// Maps to POST /negotiations/:consumerPid/offers
    async fn on_provider_offer(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationOfferMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>>;

    /// Handles the reception of a finalized Agreement from the Provider.
    /// Maps to POST /negotiations/:consumerPid/agreement
    async fn on_agreement_reception(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationAgreementMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>>;

    // ============================================================================================
    //  STATE MACHINE EVENTS & TERMINATION (Shared)
    // ============================================================================================

    /// Handles state transition events (e.g., ACCEPTED, FINALIZED).
    /// Maps to POST .../events
    async fn on_negotiation_event(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationEventMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>>;

    /// Handles the termination of the negotiation process.
    /// Maps to POST .../termination
    async fn on_negotiation_termination(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationTerminationMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>>;
}
