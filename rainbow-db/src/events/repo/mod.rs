use crate::events::entities::notification;
use crate::events::entities::subscription;
use anyhow::Error;
use axum::async_trait;
use sea_orm::DatabaseConnection;
use thiserror::Error;
use urn::Urn;

pub mod sql;

pub trait EventsRepoFactory: SubscriptionRepo + NotificationRepo + Send + Sync + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

pub struct NewSubscription {
    pub callback_address: String,
    pub transfer_process: bool,
    pub contract_negotiation_process: bool,
    pub catalog: bool,
    pub data_plane: bool,
    pub active: bool,
    pub expiration_time: Option<chrono::NaiveDateTime>,
}

pub struct EditSubscription {
    pub callback_address: Option<String>,
    pub transfer_process: Option<bool>,
    pub contract_negotiation_process: Option<bool>,
    pub catalog: Option<bool>,
    pub data_plane: Option<bool>,
    pub active: Option<bool>,
    pub expiration_time: Option<chrono::NaiveDateTime>,
}

impl Default for EditSubscription {
    fn default() -> Self {
        Self {
            callback_address: None,
            transfer_process: None,
            contract_negotiation_process: None,
            catalog: None,
            data_plane: None,
            active: None,
            expiration_time: None,
        }
    }
}

#[async_trait]
pub trait SubscriptionRepo {
    async fn get_all_subscriptions(
        &self,
    ) -> anyhow::Result<Vec<subscription::Model>, EventRepoErrors>;
    async fn get_subscription_by_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<Option<subscription::Model>, EventRepoErrors>;
    async fn put_subscription_by_id(
        &self,
        subscription_id: Urn,
        edit_subscription: EditSubscription,
    ) -> anyhow::Result<subscription::Model, EventRepoErrors>;
    async fn create_subscription(
        &self,
        new_subscription: NewSubscription,
    ) -> anyhow::Result<subscription::Model, EventRepoErrors>;
    async fn delete_subscription_by_id(
        &self, subscription_id: Urn) -> anyhow::Result<(), EventRepoErrors>;
}

pub struct NewNotification {
    pub category: String,
    pub message_type: String,
    pub message_content: serde_json::Value,
    pub status: String,
}

#[async_trait]
pub trait NotificationRepo {
    async fn get_all_notifications(
        &self,
    ) -> anyhow::Result<Vec<notification::Model>, EventRepoErrors>;
    async fn get_notifications_by_subscription_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<Vec<notification::Model>, EventRepoErrors>;
    async fn get_pending_notifications_by_subscription_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<Vec<notification::Model>, EventRepoErrors>;
    async fn get_notification_by_id(
        &self,
        subscription_id: Urn,
        notification_id: Urn,
    ) -> anyhow::Result<Option<notification::Model>, EventRepoErrors>;
    async fn create_notification(
        &self,
        subscription_id: Urn,
        new_notification: NewNotification,
    ) -> anyhow::Result<notification::Model, EventRepoErrors>;
}

#[derive(Debug, Error)]
pub enum EventRepoErrors {
    #[error("Subscription not found")]
    SubscriptionNotFound,
    #[error("Notification not found")]
    NotificationNotFound,

    #[error("Error fetching subscription. {0}")]
    ErrorFetchingSubscription(Error),
    #[error("Error creating subscription. {0}")]
    ErrorCreatingSubscription(Error),
    #[error("Error deleting subscription. {0}")]
    ErrorDeletingSubscription(Error),
    #[error("Error updating subscription. {0}")]
    ErrorUpdatingSubscription(Error),

    #[error("Error fetching notification. {0}")]
    ErrorFetchingNotification(Error),
    #[error("Error creating notification. {0}")]
    ErrorCreatingNotification(Error),
    #[error("Error deleting notification. {0}")]
    ErrorDeletingNotification(Error),
    #[error("Error updating notification. {0}")]
    ErrorUpdatingNotification(Error),
}
