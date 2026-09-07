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
use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, Order,
    QueryFilter, QueryOrder, QuerySelect,
};
use urn::Urn;
use uuid::Uuid;
use ymir::errors::{BadFormat, Errors, Outcome};

use crate::bus::envelope::{EventEnvelope, Topic, TopicPattern};
use crate::data::entities::{dead_letter, delivery, event, subscription};
use crate::data::repo::{
    CreateSubscriptionDto, DeadLetterRecord, DeadLetterStatus, DeliveryStatus,
    EventDeadLetterRepo, EventDeliveryRecord, EventDeliveryRepo, EventStoreRepo,
    EventSubscriptionRepo, SubscriptionRecord, UpdateSubscriptionDto,
};

/// SeaORM-backed implementation of event bus persistence.
#[derive(Clone)]
pub struct SeaOrmEventBusRepo {
    db: DatabaseConnection,
}

impl SeaOrmEventBusRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl EventStoreRepo for SeaOrmEventBusRepo {
    async fn insert_event(&self, ev: &EventEnvelope) -> Outcome<()> {
        let active = event::ActiveModel {
            id: ActiveValue::Set(ev.id.to_string()),
            topic: ActiveValue::Set(ev.topic.to_string()),
            source_crate: ActiveValue::Set(ev.source_crate.clone()),
            schema_version: ActiveValue::Set(ev.schema_version as i32),
            correlation_id: ActiveValue::Set(ev.correlation_id.as_ref().map(ToString::to_string)),
            payload: ActiveValue::Set(ev.payload.clone()),
            timestamp: ActiveValue::Set(ev.timestamp.naive_utc()),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
        };

        active
            .insert(&self.db)
            .await
            .map_err(|e| Errors::db("failed to insert event", Some(Box::new(e))))?;

        Ok(())
    }

    async fn get_event_by_id(&self, id: &Urn) -> Outcome<Option<EventEnvelope>> {
        let model = event::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| Errors::db("failed to query event", Some(Box::new(e))))?;

        let envelope = model.map(|m| {
            let topic = Topic::new(m.topic).unwrap_or_else(|_| Topic::new("unknown").unwrap());
            let id = Urn::from_str(&m.id).unwrap_or_else(|_| Urn::from_str("urn:uuid:00000000-0000-0000-0000-000000000000").unwrap());
            let correlation_id = m.correlation_id.and_then(|c| Urn::from_str(&c).ok());
            EventEnvelope {
                id,
                topic,
                source_crate: m.source_crate,
                schema_version: m.schema_version as u32,
                timestamp: DateTime::from_naive_utc_and_offset(m.timestamp, Utc),
                correlation_id,
                payload: m.payload,
            }
        });

        Ok(envelope)
    }

    async fn list_events(
        &self,
        topic: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Outcome<Vec<EventEnvelope>> {
        let mut query = event::Entity::find();
        if let Some(t) = topic {
            query = query.filter(event::Column::Topic.eq(t));
        }

        let models = query
            .order_by(event::Column::Timestamp, Order::Desc)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await
            .map_err(|e| Errors::db("failed to list events", Some(Box::new(e))))?;

        let list = models
            .into_iter()
            .map(|m| {
                let topic = Topic::new(m.topic).unwrap_or_else(|_| Topic::new("unknown").unwrap());
                let id = Urn::from_str(&m.id).unwrap_or_else(|_| Urn::from_str("urn:uuid:00000000-0000-0000-0000-000000000000").unwrap());
                let correlation_id = m.correlation_id.and_then(|c| Urn::from_str(&c).ok());
                EventEnvelope {
                    id,
                    topic,
                    source_crate: m.source_crate,
                    schema_version: m.schema_version as u32,
                    timestamp: DateTime::from_naive_utc_and_offset(m.timestamp, Utc),
                    correlation_id,
                    payload: m.payload,
                }
            })
            .collect();

        Ok(list)
    }
}

#[async_trait]
impl EventSubscriptionRepo for SeaOrmEventBusRepo {
    async fn create_subscription(
        &self,
        dto: CreateSubscriptionDto,
    ) -> Outcome<SubscriptionRecord> {
        let id = format!("urn:uuid:{}", Uuid::new_v4());
        let pattern = TopicPattern::new(&dto.topic_pattern)
            .map_err(|e| Errors::format(BadFormat::Received, e, None))?;

        let headers_val = dto
            .headers
            .as_ref()
            .map(|h| serde_json::to_value(h).unwrap_or_default());

        let active = subscription::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            callback_address: ActiveValue::Set(dto.callback_address.clone()),
            topic_pattern: ActiveValue::Set(Some(dto.topic_pattern)),
            secret: ActiveValue::Set(dto.secret.clone()),
            headers: ActiveValue::Set(headers_val),
            retry_limit: ActiveValue::Set(dto.retry_limit.map(|r| r as i32)),
            transfer_process: ActiveValue::Set(false),
            contract_negotiation_process: ActiveValue::Set(false),
            catalog: ActiveValue::Set(false),
            data_plane: ActiveValue::Set(false),
            active: ActiveValue::Set(true),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(None),
            expiration_time: ActiveValue::Set(dto.expiration_time.map(|e| e.naive_utc())),
        };

        active
            .insert(&self.db)
            .await
            .map_err(|e| Errors::db("failed to create subscription", Some(Box::new(e))))?;

        Ok(SubscriptionRecord {
            id,
            callback_address: dto.callback_address,
            topic_pattern: pattern,
            secret: dto.secret,
            headers: dto.headers,
            retry_limit: dto.retry_limit,
            active: true,
            created_at: Utc::now(),
            updated_at: None,
            expiration_time: dto.expiration_time,
        })
    }

    async fn get_subscription(&self, id: &str) -> Outcome<Option<SubscriptionRecord>> {
        let model = subscription::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| Errors::db("failed to query subscription", Some(Box::new(e))))?;

        Ok(model.map(|m| Self::map_subscription_model(m)))
    }

    async fn list_subscriptions(&self) -> Outcome<Vec<SubscriptionRecord>> {
        let models = subscription::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| Errors::db("failed to list subscriptions", Some(Box::new(e))))?;

        Ok(models.into_iter().map(Self::map_subscription_model).collect())
    }

    async fn update_subscription(
        &self,
        id: &str,
        dto: UpdateSubscriptionDto,
    ) -> Outcome<SubscriptionRecord> {
        let model = subscription::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| Errors::db("failed to find subscription", Some(Box::new(e))))?
            .ok_or_else(|| Errors::missing_resource(id, "subscription not found", None))?;

        let mut active: subscription::ActiveModel = model.into();
        if let Some(addr) = dto.callback_address {
            active.callback_address = ActiveValue::Set(addr);
        }
        if let Some(pat) = dto.topic_pattern {
            active.topic_pattern = ActiveValue::Set(Some(pat));
        }
        if dto.secret.is_some() {
            active.secret = ActiveValue::Set(dto.secret);
        }
        if let Some(headers) = dto.headers {
            active.headers = ActiveValue::Set(Some(serde_json::to_value(headers).unwrap_or_default()));
        }
        if dto.retry_limit.is_some() {
            active.retry_limit = ActiveValue::Set(dto.retry_limit.map(|r| r as i32));
        }
        if let Some(act) = dto.active {
            active.active = ActiveValue::Set(act);
        }
        if dto.expiration_time.is_some() {
            active.expiration_time = ActiveValue::Set(dto.expiration_time.map(|e| e.naive_utc()));
        }
        active.updated_at = ActiveValue::Set(Some(Utc::now().naive_utc()));

        let updated = active
            .update(&self.db)
            .await
            .map_err(|e| Errors::db("failed to update subscription", Some(Box::new(e))))?;

        Ok(Self::map_subscription_model(updated))
    }

    async fn delete_subscription(&self, id: &str) -> Outcome<()> {
        subscription::Entity::delete_by_id(id.to_string())
            .exec(&self.db)
            .await
            .map_err(|e| Errors::db("failed to delete subscription", Some(Box::new(e))))?;
        Ok(())
    }

    async fn get_matching_subscriptions(&self, topic: &Topic) -> Outcome<Vec<SubscriptionRecord>> {
        let all = self.list_subscriptions().await?;
        Ok(all.into_iter().filter(|s| s.matches(topic)).collect())
    }
}

impl SeaOrmEventBusRepo {
    fn map_subscription_model(m: subscription::Model) -> SubscriptionRecord {
        let pat_str = m.topic_pattern.unwrap_or_else(|| {
            if m.transfer_process {
                "transfer-agent.**".to_string()
            } else if m.catalog {
                "catalog-agent.**".to_string()
            } else if m.contract_negotiation_process {
                "negotiation-agent.**".to_string()
            } else if m.data_plane {
                "dataplane.**".to_string()
            } else {
                "**".to_string()
            }
        });
        let pattern = TopicPattern::new(&pat_str).unwrap_or_else(|_| TopicPattern::match_all());

        let headers: Option<HashMap<String, String>> = m
            .headers
            .and_then(|v| serde_json::from_value(v).ok());

        SubscriptionRecord {
            id: m.id,
            callback_address: m.callback_address,
            topic_pattern: pattern,
            secret: m.secret,
            headers,
            retry_limit: m.retry_limit.map(|r| r as u32),
            active: m.active,
            created_at: DateTime::from_naive_utc_and_offset(m.created_at, Utc),
            updated_at: m.updated_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            expiration_time: m.expiration_time.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
        }
    }
}

#[async_trait]
impl EventDeliveryRepo for SeaOrmEventBusRepo {
    async fn create_delivery(&self, d: &EventDeliveryRecord) -> Outcome<EventDeliveryRecord> {
        let active = delivery::ActiveModel {
            id: ActiveValue::Set(d.id.clone()),
            event_id: ActiveValue::Set(d.event_id.clone()),
            subscription_id: ActiveValue::Set(d.subscription_id.clone()),
            status: ActiveValue::Set(d.status.as_str().to_string()),
            attempts: ActiveValue::Set(d.attempts as i32),
            last_attempt_at: ActiveValue::Set(d.last_attempt_at.map(|t| t.naive_utc())),
            next_retry_at: ActiveValue::Set(d.next_retry_at.map(|t| t.naive_utc())),
            error_message: ActiveValue::Set(d.error_message.clone()),
            response_status_code: ActiveValue::Set(d.response_status_code.map(|s| s as i32)),
            delivered_at: ActiveValue::Set(d.delivered_at.map(|t| t.naive_utc())),
            created_at: ActiveValue::Set(d.created_at.naive_utc()),
        };

        active
            .insert(&self.db)
            .await
            .map_err(|e| Errors::db("failed to create delivery record", Some(Box::new(e))))?;

        Ok(d.clone())
    }

    async fn get_delivery(&self, id: &str) -> Outcome<Option<EventDeliveryRecord>> {
        let model = delivery::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| Errors::db("failed to query delivery record", Some(Box::new(e))))?;

        Ok(model.map(|m| EventDeliveryRecord {
            id: m.id,
            event_id: m.event_id,
            subscription_id: m.subscription_id,
            status: DeliveryStatus::from_str(&m.status).unwrap_or(DeliveryStatus::Failed),
            attempts: m.attempts as u32,
            last_attempt_at: m.last_attempt_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            next_retry_at: m.next_retry_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            error_message: m.error_message,
            response_status_code: m.response_status_code.map(|s| s as u16),
            delivered_at: m.delivered_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            created_at: DateTime::from_naive_utc_and_offset(m.created_at, Utc),
        }))
    }

    async fn get_due_retries(
        &self,
        now: DateTime<Utc>,
        limit: u64,
    ) -> Outcome<Vec<EventDeliveryRecord>> {
        let models = delivery::Entity::find()
            .filter(delivery::Column::Status.eq(DeliveryStatus::Failed.as_str()))
            .filter(delivery::Column::NextRetryAt.lte(now.naive_utc()))
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(|e| Errors::db("failed to query due retries", Some(Box::new(e))))?;

        let list = models
            .into_iter()
            .map(|m| EventDeliveryRecord {
                id: m.id,
                event_id: m.event_id,
                subscription_id: m.subscription_id,
                status: DeliveryStatus::from_str(&m.status).unwrap_or(DeliveryStatus::Failed),
                attempts: m.attempts as u32,
                last_attempt_at: m.last_attempt_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                next_retry_at: m.next_retry_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                error_message: m.error_message,
                response_status_code: m.response_status_code.map(|s| s as u16),
                delivered_at: m.delivered_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                created_at: DateTime::from_naive_utc_and_offset(m.created_at, Utc),
            })
            .collect();

        Ok(list)
    }

    async fn mark_delivered(&self, id: &str, status_code: u16) -> Outcome<()> {
        if let Some(model) = delivery::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| Errors::db("failed to find delivery", Some(Box::new(e))))?
        {
            let mut active: delivery::ActiveModel = model.into();
            active.status = ActiveValue::Set(DeliveryStatus::Delivered.as_str().to_string());
            active.delivered_at = ActiveValue::Set(Some(Utc::now().naive_utc()));
            active.response_status_code = ActiveValue::Set(Some(status_code as i32));
            active.error_message = ActiveValue::Set(None);
            active.update(&self.db).await.map_err(|e| Errors::db("failed to update delivery", Some(Box::new(e))))?;
        }
        Ok(())
    }

    async fn record_failed_attempt(
        &self,
        id: &str,
        attempts: u32,
        next_retry_at: Option<DateTime<Utc>>,
        error: &str,
        status_code: Option<u16>,
    ) -> Outcome<()> {
        if let Some(model) = delivery::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| Errors::db("failed to find delivery", Some(Box::new(e))))?
        {
            let mut active: delivery::ActiveModel = model.into();
            active.status = ActiveValue::Set(DeliveryStatus::Failed.as_str().to_string());
            active.attempts = ActiveValue::Set(attempts as i32);
            active.last_attempt_at = ActiveValue::Set(Some(Utc::now().naive_utc()));
            active.next_retry_at = ActiveValue::Set(next_retry_at.map(|t| t.naive_utc()));
            active.error_message = ActiveValue::Set(Some(error.to_string()));
            active.response_status_code = ActiveValue::Set(status_code.map(|s| s as i32));
            active.update(&self.db).await.map_err(|e| Errors::db("failed to update delivery", Some(Box::new(e))))?;
        }
        Ok(())
    }

    async fn mark_dead_letter(&self, id: &str) -> Outcome<()> {
        if let Some(model) = delivery::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| Errors::db("failed to find delivery", Some(Box::new(e))))?
        {
            let mut active: delivery::ActiveModel = model.into();
            active.status = ActiveValue::Set(DeliveryStatus::DeadLetter.as_str().to_string());
            active.next_retry_at = ActiveValue::Set(None);
            active.update(&self.db).await.map_err(|e| Errors::db("failed to update delivery", Some(Box::new(e))))?;
        }
        Ok(())
    }

    async fn list_by_event(&self, event_id: &str) -> Outcome<Vec<EventDeliveryRecord>> {
        let models = delivery::Entity::find()
            .filter(delivery::Column::EventId.eq(event_id))
            .all(&self.db)
            .await
            .map_err(|e| Errors::db("failed to list deliveries", Some(Box::new(e))))?;

        let list = models
            .into_iter()
            .map(|m| EventDeliveryRecord {
                id: m.id,
                event_id: m.event_id,
                subscription_id: m.subscription_id,
                status: DeliveryStatus::from_str(&m.status).unwrap_or(DeliveryStatus::Failed),
                attempts: m.attempts as u32,
                last_attempt_at: m.last_attempt_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                next_retry_at: m.next_retry_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                error_message: m.error_message,
                response_status_code: m.response_status_code.map(|s| s as u16),
                delivered_at: m.delivered_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                created_at: DateTime::from_naive_utc_and_offset(m.created_at, Utc),
            })
            .collect();

        Ok(list)
    }
}

#[async_trait]
impl EventDeadLetterRepo for SeaOrmEventBusRepo {
    async fn create_dead_letter(&self, record: &DeadLetterRecord) -> Outcome<DeadLetterRecord> {
        let active = dead_letter::ActiveModel {
            id: ActiveValue::Set(record.id.clone()),
            delivery_id: ActiveValue::Set(record.delivery_id.clone()),
            event_id: ActiveValue::Set(record.event_id.clone()),
            subscription_id: ActiveValue::Set(record.subscription_id.clone()),
            topic: ActiveValue::Set(record.topic.clone()),
            callback_address: ActiveValue::Set(record.callback_address.clone()),
            payload: ActiveValue::Set(record.payload.clone()),
            error_message: ActiveValue::Set(record.error_message.clone()),
            attempts: ActiveValue::Set(record.attempts as i32),
            status: ActiveValue::Set(record.status.as_str().to_string()),
            failed_at: ActiveValue::Set(record.failed_at.naive_utc()),
            replayed_at: ActiveValue::Set(record.replayed_at.map(|t| t.naive_utc())),
        };

        active
            .insert(&self.db)
            .await
            .map_err(|e| Errors::db("failed to create dead letter", Some(Box::new(e))))?;

        Ok(record.clone())
    }

    async fn get_dead_letter(&self, id: &str) -> Outcome<Option<DeadLetterRecord>> {
        let model = dead_letter::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| Errors::db("failed to query dead letter", Some(Box::new(e))))?;

        Ok(model.map(|m| DeadLetterRecord {
            id: m.id,
            delivery_id: m.delivery_id,
            event_id: m.event_id,
            subscription_id: m.subscription_id,
            topic: m.topic,
            callback_address: m.callback_address,
            payload: m.payload,
            error_message: m.error_message,
            attempts: m.attempts as u32,
            status: DeadLetterStatus::from_str(&m.status).unwrap_or(DeadLetterStatus::Unresolved),
            failed_at: DateTime::from_naive_utc_and_offset(m.failed_at, Utc),
            replayed_at: m.replayed_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
        }))
    }

    async fn list_dead_letters(
        &self,
        status: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Outcome<Vec<DeadLetterRecord>> {
        let mut query = dead_letter::Entity::find();
        if let Some(s) = status {
            query = query.filter(dead_letter::Column::Status.eq(s));
        }

        let models = query
            .order_by(dead_letter::Column::FailedAt, Order::Desc)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await
            .map_err(|e| Errors::db("failed to list dead letters", Some(Box::new(e))))?;

        let list = models
            .into_iter()
            .map(|m| DeadLetterRecord {
                id: m.id,
                delivery_id: m.delivery_id,
                event_id: m.event_id,
                subscription_id: m.subscription_id,
                topic: m.topic,
                callback_address: m.callback_address,
                payload: m.payload,
                error_message: m.error_message,
                attempts: m.attempts as u32,
                status: DeadLetterStatus::from_str(&m.status).unwrap_or(DeadLetterStatus::Unresolved),
                failed_at: DateTime::from_naive_utc_and_offset(m.failed_at, Utc),
                replayed_at: m.replayed_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            })
            .collect();

        Ok(list)
    }

    async fn mark_replayed(&self, id: &str) -> Outcome<()> {
        if let Some(model) = dead_letter::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| Errors::db("failed to find dead letter", Some(Box::new(e))))?
        {
            let mut active: dead_letter::ActiveModel = model.into();
            active.status = ActiveValue::Set(DeadLetterStatus::Replayed.as_str().to_string());
            active.replayed_at = ActiveValue::Set(Some(Utc::now().naive_utc()));
            active.update(&self.db).await.map_err(|e| Errors::db("failed to update dead letter", Some(Box::new(e))))?;
        }
        Ok(())
    }

    async fn delete_dead_letter(&self, id: &str) -> Outcome<()> {
        dead_letter::Entity::delete_by_id(id.to_string())
            .exec(&self.db)
            .await
            .map_err(|e| Errors::db("failed to delete dead letter", Some(Box::new(e))))?;
        Ok(())
    }
}
