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

use crate::ssi_auth_provider::entities::ssi_auth_data;
use crate::ssi_auth_provider::repo::sql::AuthDataRepo;
use axum::async_trait;
use once_cell::sync::Lazy;
use rainbow_common::config::config::GLOBAL_CONFIG;

pub mod sql;

pub static SSI_AUTH_PR_REPO: Lazy<Box<dyn AuthDataRepoTrait + Send + Sync>> = Lazy::new(|| {
    let repo_type = GLOBAL_CONFIG.get().unwrap().db_type.clone();
    match repo_type.as_str() {
        "postgres" => Box::new(AuthDataRepo {}),
        "memory" => Box::new(AuthDataRepo {}),
        "mysql" => Box::new(AuthDataRepo {}),
        _ => panic!("Unknown REPO_TYPE: {}", repo_type),
    }
});

#[async_trait]
pub trait AuthDataRepoTrait {
    async fn get_all_ssi_auth_data(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<ssi_auth_data::Model>>;
    async fn get_ssi_auth_data_by_id(
        &self,
        id: i64,
    ) -> anyhow::Result<Option<ssi_auth_data::Model>>;
    async fn create_ssi_auth_data(&self) -> anyhow::Result<ssi_auth_data::Model>;
    async fn update_status_ssi_auth_data(
        &self,
        id: i64,
        status: ssi_auth_data::Status,
    ) -> anyhow::Result<ssi_auth_data::Model>;
    async fn delete_ssi_auth_data(&self, id: i64) -> anyhow::Result<ssi_auth_data::Model>;
}
