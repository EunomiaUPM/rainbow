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

use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupAcceptanceRequest, SetupAcceptanceResponse, SetupRequestRequest, SetupRequestResponse,
    SetupTerminationRequest, SetupTerminationResponse, SetupVerificationRequest, SetupVerificationResponse,
};
use axum::async_trait;

pub mod ds_protocol_rpc;
pub mod ds_protocol_rpc_errors;
pub mod ds_protocol_rpc_types;

#[mockall::automock]
#[async_trait]
pub trait DSRPCContractNegotiationConsumerTrait: Send + Sync {
    async fn setup_request(&self, input: SetupRequestRequest) -> anyhow::Result<SetupRequestResponse>;
    async fn setup_rerequest(&self, input: SetupRequestRequest) -> anyhow::Result<SetupRequestResponse>;
    async fn setup_acceptance(&self, input: SetupAcceptanceRequest) -> anyhow::Result<SetupAcceptanceResponse>;
    async fn setup_verification(&self, input: SetupVerificationRequest) -> anyhow::Result<SetupVerificationResponse>;
    async fn setup_termination(&self, input: SetupTerminationRequest) -> anyhow::Result<SetupTerminationResponse>;
}
