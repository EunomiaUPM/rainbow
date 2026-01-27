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

use crate::data::entities::offer;
use crate::data::entities::offer::{Model, NewOfferModel};
use crate::data::repo_traits::offer_repo::{OfferRepoErrors, OfferRepoTrait};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use urn::Urn;

pub struct OfferRepoForSql {
    db_connection: DatabaseConnection,
}

impl OfferRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl OfferRepoTrait for OfferRepoForSql {
    async fn get_all_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<Model>, OfferRepoErrors> {
        let offers = offer::Entity::find()
            .limit(limit.unwrap_or(20))
            .offset(page.map(|p| p * limit.unwrap_or(20)).unwrap_or(0))
            .order_by_desc(offer::Column::CreatedAt)
            .all(&self.db_connection)
            .await;

        match offers {
            Ok(offers) => Ok(offers),
            Err(e) => Err(OfferRepoErrors::ErrorFetchingOffer(e.into())),
        }
    }

    async fn get_batch_offers(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<Model>, OfferRepoErrors> {
        let offer_ids = ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let offers = offer::Entity::find()
            .filter(offer::Column::Id.is_in(offer_ids))
            .all(&self.db_connection)
            .await;

        match offers {
            Ok(offers) => Ok(offers),
            Err(e) => Err(OfferRepoErrors::ErrorFetchingOffer(e.into())),
        }
    }

    async fn get_offers_by_negotiation_process(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Vec<Model>, OfferRepoErrors> {
        let pid = id.to_string();
        let offers = offer::Entity::find()
            .filter(offer::Column::NegotiationAgentProcessId.eq(pid))
            .order_by_asc(offer::Column::CreatedAt)
            .all(&self.db_connection)
            .await;

        match offers {
            Ok(offers) => Ok(offers),
            Err(e) => Err(OfferRepoErrors::ErrorFetchingOffer(e.into())),
        }
    }

    async fn get_last_offer_by_negotiation_process(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<Model>, OfferRepoErrors> {
        let pid = id.to_string();
        let offers = offer::Entity::find()
            .filter(offer::Column::NegotiationAgentProcessId.eq(pid))
            .order_by_desc(offer::Column::CreatedAt)
            .one(&self.db_connection)
            .await;

        match offers {
            Ok(offers) => Ok(offers),
            Err(e) => Err(OfferRepoErrors::ErrorFetchingOffer(e.into())),
        }
    }

    async fn get_offer_by_id(&self, id: &Urn) -> anyhow::Result<Option<Model>, OfferRepoErrors> {
        let oid = id.to_string();
        let offer = offer::Entity::find_by_id(oid).one(&self.db_connection).await;

        match offer {
            Ok(offer) => Ok(offer),
            Err(e) => Err(OfferRepoErrors::ErrorFetchingOffer(e.into())),
        }
    }

    async fn get_offer_by_negotiation_message(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<Model>, OfferRepoErrors> {
        let mid = id.to_string();
        let offer = offer::Entity::find()
            .filter(offer::Column::NegotiationAgentMessageId.eq(mid))
            .one(&self.db_connection)
            .await;

        match offer {
            Ok(offer) => Ok(offer),
            Err(e) => Err(OfferRepoErrors::ErrorFetchingOffer(e.into())),
        }
    }

    async fn get_offer_by_offer_id(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<Model>, OfferRepoErrors> {
        let external_offer_id = id.to_string();
        let offer = offer::Entity::find()
            .filter(offer::Column::OfferId.eq(external_offer_id))
            .one(&self.db_connection)
            .await;

        match offer {
            Ok(offer) => Ok(offer),
            Err(e) => Err(OfferRepoErrors::ErrorFetchingOffer(e.into())),
        }
    }

    async fn create_offer(
        &self,
        new_model: &NewOfferModel,
    ) -> anyhow::Result<Model, OfferRepoErrors> {
        let model: offer::ActiveModel = new_model.clone().into();
        let result = offer::Entity::insert(model).exec_with_returning(&self.db_connection).await;

        match result {
            Ok(offer) => Ok(offer),
            Err(e) => Err(OfferRepoErrors::ErrorCreatingOffer(e.into())),
        }
    }

    async fn delete_offer(&self, id: &Urn) -> anyhow::Result<(), OfferRepoErrors> {
        let oid = id.to_string();
        let result = offer::Entity::delete_by_id(oid).exec(&self.db_connection).await;

        match result {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(OfferRepoErrors::OfferNotFound),
                _ => Ok(()),
            },
            Err(e) => Err(OfferRepoErrors::ErrorDeletingOffer(e.into())),
        }
    }
}
