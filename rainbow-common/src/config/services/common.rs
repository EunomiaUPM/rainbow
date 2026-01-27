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

use serde::{Deserialize, Serialize};
use ymir::config::traits::{
    ApiConfigTrait, DatabaseConfigTrait, HostsConfigTrait, IsLocalConfigTrait,
};
use ymir::config::types::{ApiConfig, CommonHostsConfig, DatabaseConfig, HostConfig};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommonConfig {
    pub hosts: CommonHostsConfig,
    pub db: DatabaseConfig,
    pub api: ApiConfig,
    pub is_local: bool,
}

impl CommonConfig {
    pub fn hosts(&self) -> &CommonHostsConfig {
        &self.hosts
    }
    pub fn db(&self) -> &DatabaseConfig {
        &self.db
    }
    pub fn api(&self) -> &ApiConfig {
        &self.api
    }
    pub fn is_local(&self) -> bool {
        self.is_local
    }
}

impl HostsConfigTrait for CommonConfig {
    fn http(&self) -> &HostConfig {
        &self.hosts().http()
    }

    fn grpc(&self) -> Option<&HostConfig> {
        self.hosts().grpc()
    }

    fn graphql(&self) -> Option<&HostConfig> {
        self.hosts().graphql()
    }
}

impl DatabaseConfigTrait for CommonConfig {
    fn db(&self) -> &DatabaseConfig {
        &self.db
    }
}

impl IsLocalConfigTrait for CommonConfig {
    fn is_local(&self) -> bool {
        self.is_local
    }
}

impl ApiConfigTrait for CommonConfig {
    fn api(&self) -> &ApiConfig {
        &self.api
    }
}
