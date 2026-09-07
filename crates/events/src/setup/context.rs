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

use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio_util::sync::CancellationToken;

use crate::bus::policy::RetryPolicy;
use crate::bus::worker::RetryWorker;
use crate::bus::EventBus;
use crate::data::repo::in_memory::InMemoryEventBusRepo;
use crate::data::repo::sea_orm_repo::SeaOrmEventBusRepo;
use crate::data::repo::{
    EventDeadLetterRepo, EventDeliveryRepo, EventStoreRepo, EventSubscriptionRepo,
};
use crate::setup::workers::RetryWorkerHandle;

/// Shared application context holding the event bus, repositories, and worker handles.
#[derive(Clone)]
pub struct AppContext {
    pub db: Option<DatabaseConnection>,
    pub event_bus: Arc<EventBus>,
    pub event_repo: Arc<dyn EventStoreRepo>,
    pub subscription_repo: Arc<dyn EventSubscriptionRepo>,
    pub delivery_repo: Arc<dyn EventDeliveryRepo>,
    pub dlq_repo: Arc<dyn EventDeadLetterRepo>,
    pub retry_worker: Arc<RetryWorker>,
    pub cancel_token: CancellationToken,
}

impl AppContext {
    /// Build context with real SeaORM database connection.
    pub fn build(db: DatabaseConnection, policy: Option<RetryPolicy>) -> Self {
        let policy = policy.unwrap_or_default();
        let sea_repo = Arc::new(SeaOrmEventBusRepo::new(db.clone()));

        let event_repo: Arc<dyn EventStoreRepo> = sea_repo.clone();
        let subscription_repo: Arc<dyn EventSubscriptionRepo> = sea_repo.clone();
        let delivery_repo: Arc<dyn EventDeliveryRepo> = sea_repo.clone();
        let dlq_repo: Arc<dyn EventDeadLetterRepo> = sea_repo;

        let event_bus = Arc::new(EventBus::new(
            event_repo.clone(),
            subscription_repo.clone(),
            delivery_repo.clone(),
            dlq_repo.clone(),
            policy.clone(),
            1024,
        ));

        let retry_worker = Arc::new(RetryWorker::new(
            event_repo.clone(),
            subscription_repo.clone(),
            delivery_repo.clone(),
            dlq_repo.clone(),
            event_bus.dispatcher(),
            policy,
        ));

        Self {
            db: Some(db),
            event_bus,
            event_repo,
            subscription_repo,
            delivery_repo,
            dlq_repo,
            retry_worker,
            cancel_token: CancellationToken::new(),
        }
    }

    /// Build context with in-memory repositories for testing.
    pub fn in_memory(policy: Option<RetryPolicy>) -> Self {
        let policy = policy.unwrap_or_default();
        let mem_repo = Arc::new(InMemoryEventBusRepo::new());

        let event_repo: Arc<dyn EventStoreRepo> = mem_repo.clone();
        let subscription_repo: Arc<dyn EventSubscriptionRepo> = mem_repo.clone();
        let delivery_repo: Arc<dyn EventDeliveryRepo> = mem_repo.clone();
        let dlq_repo: Arc<dyn EventDeadLetterRepo> = mem_repo;

        let event_bus = Arc::new(EventBus::new(
            event_repo.clone(),
            subscription_repo.clone(),
            delivery_repo.clone(),
            dlq_repo.clone(),
            policy.clone(),
            1024,
        ));

        let retry_worker = Arc::new(RetryWorker::new(
            event_repo.clone(),
            subscription_repo.clone(),
            delivery_repo.clone(),
            dlq_repo.clone(),
            event_bus.dispatcher(),
            policy,
        ));

        Self {
            db: None,
            event_bus,
            event_repo,
            subscription_repo,
            delivery_repo,
            dlq_repo,
            retry_worker,
            cancel_token: CancellationToken::new(),
        }
    }

    /// Spawn the background retry worker task.
    pub fn spawn_retry_worker(&self) -> RetryWorkerHandle {
        RetryWorkerHandle::spawn(self.retry_worker.clone(), self.cancel_token.clone())
    }
}
