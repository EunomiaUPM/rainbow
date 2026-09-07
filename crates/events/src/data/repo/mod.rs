/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use urn::Urn;
use ymir::errors::{Outcome, RepoIntoErrors};

use crate::bus::envelope::{EventEnvelope, Topic, TopicPattern};
use crate::data::entities::{notification, subscription};

pub mod in_memory;
pub mod sea_orm_repo;
pub mod sql;

/// Record representing a webhook subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRecord {
    pub id: String,
    pub callback_address: String,
    pub topic_pattern: TopicPattern,
    pub secret: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub retry_limit: Option<u32>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expiration_time: Option<DateTime<Utc>>,
}

impl SubscriptionRecord {
    /// Check if this subscription is currently active and not expired.
    pub fn is_active(&self) -> bool {
        if !self.active {
            return false;
        }
        if let Some(exp) = self.expiration_time {
            if exp <= Utc::now() {
                return false;
            }
        }
        true
    }

    /// Check if a topic matches this subscription's pattern.
    pub fn matches(&self, topic: &Topic) -> bool {
        self.is_active() && self.topic_pattern.matches(topic)
    }
}

/// Parameters to create a new subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionDto {
    pub callback_address: String,
    pub topic_pattern: String,
    pub secret: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub retry_limit: Option<u32>,
    pub expiration_time: Option<DateTime<Utc>>,
}

/// Parameters to update an existing subscription.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSubscriptionDto {
    pub callback_address: Option<String>,
    pub topic_pattern: Option<String>,
    pub secret: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub retry_limit: Option<u32>,
    pub active: Option<bool>,
    pub expiration_time: Option<DateTime<Utc>>,
}

/// Record representing a specific delivery attempt for an event and subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDeliveryRecord {
    pub id: String,
    pub event_id: String,
    pub subscription_id: String,
    pub status: DeliveryStatus,
    pub attempts: u32,
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub response_status_code: Option<u16>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryStatus {
    Pending,
    Delivered,
    Failed,
    DeadLetter,
}

impl DeliveryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeliveryStatus::Pending => "Pending",
            DeliveryStatus::Delivered => "Delivered",
            DeliveryStatus::Failed => "Failed",
            DeliveryStatus::DeadLetter => "DeadLetter",
        }
    }
}

impl std::str::FromStr for DeliveryStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(DeliveryStatus::Pending),
            "Delivered" => Ok(DeliveryStatus::Delivered),
            "Failed" => Ok(DeliveryStatus::Failed),
            "DeadLetter" => Ok(DeliveryStatus::DeadLetter),
            other => Err(format!("unknown delivery status: {other}")),
        }
    }
}

/// Record representing a failed delivery permanently moved to the Dead Letter Queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterRecord {
    pub id: String,
    pub delivery_id: Option<String>,
    pub event_id: String,
    pub subscription_id: String,
    pub topic: String,
    pub callback_address: String,
    pub payload: serde_json::Value,
    pub error_message: String,
    pub attempts: u32,
    pub status: DeadLetterStatus,
    pub failed_at: DateTime<Utc>,
    pub replayed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeadLetterStatus {
    Unresolved,
    Replayed,
    Purged,
}

impl DeadLetterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeadLetterStatus::Unresolved => "Unresolved",
            DeadLetterStatus::Replayed => "Replayed",
            DeadLetterStatus::Purged => "Purged",
        }
    }
}

impl std::str::FromStr for DeadLetterStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Unresolved" => Ok(DeadLetterStatus::Unresolved),
            "Replayed" => Ok(DeadLetterStatus::Replayed),
            "Purged" => Ok(DeadLetterStatus::Purged),
            other => Err(format!("unknown dead letter status: {other}")),
        }
    }
}

/// Repository for immutable domain event persistence.
#[async_trait]
pub trait EventStoreRepo: Send + Sync + 'static {
    async fn insert_event(&self, event: &EventEnvelope) -> Outcome<()>;
    async fn get_event_by_id(&self, id: &Urn) -> Outcome<Option<EventEnvelope>>;
    async fn list_events(
        &self,
        topic: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Outcome<Vec<EventEnvelope>>;
}

/// Repository for webhook subscription management.
#[async_trait]
pub trait EventSubscriptionRepo: Send + Sync + 'static {
    async fn create_subscription(
        &self,
        dto: CreateSubscriptionDto,
    ) -> Outcome<SubscriptionRecord>;
    async fn get_subscription(&self, id: &str) -> Outcome<Option<SubscriptionRecord>>;
    async fn list_subscriptions(&self) -> Outcome<Vec<SubscriptionRecord>>;
    async fn update_subscription(
        &self,
        id: &str,
        dto: UpdateSubscriptionDto,
    ) -> Outcome<SubscriptionRecord>;
    async fn delete_subscription(&self, id: &str) -> Outcome<()>;
    async fn get_matching_subscriptions(&self, topic: &Topic) -> Outcome<Vec<SubscriptionRecord>>;
}

/// Repository for tracking delivery attempts and retry scheduling.
#[async_trait]
pub trait EventDeliveryRepo: Send + Sync + 'static {
    async fn create_delivery(&self, delivery: &EventDeliveryRecord) -> Outcome<EventDeliveryRecord>;
    async fn get_delivery(&self, id: &str) -> Outcome<Option<EventDeliveryRecord>>;
    async fn get_due_retries(
        &self,
        now: DateTime<Utc>,
        limit: u64,
    ) -> Outcome<Vec<EventDeliveryRecord>>;
    async fn mark_delivered(&self, id: &str, status_code: u16) -> Outcome<()>;
    async fn record_failed_attempt(
        &self,
        id: &str,
        attempts: u32,
        next_retry_at: Option<DateTime<Utc>>,
        error: &str,
        status_code: Option<u16>,
    ) -> Outcome<()>;
    async fn mark_dead_letter(&self, id: &str) -> Outcome<()>;
    async fn list_by_event(&self, event_id: &str) -> Outcome<Vec<EventDeliveryRecord>>;
}

/// Repository for the Dead Letter Queue.
#[async_trait]
pub trait EventDeadLetterRepo: Send + Sync + 'static {
    async fn create_dead_letter(&self, record: &DeadLetterRecord) -> Outcome<DeadLetterRecord>;
    async fn get_dead_letter(&self, id: &str) -> Outcome<Option<DeadLetterRecord>>;
    async fn list_dead_letters(
        &self,
        status: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Outcome<Vec<DeadLetterRecord>>;
    async fn mark_replayed(&self, id: &str) -> Outcome<()>;
    async fn delete_dead_letter(&self, id: &str) -> Outcome<()>;
}

// Legacy traits and structs for backward compatibility ────────────────────────

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
    async fn get_all_subscriptions(&self) -> Result<Vec<subscription::Model>, EventRepoErrors>;
    async fn get_subscription_by_id(
        &self,
        subscription_id: Urn,
    ) -> Result<Option<subscription::Model>, EventRepoErrors>;
    async fn get_subscription_by_callback_string(
        &self,
        callback_string: String,
    ) -> Result<Option<subscription::Model>, EventRepoErrors>;
    async fn put_subscription_by_id(
        &self,
        subscription_id: Urn,
        edit_subscription: EditSubscription,
    ) -> Result<subscription::Model, EventRepoErrors>;
    async fn create_subscription(
        &self,
        new_subscription: NewSubscription,
    ) -> Result<subscription::Model, EventRepoErrors>;
    async fn delete_subscription_by_id(&self, subscription_id: Urn) -> Result<(), EventRepoErrors>;
}

pub struct NewNotification {
    pub category: String,
    pub subcategory: String,
    pub message_type: String,
    pub message_operation: String,
    pub message_content: serde_json::Value,
    pub status: String,
}

#[async_trait]
pub trait NotificationRepo {
    async fn get_all_notifications(&self) -> Result<Vec<notification::Model>, EventRepoErrors>;
    async fn get_notifications_by_subscription_id(
        &self,
        subscription_id: Urn,
    ) -> Result<Vec<notification::Model>, EventRepoErrors>;
    async fn get_pending_notifications_by_subscription_id(
        &self,
        subscription_id: Urn,
    ) -> Result<Vec<notification::Model>, EventRepoErrors>;
    async fn ack_pending_notifications_by_subscription_id(
        &self,
        subscription_id: Urn,
    ) -> Result<Vec<notification::Model>, EventRepoErrors>;
    async fn get_notification_by_id(
        &self,
        subscription_id: Urn,
        notification_id: Urn,
    ) -> Result<Option<notification::Model>, EventRepoErrors>;
    async fn create_notification(
        &self,
        subscription_id: Urn,
        new_notification: NewNotification,
    ) -> Result<notification::Model, EventRepoErrors>;
}

#[derive(Debug, Error)]
pub enum EventRepoErrors {
    #[error("Subscription not found")]
    SubscriptionNotFound,
    #[error("Notification not found")]
    NotificationNotFound,
    #[error("Error fetching subscription: {0}")]
    ErrorFetchingSubscription(String),
    #[error("Error creating subscription: {0}")]
    ErrorCreatingSubscription(String),
    #[error("Error deleting subscription: {0}")]
    ErrorDeletingSubscription(String),
    #[error("Error updating subscription: {0}")]
    ErrorUpdatingSubscription(String),
    #[error("Error fetching notification: {0}")]
    ErrorFetchingNotification(String),
    #[error("Error creating notification: {0}")]
    ErrorCreatingNotification(String),
    #[error("Error deleting notification: {0}")]
    ErrorDeletingNotification(String),
    #[error("Error updating notification: {0}")]
    ErrorUpdatingNotification(String),
}

impl RepoIntoErrors for EventRepoErrors {}
