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

pub mod rainbow_entities;
pub mod rainbow_err;
pub mod rainbow_types;

use crate::consumer::core::rainbow_entities::rainbow_types::{EditTransferConsumerRequest, NewTransferConsumerRequest};
use axum::async_trait;
use rainbow_db::transfer_consumer::entities::transfer_callback;
use urn::Urn;

#[mockall::automock]
#[async_trait]
pub trait RainbowTransferConsumerServiceTrait: Send + Sync {
    async fn get_all_transfers(&self) -> anyhow::Result<Vec<transfer_callback::Model>>;
    async fn get_transfer_by_id(&self, process_id: Urn) -> anyhow::Result<transfer_callback::Model>;
    async fn get_transfer_by_consumer_id(&self, consumer_pid: Urn) -> anyhow::Result<transfer_callback::Model>;
    async fn get_transfer_by_provider_id(&self, provider_pid: Urn) -> anyhow::Result<transfer_callback::Model>;
    async fn put_transfer_by_id(
        &self,
        process_id: Urn,
        edit_transfer: EditTransferConsumerRequest,
    ) -> anyhow::Result<transfer_callback::Model>;
    async fn create_transfer(
        &self,
        new_transfer: NewTransferConsumerRequest,
    ) -> anyhow::Result<transfer_callback::Model>;
    async fn delete_transfer(&self, process_id: Urn) -> anyhow::Result<()>;
}
