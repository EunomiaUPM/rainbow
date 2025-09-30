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

pub mod client_config;
pub mod config_trait;

use super::database::{DatabaseConfig, DbType};
use super::AuthorityApplicationConfigTrait;

use super::config::client_config::ClientConfig;
use crate::types::wallet::SSIWalletConfig;
use serde::Serialize;
use serde_json::json;
use std::{env, fs};

#[derive(Serialize, Clone, Debug)]
pub struct HostConfig {
    pub protocol: String,
    pub url: String,
    pub port: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct AuthorityApplicationConfig {
    authority_host: Option<HostConfig>,
    database_config: DatabaseConfig,
    client_config: ClientConfig,
    ssi_wallet_config: SSIWalletConfig,
    ssi_issuer_api: String,
}

impl Default for AuthorityApplicationConfig {
    fn default() -> Self {
        Self {
            authority_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1500".to_string(),
            }),
            database_config: DatabaseConfig {
                db_type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "1450".to_string(),
                user: "ds_authority".to_string(),
                password: "ds_authority".to_string(),
                name: "ds_authority".to_string(),
            },
            client_config: ClientConfig {
                class_id: "RainbowAuthorityEntity".to_string(),
                cert_path: "./../static/certificates/authority/cert.pem".to_string(),
                display: None,
            },
            ssi_wallet_config: SSIWalletConfig {
                wallet_portal_url: "127.0.0.1".to_string(),
                wallet_portal_port: "7001".to_string(),
                wallet_type: "email".to_string(),
                wallet_name: "RainbowAuthority".to_string(),
                wallet_email: "RainbowAuthority@rainbow.com".to_string(),
                wallet_password: "rainbow".to_string(),
                wallet_id: None,
            },
            ssi_issuer_api: "http://127.0.0.1:7002".to_string(),
        }
    }
}

impl AuthorityApplicationConfigTrait for AuthorityApplicationConfig {
    fn get_raw_database_config(&self) -> &DatabaseConfig {
        &self.database_config
    }

    fn get_raw_client_config(&self) -> &ClientConfig {
        &self.client_config
    }
    fn get_raw_ssi_wallet_config(&self) -> &SSIWalletConfig {
        &self.ssi_wallet_config
    }

    fn get_wallet_portal_url(&self) -> String {
        let url = self.get_raw_ssi_wallet_config().clone().wallet_portal_url;
        let port = self.get_raw_ssi_wallet_config().clone().wallet_portal_port;
        format!("http://{}:{}", url, port)
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
    fn get_wallet_data(&self) -> serde_json::Value {
        let _type = self.get_raw_ssi_wallet_config().clone().wallet_type;
        let name = self.get_raw_ssi_wallet_config().clone().wallet_name;
        let email = self.get_raw_ssi_wallet_config().clone().wallet_email;
        let password = self.get_raw_ssi_wallet_config().clone().wallet_password;
        json!({
            "type": _type,
            "name": name,
            "email": email,
            "password": password,
        })
    }

    fn get_issuer_api(&self) -> String {
        self.ssi_issuer_api.clone()
    }

    fn get_cert(&self) -> String {
        let path = fs::read(self.client_config.cert_path.clone()).unwrap();
        String::from_utf8(path).unwrap()
    }
    fn get_priv_key(&self) -> String {
        let bad_path = self.client_config.cert_path.clone();
        let inc_path = match bad_path.rfind('/') {
            Some(pos) => (&bad_path[..pos]).to_string(),
            None => bad_path,
        };
        let path = format!("{}/private_key.pem", inc_path);
        let file = fs::read(path).unwrap();
        String::from_utf8(file).unwrap()
    }
    fn get_pub_key(&self) -> String {
        let bad_path = self.client_config.cert_path.clone();
        let inc_path = match bad_path.rfind('/') {
            Some(pos) => (&bad_path[..pos]).to_string(),
            None => bad_path,
        };
        let path = format!("{}/public_key.pem", inc_path);
        let file = fs::read(path).unwrap();
        String::from_utf8(file).unwrap()
    }

    fn merge_dotenv_configuration(&self) -> Self {
        dotenvy::dotenv().ok();
        let default = AuthorityApplicationConfig::default();
        let compound_config = Self {
            authority_host: Some(HostConfig {
                protocol: extract_env(
                    "AUTHORITY_HOST_PROTOCOL",
                    default.authority_host.clone().unwrap().protocol,
                ),
                url: extract_env(
                    "AUTHORITY_HOST_URL",
                    default.authority_host.clone().unwrap().url,
                ),
                port: extract_env(
                    "AUTHORITY_HOST_PORT",
                    default.authority_host.clone().unwrap().port,
                ),
            }),
            database_config: DatabaseConfig {
                db_type: extract_env("DB_TYPE", default.database_config.db_type.to_string()).parse().unwrap(),
                url: extract_env("DB_URL", default.database_config.url),
                port: extract_env("DB_PORT", default.database_config.port),
                user: extract_env("DB_USER", default.database_config.user),
                password: extract_env("DB_PASSWORD", default.database_config.password),
                name: extract_env("DB_DATABASE", default.database_config.name),
            },
            client_config: ClientConfig {
                class_id: extract_env("CLASS_ID", default.client_config.class_id),
                cert_path: extract_env("CERT_PATH", default.client_config.cert_path),
                display: None,
            },
            ssi_wallet_config: SSIWalletConfig {
                wallet_portal_url: extract_env(
                    "WALLET_PORTAL_URL",
                    default.ssi_wallet_config.wallet_portal_url,
                ),
                wallet_portal_port: extract_env(
                    "WALLET_PORTAL_PORT",
                    default.ssi_wallet_config.wallet_portal_port,
                ),
                wallet_type: extract_env("WALLET_PORTAL_TYPE", default.ssi_wallet_config.wallet_type),
                wallet_name: extract_env("WALLET_PORTAL_NAME", default.ssi_wallet_config.wallet_name),
                wallet_email: extract_env(
                    "WALLET_PORTAL_EMAIL",
                    default.ssi_wallet_config.wallet_email,
                ),
                wallet_password: extract_env(
                    "WALLET_PORTAL_PASSWORD",
                    default.ssi_wallet_config.wallet_password,
                ),
                wallet_id: None,
            },
            ssi_issuer_api: extract_env("SSI_ISSUER_API", default.ssi_issuer_api),
        };
        compound_config
    }
}

pub trait AuthorityFunctions {
    fn get_host(&self) -> String;
    fn get_host_without_protocol(&self) -> String;
}

impl AuthorityFunctions for AuthorityApplicationConfig {
    fn get_host(&self) -> String {
        let host = self.authority_host.clone().unwrap();
        format!("{}://{}:{}", host.protocol, host.url, host.port)
    }
    fn get_host_without_protocol(&self) -> String {
        let host = self.authority_host.clone().unwrap();
        format!("{}:{}", host.url, host.port)
    }
}

pub fn extract_env(env_var_name: &str, default: String) -> String {
    env::var(env_var_name).unwrap_or(default)
}
