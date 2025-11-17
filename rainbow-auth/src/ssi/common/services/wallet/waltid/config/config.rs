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
use serde_json::{json, Value};
use rainbow_common::config::global_config::HostConfig;
use crate::ssi::common::services::wallet::waltid::config::config_trait::WaltIdConfigTrait;
use rainbow_common::ssi::WalletConfig;
use crate::ssi::consumer::config::AuthConsumerConfig;
use crate::ssi::provider::config::AuthProviderConfig;
use crate::ssi::common::utils::read;

#[derive(Clone)]
pub struct WaltIdConfig {
    host: HostConfig,
    ssi_wallet_config: WalletConfig,
    keys_path: String,
}

impl From<AuthProviderConfig> for WaltIdConfig {
    fn from(value: AuthProviderConfig) -> Self {
        WaltIdConfig {
            host: value.common_config.host,
            ssi_wallet_config: value.common_config.ssi_wallet_config,
            keys_path: value.common_config.keys_path,
        }
    }
}

impl From<AuthConsumerConfig> for WaltIdConfig {
    fn from(value: AuthConsumerConfig) -> Self {
        WaltIdConfig {
            host: value.common_config.host,
            ssi_wallet_config: value.common_config.ssi_wallet_config,
            keys_path: value.common_config.keys_path,
        }
    }
}

impl WaltIdConfigTrait for WaltIdConfig {
    fn get_raw_wallet_config(&self) -> WalletConfig {
        self.ssi_wallet_config.clone()
    }
    fn get_wallet_api_url(&self) -> String {
        let data = self.get_raw_wallet_config();
        match data.api_port {
            Some(port) => {
                format!("{}://{}:{}", data.api_protocol, data.api_url, port)
            }
            None => {
                format!("{}://{}", data.api_protocol, data.api_url)
            }
        }
    }
    fn get_wallet_register_data(&self) -> Value {
        let data = self.get_raw_wallet_config();
        json!({
            "type": data.r#type,
            "name": data.name,
            "email": data.email,
            "password": data.password,
        })
    }
    fn get_wallet_login_data(&self) -> Value {
        let data = self.get_raw_wallet_config();
        json!({
            "type": data.r#type,
            "email": data.email,
            "password": data.password,
        })
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
}


