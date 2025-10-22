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

use axum::async_trait;
use rainbow_db::transfer_provider::entities::{transfer_message, transfer_process};
use urn::Urn;

pub mod rainbow_entities;

#[mockall::automock]
#[async_trait]
pub trait RainbowTransferProviderServiceTrait: Send + Sync {
    async fn get_all_transfers(&self) -> anyhow::Result<Vec<transfer_process::Model>>;
    async fn get_batch_transfers(&self, transfer_ids: &Vec<Urn>) -> anyhow::Result<Vec<transfer_process::Model>>;
    async fn get_transfer_by_id(
        &self,
        provider_pid: Urn,
    ) -> anyhow::Result<transfer_process::Model>;
    async fn get_transfer_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<transfer_process::Model>;
    async fn get_messages_by_transfer(
        &self,
        transfer_id: Urn,
    ) -> anyhow::Result<Vec<transfer_message::Model>>;
    async fn get_messages_by_id(
        &self,
        transfer_id: Urn,
        message_id: Urn,
    ) -> anyhow::Result<transfer_message::Model>;
}