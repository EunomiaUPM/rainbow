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

use std::str::FromStr;
use std::sync::Arc;

use chrono::Utc;
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use urn::Urn;

use crate::bus::dispatcher::EventDispatcher;
use crate::bus::envelope::EventEnvelope;
use crate::bus::policy::RetryPolicy;
use crate::data::repo::{
    DeadLetterRecord, DeadLetterStatus, EventDeadLetterRepo, EventDeliveryRecord,
    EventDeliveryRepo, EventStoreRepo, EventSubscriptionRepo, SubscriptionRecord,
};

/// Background worker that periodically inspects and executes due webhook retries.
pub struct RetryWorker {
    event_repo: Arc<dyn EventStoreRepo>,
    subscription_repo: Arc<dyn EventSubscriptionRepo>,
    delivery_repo: Arc<dyn EventDeliveryRepo>,
    dlq_repo: Arc<dyn EventDeadLetterRepo>,
    dispatcher: Arc<EventDispatcher>,
    policy: RetryPolicy,
    concurrency_limit: usize,
}

impl RetryWorker {
    /// Create a new RetryWorker with dependencies and retry policy.
    pub fn new(
        event_repo: Arc<dyn EventStoreRepo>,
        subscription_repo: Arc<dyn EventSubscriptionRepo>,
        delivery_repo: Arc<dyn EventDeliveryRepo>,
        dlq_repo: Arc<dyn EventDeadLetterRepo>,
        dispatcher: Arc<EventDispatcher>,
        policy: RetryPolicy,
    ) -> Self {
        Self {
            event_repo,
            subscription_repo,
            delivery_repo,
            dlq_repo,
            dispatcher,
            policy,
            concurrency_limit: 10,
        }
    }

    /// Set maximum concurrent HTTP delivery tasks.
    pub fn with_concurrency_limit(mut self, limit: usize) -> Self {
        self.concurrency_limit = limit.max(1);
        self
    }

    /// Run the poller loop until cancelled by the given CancellationToken.
    pub async fn run(self: Arc<Self>, cancel_token: CancellationToken) {
        let interval_duration = self.policy.poll_interval();
        info!(
            interval_secs = interval_duration.as_secs(),
            "Starting event bus retry worker loop"
        );

        let mut ticker = tokio::time::interval(interval_duration);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    info!("Event bus retry worker received cancellation signal, stopping");
                    break;
                }
                _ = ticker.tick() => {
                    if let Err(e) = self.process_batch().await {
                        error!(error = %e, "Error during retry worker batch execution");
                    }
                }
            }
        }
    }

    /// Fetch and process one batch of due retries.
    pub async fn process_batch(&self) -> Result<usize, String> {
        let now = Utc::now();
        let due = self
            .delivery_repo
            .get_due_retries(now, 50)
            .await
            .map_err(|e| format!("failed to fetch due retries: {e:?}"))?;

        if due.is_empty() {
            return Ok(0);
        }

        debug!(count = due.len(), "Processing due retry batch");
        let count = due.len();
        let semaphore = Arc::new(Semaphore::new(self.concurrency_limit));
        let mut handles = Vec::with_capacity(count);

        for delivery in due {
            let permit = semaphore.clone().acquire_owned().await.map_err(|e| e.to_string())?;
            let worker = self.clone_worker();
            handles.push(tokio::spawn(async move {
                let _permit = permit;
                worker.process_single_retry(delivery).await;
            }));
        }

        for handle in handles {
            let _ = handle.await;
        }

        Ok(count)
    }

    /// Process a single due delivery attempt and apply retry or dead-letter transitions.
    pub async fn process_single_retry(&self, delivery: EventDeliveryRecord) {
        let subscription = match self.subscription_repo.get_subscription(&delivery.subscription_id).await {
            Ok(Some(s)) if s.is_active() => s,
            Ok(Some(_)) => {
                warn!(sub_id = %delivery.subscription_id, "Subscription inactive, skipping delivery");
                let _ = self.delivery_repo.record_failed_attempt(&delivery.id, delivery.attempts, None, "Subscription is inactive", None).await;
                return;
            }
            Ok(None) => {
                warn!(sub_id = %delivery.subscription_id, "Subscription not found, marking dead letter");
                let _ = self.delivery_repo.mark_dead_letter(&delivery.id).await;
                return;
            }
            Err(e) => {
                error!(error = ?e, "Failed to load subscription for retry");
                return;
            }
        };

        let event_urn = match Urn::from_str(&delivery.event_id) {
            Ok(u) => u,
            Err(_) => match Urn::from_str(&format!("urn:uuid:{}", delivery.event_id)) {
                Ok(u) => u,
                Err(e) => {
                    error!(error = ?e, "Invalid event ID URN for delivery");
                    return;
                }
            },
        };

        let event = match self.event_repo.get_event_by_id(&event_urn).await {
            Ok(Some(ev)) => ev,
            Ok(None) => {
                error!(event_id = %delivery.event_id, "Event not found for delivery");
                let _ = self.delivery_repo.mark_dead_letter(&delivery.id).await;
                return;
            }
            Err(e) => {
                error!(error = ?e, "Failed to load event for retry");
                return;
            }
        };

        let max_attempts = subscription.retry_limit.unwrap_or(self.policy.max_attempts);
        let current_attempt = delivery.attempts + 1;

        match self.dispatcher.dispatch(
            &subscription.callback_address,
            &event,
            subscription.secret.as_deref(),
            subscription.headers.as_ref(),
        ).await {
            Ok(status) if status.is_success() => {
                info!(delivery_id = %delivery.id, status = %status, "Retry delivery succeeded");
                let _ = self.delivery_repo.mark_delivered(&delivery.id, status.as_u16()).await;
            }
            Ok(status) => {
                let err_msg = format!("HTTP {}", status);
                let is_retryable = RetryPolicy::is_retryable_status(status.as_u16());
                self.handle_failure(&delivery, &event, &subscription, current_attempt, max_attempts, is_retryable, &err_msg, Some(status.as_u16())).await;
            }
            Err(err_msg) => {
                self.handle_failure(&delivery, &event, &subscription, current_attempt, max_attempts, true, &err_msg, None).await;
            }
        }
    }

    async fn handle_failure(
        &self,
        delivery: &EventDeliveryRecord,
        event: &EventEnvelope,
        subscription: &SubscriptionRecord,
        attempt: u32,
        max_attempts: u32,
        is_retryable: bool,
        error_msg: &str,
        status_code: Option<u16>,
    ) {
        if is_retryable && attempt < max_attempts {
            let next_retry = self.policy.calculate_next_retry(attempt);
            warn!(
                delivery_id = %delivery.id,
                attempt,
                max_attempts,
                next_retry = %next_retry,
                error = %error_msg,
                "Delivery attempt failed, scheduling next retry"
            );
            let _ = self.delivery_repo.record_failed_attempt(
                &delivery.id,
                attempt,
                Some(next_retry),
                error_msg,
                status_code,
            ).await;
        } else {
            error!(
                delivery_id = %delivery.id,
                attempt,
                max_attempts,
                error = %error_msg,
                "Delivery exhausted or non-retryable; routing to dead-letter queue"
            );
            let _ = self.delivery_repo.record_failed_attempt(
                &delivery.id,
                attempt,
                None,
                error_msg,
                status_code,
            ).await;
            let _ = self.delivery_repo.mark_dead_letter(&delivery.id).await;

            let dlq_record = DeadLetterRecord {
                id: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
                delivery_id: Some(delivery.id.clone()),
                event_id: delivery.event_id.clone(),
                subscription_id: delivery.subscription_id.clone(),
                topic: event.topic.to_string(),
                callback_address: subscription.callback_address.clone(),
                payload: event.payload.clone(),
                error_message: error_msg.to_string(),
                attempts: attempt,
                status: DeadLetterStatus::Unresolved,
                failed_at: Utc::now(),
                replayed_at: None,
            };
            let _ = self.dlq_repo.create_dead_letter(&dlq_record).await;
        }
    }

    fn clone_worker(&self) -> Self {
        Self {
            event_repo: self.event_repo.clone(),
            subscription_repo: self.subscription_repo.clone(),
            delivery_repo: self.delivery_repo.clone(),
            dlq_repo: self.dlq_repo.clone(),
            dispatcher: self.dispatcher.clone(),
            policy: self.policy.clone(),
            concurrency_limit: self.concurrency_limit,
        }
    }
}
