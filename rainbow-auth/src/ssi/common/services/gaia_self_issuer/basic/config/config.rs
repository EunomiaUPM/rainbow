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

use super::GaiaGaiaSelfIssuerConfigTrait;
use crate::ssi::common::types::enums::VcDataModelVersion;
use crate::ssi::consumer::config::{AuthConsumerConfig, AuthConsumerConfigTrait};
use crate::ssi::provider::config::{AuthProviderConfig, AuthProviderConfigTrait};
use rainbow_common::config::global_config::HostConfig;
use rainbow_common::utils::read;

pub struct GaiaSelfIssuerConfig {
    host: HostConfig,
    is_local: bool,
    keys_path: String,
    api_path: String,
    vc_data_model: VcDataModelVersion,
}

impl From<AuthConsumerConfig> for GaiaSelfIssuerConfig {
    fn from(value: AuthConsumerConfig) -> Self {
        let api_path = value.get_api_path();
        Self {
            host: value.common_config.host,
            is_local: value.common_config.is_local,
            keys_path: value.common_config.keys_path,
            api_path,
            vc_data_model: VcDataModelVersion::V1,
        }
    }
}

impl From<AuthProviderConfig> for GaiaSelfIssuerConfig {
    fn from(value: AuthProviderConfig) -> Self {
        let api_path = value.get_api_path();
        Self {
            host: value.common_config.host,
            is_local: value.common_config.is_local,
            keys_path: value.common_config.keys_path,
            api_path,
            vc_data_model: VcDataModelVersion::V1,
        }
    }
}

impl GaiaGaiaSelfIssuerConfigTrait for GaiaSelfIssuerConfig {
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

    fn get_host_without_protocol(&self) -> String {
        let host = self.host.clone();
        match host.port.is_empty() {
            true => {
                format!("{}:{}", host.url, host.port)
            }
            false => {
                format!("{}", host.url,)
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
    fn get_api_path(&self) -> String {
        self.api_path.clone()
    }
    fn get_data_model_version(&self) -> VcDataModelVersion {
        self.vc_data_model.clone()
    }
}
