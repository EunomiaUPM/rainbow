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

use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupAgreementRequest, SetupAgreementResponse, SetupFinalizationRequest, SetupFinalizationResponse,
    SetupOfferRequest, SetupOfferResponse, SetupTerminationRequest, SetupTerminationResponse,
};
use axum::async_trait;

pub mod ds_protocol_rpc;
pub mod ds_protocol_rpc_errors;
pub mod ds_protocol_rpc_types;

#[mockall::automock]
#[async_trait]
pub trait DSRPCContractNegotiationProviderTrait: Send + Sync {
    async fn setup_offer(&self, input: SetupOfferRequest) -> anyhow::Result<SetupOfferResponse>;
    async fn setup_reoffer(&self, input: SetupOfferRequest) -> anyhow::Result<SetupOfferResponse>;
    async fn setup_agreement(&self, input: SetupAgreementRequest) -> anyhow::Result<SetupAgreementResponse>;
    async fn setup_finalization(&self, input: SetupFinalizationRequest) -> anyhow::Result<SetupFinalizationResponse>;
    async fn setup_termination(&self, input: SetupTerminationRequest) -> anyhow::Result<SetupTerminationResponse>;
}
