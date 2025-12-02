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
pub struct CatalogConfig {
    hosts: CommonHostsConfig,
    is_datahub: bool,
    datahub_host: Option<HostConfig>,
    datahub_token: Option<String>,
    database: DatabaseConfig,
    api: ApiConfig,
    role: RoleConfig,
    is_local: bool,
    keys_path: String,
}

impl HostConfigTrait for CatalogConfig {
    fn http(&self) -> &HostConfig {
        &self.hosts.http
    }
}

impl DatabaseConfigTrait for CatalogConfig {}

impl IsLocalTrait for CatalogConfig {
    fn is_local(&self) -> bool {
        self.is_local
    }
}

impl RoleTrait for CatalogConfig {
    fn role(&self) -> RoleConfig {
        self.role.clone()
    }
}

impl KeysPathTrait for CatalogConfig {
    fn keys_path(&self) -> String {
        self.keys_path.clone()
    }
}

impl CommonConfigTraits for CatalogConfig {}

impl CatalogConfig {
    pub fn is_datahub(&self) -> bool {
        self.is_datahub
    }
}
