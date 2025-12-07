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
use crate::protocols::dsp::validator::traits::validate_payload::ValidatePayload;
use crate::protocols::dsp::validator::traits::validate_state_transition::ValidateStateTransition;
use crate::protocols::dsp::validator::traits::validation_dsp_steps::ValidationDspSteps;
use crate::protocols::dsp::validator::traits::validation_helpers::ValidationHelpers;
use std::sync::Arc;

pub struct ValidationDspStepsService {
    payload_validator: Arc<dyn ValidatePayload>,
    step_transition_validator: Arc<dyn ValidateStateTransition>,
    helpers: Arc<dyn ValidationHelpers>,
}
impl ValidationDspStepsService {
    pub fn new(
        payload_validator: Arc<dyn ValidatePayload>,
        step_transition_validator: Arc<dyn ValidateStateTransition>,
        helpers: Arc<dyn ValidationHelpers>,
    ) -> Self {
        Self { payload_validator, step_transition_validator, helpers }
    }
}

#[async_trait::async_trait]
impl ValidationDspSteps for ValidationDspStepsService {
    async fn on_contract_request(
        &self,
        _uri_id: Option<&String>,
        _input: &NegotiationProcessMessageWrapper<NegotiationRequestMessageDto>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_contract_offer(
        &self,
        _uri_id: Option<&String>,
        _input: &NegotiationProcessMessageWrapper<NegotiationOfferMessageDto>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_contract_agreement(
        &self,
        _uri_id: &String,
        _input: &NegotiationProcessMessageWrapper<NegotiationAgreementMessageDto>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_contract_agreement_verification(
        &self,
        _uri_id: &String,
        _input: &NegotiationProcessMessageWrapper<NegotiationVerificationMessageDto>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_contract_event(
        &self,
        _uri_id: &String,
        _input: &NegotiationProcessMessageWrapper<NegotiationEventMessageDto>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_contract_termination(
        &self,
        _uri_id: &String,
        _input: &NegotiationProcessMessageWrapper<NegotiationTerminationMessageDto>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
