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

use crate::config::services::SsiAuthConfig;
use crate::config::traits::{ApiConfigTrait, CommonConfigTrait, ExtraHostsTrait};
use crate::config::types::{CommonHostsConfig, HostType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinKnownConfig {
    pub hosts: CommonHostsConfig,
    pub api_version: String,
}

impl MinKnownConfig {
    pub fn get_host(&self, host_type: HostType) -> String {
        self.hosts.get_host(host_type)
    }
    pub fn get_api_version(&self) -> String {
        format!("/api/{}", self.api_version)
    }
}

impl From<SsiAuthConfig> for MinKnownConfig {
    fn from(value: SsiAuthConfig) -> Self {
        Self { hosts: value.common().hosts.clone(), api_version: value.get_api_version() }
    }
}
