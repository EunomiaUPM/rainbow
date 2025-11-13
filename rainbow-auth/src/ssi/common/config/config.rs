/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use crate::ssi::common::config::CommonConfigTrait;
use crate::ssi::common::types::wallet::WalletConfig;
use crate::ssi::utils::read;
use rainbow_common::config::database::DbType;
use rainbow_common::config::global_config::{extract_env, option_extract_env, DatabaseConfig, HostConfig};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Clone, Serialize, Debug)]
pub struct CommonAuthConfig {
    pub host: HostConfig,
    pub database_config: DatabaseConfig,
    pub ssi_wallet_config: WalletConfig,
    pub keys_path: String,
    pub is_local: bool,
}

impl Default for CommonAuthConfig {
    fn default() -> Self {
        CommonAuthConfig {
            host: HostConfig { protocol: "http".to_string(), url: "127.0.0.1".to_string(), port: "1100".to_string() },
            database_config: DatabaseConfig {
                db_type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "1301".to_string(),
                user: "ds_consumer".to_string(),
                password: "ds_consumer".to_string(),
                name: "ds_consumer".to_string(),
            },
            ssi_wallet_config: WalletConfig {
                api_protocol: "http".to_string(),
                api_url: "127.0.0.1".to_string(),
                api_port: Some("7001".to_string()),
                r#type: "email".to_string(),
                name: "RainbowConsumer".to_string(),
                email: "RainbowConsumer@rainbow.com".to_string(),
                password: "rainbow".to_string(),
                id: None,
            },
            keys_path: "./../../static/certificates/consumer/".to_string(),
            is_local: true,
        }
    }
}

impl CommonConfigTrait for CommonAuthConfig {
    fn get_raw_database_config(&self) -> &DatabaseConfig {
        &self.database_config
    }
    fn get_full_db_url(&self) -> String {
        let db_config = self.get_raw_database_config();
        match db_config.db_type {
            DbType::Memory => ":memory:".to_string(),
            _ => format!(
                "{}://{}:{}@{}:{}/{}",
                db_config.db_type, // Asumiendo que DbType implementa Display
                db_config.user,
                db_config.password,
                db_config.url,
                db_config.port,
                db_config.name
            ),
        }
    }
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

    fn is_local(&self) -> bool {
        self.is_local
    }

    fn get_weird_port(&self) -> String {
        let host = self.host.clone();
        match host.port.is_empty() {
            false => {
                format!(":{}", host.port)
            }
            true => "".to_string(),
        }
    }
}

impl CommonAuthConfig {
    pub(crate) fn merge_dotenv_configuration(env_file: Option<String>) -> Self {
        if let Some(env_file) = env_file {
            dotenvy::from_filename(env_file).expect("No env file found");
        }

        // dotenvy::dotenv().ok();
        let default = CommonAuthConfig::default();
        let config = Self {
            host: HostConfig {
                protocol: extract_env("AUTH_HOST_PROTOCOL", default.host.clone().protocol),
                url: extract_env("AUTH_HOST_URL", default.host.clone().url),
                port: extract_env("AUTH_HOST_PORT", default.host.clone().port),
            },
            database_config: DatabaseConfig {
                db_type: extract_env("DB_TYPE", default.database_config.db_type.to_string()).parse().unwrap(),
                url: extract_env("DB_URL", default.database_config.url),
                port: extract_env("DB_PORT", default.database_config.port),
                user: extract_env("DB_USER", default.database_config.user),
                password: extract_env("DB_PASSWORD", default.database_config.password),
                name: extract_env("DB_DATABASE", default.database_config.name),
            },
            ssi_wallet_config: WalletConfig {
                api_protocol: extract_env(
                    "WALLET_API_PROTOCOL",
                    default.ssi_wallet_config.api_protocol,
                ),
                api_url: extract_env("WALLET_API_URL", default.ssi_wallet_config.api_url),
                api_port: option_extract_env("WALLET_API_PORT"),
                r#type: extract_env("WALLET_TYPE", default.ssi_wallet_config.r#type),
                name: extract_env("WALLET_NAME", default.ssi_wallet_config.name),
                email: extract_env("WALLET_EMAIL", default.ssi_wallet_config.email),
                password: extract_env("WALLET_PASSWORD", default.ssi_wallet_config.password),
                id: None,
            },
            keys_path: extract_env("KEYS_PATH", default.keys_path),
            is_local: extract_env("IS_LOCAL", default.is_local.to_string()).parse().unwrap(),
        };
        config
    }
}
