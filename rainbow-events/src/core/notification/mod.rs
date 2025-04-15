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

use crate::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationCreationRequest, RainbowEventsNotificationResponse};
use axum::async_trait;
use urn::Urn;

pub mod notification_err;
pub mod notification_types;
pub mod notification;

#[mockall::automock]
#[async_trait]
pub trait RainbowEventsNotificationTrait: Send + Sync {
    async fn get_all_notifications(&self) -> anyhow::Result<Vec<RainbowEventsNotificationResponse>>;
    async fn get_notifications_by_subscription_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<Vec<RainbowEventsNotificationResponse>>;
    async fn get_pending_notifications_by_subscription_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<Vec<RainbowEventsNotificationResponse>>;

    async fn ack_pending_notifications_by_subscription_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<Vec<RainbowEventsNotificationResponse>>;

    async fn get_notification_by_id(
        &self,
        subscription_id: Urn,
        notification_id: Urn,
    ) -> anyhow::Result<RainbowEventsNotificationResponse>;
    async fn create_notification(
        &self,
        subscription_id: Urn,
        input: RainbowEventsNotificationCreationRequest,
    ) -> anyhow::Result<RainbowEventsNotificationResponse>;

    async fn broadcast_notification(
        &self,
        input: RainbowEventsNotificationBroadcastRequest,
    ) -> anyhow::Result<()>;
}
