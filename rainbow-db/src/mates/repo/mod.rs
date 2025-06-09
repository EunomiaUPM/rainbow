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

use crate::mates::entities::mates;
use axum::async_trait;
use rainbow_common::mates::Mates;
use sea_orm::DatabaseConnection;

pub mod sql;

pub trait MateRepoFactory: MateRepoTrait + Send + Sync + Clone + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

#[async_trait]
pub trait MateRepoTrait {
    async fn get_all_mates(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<mates::Model>>;
    async fn get_mate_by_id(&self, id: String) -> anyhow::Result<mates::Model>;
    async fn get_mate_me(&self) -> anyhow::Result<Option<mates::Model>>;
    async fn create_mate(&self, mate: Mates, is_me: bool) -> anyhow::Result<mates::Model>;
    async fn update_mate(&self, mate: Mates) -> anyhow::Result<mates::Model>;
    async fn delete_mate(&self, id: String) -> anyhow::Result<()>;
}
