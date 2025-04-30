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

use crate::auth_consumer::entities::auth_interaction;
use crate::auth_consumer::entities::{auth, auth_verification};
use axum::async_trait;
use rainbow_common::auth::Interact4GR;
use sea_orm::DatabaseConnection;

pub mod sql;

pub trait AuthConsumerRepoFactory: AuthConsumerRepoTrait + Send + Sync + Clone + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

#[async_trait]
pub trait AuthConsumerRepoTrait {
    async fn get_all_auths(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<auth::Model>>;

    async fn get_auth_by_id(&self, id: String) -> anyhow::Result<auth::Model>;

    async fn create_auth(
        &self,
        provider: String,
        actions: Vec<String>,
        interact: Interact4GR,
    ) -> anyhow::Result<auth::Model>;

    async fn auth_accepted(&self, id: String, assigned_id: String) -> anyhow::Result<auth::Model>;

    async fn delete_auth(&self, id: String) -> anyhow::Result<auth::Model>;

    async fn get_interaction_by_id(&self, id: String) -> anyhow::Result<auth_interaction::Model>;

    async fn create_auth_verification(&self, id: String, uri: String) -> anyhow::Result<auth_verification::Model>;
}
