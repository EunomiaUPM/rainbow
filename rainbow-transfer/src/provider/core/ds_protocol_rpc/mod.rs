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
use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::{DSRPCTransferProviderCompletionRequest, DSRPCTransferProviderCompletionResponse, DSRPCTransferProviderStartRequest, DSRPCTransferProviderStartResponse, DSRPCTransferProviderSuspensionRequest, DSRPCTransferProviderSuspensionResponse, DSRPCTransferProviderTerminationRequest, DSRPCTransferProviderTerminationResponse};
use axum::async_trait;

pub mod ds_protocol_rpc_types;
pub mod ds_protocol_rpc;
pub mod ds_protocol_rpc_err;

#[mockall::automock]
#[async_trait]
pub trait DSRPCTransferProviderTrait: Send + Sync {
    async fn setup_start(&self, input: DSRPCTransferProviderStartRequest) -> anyhow::Result<DSRPCTransferProviderStartResponse>;
    async fn setup_suspension(&self, input: DSRPCTransferProviderSuspensionRequest) -> anyhow::Result<DSRPCTransferProviderSuspensionResponse>;
    async fn setup_completion(&self, input: DSRPCTransferProviderCompletionRequest) -> anyhow::Result<DSRPCTransferProviderCompletionResponse>;
    async fn setup_termination(&self, input: DSRPCTransferProviderTerminationRequest) -> anyhow::Result<DSRPCTransferProviderTerminationResponse>;
}
