/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use crate::core::subscription::subscription_err::SubscriptionErrors;
use crate::core::subscription::subscription_types::{
    RainbowEventsSubscriptionCreationRequest, RainbowEventsSubscriptionCreationResponse, SubscriptionEntities,
};
use crate::core::subscription::RainbowEventsSubscriptionTrait;
use axum::async_trait;
use rainbow_db::events::repo::{EditSubscription, EventsRepoFactory, NewSubscription};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowEventsSubscriptionService<T> {
    repo: Arc<T>,
}

impl<T> RainbowEventsSubscriptionService<T> {
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> RainbowEventsSubscriptionTrait for RainbowEventsSubscriptionService<T>
where
    T: EventsRepoFactory + Send + Sync + 'static,
{
    async fn get_all_subscriptions(&self) -> anyhow::Result<Vec<RainbowEventsSubscriptionCreationResponse>> {
        let subscriptions = self.repo.get_all_subscriptions().await.map_err(|e| SubscriptionErrors::DbErr(e.into()))?;
        let subscriptions = subscriptions
            .iter()
            .map(|sub| RainbowEventsSubscriptionCreationResponse::try_from(sub.to_owned()).unwrap())
            .collect();
        Ok(subscriptions)
    }

    async fn get_subscription_by_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<RainbowEventsSubscriptionCreationResponse> {
        let subscription = self
            .repo
            .get_subscription_by_id(subscription_id.clone())
            .await
            .map_err(|e| SubscriptionErrors::DbErr(e.into()))?
            .ok_or(SubscriptionErrors::NotFound { id: subscription_id, entity: "Subscription".to_string() })?;
        let subscription = RainbowEventsSubscriptionCreationResponse::try_from(subscription)?;
        Ok(subscription)
    }

    async fn put_subscription_by_id(
        &self,
        subscription_id: Urn,
        input: RainbowEventsSubscriptionCreationRequest,
    ) -> anyhow::Result<RainbowEventsSubscriptionCreationResponse> {
        let subscription = self
            .repo
            .put_subscription_by_id(
                subscription_id,
                EditSubscription {
                    callback_address: Option::from(input.callback_address),
                    expiration_time: input.expiration_time,
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| SubscriptionErrors::DbErr(e.into()))?;
        let subscription = RainbowEventsSubscriptionCreationResponse::try_from(subscription)?;
        Ok(subscription)
    }

    async fn create_subscription(
        &self,
        input: RainbowEventsSubscriptionCreationRequest,
        subscription_type: SubscriptionEntities,
    ) -> anyhow::Result<RainbowEventsSubscriptionCreationResponse> {
        let subscription = self
            .repo
            .create_subscription(NewSubscription {
                callback_address: input.callback_address,
                transfer_process: subscription_type == SubscriptionEntities::TransferProcess,
                contract_negotiation_process: subscription_type == SubscriptionEntities::ContractNegotiationProcess,
                catalog: subscription_type == SubscriptionEntities::Catalog,
                data_plane: subscription_type == SubscriptionEntities::DataPlaneProcess,
                active: true,
                expiration_time: input.expiration_time,
            })
            .await
            .map_err(|e| SubscriptionErrors::DbErr(e.into()))?;
        let subscription = RainbowEventsSubscriptionCreationResponse::try_from(subscription)?;
        Ok(subscription)
    }

    async fn delete_subscription_by_id(&self, subscription_id: Urn) -> anyhow::Result<()> {
        let _ = self
            .repo
            .delete_subscription_by_id(subscription_id)
            .await
            .map_err(|e| SubscriptionErrors::DbErr(e.into()))?;
        Ok(())
    }
}
