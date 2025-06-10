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

use crate::transfer_consumer::entities::{transfer_callback, transfer_message};
use anyhow::Error;
use axum::async_trait;
use rainbow_common::protocol::transfer::TransferRoles;
use sea_orm::DatabaseConnection;
use thiserror::Error;
use urn::Urn;

pub mod sql;


pub trait TransferConsumerRepoFactory:
TransferCallbackRepo + TransferMessagesConsumerRepo + Send + Sync + 'static
{
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

pub struct NewTransferCallback {
    pub callback_id: Option<Urn>,
    pub consumer_pid: Option<Urn>,
    pub provider_pid: Option<Urn>,
    pub data_address: Option<serde_json::Value>,
    pub associated_provider: Option<Urn>,
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

pub struct NewTransferMessageModel {
    pub message_type: String,
    pub from: TransferRoles,
    pub to: TransferRoles,
    pub content: serde_json::Value,
}

pub struct EditTransferMessageModel {}

#[async_trait]
pub trait TransferMessagesConsumerRepo {
    async fn get_all_transfer_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferConsumerRepoErrors>;

    async fn get_all_transfer_messages_by_consumer(
        &self,
        pid: Urn,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferConsumerRepoErrors>;

    async fn get_transfer_message_by_id(
        &self,
        pid: Urn,
        mid: Urn,
    ) -> anyhow::Result<Option<transfer_message::Model>, TransferConsumerRepoErrors>;
    async fn put_transfer_message(
        &self,
        pid: Urn,
        edit_transfer_message: EditTransferMessageModel,
    ) -> anyhow::Result<Option<transfer_message::Model>, TransferConsumerRepoErrors>;
    async fn create_transfer_message(
        &self,
        pid: Urn,
        new_transfer_message: NewTransferMessageModel,
    ) -> anyhow::Result<transfer_message::Model, TransferConsumerRepoErrors>;
    async fn delete_transfer_message(&self, pid: Urn) -> anyhow::Result<(), TransferConsumerRepoErrors>;
}

#[derive(Debug, Error)]
pub enum TransferConsumerRepoErrors {
    #[error("Consumer Transfer Process not found")]
    ConsumerTransferProcessNotFound,
    #[error("Consumer Transfer Message not found")]
    ConsumerTransferMessageNotFound,

    #[error("Error fetching consumer transfer process. {0}")]
    ErrorFetchingConsumerTransferProcess(Error),
    #[error("Error fetching consumer transfer message. {0}")]
    ErrorFetchingConsumerTransferMessage(Error),
    #[error("Error creating consumer transfer process. {0}")]
    ErrorCreatingConsumerTransferProcess(Error),
    #[error("Error creating consumer transfer message. {0}")]
    ErrorCreatingConsumerTransferMessage(Error),
    #[error("Error deleting consumer transfer process. {0}")]
    ErrorDeletingConsumerTransferProcess(Error),
    #[error("Error deleting consumer transfer message. {0}")]
    ErrorDeletingConsumerTransferMessage(Error),
    #[error("Error updating consumer transfer process. {0}")]
    ErrorUpdatingConsumerTransferProcess(Error),
    #[error("Error updating consumer transfer message. {0}")]
    ErrorUpdatingConsumerTransferMessage(Error),
}

impl Default for EditTransferCallback {
    fn default() -> Self {
        Self { consumer_pid: None, provider_pid: None, data_plane_id: None, data_address: None }
    }
}
