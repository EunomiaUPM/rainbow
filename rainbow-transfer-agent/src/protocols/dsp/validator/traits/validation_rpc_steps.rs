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

#![allow(unused)]
use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcTransferCompletionMessageDto, RpcTransferRequestMessageDto, RpcTransferStartMessageDto,
    RpcTransferSuspensionMessageDto, RpcTransferTerminationMessageDto,
};

#[async_trait::async_trait]
pub trait ValidationRpcSteps: Send + Sync + 'static {
    async fn transfer_request_rpc(
        &self,
        input: &RpcTransferRequestMessageDto,
    ) -> anyhow::Result<()>;
    async fn transfer_start_rpc(&self, input: &RpcTransferStartMessageDto) -> anyhow::Result<()>;
    async fn transfer_completion_rpc(
        &self,
        input: &RpcTransferCompletionMessageDto,
    ) -> anyhow::Result<()>;
    async fn transfer_suspension_rpc(
        &self,
        input: &RpcTransferSuspensionMessageDto,
    ) -> anyhow::Result<()>;
    async fn transfer_termination_rpc(
        &self,
        input: &RpcTransferTerminationMessageDto,
    ) -> anyhow::Result<()>;
}
