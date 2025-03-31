use crate::core::notification::notification_types::{RainbowEventsNotificationCreationRequest, RainbowEventsNotificationResponse};
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
        input: RainbowEventsNotificationCreationRequest,
    ) -> anyhow::Result<()>;
}
