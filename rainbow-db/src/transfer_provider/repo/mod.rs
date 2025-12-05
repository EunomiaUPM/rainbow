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

use crate::transfer_provider::entities::transfer_message;
use crate::transfer_provider::entities::transfer_process;
use anyhow::Error;
use rainbow_common::protocol::transfer::{TransferRoles, TransferState, TransferStateAttribute};
use sea_orm::DatabaseConnection;
use sea_orm_migration::async_trait::async_trait;
use thiserror::Error;
use urn::Urn;
use rainbow_common::dcat_formats::DctFormats;

pub mod sql;

#[async_trait]
pub trait TransferProviderRepoFactory: TransferProcessRepo + TransferMessagesRepo + Send + Sync + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

pub struct NewTransferProcessModel {
    pub provider_pid: Urn,
    pub consumer_pid: Urn,
    pub agreement_id: Urn,
    pub callback_address: String,
    pub associated_consumer: Option<String>,
    pub format: DctFormats,
}

pub struct EditTransferProcessModel {
    pub provider_pid: Option<Urn>,
    pub consumer_pid: Option<Urn>,
    pub agreement_id: Option<Urn>,
    pub callback_address: Option<String>,
    pub state: Option<TransferState>,
    pub state_attribute: Option<TransferStateAttribute>,
}

#[async_trait]
pub trait TransferProcessRepo {
    async fn get_all_transfer_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_process::Model>, TransferProviderRepoErrors>;
    async fn get_batch_transfer_processes(
        &self,
        transfer_ids: &Vec<Urn>,
    ) -> Result<Vec<transfer_process::Model>, TransferProviderRepoErrors>;
    async fn get_transfer_process_by_provider(
        &self,
        pid: Urn,
    ) -> anyhow::Result<Option<transfer_process::Model>, TransferProviderRepoErrors>;
    async fn get_transfer_process_by_consumer(
        &self,
        pid: Urn,
    ) -> anyhow::Result<Option<transfer_process::Model>, TransferProviderRepoErrors>;
    async fn get_transfer_process_by_data_plane(
        &self,
        pid: Urn,
    ) -> anyhow::Result<Option<transfer_process::Model>, TransferProviderRepoErrors>;
    async fn put_transfer_process(
        &self,
        pid: Urn,
        edit_transfer_process: EditTransferProcessModel,
    ) -> anyhow::Result<transfer_process::Model, TransferProviderRepoErrors>;
    async fn create_transfer_process(
        &self,
        new_transfer_process: NewTransferProcessModel,
    ) -> anyhow::Result<transfer_process::Model, TransferProviderRepoErrors>;
    async fn delete_transfer_process(&self, pid: Urn) -> anyhow::Result<(), TransferProviderRepoErrors>;
}

pub struct NewTransferMessageModel {
    pub message_type: String,
    pub from: TransferRoles,
    pub to: TransferRoles,
    pub content: serde_json::Value,
}

pub struct EditTransferMessageModel {}

#[async_trait]
pub trait TransferMessagesRepo {
    async fn get_all_transfer_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferProviderRepoErrors>;

    async fn get_all_transfer_messages_by_provider(
        &self,
        pid: Urn,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferProviderRepoErrors>;

    async fn get_transfer_message_by_id(
        &self,
        pid: Urn,
        mid: Urn,
    ) -> anyhow::Result<Option<transfer_message::Model>, TransferProviderRepoErrors>;
    async fn put_transfer_message(
        &self,
        pid: Urn,
        edit_transfer_message: EditTransferMessageModel,
    ) -> anyhow::Result<Option<transfer_message::Model>, TransferProviderRepoErrors>;
    async fn create_transfer_message(
        &self,
        pid: Urn,
        new_transfer_message: NewTransferMessageModel,
    ) -> anyhow::Result<transfer_message::Model, TransferProviderRepoErrors>;
    async fn delete_transfer_message(&self, pid: Urn) -> anyhow::Result<(), TransferProviderRepoErrors>;
}

#[derive(Debug, Error)]
pub enum TransferProviderRepoErrors {
    #[error("Provider Transfer Process not found")]
    ProviderTransferProcessNotFound,
    #[error("Provider Transfer Message not found")]
    ProviderTransferMessageNotFound,

    #[error("Error fetching provider transfer process. {0}")]
    ErrorFetchingProviderTransferProcess(Error),
    #[error("Error fetching provider transfer message. {0}")]
    ErrorFetchingProviderTransferMessage(Error),

    #[error("Error creating provider transfer process. {0}")]
    ErrorCreatingProviderTransferProcess(Error),
    #[error("Error creating provider transfer message. {0}")]
    ErrorCreatingProviderTransferMessage(Error),

    #[error("Error deleting provider transfer process. {0}")]
    ErrorDeletingProviderTransferProcess(Error),
    #[error("Error deleting provider transfer message. {0}")]
    ErrorDeletingProviderTransferMessage(Error),

    #[error("Error updating provider transfer process. {0}")]
    ErrorUpdatingProviderTransferProcess(Error),
    #[error("Error updating provider transfer message. {0}")]
    ErrorUpdatingProviderTransferMessage(Error),
}

impl Default for EditTransferProcessModel {
    fn default() -> Self {
        EditTransferProcessModel {
            provider_pid: None,
            consumer_pid: None,
            agreement_id: None,
            callback_address: None,
            state: None,
            state_attribute: None,
        }
    }
}
