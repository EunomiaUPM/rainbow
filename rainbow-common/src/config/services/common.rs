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
use crate::config::traits::{
    CommonConfigTraits, DatabaseConfigTrait, HostConfigTrait, IsLocalTrait, KeysPathTrait, RoleTrait,
};
use crate::config::types::database::DatabaseConfig;
use crate::config::types::roles::RoleConfig;
use crate::config::types::{ApiConfig, CommonHostsConfig, HostConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommonConfig {
    hosts: CommonHostsConfig,
    database: DatabaseConfig,
    role: RoleConfig,
    api: ApiConfig,
    keys_path: String,
    is_local: bool,
}

impl HostConfigTrait for CommonConfig {
    fn http(&self) -> &HostConfig {
        &self.hosts.http
    }
}

impl DatabaseConfigTrait for CommonConfig {}

impl IsLocalTrait for CommonConfig {
    fn is_local(&self) -> bool {
        self.is_local
    }
}

impl RoleTrait for CommonConfig {
    fn role(&self) -> &RoleConfig {
        &self.role
    }
}

impl KeysPathTrait for CommonConfig {
    fn keys_path(&self) -> &str {
        &self.keys_path
    }
}

impl CommonConfigTraits for CommonConfig {}
