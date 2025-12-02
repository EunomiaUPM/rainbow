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

#![allow(unused)]

use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcNegotiationAgreementMessageDto, RpcNegotiationEventMessageDto, RpcNegotiationOfferMessageDto,
    RpcNegotiationRequestMessageDto, RpcNegotiationTerminationMessageDto, RpcNegotiationVerificationMessageDto,
};
use crate::protocols::dsp::validator::traits::validate_payload::ValidatePayload;
use crate::protocols::dsp::validator::traits::validate_state_transition::ValidateStateTransition;
use crate::protocols::dsp::validator::traits::validation_helpers::ValidationHelpers;
use crate::protocols::dsp::validator::traits::validation_rpc_steps::ValidationRpcSteps;
use anyhow::bail;
use std::sync::Arc;

pub struct ValidationRpcStepsService {
    payload_validator: Arc<dyn ValidatePayload>,
    step_transition_validator: Arc<dyn ValidateStateTransition>,
    helpers: Arc<dyn ValidationHelpers>,
}
impl ValidationRpcStepsService {
    pub fn new(
        payload_validator: Arc<dyn ValidatePayload>,
        step_transition_validator: Arc<dyn ValidateStateTransition>,
        helpers: Arc<dyn ValidationHelpers>,
    ) -> Self {
        Self { payload_validator, step_transition_validator, helpers }
    }
}

#[async_trait::async_trait]
impl ValidationRpcSteps for ValidationRpcStepsService {
    async fn negotiation_request_rpc(&self, _input: &RpcNegotiationRequestMessageDto) -> anyhow::Result<()> {
        Ok(())
    }

    async fn negotiation_offer_rpc(&self, _input: &RpcNegotiationOfferMessageDto) -> anyhow::Result<()> {
        Ok(())
    }

    async fn negotiation_agreement_rpc(&self, _input: &RpcNegotiationAgreementMessageDto) -> anyhow::Result<()> {
        Ok(())
    }

    async fn negotiation_agreement_verification_rpc(
        &self,
        _input: &RpcNegotiationVerificationMessageDto,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn negotiation_event_rpc(&self, _input: &RpcNegotiationEventMessageDto) -> anyhow::Result<()> {
        Ok(())
    }

    async fn negotiation_termination_rpc(&self, _input: &RpcNegotiationTerminationMessageDto) -> anyhow::Result<()> {
        Ok(())
    }
}
