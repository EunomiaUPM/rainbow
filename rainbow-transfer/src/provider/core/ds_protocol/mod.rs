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
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_request::TransferRequestMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use urn::Urn;

pub mod ds_protocol;

#[mockall::automock]
#[async_trait]
pub trait DSProtocolTransferProviderTrait: Send + Sync {
    async fn get_transfer_requests_by_provider(&self, provider_pid: Urn) -> anyhow::Result<TransferProcessMessage>;
    async fn get_transfer_requests_by_consumer(&self, consumer_pid: Urn) -> anyhow::Result<Option<TransferProcessMessage>>;
    async fn transfer_request(&self, input: TransferRequestMessage, token: String) -> anyhow::Result<TransferProcessMessage>;
    async fn transfer_start(
        &self,
        provider_pid: Urn,
        input: TransferStartMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage>;
    async fn transfer_suspension(
        &self,
        provider_pid: Urn,
        input: TransferSuspensionMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage>;
    async fn transfer_completion(
        &self,
        provider_pid: Urn,
        input: TransferCompletionMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage>;
    async fn transfer_termination(
        &self,
        provider_pid: Urn,
        input: TransferTerminationMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage>;
}
