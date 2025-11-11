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
use crate::services::issuer::basic::config::config_trait::BasicIssuerConfigTrait;
use crate::setup::AuthorityApplicationConfig;
use crate::types::host::HostConfig;
use crate::utils::read;

pub struct BasicIssuerConfig {
    host: HostConfig,
    is_local: bool,
    keys_path: String,
}

impl From<AuthorityApplicationConfig> for BasicIssuerConfig {
    fn from(config: AuthorityApplicationConfig) -> BasicIssuerConfig {
        BasicIssuerConfig { host: config.authority_host, is_local: config.is_local, keys_path: config.keys_path }
    }
}

impl BasicIssuerConfigTrait for BasicIssuerConfig {
    fn get_host_without_protocol(&self) -> String {
        let host = self.host.clone();
        match host.port {
            Some(port) => {
                format!("{}:{}", host.url, port)
            }
            None => {
                format!("{}", host.url,)
            }
        }
    }

    fn get_host(&self) -> String {
        let host = self.host.clone();
        match host.port {
            Some(port) => {
                format!("{}://{}:{}", host.protocol, host.url, port)
            }
            None => {
                format!("{}://{}", host.protocol, host.url)
            }
        }
    }

    fn is_local(&self) -> bool {
        self.is_local
    }

    fn get_cert(&self) -> anyhow::Result<String> {
        let path = format!("{}/cert.pem", self.keys_path);
        read(&path)
    }
    fn get_priv_key(&self) -> anyhow::Result<String> {
        let path = format!("{}/private_key.pem", self.keys_path);
        read(&path)
    }
    fn get_pub_key(&self) -> anyhow::Result<String> {
        let path = format!("{}/public_key.pem", self.keys_path);
        read(&path)
    }
}
