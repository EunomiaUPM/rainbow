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
pub mod sql;
mod authority_factory;
mod basic_repo_trait;
mod authority_repo_trait;

use super::entities::{auth, auth_interaction, auth_verification};
use axum::async_trait;
use sea_orm::DatabaseConnection;

pub trait AuthorityRepoFactory: Send + Sync + Clone + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Box<dyn BasicRepoTrait>
    where
        T: Send + Sync + 'static;
}



#[async_trait]
pub trait AuthRepoTrait {
    async fn get_all_auths(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<auth::Model>>;
    async fn get_auth_by_id(&self, id: &str) -> anyhow::Result<Option<auth::Model>>;
    async fn create_auth(&self, model: auth::Model) -> anyhow::Result<auth::Model>;
    async fn delete_auth(&self, id: &str) -> anyhow::Result<()>;
    async fn edit_auth(&self, model: auth::Model) -> anyhow::Result<auth::Model>;
}

#[async_trait]
pub trait AuthInteractionRepoTrait {
    async fn get_all_auths_int(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<auth_interaction::Model>>;
    async fn get_auth_int_by_id(&self, id: &str) -> anyhow::Result<Option<auth_interaction::Model>>;
    async fn create_auth_int(&self, model: auth_interaction::Model) -> anyhow::Result<auth_interaction::Model>;
    async fn delete_auth_int(&self, id: &str) -> anyhow::Result<()>;
    async fn edit_auth_int(&self, model: auth_interaction::Model) -> anyhow::Result<auth_interaction::Model>;
}

#[async_trait]
pub trait AuthVerificationRepoTrait {
    async fn get_all_auths_ver(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<auth_verification::Model>>;
    async fn get_auth_ver_by_id(&self, id: &str) -> anyhow::Result<Option<auth_verification::Model>>;
    async fn create_auth_ver(&self, model: auth_verification::Model) -> anyhow::Result<auth_verification::Model>;
    async fn delete_auth_ver(&self, id: &str) -> anyhow::Result<()>;
    async fn edit_auth_ver(&self, model: auth_verification::Model) -> anyhow::Result<auth_verification::Model>;
}
