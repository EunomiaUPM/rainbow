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

use crate::core::notification::notification_err::NotificationErrors;
use crate::core::notification::notification_types::{
    RainbowEventsNotificationCreationRequest
    , RainbowEventsNotificationResponse,
};
use crate::core::notification::RainbowEventsNotificationTrait;
use crate::core::subscription::subscription_err::SubscriptionErrors;
use axum::async_trait;
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::events::repo::{EventsRepoFactory, NewNotification};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use urn::Urn;

pub struct RainbowEventsNotificationsService<T> {
    repo: Arc<T>,
    client: Client,
}
impl<T> RainbowEventsNotificationsService<T>
where
    T: EventsRepoFactory + Sync + Send + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { repo, client }
    }
}

#[async_trait]
impl<T> RainbowEventsNotificationTrait for RainbowEventsNotificationsService<T>
where
    T: EventsRepoFactory + Sync + Send + 'static,
{
    async fn get_all_notifications(&self) -> anyhow::Result<Vec<RainbowEventsNotificationResponse>> {
        let notifications = self.repo.get_all_notifications().await.map_err(|e| NotificationErrors::DbErr(e.into()))?;
        let notifications = notifications
            .iter()
            .map(|sub| RainbowEventsNotificationResponse::try_from(sub.to_owned()).unwrap())
            .collect();
        Ok(notifications)
    }

    async fn get_notifications_by_subscription_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<Vec<RainbowEventsNotificationResponse>> {
        let notifications = self
            .repo
            .get_notifications_by_subscription_id(subscription_id)
            .await
            .map_err(|e| NotificationErrors::DbErr(e.into()))?;
        let notifications = notifications
            .iter()
            .map(|sub| RainbowEventsNotificationResponse::try_from(sub.to_owned()).unwrap())
            .collect();
        Ok(notifications)
    }

    async fn get_pending_notifications_by_subscription_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<Vec<RainbowEventsNotificationResponse>> {
        let notifications = self
            .repo
            .get_pending_notifications_by_subscription_id(subscription_id)
            .await
            .map_err(|e| NotificationErrors::DbErr(e.into()))?;
        let notifications = notifications
            .iter()
            .map(|sub| RainbowEventsNotificationResponse::try_from(sub.to_owned()).unwrap())
            .collect();
        Ok(notifications)
    }

    async fn get_notification_by_id(
        &self,
        subscription_id: Urn,
        notification_id: Urn,
    ) -> anyhow::Result<RainbowEventsNotificationResponse> {
        let notifications = self
            .repo
            .get_notification_by_id(subscription_id.clone(), notification_id.clone())
            .await
            .map_err(|e| NotificationErrors::DbErr(e.into()))?
            .ok_or(SubscriptionErrors::NotFound { id: subscription_id, entity: "Notifications".to_string() })?;
        let notifications = RainbowEventsNotificationResponse::try_from(notifications)?;
        Ok(notifications)
    }

    async fn create_notification(
        &self,
        subscription_id: Urn,
        input: RainbowEventsNotificationCreationRequest,
    ) -> anyhow::Result<RainbowEventsNotificationResponse> {
        let notification = self
            .repo
            .create_notification(
                subscription_id,
                NewNotification {
                    category: input.category.to_string(),
                    message_type: input.message_type.to_string(),
                    message_content: input.message_content,
                    status: input.status.to_string(),
                },
            )
            .await
            .map_err(|e| NotificationErrors::DbErr(e.into()))?;

        let notifications = RainbowEventsNotificationResponse::try_from(notification)?;
        Ok(notifications)
    }

    async fn broadcast_notification(
        &self,
        input: RainbowEventsNotificationCreationRequest,
    ) -> anyhow::Result<()> {
        let subscriptions = self.repo.get_all_subscriptions().await.map_err(|e| NotificationErrors::DbErr(e.into()))?;
        for subscription in subscriptions {
            let callback = subscription.callback_address;
            let message = RainbowEventsNotificationResponse {
                id: get_urn(None),
                timestamp: subscription.created_at,
                category: input.category.to_string(),
                message_type: input.message_type.to_string(),
                message_content: input.message_content.clone(),
                subscription_id: get_urn_from_string(&subscription.id)?,
            };
            let res = self.client.post(callback).json(&message).send().await;
            match res {
                Ok(res) => {
                    self.repo.create_notification(get_urn_from_string(&subscription.id)?, NewNotification {
                        category: message.category.to_string(),
                        message_type: message.message_type.to_string(),
                        message_content: message.message_content,
                        status: match res.status().is_success() {
                            true => "Ok".to_string(),
                            false => "Pending".to_string()
                        },
                    }).await?;
                }
                Err(_) => {
                    self.repo.create_notification(get_urn_from_string(&subscription.id)?, NewNotification {
                        category: message.category.to_string(),
                        message_type: message.message_type.to_string(),
                        message_content: message.message_content,
                        status: "Pending".to_string(),
                    }).await?;
                }
            }
        }
        Ok(())
    }
}
