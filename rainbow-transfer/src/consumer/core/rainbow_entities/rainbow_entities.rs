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

use crate::consumer::core::rainbow_entities::rainbow_types::{EditTransferConsumerRequest, NewTransferConsumerRequest};
use crate::consumer::core::rainbow_entities::RainbowTransferConsumerServiceTrait;
use anyhow::anyhow;
use axum::async_trait;
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::transfer::transfer_consumer_process::TransferConsumerProcess;
use rainbow_db::transfer_consumer::entities::transfer_callback;
use rainbow_db::transfer_consumer::entities::transfer_message;
use rainbow_db::transfer_consumer::repo::{TransferConsumerRepoErrors, TransferConsumerRepoFactory};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowTransferConsumerServiceImpl<T>
where
    T: TransferConsumerRepoFactory + Send + Sync,
{
    repo: Arc<T>,
}

impl<T> RainbowTransferConsumerServiceImpl<T>
where
    T: TransferConsumerRepoFactory + Send + Sync,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> RainbowTransferConsumerServiceTrait for RainbowTransferConsumerServiceImpl<T>
where
    T: TransferConsumerRepoFactory + Send + Sync,
{
    async fn get_all_transfers(&self) -> anyhow::Result<Vec<TransferConsumerProcess>> {
        let transfer_processes = self.repo.get_all_transfer_callbacks(None, None).await.map_err(|e| {
            let e = CommonErrors::database_new(Some(e.to_string()));
            error!("{}", e.log());
            anyhow!(e)
        })?;
        let transfer_processes =
            transfer_processes.iter().map(|t| TransferConsumerProcess::from(t.to_owned())).collect();
        Ok(transfer_processes)
    }

    async fn get_transfer_by_id(&self, process_id: Urn) -> anyhow::Result<TransferConsumerProcess> {
        let transfer_process = self
            .repo
            .get_transfer_callbacks_by_id(process_id.clone())
            .await
            .map_err(|e| {
                let e = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", e.log());
                anyhow!(e)
            })?
            .ok_or_else(|| {
                let e = CommonErrors::missing_resource_new(
                    process_id.clone().to_string(),
                    Some("Transfer process not found".to_string()),
                );
                error!("{}", e.log());
                anyhow!(e)
            })?;
        let transfer_process = TransferConsumerProcess::from(transfer_process);
        Ok(transfer_process)
    }

    async fn get_transfer_by_consumer_id(&self, consumer_pid: Urn) -> anyhow::Result<TransferConsumerProcess> {
        let transfer_process = self
            .repo
            .get_transfer_callback_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(|e| {
                let e = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", e.log());
                anyhow!(e)
            })?
            .ok_or_else(|| {
                let e = CommonErrors::missing_resource_new(
                    consumer_pid.clone().to_string(),
                    Some("Transfer process not found".to_string()),
                );
                error!("{}", e.log());
                anyhow!(e)
            })?;
        let transfer_process = TransferConsumerProcess::from(transfer_process);
        Ok(transfer_process)
    }

    async fn get_transfer_by_provider_id(&self, provider_pid: Urn) -> anyhow::Result<TransferConsumerProcess> {
        let transfer_process = self
            .repo
            .get_transfer_callbacks_by_id(provider_pid.clone())
            .await
            .map_err(|e| {
                let e = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", e.log());
                anyhow!(e)
            })?
            .ok_or_else(|| {
                let e = CommonErrors::missing_resource_new(
                    provider_pid.clone().to_string(),
                    Some("Transfer process not found".to_string()),
                );
                error!("{}", e.log());
                anyhow!(e)
            })?;
        let transfer_process = TransferConsumerProcess::from(transfer_process);
        Ok(transfer_process)
    }

    async fn put_transfer_by_id(
        &self,
        process_id: Urn,
        edit_transfer: EditTransferConsumerRequest,
    ) -> anyhow::Result<transfer_callback::Model> {
        let transfer_process =
            self.repo.put_transfer_callback(process_id.clone(), edit_transfer.into()).await.map_err(|e| match e {
                TransferConsumerRepoErrors::ConsumerTransferProcessNotFound => {
                    let e = CommonErrors::missing_resource_new(
                        process_id.clone().to_string(),
                        Some("Transfer process not found".to_string()),
                    );
                    error!("{}", e.log());
                    anyhow!(e)
                }
                e => {
                    let e = CommonErrors::database_new(Some(e.to_string()));
                    error!("{}", e.log());
                    anyhow!(e)
                }
            })?;
        Ok(transfer_process)
    }

    async fn create_transfer(
        &self,
        new_transfer: NewTransferConsumerRequest,
    ) -> anyhow::Result<transfer_callback::Model> {
        let transfer_process = self.repo.create_transfer_callback(new_transfer.into()).await.map_err(|e| {
            let e = CommonErrors::database_new(Some(e.to_string()));
            error!("{}", e.log());
            anyhow!(e)
        })?;
        Ok(transfer_process)
    }

    async fn delete_transfer(&self, process_id: Urn) -> anyhow::Result<()> {
        let _ = self.repo.delete_transfer_callback(process_id.clone()).await.map_err(|e| match e {
            TransferConsumerRepoErrors::ConsumerTransferProcessNotFound => {
                let e_ = CommonErrors::missing_resource_new(
                    process_id.clone().to_string(),
                    Some("Transfer process not found".to_string()),
                );
                error!("{}", e_.log());
                anyhow!(e_)
            }
            _ => {
                let e_ = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", e_.log());
                anyhow!(e_)
            }
        })?;
        Ok(())
    }
    async fn get_messages_by_transfer(&self, transfer_id: Urn) -> anyhow::Result<Vec<transfer_message::Model>> {
        let messages = self.repo.get_all_transfer_messages_by_consumer(transfer_id.clone()).await.map_err(|e| {
            let e = CommonErrors::database_new(Some(e.to_string()));
            error!("{}", e.log());
            anyhow!(e)
        })?;
        Ok(messages)
    }

    async fn get_messages_by_id(&self, transfer_id: Urn, message_id: Urn) -> anyhow::Result<transfer_message::Model> {
        let message = self
            .repo
            .get_transfer_message_by_id(transfer_id.clone(), message_id.clone())
            .await
            .map_err(|e| match e {
                TransferConsumerRepoErrors::ConsumerTransferProcessNotFound => {
                    let e = CommonErrors::missing_resource_new(
                        transfer_id.clone().to_string(),
                        Some("Transfer process not found".to_string()),
                    );
                    error!("{}", e.log());
                    anyhow!(e)
                }
                e => {
                    let e = CommonErrors::database_new(Some(e.to_string()));
                    error!("{}", e.log());
                    anyhow!(e)
                }
            })?
            .ok_or_else(|| {
                let e = CommonErrors::missing_resource_new(
                    message_id.clone().to_string(),
                    Some("Transfer message not found".to_string()),
                );
                error!("{}", e.log());
                anyhow!(e)
            })?;

        Ok(message)
    }
}
