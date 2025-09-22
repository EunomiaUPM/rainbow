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

use crate::core::subscription::subscription_types::{RainbowEventsSubscriptionCreationRequest, RainbowEventsSubscriptionCreationResponse, SubscriptionEntities};
use axum::async_trait;
use urn::Urn;

pub mod subscription;
pub mod subscription_err;
pub mod subscription_types;

#[mockall::automock]
#[async_trait]
pub trait RainbowEventsSubscriptionTrait: Send + Sync {
    async fn get_all_subscriptions(&self) -> anyhow::Result<Vec<RainbowEventsSubscriptionCreationResponse>>;
    async fn get_subscription_by_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<RainbowEventsSubscriptionCreationResponse>;
    async fn get_subscription_by_callback_url(
        &self,
        callback_url: String,
    ) -> anyhow::Result<RainbowEventsSubscriptionCreationResponse>;
    async fn put_subscription_by_id(
        &self,
        subscription_id: Urn,
        input: RainbowEventsSubscriptionCreationRequest,
    ) -> anyhow::Result<RainbowEventsSubscriptionCreationResponse>;
    async fn create_subscription(
        &self,
        input: RainbowEventsSubscriptionCreationRequest,
        subscription_type: SubscriptionEntities,
    ) -> anyhow::Result<RainbowEventsSubscriptionCreationResponse>;
    async fn delete_subscription_by_id(&self, subscription_id: Urn) -> anyhow::Result<()>;
}
