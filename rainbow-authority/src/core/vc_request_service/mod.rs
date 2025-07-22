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

use crate::core::vc_request_service::vc_request_types::VCRequest;
use crate::data::entities::vc_requests;
use axum::async_trait;
use urn::Urn;

pub mod vc_request_service;
pub mod vc_request_types;

#[mockall::automock]
#[async_trait]
pub trait VCRequestTrait: Send + Sync {
    async fn get_all_vc_requests(&self) -> anyhow::Result<Vec<vc_requests::Model>>;
    async fn get_vc_request_by_id(&self, vc_request_id: Urn) -> anyhow::Result<vc_requests::Model>;
    async fn validate_vc_request_by_id(&self, vc_request_id: Urn) -> anyhow::Result<vc_requests::Model>;
    async fn reject_vc_request_by_id(&self, vc_request_id: Urn) -> anyhow::Result<vc_requests::Model>;
    async fn create_vc_request(&self, input: VCRequest) -> anyhow::Result<vc_requests::Model>;
}