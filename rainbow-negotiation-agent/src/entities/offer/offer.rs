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

use crate::data::entities::offer::NewOfferModel;
use crate::data::factory_trait::NegotiationAgentRepoTrait;
use crate::data::repo_traits::offer_repo::OfferRepoErrors;
use crate::entities::offer::{NegotiationAgentOffersTrait, NewOfferDto, OfferDto};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct NegotiationAgentOffersService {
    pub negotiation_repo: Arc<dyn NegotiationAgentRepoTrait>,
}

impl NegotiationAgentOffersService {
    pub fn new(negotiation_repo: Arc<dyn NegotiationAgentRepoTrait>) -> Self {
        Self { negotiation_repo }
    }
}

#[async_trait::async_trait]
impl NegotiationAgentOffersTrait for NegotiationAgentOffersService {
    async fn get_all_offers(&self, limit: Option<u64>, page: Option<u64>) -> anyhow::Result<Vec<OfferDto>> {
        let offers = self.negotiation_repo.get_offer_repo().get_all_offers(limit, page).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        Ok(offers.into_iter().map(|m| OfferDto { inner: m }).collect())
    }

    async fn get_batch_offers(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<OfferDto>> {
        let offers = self.negotiation_repo.get_offer_repo().get_batch_offers(ids).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        Ok(offers.into_iter().map(|m| OfferDto { inner: m }).collect())
    }

    async fn get_offers_by_negotiation_process(&self, id: &Urn) -> anyhow::Result<Vec<OfferDto>> {
        let offers =
            self.negotiation_repo.get_offer_repo().get_offers_by_negotiation_process(id).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        Ok(offers.into_iter().map(|m| OfferDto { inner: m }).collect())
    }

    async fn get_last_offer_by_negotiation_process(&self, id: &Urn) -> anyhow::Result<Option<OfferDto>> {
        let offers =
            self.negotiation_repo.get_offer_repo().get_last_offer_by_negotiation_process(id).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        Ok(offers.map(|m| OfferDto { inner: m }))
    }

    async fn get_offer_by_id(&self, id: &Urn) -> anyhow::Result<Option<OfferDto>> {
        let offer = self.negotiation_repo.get_offer_repo().get_offer_by_id(id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        Ok(offer.map(|m| OfferDto { inner: m }))
    }

    async fn get_offer_by_negotiation_message(&self, id: &Urn) -> anyhow::Result<Option<OfferDto>> {
        let offer = self.negotiation_repo.get_offer_repo().get_offer_by_negotiation_message(id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        Ok(offer.map(|m| OfferDto { inner: m }))
    }

    async fn get_offer_by_offer_id(&self, id: &Urn) -> anyhow::Result<Option<OfferDto>> {
        let offer = self.negotiation_repo.get_offer_repo().get_offer_by_offer_id(id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        Ok(offer.map(|m| OfferDto { inner: m }))
    }

    async fn create_offer(&self, new_model_dto: &NewOfferDto) -> anyhow::Result<OfferDto> {
        let new_model: NewOfferModel = new_model_dto.clone().into();

        let created = self.negotiation_repo.get_offer_repo().create_offer(&new_model).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;

        Ok(OfferDto { inner: created })
    }

    async fn delete_offer(&self, id: &Urn) -> anyhow::Result<()> {
        self.negotiation_repo.get_offer_repo().delete_offer(id).await.map_err(|e| match e {
            OfferRepoErrors::OfferNotFound => {
                let err = CommonErrors::missing_resource_new(&id.to_string(), "Offer not found for deletion");
                error!("{}", err.log());
                err
            }
            _ => {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            }
        })?;
        Ok(())
    }
}
