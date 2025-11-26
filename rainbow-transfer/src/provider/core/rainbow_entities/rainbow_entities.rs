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

use crate::provider::core::rainbow_entities::RainbowTransferProviderServiceTrait;
use axum::async_trait;
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_db::transfer_provider::entities::transfer_process::Model;
use rainbow_db::transfer_provider::entities::{transfer_message, transfer_process};
use rainbow_db::transfer_provider::repo::{TransferProviderRepoErrors, TransferProviderRepoFactory};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use std::sync::Arc;
use urn::Urn;

pub struct RainbowTransferProviderServiceImpl<T, U>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: RainbowEventsNotificationTrait + Sync + Send,
{
    repo: Arc<T>,
    _notification_service: Arc<U>,
}

impl<T, U> RainbowTransferProviderServiceImpl<T, U>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: RainbowEventsNotificationTrait + Sync + Send,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, _notification_service: notification_service }
    }
}

#[async_trait]
impl<T, U> RainbowTransferProviderServiceTrait for RainbowTransferProviderServiceImpl<T, U>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: RainbowEventsNotificationTrait + Sync + Send,
{
    async fn get_all_transfers(&self) -> anyhow::Result<Vec<transfer_process::Model>> {
        let transfer_processes = self.repo.get_all_transfer_processes(None, None).await.map_err(|e| {
            let e = CommonErrors::database_new(&e.to_string());
            error!("{}", e.log());
            e
        })?;
        Ok(transfer_processes)
    }

    async fn get_batch_transfers(&self, transfer_ids: &Vec<Urn>) -> anyhow::Result<Vec<Model>> {
        let transfer_processes = self.repo.get_batch_transfer_processes(transfer_ids).await.map_err(|e| {
            let e = CommonErrors::database_new(&e.to_string());
            error!("{}", e.log());
            e
        })?;
        Ok(transfer_processes)
    }

    async fn get_transfer_by_id(&self, provider_pid: Urn) -> anyhow::Result<transfer_process::Model> {
        let transfer_processes = self
            .repo
            .get_transfer_process_by_provider(provider_pid.clone())
            .await
            .map_err(|e| {
                let e = CommonErrors::database_new(&e.to_string());
                error!("{}", e.log());
                e
            })?
            .ok_or_else(|| {
                let e = CommonErrors::missing_resource_new(&provider_pid.to_string(), "Transfer process not found");
                error!("{}", e.log());
                e
            })?;

        Ok(transfer_processes)
    }

    async fn get_transfer_by_consumer_id(&self, consumer_id: Urn) -> anyhow::Result<transfer_process::Model> {
        let transfer_processes = self
            .repo
            .get_transfer_process_by_consumer(consumer_id.clone())
            .await
            .map_err(|e| {
                let e = CommonErrors::database_new(&e.to_string());
                error!("{}", e.log());
                e
            })?
            .ok_or_else(|| {
                let e = CommonErrors::missing_resource_new(&consumer_id.to_string(), "Transfer process not found");
                error!("{}", e.log());
                e
            })?;

        Ok(transfer_processes)
    }

    async fn get_messages_by_transfer(&self, transfer_id: Urn) -> anyhow::Result<Vec<transfer_message::Model>> {
        let messages =
            self.repo.get_all_transfer_messages_by_provider(transfer_id.clone()).await.map_err(|e| match e {
                TransferProviderRepoErrors::ProviderTransferProcessNotFound => {
                    let e_ = CommonErrors::missing_resource_new(&transfer_id.to_string(), "Transfer process not found");
                    error!("{}", e_.log());
                    e_
                }
                _ => {
                    let e_ = CommonErrors::database_new(&e.to_string());
                    error!("{}", e_.log());
                    e_
                }
            })?;
        Ok(messages)
    }

    async fn get_messages_by_id(&self, transfer_id: Urn, message_id: Urn) -> anyhow::Result<transfer_message::Model> {
        let message = self
            .repo
            .get_transfer_message_by_id(transfer_id.clone(), message_id.clone())
            .await
            .map_err(|e| match e {
                TransferProviderRepoErrors::ProviderTransferProcessNotFound => {
                    let e_ = CommonErrors::missing_resource_new(&transfer_id.to_string(), "Transfer process not found");
                    error!("{}", e_.log());
                    e_
                }
                _ => {
                    let e_ = CommonErrors::database_new(&e.to_string());
                    error!("{}", e_.log());
                    e_
                }
            })?
            .ok_or_else(|| {
                let e_ = CommonErrors::missing_resource_new(&message_id.to_string(), "Transfer message not found");
                error!("{}", e_.log());
                e_
            })?;
        Ok(message)
    }
}