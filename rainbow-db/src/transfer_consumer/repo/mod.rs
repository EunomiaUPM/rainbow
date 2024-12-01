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
use crate::transfer_consumer::repo::sql::TransferCallbackRepoForSql;
use crate::transfer_provider::repo::{AgreementsRepo, TransferMessagesRepo, TransferProcessRepo};
use axum::async_trait;
use once_cell::sync::Lazy;
use rainbow_common::config::config::GLOBAL_CONFIG;
use uuid::Uuid;

pub trait CombinedRepo: TransferCallbackRepo {}
impl<T> CombinedRepo for T where T: TransferCallbackRepo {}
pub static TRANSFER_CONSUMER_REPO: Lazy<Box<dyn CombinedRepo + Send + Sync>> = Lazy::new(|| {
    let repo_type = GLOBAL_CONFIG.get().unwrap().db_type.clone();
    match repo_type.as_str() {
        "postgres" => Box::new(TransferCallbackRepoForSql {}),
        "memory" => Box::new(TransferCallbackRepoForSql {}),
        "mysql" => Box::new(TransferCallbackRepoForSql {}),
        _ => panic!("Unknown REPO_TYPE: {}", repo_type),
    }
});

pub struct NewTransferCallback {
    pub data_address: Option<serde_json::Value>,
}
pub struct EditTransferCallback {
    pub consumer_pid: Option<Uuid>,
    pub provider_pid: Option<Uuid>,
    pub data_plane_id: Option<Uuid>,
    pub data_address: Option<serde_json::Value>,
}

#[async_trait]
pub trait TransferCallbackRepo {
    async fn get_all_transfer_callbacks(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_callback::Model>>;
    async fn get_transfer_callbacks_by_id(
        &self,
        callback_id: Uuid,
    ) -> anyhow::Result<Option<transfer_callback::Model>>;

    async fn get_transfer_callbacks_by_consumer_id(
        &self,
        consumer_pid: Uuid,
    ) -> anyhow::Result<Option<transfer_callback::Model>>;

    async fn put_transfer_callback(
        &self,
        callback_id: Uuid,
        new_transfer_callback: EditTransferCallback,
    ) -> anyhow::Result<transfer_callback::Model>;

    async fn put_transfer_callback_by_consumer(
        &self,
        callback_id: Uuid,
        new_transfer_callback: EditTransferCallback,
    ) -> anyhow::Result<transfer_callback::Model>;

    async fn create_transfer_callback(
        &self,
        new_transfer_callback: NewTransferCallback,
    ) -> anyhow::Result<transfer_callback::Model>;

    async fn delete_transfer_callback(&self, callback_id: Uuid) -> anyhow::Result<()>;
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