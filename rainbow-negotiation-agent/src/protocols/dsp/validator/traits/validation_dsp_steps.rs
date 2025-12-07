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
use crate::protocols::dsp::protocol_types::{
    NegotiationAgreementMessageDto, NegotiationEventMessageDto, NegotiationOfferMessageDto,
    NegotiationProcessMessageWrapper, NegotiationRequestMessageDto, NegotiationTerminationMessageDto,
    NegotiationVerificationMessageDto,
};

#[async_trait::async_trait]
pub trait ValidationDspSteps: Send + Sync + 'static {
    async fn on_contract_request(
        &self,
        uri_id: Option<&String>,
        input: &NegotiationProcessMessageWrapper<NegotiationRequestMessageDto>,
    ) -> anyhow::Result<()>;
    async fn on_contract_offer(
        &self,
        uri_id: Option<&String>,
        input: &NegotiationProcessMessageWrapper<NegotiationOfferMessageDto>,
    ) -> anyhow::Result<()>;
    async fn on_contract_agreement(
        &self,
        uri_id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationAgreementMessageDto>,
    ) -> anyhow::Result<()>;
    async fn on_contract_agreement_verification(
        &self,
        uri_id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationVerificationMessageDto>,
    ) -> anyhow::Result<()>;
    async fn on_contract_event(
        &self,
        uri_id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationEventMessageDto>,
    ) -> anyhow::Result<()>;
    async fn on_contract_termination(
        &self,
        uri_id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationTerminationMessageDto>,
    ) -> anyhow::Result<()>;
}
