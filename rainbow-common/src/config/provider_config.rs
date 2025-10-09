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
use crate::ssi::{ClientConfig, SSIWalletConfig};
use serde::Serialize;
use serde_json::{json, Value};
use std::{env, fs};

#[derive(Serialize, Clone, Debug)]
pub struct ApplicationProviderConfig {
    pub transfer_process_host: Option<HostConfig>,
    pub business_system_host: Option<HostConfig>,
    pub catalog_host: Option<HostConfig>,
    pub catalog_as_datahub: bool,
    pub datahub_host: Option<HostConfig>,
    pub datahub_token: String,
    pub contract_negotiation_host: Option<HostConfig>,
    pub auth_host: Option<HostConfig>,
    pub ssi_auth_host: Option<HostConfig>,
    pub gateway_host: Option<HostConfig>,
    pub database_config: DatabaseConfig,
    pub ssh_user: Option<String>,
    pub ssh_private_key_path: Option<String>,
    pub ssi_wallet_config: SSIWalletConfig,
    pub client_config: ClientConfig,
    pub role: ConfigRoles,
    pub is_local: bool,
}

impl Default for ApplicationProviderConfig {
    fn default() -> Self {
        Self {
            transfer_process_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1200".to_string(),
            }),
            business_system_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "".to_string(),
            }),
            catalog_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1201".to_string(),
            }),
            catalog_as_datahub: false,
            datahub_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "8086".to_string(),
            }),
            datahub_token: "".to_string(),
            contract_negotiation_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1202".to_string(),
            }),
            auth_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1203".to_string(),
            }),
            ssi_auth_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1204".to_string(),
            }),
            gateway_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1205".to_string(),
            }),
            database_config: DatabaseConfig {
                db_type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "1300".to_string(),
                user: "ds_provider".to_string(),
                password: "ds_provider".to_string(),
                name: "ds_provider".to_string(),
            },
            ssh_user: None,
            ssh_private_key_path: None,
            ssi_wallet_config: SSIWalletConfig {
                wallet_api_protocol: "http".to_string(),
                wallet_api_url: "127.0.0.1".to_string(),
                wallet_api_port: Some("7001".to_string()),
                wallet_type: "email".to_string(),
                wallet_name: "RainbowProvider".to_string(),
                wallet_email: "RainbowProvider@rainbow.com".to_string(),
                wallet_password: "rainbow".to_string(),
                wallet_id: None,
            },
            client_config: ClientConfig {
                class_id: "rainbow_provider".to_string(),
                cert_path: "./../static/certificates/provider/cert.pem".to_string(),
                display: None,
            },
            role: ConfigRoles::Provider,
            is_local: true,
        }
    }
}

pub trait ApplicationProviderConfigTrait {
    fn ssh_user(&self) -> Option<String>;
    fn ssh_private_key_path(&self) -> Option<String>;
    fn is_datahub_as_catalog(&self) -> bool;
    fn get_role(&self) -> ConfigRoles;
    // raw stuff
    fn get_raw_transfer_process_host(&self) -> &Option<HostConfig>;
    fn get_raw_business_system_host(&self) -> &Option<HostConfig>;
    fn get_raw_catalog_host(&self) -> &Option<HostConfig>;
    fn get_raw_datahub_host(&self) -> &Option<HostConfig>;
    fn get_raw_datahub_token(&self) -> &String;
    fn get_raw_ssi_wallet_config(&self) -> &SSIWalletConfig;
    fn get_raw_contract_negotiation_host(&self) -> &Option<HostConfig>;
    fn get_raw_auth_host(&self) -> &Option<HostConfig>;
    fn get_raw_gateway_host(&self) -> &Option<HostConfig>;
    fn get_raw_ssi_auth_host(&self) -> &Option<HostConfig>;
    fn get_raw_database_config(&self) -> &DatabaseConfig;
    fn get_raw_client_config(&self) -> &ClientConfig;
    // implemented stuff
    fn get_transfer_host_url(&self) -> Option<String> {
        self.get_raw_transfer_process_host().as_ref().map(format_host_config_to_url_string)
    }
    fn get_business_system_host_url(&self) -> Option<String> {
        self.get_raw_business_system_host().as_ref().map(format_host_config_to_url_string)
    }
    fn get_catalog_host_url(&self) -> Option<String> {
        self.get_raw_catalog_host().as_ref().map(format_host_config_to_url_string)
    }
    fn get_datahub_host_url(&self) -> Option<String> {
        if self.is_datahub_as_catalog() {
            self.get_raw_datahub_host().as_ref().map(format_host_config_to_url_string)
        } else {
            None
        }
    }
    fn get_datahub_token(&self) -> Option<String> {
        Some(self.get_raw_datahub_token().to_owned())
    }
    fn get_contract_negotiation_host_url(&self) -> Option<String> {
        self.get_raw_contract_negotiation_host().as_ref().map(format_host_config_to_url_string)
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
        let protocol = self.get_raw_ssi_wallet_config().clone().wallet_api_protocol;
        let url = self.get_raw_ssi_wallet_config().clone().wallet_api_url;
        match self.get_raw_ssi_wallet_config().clone().wallet_api_port {
            Some(port) => {
                format!("{}://{}:{}", protocol, url, port)
            }
            None => {
                format!("{}://{}", protocol, url)
            }
        }
    }
    fn get_wallet_data(&self) -> Value {
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

    fn get_environment_scenario(&self) -> bool;

    // merge dotenv
    fn merge_dotenv_configuration(&self, env_file: Option<String>) -> Self
    where
        Self: Sized;
    fn get_pretty_client_config(&self) -> Value;
    fn get_pub_key(&self) -> String;
    fn get_priv_key(&self) -> String;
    fn get_cert(&self) -> String;
}

impl ApplicationProviderConfigTrait for ApplicationProviderConfig {
    fn ssh_user(&self) -> Option<String> {
        self.ssh_user.clone()
    }
    fn ssh_private_key_path(&self) -> Option<String> {
        self.ssh_private_key_path.clone()
    }
    fn is_datahub_as_catalog(&self) -> bool {
        self.catalog_as_datahub
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
    fn get_raw_catalog_host(&self) -> &Option<HostConfig> {
        &self.catalog_host
    }
    fn get_raw_datahub_host(&self) -> &Option<HostConfig> {
        &self.datahub_host
    }
    fn get_raw_datahub_token(&self) -> &String {
        &self.datahub_token
    }

    fn get_raw_ssi_wallet_config(&self) -> &SSIWalletConfig {
        &self.ssi_wallet_config
    }

    fn get_raw_contract_negotiation_host(&self) -> &Option<HostConfig> {
        &self.contract_negotiation_host
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

    fn get_raw_client_config(&self) -> &ClientConfig {
        &self.client_config
    }

    fn merge_dotenv_configuration(&self, env_file: Option<String>) -> Self {
        if let Some(env_file) = env_file {
            dotenvy::from_filename(env_file).expect("No env file found");
        }
        dotenvy::dotenv().ok();

        let default = ApplicationProviderConfig::default();
        let catalog_as_datahub: bool =
            extract_env("CATALOG_AS_DATAHUB", default.catalog_as_datahub.to_string()).parse().unwrap();
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
            catalog_host: Some(HostConfig {
                protocol: extract_env(
                    "CATALOG_PROTOCOL",
                    default.catalog_host.clone().unwrap().protocol,
                ),
                url: extract_env("CATALOG_URL", default.catalog_host.clone().unwrap().url),
                port: extract_env("CATALOG_PORT", default.catalog_host.clone().unwrap().port),
            }),
            catalog_as_datahub,
            datahub_host: match catalog_as_datahub {
                true => Some(HostConfig {
                    protocol: extract_env(
                        "DATAHUB_PROTOCOL",
                        default.datahub_host.clone().unwrap().protocol,
                    ),
                    url: extract_env("DATAHUB_URL", default.datahub_host.clone().unwrap().url),
                    port: extract_env("DATAHUB_PORT", default.datahub_host.clone().unwrap().port),
                }),
                false => None,
            },
            datahub_token: extract_env("DATAHUB_TOKEN", default.datahub_token),
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
            ssi_wallet_config: SSIWalletConfig {
                wallet_api_protocol: extract_env(
                    "WALLET_API_PROTOCOL",
                    default.ssi_wallet_config.wallet_api_protocol,
                ),
                wallet_api_url: extract_env("WALLET_API_URL", default.ssi_wallet_config.wallet_api_url),
                wallet_api_port: option_extract_env("WALLET_API_PORT"),
                wallet_type: extract_env("WALLET_TYPE", default.ssi_wallet_config.wallet_type),
                wallet_name: extract_env("WALLET_NAME", default.ssi_wallet_config.wallet_name),
                wallet_email: extract_env("WALLET_EMAIL", default.ssi_wallet_config.wallet_email),
                wallet_password: extract_env("WALLET_PASSWORD", default.ssi_wallet_config.wallet_password),
                wallet_id: None,
            },
            client_config: ClientConfig {
                class_id: extract_env("CLIENT_DEF", default.client_config.class_id),
                cert_path: extract_env("CERT_PATH", default.client_config.cert_path),
                display: None,
            },
            role: ConfigRoles::Provider,
            is_local: extract_env("IS_LOCAL", default.is_local.to_string()).parse().unwrap(),
        };
        compound_config
    }

    fn get_pretty_client_config(&self) -> Value {
        let cert = String::from_utf8(fs::read(self.client_config.cert_path.clone()).unwrap()).unwrap();
        let key = json!({
            "proof": "httpsig",
            "cert": cert
        });
        json!({
            "key" : key,
            "class_id" : self.client_config.class_id,
            "display" : self.client_config.display,
        })
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

    fn get_environment_scenario(&self) -> bool {
        self.is_local
    }
}
