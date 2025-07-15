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

use crate::mates::entities::busmates;
use crate::mates::entities::mates;
use anyhow::Error;
use axum::async_trait;
use rainbow_common::mates::mates::VerifyTokenRequest;
use rainbow_common::mates::BusMates;
use rainbow_common::mates::Mates;
use sea_orm::DatabaseConnection;
use thiserror::Error;

pub mod sql;

pub trait MateRepoFactory: MateRepoTrait + BusmateRepoTrait + Send + Sync + Clone + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

#[async_trait]
pub trait MateRepoTrait {
    async fn get_all_mates(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<mates::Model>, MateRepoErrors>;
    async fn get_mate_by_id(&self, id: String) -> anyhow::Result<mates::Model, MateRepoErrors>;
    async fn get_mate_me(&self) -> anyhow::Result<Option<mates::Model>, MateRepoErrors>;
    async fn get_mate_by_token(
        &self,
        verify_token_request: VerifyTokenRequest,
    ) -> anyhow::Result<mates::Model, MateRepoErrors>;
    async fn create_mate(&self, mate: Mates) -> anyhow::Result<mates::Model, MateRepoErrors>;
    async fn update_mate(&self, mate: Mates) -> anyhow::Result<mates::Model, MateRepoErrors>;
    async fn delete_mate(&self, id: String) -> anyhow::Result<(), MateRepoErrors>;
}

#[derive(Debug, Error)]
pub enum MateRepoErrors {
    #[error("Mate not found")]
    MateNotFound,

    #[error("Error fetching mate. {0}")]
    ErrorFetchingMates(Error),
    #[error("Error creating mate. {0}")]
    ErrorCreatingMates(Error),
    #[error("Error deleting mate. {0}")]
    ErrorDeletingMates(Error),
    #[error("Error updating mate. {0}")]
    ErrorUpdatingMates(Error),
}

#[async_trait]
pub trait BusmateRepoTrait {
    async fn get_all_busmates(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<busmates::Model>, BusmateRepoErrors>;
    async fn get_busmate_by_id(&self, id: String) -> anyhow::Result<busmates::Model, BusmateRepoErrors>;
    async fn create_busmate(&self, mate: BusMates) -> anyhow::Result<busmates::Model, BusmateRepoErrors>;
    async fn update_busmate(&self, mate: BusMates) -> anyhow::Result<busmates::Model, BusmateRepoErrors>;
    async fn delete_busmate(&self, id: String) -> anyhow::Result<(), BusmateRepoErrors>;
}

#[derive(Debug, Error)]
pub enum BusmateRepoErrors {
    #[error("Busmate not found")]
    BusmateNotFound,

    #[error("Error fetching busmate. {0}")]
    ErrorFetchingBusmates(Error),
    #[error("Error creating busmate. {0}")]
    ErrorCreatingBusmates(Error),
    #[error("Error deleting busmate. {0}")]
    ErrorDeletingBusmates(Error),
    #[error("Error updating busmate. {0}")]
    ErrorUpdatingBusmates(Error),
}
