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

use crate::config::traits::HostConfigTrait;
use crate::config::types::database::DatabaseConfig;
use crate::config::types::roles::RoleConfig;
use crate::config::types::{ApiConfig, ClientConfig, CommonHostsConfig, HostConfig, WalletConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SsiAuthConfig {
    hosts: CommonHostsConfig,
    wallet: WalletConfig,
    client: ClientConfig,
    database: DatabaseConfig,
    api: ApiConfig,
    keys_path: String,
    role: RoleConfig,
    is_local: bool,
}

impl HostConfigTrait for SsiAuthConfig {
    fn http(&self) -> &HostConfig {
        &self.hosts.http
    }
}
