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

use crate::transfer_provider::entities::agreements;
use crate::transfer_provider::entities::transfer_message;
use crate::transfer_provider::entities::transfer_process;
use crate::transfer_provider::repo::sql::TransferProviderRepoForSql;
use once_cell::sync::Lazy;
use rainbow_common::config::config::GLOBAL_CONFIG;
use rainbow_common::protocol::transfer::{TransferRoles, TransferStateForDb};
use sea_orm_migration::async_trait::async_trait;
use uuid::Uuid;

pub trait CombinedRepo: TransferProcessRepo + TransferMessagesRepo + AgreementsRepo {}
impl<T> CombinedRepo for T where T: TransferProcessRepo + TransferMessagesRepo + AgreementsRepo {}
pub static TRANSFER_PROVIDER_REPO: Lazy<Box<dyn CombinedRepo + Send + Sync>> = Lazy::new(|| {
    let repo_type = GLOBAL_CONFIG.get().unwrap().db_type.clone();
    match repo_type.as_str() {
        "postgres" => Box::new(TransferProviderRepoForSql {}),
        "memory" => Box::new(TransferProviderRepoForSql {}),
        "mysql" => Box::new(TransferProviderRepoForSql {}),
        _ => panic!("Unknown REPO_TYPE: {}", repo_type),
    }
});

pub struct NewTransferProcessModel {
    pub provider_pid: Uuid,
    pub consumer_pid: Uuid,
    pub agreement_id: Uuid,
    pub data_plane_id: Uuid,
}

pub struct EditTransferProcessModel {
    pub provider_pid: Option<Uuid>,
    pub consumer_pid: Option<Uuid>,
    pub agreement_id: Option<Uuid>,
    pub data_plane_id: Option<Uuid>,
    pub state: Option<TransferStateForDb>,
}

#[async_trait]
pub trait TransferProcessRepo {
    async fn get_all_transfer_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_process::Model>>;
    async fn get_transfer_process_by_provider(
        &self,
        pid: Uuid,
    ) -> anyhow::Result<Option<transfer_process::Model>>;
    async fn get_transfer_process_by_consumer(
        &self,
        pid: Uuid,
    ) -> anyhow::Result<Option<transfer_process::Model>>;
    async fn put_transfer_process(
        &self,
        pid: Uuid,
        new_transfer_process: EditTransferProcessModel,
    ) -> anyhow::Result<transfer_process::Model>;
    async fn create_transfer_process(
        &self,
        new_transfer_process: NewTransferProcessModel,
    ) -> anyhow::Result<transfer_process::Model>;
    async fn delete_transfer_process(&self, pid: Uuid) -> anyhow::Result<()>;
}

pub struct NewTransferMessageModel {
    pub message_type: String,
    pub from: TransferRoles,
    pub to: TransferRoles,
    pub content: serde_json::Value,
}

#[async_trait]
pub trait TransferMessagesRepo {
    async fn get_all_transfer_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_message::Model>>;

    async fn get_all_transfer_messages_by_provider(
        &self,
        pid: Uuid,
    ) -> anyhow::Result<Vec<transfer_message::Model>>;

    async fn get_transfer_message_by_id(
        &self,
        pid: Uuid,
    ) -> anyhow::Result<Option<transfer_message::Model>>;
    async fn put_transfer_message(
        &self,
        pid: Uuid,
        new_transfer_message: transfer_message::ActiveModel,
    ) -> anyhow::Result<Option<transfer_message::Model>>;
    async fn create_transfer_message(
        &self,
        pid: Uuid,
        new_transfer_message: NewTransferMessageModel,
    ) -> anyhow::Result<transfer_message::Model>;
    async fn delete_transfer_message(&self, pid: Uuid) -> anyhow::Result<()>;
}

pub struct NewAgreementModel {
    pub data_service_id: Uuid,
    pub identity: Option<String>,
    pub identity_token: Option<String>,
}

pub struct EditAgreementModel {
    pub data_service_id: Option<Uuid>,
    pub identity: Option<String>,
    pub identity_token: Option<String>,
}

#[async_trait]
pub trait AgreementsRepo {
    async fn get_all_agreements(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<agreements::Model>>;
    async fn get_agreement_by_id(&self, id: Uuid) -> anyhow::Result<Option<agreements::Model>>;
    async fn put_agreement(
        &self,
        id: Uuid,
        new_agreement: EditAgreementModel,
    ) -> anyhow::Result<agreements::Model>;
    async fn create_agreement(
        &self,
        new_agreement: NewAgreementModel,
    ) -> anyhow::Result<agreements::Model>;
    async fn delete_agreement(&self, id: Uuid) -> anyhow::Result<()>;
}

impl Default for EditTransferProcessModel {
    fn default() -> Self {
        EditTransferProcessModel {
            provider_pid: None,
            consumer_pid: None,
            agreement_id: None,
            data_plane_id: None,
            state: None,
        }
    }
}