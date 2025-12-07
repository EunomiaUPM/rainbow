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

use crate::protocols::dsp::facades::FacadeTrait;
use crate::protocols::dsp::orchestrator::protocol::ProtocolOrchestratorTrait;
use crate::protocols::dsp::persistence::NegotiationPersistenceTrait;
use crate::protocols::dsp::protocol_types::{
    NegotiationAckMessageDto, NegotiationAgreementMessageDto, NegotiationEventMessageDto,
    NegotiationOfferInitMessageDto, NegotiationOfferMessageDto, NegotiationProcessMessageWrapper,
    NegotiationRequestInitMessageDto, NegotiationRequestMessageDto, NegotiationTerminationMessageDto,
    NegotiationVerificationMessageDto,
};
use crate::protocols::dsp::validator::traits::validation_dsp_steps::ValidationDspSteps;
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use std::sync::Arc;

pub struct ProtocolOrchestratorService {
    facades: Arc<dyn FacadeTrait>,
    validator: Arc<dyn ValidationDspSteps>,
    persistence_service: Arc<dyn NegotiationPersistenceTrait>,
    _config: Arc<ApplicationGlobalConfig>,
}

impl ProtocolOrchestratorService {
    pub fn new(
        validator: Arc<dyn ValidationDspSteps>,
        persistence_service: Arc<dyn NegotiationPersistenceTrait>,
        facades: Arc<dyn FacadeTrait>,
        _config: Arc<ApplicationGlobalConfig>,
    ) -> ProtocolOrchestratorService {
        ProtocolOrchestratorService { validator, persistence_service, _config, facades }
    }
}

#[async_trait::async_trait]
impl ProtocolOrchestratorTrait for ProtocolOrchestratorService {
    async fn on_get_negotiation(
        &self,
        id: &String,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>> {
        let process = self.persistence_service.fetch_process(id.as_str()).await?;
        let negotiation_process_dto = NegotiationProcessMessageWrapper::try_from(process)?;
        Ok(negotiation_process_dto)
    }

    async fn on_initial_contract_request(
        &self,
        input: &NegotiationProcessMessageWrapper<NegotiationRequestInitMessageDto>,
    ) -> anyhow::Result<(
        NegotiationProcessMessageWrapper<NegotiationAckMessageDto>,
        bool,
    )> {
        let input = Arc::new(input.clone());
        self.validator.on_contract_request_init(&input).await?;

        // persist
        let negotiation = self
            .persistence_service
            .create_process(
                "DSP",
                "INBOUND",
                Some(input.dto.clone().consumer_pid.to_string()),
                Arc::new(input.dto.clone()),
                serde_json::to_value(&input).unwrap(),
            )
            .await?;

        // notify

        let negotiation_process_dto = NegotiationProcessMessageWrapper::try_from(negotiation)?;
        Ok((negotiation_process_dto, false))
    }

    async fn on_consumer_request(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationRequestMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>> {
        let input = Arc::new(input.clone());
        self.validator.on_contract_request(id, &input).await?;

        // persist
        let negotiation = self
            .persistence_service
            .update_process(
                id.as_str(),
                Arc::new(input.dto.clone()),
                serde_json::to_value(&input).unwrap(),
            )
            .await?;

        // notify

        let negotiation_process_dto = NegotiationProcessMessageWrapper::try_from(negotiation)?;
        Ok(negotiation_process_dto)
    }

    async fn on_agreement_verification(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationVerificationMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>> {
        let input = Arc::new(input.clone());
        self.validator.on_contract_agreement_verification(&id, &input).await?;

        // persist
        let negotiation = self
            .persistence_service
            .update_process(
                id.as_str(),
                Arc::new(input.dto.clone()),
                serde_json::to_value(&input).unwrap(),
            )
            .await?;

        // notify

        let negotiation_process_dto = NegotiationProcessMessageWrapper::try_from(negotiation)?;
        Ok(negotiation_process_dto)
    }

    async fn on_initial_provider_offer(
        &self,
        input: &NegotiationProcessMessageWrapper<NegotiationOfferInitMessageDto>,
    ) -> anyhow::Result<(
        NegotiationProcessMessageWrapper<NegotiationAckMessageDto>,
        bool,
    )> {
        todo!()
    }

    async fn on_provider_offer(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationOfferMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>> {
        let input = Arc::new(input.clone());
        self.validator.on_contract_offer(id, &input).await?;

        // persist
        let negotiation = self
            .persistence_service
            .update_process(
                id.as_str(),
                Arc::new(input.dto.clone()),
                serde_json::to_value(&input).unwrap(),
            )
            .await?;

        // notify

        let negotiation_process_dto = NegotiationProcessMessageWrapper::try_from(negotiation)?;
        Ok(negotiation_process_dto)
    }

    async fn on_agreement_reception(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationAgreementMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>> {
        let input = Arc::new(input.clone());
        self.validator.on_contract_agreement(id, &input).await?;

        // persist
        let negotiation = self
            .persistence_service
            .update_process(
                id.as_str(),
                Arc::new(input.dto.clone()),
                serde_json::to_value(&input).unwrap(),
            )
            .await?;

        // notify

        let negotiation_process_dto = NegotiationProcessMessageWrapper::try_from(negotiation)?;
        Ok(negotiation_process_dto)
    }

    async fn on_negotiation_event(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationEventMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>> {
        let input = Arc::new(input.clone());
        self.validator.on_contract_event(id, &input).await?;

        // persist
        let negotiation = self
            .persistence_service
            .update_process(
                id.as_str(),
                Arc::new(input.dto.clone()),
                serde_json::to_value(&input).unwrap(),
            )
            .await?;

        // notify

        let negotiation_process_dto = NegotiationProcessMessageWrapper::try_from(negotiation)?;
        Ok(negotiation_process_dto)
    }

    async fn on_negotiation_termination(
        &self,
        id: &String,
        input: &NegotiationProcessMessageWrapper<NegotiationTerminationMessageDto>,
    ) -> anyhow::Result<NegotiationProcessMessageWrapper<NegotiationAckMessageDto>> {
        let input = Arc::new(input.clone());
        self.validator.on_contract_termination(id, &input).await?;

        // persist
        let negotiation = self
            .persistence_service
            .update_process(
                id.as_str(),
                Arc::new(input.dto.clone()),
                serde_json::to_value(&input).unwrap(),
            )
            .await?;

        // notify

        let negotiation_process_dto = NegotiationProcessMessageWrapper::try_from(negotiation)?;
        Ok(negotiation_process_dto)
    }
}
