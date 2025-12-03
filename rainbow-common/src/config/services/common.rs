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

use crate::config::types::database::DatabaseConfig;
use crate::config::types::roles::RoleConfig;
use crate::config::types::{ApiConfig, CommonHostsConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommonConfig {
    pub hosts: CommonHostsConfig,
    pub db: DatabaseConfig,
    pub role: RoleConfig,
    pub api: ApiConfig,
    pub keys_path: String,
    pub is_local: bool,
}

impl CommonConfig {
    pub fn hosts(&self) -> &CommonHostsConfig {
        &self.hosts
    }
    pub fn db(&self) -> &DatabaseConfig {
        &self.db
    }
    pub fn role(&self) -> &RoleConfig {
        &self.role
    }
    pub fn api(&self) -> &ApiConfig {
        &self.api
    }
    pub fn keys_path(&self) -> &str {
        &self.keys_path
    }
    pub fn is_local(&self) -> bool {
        self.is_local
    }
}
