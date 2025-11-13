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
use super::AuthorityApplicationConfigTrait;
use crate::setup::database::{DatabaseConfig, DbType};
use crate::types::host::HostConfig;
use crate::types::wallet::SSIWalletConfig;
use serde::Serialize;
use std::env;

#[derive(Serialize, Clone, Debug)]
pub struct AuthorityApplicationConfig {
    pub authority_host: HostConfig,
    pub is_local: bool,
    pub database_config: DatabaseConfig,
    pub ssi_wallet_config: SSIWalletConfig,
    pub keys_path: String,
    // client_config: ClientConfig,
    // ssi_issuer_api: String,
}

impl Default for AuthorityApplicationConfig {
    fn default() -> Self {
        Self {
            authority_host: HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: Some("1500".to_string()),
            },
            database_config: DatabaseConfig {
                db_type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "1450".to_string(),
                user: "ds_authority".to_string(),
                password: "ds_authority".to_string(),
                name: "ds_authority".to_string(),
            },
            ssi_wallet_config: SSIWalletConfig {
                api_protocol: "http".to_string(),
                api_url: "127.0.0.1".to_string(),
                api_port: Some("7001".to_string()),
                r#type: "email".to_string(),
                name: "RainbowAuthority".to_string(),
                email: "RainbowAuthority@rainbow.com".to_string(),
                password: "rainbow".to_string(),
                id: None,
            },
            is_local: true,
            keys_path: "./../static/certificates/authority/".to_string(),
            // ssi_issuer_api: "http://127.0.0.1:7002".to_string(),
            // client_config: ClientConfig {
            //     class_id: "RainbowAuthorityEntity".to_string(),
            //     cert_path: "./../static/certificates/authority/cert.pem".to_string(),
            //     display: None,
            // },
        }
    }
}

impl AuthorityApplicationConfigTrait for AuthorityApplicationConfig {
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
    fn merge_dotenv_configuration(&self, env_file: Option<String>) -> Self {
        if let Some(env_file) = env_file {
            dotenvy::from_filename(env_file).expect("No env file found");
        }

        dotenvy::dotenv().ok();
        let default = AuthorityApplicationConfig::default();
        let compound_config = Self {
            authority_host: HostConfig {
                protocol: extract_env(
                    "AUTHORITY_HOST_PROTOCOL",
                    default.authority_host.clone().protocol,
                ),
                url: extract_env("AUTHORITY_HOST_URL", default.authority_host.clone().url),
                port: option_extract_env("AUTHORITY_HOST_PORT"),
            },
            database_config: DatabaseConfig {
                db_type: extract_env("DB_TYPE", default.database_config.db_type.to_string()).parse().unwrap(),
                url: extract_env("DB_URL", default.database_config.url),
                port: extract_env("DB_PORT", default.database_config.port),
                user: extract_env("DB_USER", default.database_config.user),
                password: extract_env("DB_PASSWORD", default.database_config.password),
                name: extract_env("DB_DATABASE", default.database_config.name),
            },
            ssi_wallet_config: SSIWalletConfig {
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
            // client_config: ClientConfig {
            //     class_id: extract_env("CLASS_ID", default.client_config.class_id),
            //     cert_path: extract_env("CERT_PATH", default.client_config.cert_path),
            //     display: None,
            // },
            // ssi_issuer_api: extract_env("SSI_ISSUER_API", default.ssi_issuer_api),
        };
        compound_config
    }


    fn get_host(&self) -> String {
        let host = self.authority_host.clone();
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

    fn get_weird_port(&self) -> String {
        let host = self.authority_host.clone();
        match host.port {
            Some(data) => {
                format!(":{}", data)
            }
            None => "".to_string(),
        }
    }
}

fn extract_env(env_var_name: &str, default: String) -> String {
    env::var(env_var_name).unwrap_or(default)
}

fn option_extract_env(env_var_name: &str) -> Option<String> {
    match env::var(env_var_name) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
