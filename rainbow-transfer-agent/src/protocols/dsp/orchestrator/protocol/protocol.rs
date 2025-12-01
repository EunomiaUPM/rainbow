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

use crate::protocols::dsp::orchestrator::protocol::ProtocolOrchestratorTrait;
use crate::protocols::dsp::persistence::TransferPersistenceTrait;
use crate::protocols::dsp::protocol_types::{
    TransferCompletionMessageDto, TransferProcessAckDto, TransferProcessMessageTrait, TransferProcessMessageWrapper,
    TransferRequestMessageDto, TransferStartMessageDto, TransferSuspensionMessageDto, TransferTerminationMessageDto,
};
use std::str::FromStr;

use crate::protocols::dsp::facades::FacadeTrait;
use crate::protocols::dsp::validator::traits::validation_dsp_steps::ValidationDspSteps;
use anyhow::anyhow;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::dcat_formats::DctFormats;
use std::sync::Arc;
use urn::Urn;

pub struct ProtocolOrchestratorService {
    facades: Arc<dyn FacadeTrait>,
    validator: Arc<dyn ValidationDspSteps>,
    pub persistence_service: Arc<dyn TransferPersistenceTrait>,
    pub _config: Arc<ApplicationProviderConfig>,
}

impl ProtocolOrchestratorService {
    pub fn new(
        validator: Arc<dyn ValidationDspSteps>,
        persistence_service: Arc<dyn TransferPersistenceTrait>,
        facades: Arc<dyn FacadeTrait>,
        _config: Arc<ApplicationProviderConfig>,
    ) -> ProtocolOrchestratorService {
        ProtocolOrchestratorService { validator, persistence_service, _config, facades }
    }
}

#[async_trait::async_trait]
impl ProtocolOrchestratorTrait for ProtocolOrchestratorService {
    async fn on_get_transfer_process(
        &self,
        id: &String,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        let transfer_process = self.persistence_service.fetch_process(id.as_str()).await?;
        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok(transfer_process_dto)
    }

    async fn on_transfer_request(
        &self,
        input: &TransferProcessMessageWrapper<TransferRequestMessageDto>,
    ) -> anyhow::Result<(TransferProcessMessageWrapper<TransferProcessAckDto>, bool)> {
        // transform and validate
        let input = Arc::new(input.clone());
        self.validator.on_transfer_request(&input).await?;

        // resolve data service
        let agreement_id = input.dto.get_agreement_id().ok_or(anyhow!("no agreement id"))?;
        let dct_formats = input.dto.format.parse::<DctFormats>()?;
        let data_service = self
            .facades
            .get_data_service_facade()
            .await
            .resolve_data_service_by_agreement_id(&agreement_id, Option::from(&dct_formats))
            .await?;

        // check idempotency
        let consumer_pid = input.dto.get_consumer_pid().ok_or(anyhow!("no consumer id"))?;
        let process_result = self
            .persistence_service
            .get_transfer_process_service()
            .await?
            .get_transfer_process_by_key_id("consumerPid", &consumer_pid)
            .await;
        match process_result {
            Ok(transfer_process) => {
                let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
                return Ok((transfer_process_dto, true));
            }
            _ => {}
        }

        // persist and send
        let transfer_process = self
            .persistence_service
            .create_process(
                "DSP",
                "INBOUND",
                None,
                None,
                Arc::new(input.dto.clone()),
                serde_json::to_value(input).unwrap(),
            )
            .await?;

        // data plane hook
        let id = Urn::from_str(transfer_process.inner.id.as_str())?;
        self.facades.get_data_plane_facade().await.on_transfer_request(&id, &data_service, &dct_formats).await?;

        // notify

        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok((transfer_process_dto, false))
    }

    async fn on_transfer_start(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferStartMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        self.validator.on_transfer_start(id, input).await?;

        // persist and send
        let transfer_process = self
            .persistence_service
            .update_process(
                id,
                Arc::new(input.dto.clone()),
                serde_json::to_value(input).unwrap(),
            )
            .await?;

        // data plane hook
        let id = Urn::from_str(transfer_process.inner.id.as_str())?;
        self.facades.get_data_plane_facade().await.on_transfer_start(&id).await?;

        // notify

        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok(transfer_process_dto)
    }

    async fn on_transfer_suspension(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferSuspensionMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        self.validator.on_transfer_suspension(id, input).await?;

        // persist and send
        let transfer_process = self
            .persistence_service
            .update_process(
                id,
                Arc::new(input.dto.clone()),
                serde_json::to_value(input).unwrap(),
            )
            .await?;

        // data plane hook
        let id = Urn::from_str(transfer_process.inner.id.as_str())?;
        self.facades.get_data_plane_facade().await.on_transfer_suspension(&id).await?;

        // notify

        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok(transfer_process_dto)
    }

    async fn on_transfer_completion(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferCompletionMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        self.validator.on_transfer_completion(id, input).await?;

        // persist and send
        let transfer_process = self
            .persistence_service
            .update_process(
                id,
                Arc::new(input.dto.clone()),
                serde_json::to_value(input).unwrap(),
            )
            .await?;

        // data plane hook
        let id = Urn::from_str(transfer_process.inner.id.as_str())?;
        self.facades.get_data_plane_facade().await.on_transfer_completion(&id).await?;

        // notify

        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok(transfer_process_dto)
    }

    async fn on_transfer_termination(
        &self,
        id: &String,
        input: &TransferProcessMessageWrapper<TransferTerminationMessageDto>,
    ) -> anyhow::Result<TransferProcessMessageWrapper<TransferProcessAckDto>> {
        self.validator.on_transfer_termination(id, input).await?;

        // persist and send
        let transfer_process = self
            .persistence_service
            .update_process(
                id,
                Arc::new(input.dto.clone()),
                serde_json::to_value(input).unwrap(),
            )
            .await?;

        // data plane hook
        let id = Urn::from_str(transfer_process.inner.id.as_str())?;
        self.facades.get_data_plane_facade().await.on_transfer_termination(&id).await?;

        // notify

        let transfer_process_dto = TransferProcessMessageWrapper::try_from(transfer_process)?;
        Ok(transfer_process_dto)
    }
}
