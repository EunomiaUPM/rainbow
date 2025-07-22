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

use crate::data::entities::vc_requests;
use anyhow::Error;
use axum::async_trait;
use sea_orm::DatabaseConnection;
use thiserror::Error;
use urn::Urn;

pub mod sql;

pub trait VCRequestsFactory: VCRequestsRepo + Send + Sync + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

pub struct NewVCRequestModel {
    pub vc_content: serde_json::Value,
    pub state: Option<String>,
}

pub struct EditVCRequestModel {
    pub state: Option<String>,
}

#[async_trait]
pub trait VCRequestsRepo {
    async fn get_all_vc_requests(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<vc_requests::Model>, VCRequestsRepoErrors>;
    async fn get_all_vc_request_by_id(
        &self,
        id: Urn,
    ) -> anyhow::Result<vc_requests::Model, VCRequestsRepoErrors>;
    async fn put_vc_request(
        &self,
        pid: Urn,
        edit_vc_request: EditVCRequestModel,
    ) -> anyhow::Result<vc_requests::Model, VCRequestsRepoErrors>;
    async fn create_vc_request(
        &self,
        new_vc_request: NewVCRequestModel,
    ) -> anyhow::Result<vc_requests::Model, VCRequestsRepoErrors>;
    async fn delete_vc_request(&self, pid: Urn) -> anyhow::Result<(), VCRequestsRepoErrors>;
}

#[derive(Debug, Error)]
pub enum VCRequestsRepoErrors {
    #[error("Verifiable credential request not found")]
    VCRequestNotFound,
    #[error("Error fetching verifiable credential request. {0}")]
    ErrorFetchingVCRequest(Error),
    #[error("Error creating verifiable credential request. {0}")]
    ErrorCreatingVCRequest(Error),
    #[error("Error deleting verifiable credential request. {0}")]
    ErrorDeletingVCRequest(Error),
    #[error("Error updating verifiable credential request. {0}")]
    ErrorUpdatingVCRequest(Error),
}