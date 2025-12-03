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
use crate::ssi::common::utils::{get_host_url, get_pretty_client_config_helper};
use crate::ssi::consumer::services::onboarder::gnap::config::GnapOnboarderConfigTrait;
use rainbow_common::config::services::SsiAuthConfig;
use rainbow_common::config::traits::{ApiConfigTrait, CommonConfigTrait};
use rainbow_common::config::types::{ClientConfig, CommonHostsConfig};
use serde_json::Value;

pub struct GnapOnboarderConfig {
    host: CommonHostsConfig,
    client: ClientConfig,
    keys_path: String,
    api_path: String,
}

impl From<SsiAuthConfig> for GnapOnboarderConfig {
    fn from(value: SsiAuthConfig) -> Self {
        GnapOnboarderConfig {
            host: value.common().hosts.clone(),
            client: value.client(),
            keys_path: value.common().keys_path.clone(),
            api_path: value.get_api_path().clone(),
        }
    }
}

impl GnapOnboarderConfigTrait for GnapOnboarderConfig {
    fn get_pretty_client_config(&self) -> anyhow::Result<Value> {
        let path = format!("{}/cert.pem", self.keys_path);
        get_pretty_client_config_helper(&self.client, &path)
    }
    fn get_host(&self) -> String {
        get_host_url(&self.host.http)
    }
    fn get_api_path(&self) -> String {
        self.api_path.clone()
    }
}
