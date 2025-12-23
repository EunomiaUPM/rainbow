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

use crate::data::entities::negotiation_message::{self as negotiation_message_model, NewNegotiationMessageModel};
use crate::data::factory_trait::NegotiationAgentRepoTrait;
use crate::data::repo_traits::negotiation_message_repo::NegotiationMessageRepoErrors;
use crate::entities::negotiation_message::{
    NegotiationAgentMessagesTrait, NegotiationMessageDto, NewNegotiationMessageDto,
};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct NegotiationAgentMessagesService {
    pub negotiation_repo: Arc<dyn NegotiationAgentRepoTrait>,
}

impl NegotiationAgentMessagesService {
    pub fn new(negotiation_repo: Arc<dyn NegotiationAgentRepoTrait>) -> Self {
        Self { negotiation_repo }
    }

    /// Helper privado para enriquecer el mensaje con sus ofertas y acuerdos asociados
    async fn enrich_message(&self, message: negotiation_message_model::Model) -> anyhow::Result<NegotiationMessageDto> {
        let message_urn = Urn::from_str(&message.id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!(
                "Invalid URN found in database for message {}. Error: {}",
                message.id, e
            ));
            error!("{}", err.log());
            err
        })?;

        // 1. Buscar Oferta asociada
        let offer =
            self.negotiation_repo.get_offer_repo().get_offer_by_negotiation_message(&message_urn).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("Error fetching linked offer: {}", err.log());
                    err
                },
            )?;

        // 2. Buscar Acuerdo asociado
        let agreement = self
            .negotiation_repo
            .get_agreement_repo()
            .get_agreement_by_negotiation_message(&message_urn)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("Error fetching linked agreement: {}", err.log());
                err
            })?;

        Ok(NegotiationMessageDto { inner: message, offer, agreement })
    }
}

#[async_trait::async_trait]
impl NegotiationAgentMessagesTrait for NegotiationAgentMessagesService {
    async fn get_all_negotiation_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<NegotiationMessageDto>> {
        let messages = self
            .negotiation_repo
            .get_negotiation_message_repo()
            .get_all_negotiation_messages(limit, page)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        let mut dtos = Vec::with_capacity(messages.len());
        for msg in messages {
            let dto = self.enrich_message(msg).await?;
            dtos.push(dto);
        }

        Ok(dtos)
    }

    async fn get_messages_by_process_id(&self, process_id: &Urn) -> anyhow::Result<Vec<NegotiationMessageDto>> {
        let messages =
            self.negotiation_repo.get_negotiation_message_repo().get_messages_by_process_id(process_id).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;

        let mut dtos = Vec::with_capacity(messages.len());
        for msg in messages {
            let dto = self.enrich_message(msg).await?;
            dtos.push(dto);
        }

        Ok(dtos)
    }

    async fn get_negotiation_message_by_id(&self, id: &Urn) -> anyhow::Result<Option<NegotiationMessageDto>> {
        let message_opt =
            self.negotiation_repo.get_negotiation_message_repo().get_negotiation_message_by_id(id).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;

        match message_opt {
            Some(message) => Ok(Some(self.enrich_message(message).await?)),
            None => Ok(None),
        }
    }

    async fn create_negotiation_message(
        &self,
        new_model_dto: &NewNegotiationMessageDto,
    ) -> anyhow::Result<NegotiationMessageDto> {
        let new_model: NewNegotiationMessageModel = new_model_dto.clone().into();

        let created =
            self.negotiation_repo.get_negotiation_message_repo().create_negotiation_message(&new_model).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;

        // Un mensaje recién creado no tiene ofertas ni acuerdos vinculados todavía
        Ok(NegotiationMessageDto { inner: created, offer: None, agreement: None })
    }

    async fn delete_negotiation_message(&self, id: &Urn) -> anyhow::Result<()> {
        self.negotiation_repo.get_negotiation_message_repo().delete_negotiation_message(id).await.map_err(
            |e| match e {
                NegotiationMessageRepoErrors::NegotiationMessageNotFound => {
                    let err = CommonErrors::missing_resource_new(
                        &id.to_string(),
                        "Negotiation Message not found for deletion",
                    );
                    error!("{}", err.log());
                    err
                }
                _ => {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                }
            },
        )?;
        Ok(())
    }
}
