/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use rainbow_common::config::services::SsiAuthConfig;
use rainbow_common::config::traits::{ApiConfigTrait, CommonConfigTrait};
use rainbow_common::config::types::{ClientConfig, CommonHostsConfig};
use serde_json::Value;

use super::VCRequesterConfigTrait;
use crate::ssi::utils::get_pretty_client_config_helper;

pub struct VCRequesterConfig {
    hosts: CommonHostsConfig,
    client: ClientConfig,
    api_path: String
}

impl From<SsiAuthConfig> for VCRequesterConfig {
    fn from(value: SsiAuthConfig) -> Self {
        VCRequesterConfig {
            hosts: value.common().hosts.clone(),
            client: value.client(),
            api_path: value.get_api_version()
        }
    }
}

impl VCRequesterConfigTrait for VCRequesterConfig {
    fn get_pretty_client_config(&self, cert: &str) -> anyhow::Result<Value> {
        get_pretty_client_config_helper(&self.client, &cert)
    }
    fn hosts(&self) -> &CommonHostsConfig { &self.hosts }
    fn get_api_path(&self) -> String { self.api_path.clone() }
}
