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

use crate::config::database::DbType;
use crate::config::global_config::{
    extract_env, format_host_config_to_url_string, option_extract_env, DatabaseConfig, HostConfig,
};
use crate::config::ConfigRoles;
use crate::ssi::{ClientConfig, WalletConfig};
use crate::utils::read;
use serde::Serialize;
use serde_json::{json, Value};
use std::env;

#[derive(Serialize, Clone, Debug)]
pub struct ApplicationConsumerConfig {
    pub transfer_process_host: Option<HostConfig>,
    pub business_system_host: Option<HostConfig>,
    pub contract_negotiation_host: Option<HostConfig>,
    pub catalog_bypass_host: Option<HostConfig>,
    pub auth_host: Option<HostConfig>,
    pub ssi_auth_host: Option<HostConfig>,
    pub gateway_host: Option<HostConfig>,
    pub is_gateway_in_production: bool,
    pub database_config: DatabaseConfig,
    pub ssh_user: Option<String>,
    pub ssh_private_key_path: Option<String>,
    pub ssi_wallet_config: WalletConfig,
    pub client_config: ClientConfig,
    pub role: ConfigRoles,
    pub keys_path: String,
    pub is_local: bool,
    pub openapi_path: String,
    pub api_version: String
}

impl Default for ApplicationConsumerConfig {
    fn default() -> Self {
        Self {
            transfer_process_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1100".to_string(),
            }),
            business_system_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1101".to_string(),
            }),
            contract_negotiation_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1102".to_string(),
            }),
            auth_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1103".to_string(),
            }),
            ssi_auth_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1104".to_string(),
            }),
            gateway_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1105".to_string(),
            }),
            catalog_bypass_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1106".to_string(),
            }),
            database_config: DatabaseConfig {
                db_type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "1301".to_string(),
                user: "ds_consumer".to_string(),
                password: "ds_consumer".to_string(),
                name: "ds_consumer".to_string(),
            },
            ssh_user: None,
            ssh_private_key_path: None,
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
            client_config: ClientConfig { class_id: "rainbow_consumer".to_string(), display: None },
            role: ConfigRoles::Consumer,
            is_local: true,
            keys_path: "./../static/certificates/consumer/".to_string(),
            openapi_path: "./../static/specs/openapi/auth/auth_consumer.json".to_string(),
            is_gateway_in_production: false,
            api_version: "v1".to_string(),
        }
    }
}

pub trait ApplicationConsumerConfigTrait {
    fn ssh_user(&self) -> Option<String>;
    fn ssh_private_key_path(&self) -> Option<String>;
    fn get_role(&self) -> ConfigRoles;
    // raw stuff
    fn get_raw_transfer_process_host(&self) -> &Option<HostConfig>;
    fn get_raw_business_system_host(&self) -> &Option<HostConfig>;
    fn get_raw_contract_negotiation_host(&self) -> &Option<HostConfig>;
    fn get_raw_catalog_bypass_host(&self) -> &Option<HostConfig>;
    fn get_raw_auth_host(&self) -> &Option<HostConfig>;
    fn get_raw_gateway_host(&self) -> &Option<HostConfig>;
    fn get_raw_ssi_auth_host(&self) -> &Option<HostConfig>;
    fn get_raw_database_config(&self) -> &DatabaseConfig;
    fn get_raw_ssi_wallet_config(&self) -> &WalletConfig;
    fn get_raw_client_config(&self) -> &ClientConfig;

    // implemented stuff
    fn get_transfer_host_url(&self) -> Option<String> {
        self.get_raw_transfer_process_host().as_ref().map(format_host_config_to_url_string)
    }
    fn get_business_system_host_url(&self) -> Option<String> {
        self.get_raw_business_system_host().as_ref().map(format_host_config_to_url_string)
    }
    fn get_contract_negotiation_host_url(&self) -> Option<String> {
        self.get_raw_contract_negotiation_host().as_ref().map(format_host_config_to_url_string)
    }
    fn get_catalog_bypass_host_url(&self) -> Option<String> {
        self.get_raw_catalog_bypass_host().as_ref().map(format_host_config_to_url_string)
    }
    fn get_auth_host_url(&self) -> Option<String> {
        self.get_raw_auth_host().as_ref().map(format_host_config_to_url_string)
    }
    fn get_gateway_host_url(&self) -> Option<String> {
        self.get_raw_gateway_host().as_ref().map(format_host_config_to_url_string)
    }
    fn get_ssi_auth_host_url(&self) -> Option<String> {
        self.get_raw_ssi_auth_host().as_ref().map(format_host_config_to_url_string)
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
    fn get_wallet_portal_url(&self) -> String {
        let protocol = self.get_raw_ssi_wallet_config().clone().api_protocol;
        let url = self.get_raw_ssi_wallet_config().clone().api_url;
        match self.get_raw_ssi_wallet_config().clone().api_port {
            Some(port) => {
                format!("{}://{}:{}", protocol, url, port)
            }
            None => {
                format!("{}://{}", protocol, url)
            }
        }
    }
    fn get_wallet_data(&self) -> Value {
        let _type = self.get_raw_ssi_wallet_config().clone().r#type;
        let name = self.get_raw_ssi_wallet_config().clone().name;
        let email = self.get_raw_ssi_wallet_config().clone().email;
        let password = self.get_raw_ssi_wallet_config().clone().password;
        json!({
            "type": _type,
            "name": name,
            "email": email,
            "password": password,
        })
    }
    fn get_environment_scenario(&self) -> bool;

    // merge dotenv
    fn merge_dotenv_configuration(&self, env_file: Option<String>) -> Self
    where
        Self: Sized;
    fn get_openapi_json(&self) -> anyhow::Result<String>;
    fn get_api_path(&self) -> String;
}

impl ApplicationConsumerConfigTrait for ApplicationConsumerConfig {
    fn ssh_user(&self) -> Option<String> {
        self.ssh_user.clone()
    }
    fn ssh_private_key_path(&self) -> Option<String> {
        self.ssh_private_key_path.clone()
    }
    fn get_role(&self) -> ConfigRoles {
        self.role
    }
    fn get_raw_transfer_process_host(&self) -> &Option<HostConfig> {
        &self.transfer_process_host
    }
    fn get_raw_business_system_host(&self) -> &Option<HostConfig> {
        &self.business_system_host
    }
    fn get_raw_contract_negotiation_host(&self) -> &Option<HostConfig> {
        &self.contract_negotiation_host
    }
    fn get_raw_catalog_bypass_host(&self) -> &Option<HostConfig> {
        &self.catalog_bypass_host
    }
    fn get_raw_auth_host(&self) -> &Option<HostConfig> {
        &self.auth_host
    }
    fn get_raw_gateway_host(&self) -> &Option<HostConfig> {
        &self.gateway_host
    }
    fn get_raw_ssi_auth_host(&self) -> &Option<HostConfig> {
        &self.ssi_auth_host
    }
    fn get_raw_database_config(&self) -> &DatabaseConfig {
        &self.database_config
    }
    fn get_raw_ssi_wallet_config(&self) -> &WalletConfig {
        &self.ssi_wallet_config
    }
    fn get_raw_client_config(&self) -> &ClientConfig {
        &self.client_config
    }

    fn get_environment_scenario(&self) -> bool {
        self.is_local
    }
    fn get_api_path(&self) -> String {
        format!("/api/{}", self.api_version)
    }

    fn merge_dotenv_configuration(&self, env_file: Option<String>) -> Self {
        if let Some(env_file) = env_file {
            dotenvy::from_filename(env_file).expect("No env file found");
        }
        dotenvy::dotenv().ok();
        let default = ApplicationConsumerConfig::default();
        let gateway_production: bool = extract_env(
            "GATEWAY_PRODUCTION",
            default.is_gateway_in_production.to_string(),
        )
        .parse()
        .unwrap();
        let compound_config = Self {
            transfer_process_host: Some(HostConfig {
                protocol: extract_env(
                    "TRANSFER_PROTOCOL",
                    default.transfer_process_host.clone().unwrap().protocol,
                ),
                url: extract_env(
                    "TRANSFER_URL",
                    default.transfer_process_host.clone().unwrap().url,
                ),
                port: extract_env(
                    "TRANSFER_PORT",
                    default.transfer_process_host.clone().unwrap().port,
                ),
            }),
            business_system_host: Some(HostConfig {
                protocol: extract_env(
                    "BUSINESS_SYSTEM_HOST",
                    default.business_system_host.clone().unwrap().protocol,
                ),
                url: extract_env(
                    "BUSINESS_SYSTEM_URL",
                    default.business_system_host.clone().unwrap().url,
                ),
                port: extract_env(
                    "BUSINESS_SYSTEM_PORT",
                    default.business_system_host.clone().unwrap().port,
                ),
            }),
            contract_negotiation_host: Some(HostConfig {
                protocol: extract_env(
                    "CONTRACT_NEGOTIATION_PROTOCOL",
                    default.contract_negotiation_host.clone().unwrap().protocol,
                ),
                url: extract_env(
                    "CONTRACT_NEGOTIATION_URL",
                    default.contract_negotiation_host.clone().unwrap().url,
                ),
                port: extract_env(
                    "CONTRACT_NEGOTIATION_PORT",
                    default.contract_negotiation_host.clone().unwrap().port,
                ),
            }),
            catalog_bypass_host: Some(HostConfig {
                protocol: extract_env(
                    "CATALOG_BYPASS_PROTOCOL",
                    default.contract_negotiation_host.clone().unwrap().protocol,
                ),
                url: extract_env(
                    "CATALOG_BYPASS_URL",
                    default.contract_negotiation_host.clone().unwrap().url,
                ),
                port: extract_env(
                    "CATALOG_BYPASS_PORT",
                    default.contract_negotiation_host.clone().unwrap().port,
                ),
            }),
            auth_host: Some(HostConfig {
                protocol: extract_env(
                    "AUTH_HOST_PROTOCOL",
                    default.auth_host.clone().unwrap().protocol,
                ),
                url: extract_env("AUTH_HOST_URL", default.auth_host.clone().unwrap().url),
                port: extract_env("AUTH_HOST_PORT", default.auth_host.clone().unwrap().port),
            }),
            ssi_auth_host: Some(HostConfig {
                protocol: extract_env(
                    "SSI_AUTH_PROTOCOL",
                    default.ssi_auth_host.clone().unwrap().protocol,
                ),
                url: extract_env("SSI_AUTH_URL", default.ssi_auth_host.clone().unwrap().url),
                port: extract_env("SSI_AUTH_PORT", default.ssi_auth_host.clone().unwrap().port),
            }),
            gateway_host: Some(HostConfig {
                protocol: extract_env(
                    "GATEWAY_PROTOCOL",
                    default.gateway_host.clone().unwrap().protocol,
                ),
                url: extract_env("GATEWAY_HOST", default.gateway_host.clone().unwrap().url),
                port: extract_env("GATEWAY_PORT", default.gateway_host.clone().unwrap().port),
            }),
            is_gateway_in_production: gateway_production,
            database_config: DatabaseConfig {
                db_type: extract_env("DB_TYPE", default.database_config.db_type.to_string()).parse().unwrap(),
                url: extract_env("DB_URL", default.database_config.url),
                port: extract_env("DB_PORT", default.database_config.port),
                user: extract_env("DB_USER", default.database_config.user),
                password: extract_env("DB_PASSWORD", default.database_config.password),
                name: extract_env("DB_DATABASE", default.database_config.name),
            },
            ssh_user: env::var("SSH_USER").ok(),
            ssh_private_key_path: env::var("SSH_PKEY_PATH").ok(),
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
            client_config: ClientConfig {
                class_id: extract_env("CLIENT_DEF", default.client_config.class_id),
                display: None,
            },
            role: ConfigRoles::Consumer,
            keys_path: extract_env("KEYS_PATH", default.keys_path),
            is_local: extract_env("IS_LOCAL", default.is_local.to_string()).parse().unwrap(),
            openapi_path: extract_env("OPENAPI_PATH", default.openapi_path),
            api_version: extract_env("API_VERSION", default.api_version),
        };
        compound_config
    }
    fn get_openapi_json(&self) -> anyhow::Result<String> {
        read(&self.openapi_path)
    }
}
