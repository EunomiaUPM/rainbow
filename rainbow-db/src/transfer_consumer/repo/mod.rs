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

pub mod sql;

use crate::transfer_consumer::entities::transfer_callback;
use crate::transfer_provider::repo::{TransferMessagesRepo, TransferProcessRepo};
use anyhow::Error;
use axum::async_trait;
use sea_orm::DatabaseConnection;
use thiserror::Error;
use urn::Urn;


pub trait TransferConsumerRepoFactory: TransferCallbackRepo + Send + Sync + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}


pub struct NewTransferCallback {
    pub callback_id: Option<Urn>,
    pub consumer_pid: Option<Urn>,
    pub provider_pid: Option<Urn>,
    pub data_address: Option<serde_json::Value>,
}
pub struct EditTransferCallback {
    pub consumer_pid: Option<Urn>,
    pub provider_pid: Option<Urn>,
    pub data_plane_id: Option<Urn>,
    pub data_address: Option<serde_json::Value>,
}

#[async_trait]
pub trait TransferCallbackRepo {
    async fn get_all_transfer_callbacks(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_callback::Model>, TransferConsumerRepoErrors>;
    async fn get_transfer_callbacks_by_id(
        &self,
        callback_id: Urn,
    ) -> anyhow::Result<Option<transfer_callback::Model>, TransferConsumerRepoErrors>;

    async fn get_transfer_callback_by_consumer_id(
        &self,
        consumer_pid: Urn,
    ) -> anyhow::Result<Option<transfer_callback::Model>, TransferConsumerRepoErrors>;

    async fn get_transfer_callback_by_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Option<transfer_callback::Model>, TransferConsumerRepoErrors>;

    async fn put_transfer_callback(
        &self,
        callback_id: Urn,
        new_transfer_callback: EditTransferCallback,
    ) -> anyhow::Result<transfer_callback::Model, TransferConsumerRepoErrors>;

    async fn put_transfer_callback_by_consumer(
        &self,
        callback_id: Urn,
        new_transfer_callback: EditTransferCallback,
    ) -> anyhow::Result<transfer_callback::Model, TransferConsumerRepoErrors>;

    async fn create_transfer_callback(
        &self,
        new_transfer_callback: NewTransferCallback,
    ) -> anyhow::Result<transfer_callback::Model, TransferConsumerRepoErrors>;

    async fn delete_transfer_callback(&self, callback_id: Urn) -> anyhow::Result<(), TransferConsumerRepoErrors>;
}

#[derive(Debug, Error)]
pub enum TransferConsumerRepoErrors {
    #[error("Consumer Transfer Process not found")]
    ConsumerTransferProcessNotFound,

    #[error("Error fetching consumer transfer process. {0}")]
    ErrorFetchingConsumerTransferProcess(Error),
    #[error("Error creating consumer transfer process. {0}")]
    ErrorCreatingConsumerTransferProcess(Error),
    #[error("Error deleting consumer transfer process. {0}")]
    ErrorDeletingConsumerTransferProcess(Error),
    #[error("Error updating consumer transfer process. {0}")]
    ErrorUpdatingConsumerTransferProcess(Error),
}

impl Default for EditTransferCallback {
    fn default() -> Self {
        Self {
            consumer_pid: None,
            provider_pid: None,
            data_plane_id: None,
            data_address: None,
        }
    }
}