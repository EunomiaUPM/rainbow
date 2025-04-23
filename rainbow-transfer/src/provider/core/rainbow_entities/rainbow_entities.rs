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

use crate::provider::core::rainbow_entities::rainbow_err::RainbowTransferProviderErrors;
use crate::provider::core::rainbow_entities::RainbowTransferProviderServiceTrait;
use axum::async_trait;
use rainbow_db::transfer_provider::entities::{transfer_message, transfer_process};
use rainbow_db::transfer_provider::repo::TransferProviderRepoFactory;
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use std::sync::Arc;
use urn::Urn;

pub struct RainbowTransferProviderServiceImpl<T, U>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: RainbowEventsNotificationTrait + Sync + Send,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> RainbowTransferProviderServiceImpl<T, U>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: RainbowEventsNotificationTrait + Sync + Send,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, notification_service }
    }
}

#[async_trait]
impl<T, U> RainbowTransferProviderServiceTrait for RainbowTransferProviderServiceImpl<T, U>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: RainbowEventsNotificationTrait + Sync + Send,
{
    async fn get_all_transfers(&self) -> anyhow::Result<Vec<transfer_process::Model>> {
        let transfer_processes = self.repo
            .get_all_transfer_processes(None, None)
            .await
            .map_err(RainbowTransferProviderErrors::DbErr)?;
        Ok(transfer_processes)
    }

    async fn get_transfer_by_id(
        &self,
        provider_pid: Urn,
    ) -> anyhow::Result<transfer_process::Model> {
        let transfer_processes = self.repo
            .get_transfer_process_by_provider(provider_pid.clone())
            .await
            .map_err(RainbowTransferProviderErrors::DbErr)?
            .ok_or(RainbowTransferProviderErrors::ProcessNotFound {
                provider_pid: Option::from(provider_pid),
                consumer_pid: None,
            })?;

        Ok(transfer_processes)
    }

    async fn get_transfer_by_consumer_id(&self, consumer_id: Urn) -> anyhow::Result<transfer_process::Model> {
        let transfer_processes = self.repo
            .get_transfer_process_by_consumer(consumer_id.clone())
            .await
            .map_err(RainbowTransferProviderErrors::DbErr)?
            .ok_or(RainbowTransferProviderErrors::ProcessNotFound {
                provider_pid: None,
                consumer_pid: Option::from(consumer_id),
            })?;
        Ok(transfer_processes)
    }

    async fn get_messages_by_transfer(
        &self,
        transfer_id: Urn,
    ) -> anyhow::Result<Vec<transfer_message::Model>> {
        let messages = self.repo
            .get_all_transfer_messages_by_provider(transfer_id)
            .await
            .map_err(RainbowTransferProviderErrors::DbErr)?;
        Ok(messages)
    }

    async fn get_messages_by_id(
        &self,
        transfer_id: Urn,
        message_id: Urn,
    ) -> anyhow::Result<transfer_message::Model> {
        let message = self.repo
            .get_transfer_message_by_id(transfer_id.clone(), message_id.clone())
            .await
            .map_err(RainbowTransferProviderErrors::DbErr)?
            .ok_or(RainbowTransferProviderErrors::MessageNotFound {
                transfer_id: Option::from(transfer_id),
                message_id: Option::from(message_id),
            })?;

        Ok(message)
    }
}
