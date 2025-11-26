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

use rainbow_common::config::global_config::HostConfig;
use crate::ssi::provider::config::{AuthProviderConfig, AuthProviderConfigTrait};
use super::BusinessConfigTrait;

pub struct BusinessConfig {
    host: HostConfig,
    is_local: bool,
    api_path: String,
}

impl From<AuthProviderConfig> for BusinessConfig {
    fn from(config: AuthProviderConfig) -> Self {
        let api_path = config.get_api_path();
        Self {
            host: config.common_config.host,
            is_local: config.common_config.is_local,
            api_path,
        }
    }
}

impl BusinessConfigTrait for BusinessConfig {
    fn get_host(&self) -> String {
        let host = self.host.clone();
        match host.port.is_empty() {
            true => {
                format!("{}://{}", host.protocol, host.url)
            }
            false => {
                format!("{}://{}:{}", host.protocol, host.url, host.port)
            }
        }
    }
    fn is_local(&self) -> bool {
        self.is_local
    }
    fn get_api_path(&self) -> String {
        self.api_path.clone()
    }
}
