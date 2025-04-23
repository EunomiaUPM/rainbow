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

use crate::auth_provider::entities::{auth, auth_interaction, auth_verification};
use crate::auth_provider::repo::sql::AuthProviderRepo;
use axum::async_trait;
use once_cell::sync::Lazy;
use rainbow_common::auth::Interact4GR;
use rainbow_common::config::config::GLOBAL_CONFIG;

pub mod sql;

pub static AUTH_PROVIDER_REPO: Lazy<Box<dyn AuthProviderRepoTrait + Send + Sync>> =
    Lazy::new(|| {
        let repo_type = GLOBAL_CONFIG.get().unwrap().db_type.clone();
        match repo_type.as_str() {
            "postgres" => Box::new(AuthProviderRepo {}),
            "memory" => Box::new(AuthProviderRepo {}),
            "mysql" => Box::new(AuthProviderRepo {}),
            _ => panic!("Unknown REPO_TYPE: {}", repo_type),
        }
    });

#[async_trait]
pub trait AuthProviderRepoTrait {
    async fn get_all_auths(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<auth::Model>>;
    async fn get_auth_by_id(&self, id: String) -> anyhow::Result<auth::Model>;
    async fn create_auth(
        &self,
        consumer: String,
        actions: Vec<String>,
        interact: Interact4GR,
    ) -> anyhow::Result<(
        auth::Model,
        auth_interaction::Model,
        auth_verification::Model,
    )>;
    async fn update_auth_status(
        &self,
        id: String,
        status: auth::Status,
    ) -> anyhow::Result<auth::Model>;
    async fn delete_auth(&self, id: String) -> anyhow::Result<auth::Model>;

    async fn get_interaction_by_id(&self, id: String) -> anyhow::Result<auth_interaction::Model>;

    async fn get_auth_by_state(&self, state: String) -> anyhow::Result<String>;

    async fn get_av_by_id_update_holder(
        &self,
        id: String,
        vpt: String,
        holder: String,
    ) -> anyhow::Result<auth_verification::Model>;

    async fn update_verification_result(&self, id: String, result: bool) -> anyhow::Result<()>;
}
