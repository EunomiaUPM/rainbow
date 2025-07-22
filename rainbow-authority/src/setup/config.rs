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
use rainbow_common::config::database::DbType;
use rainbow_common::config::global_config::{extract_env, DatabaseConfig, HostConfig};
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use rainbow_common::config::ConfigRoles;
use rainbow_common::ssi_wallet::{ClientConfig, SSIWalletConfig};
use serde::Serialize;
use std::env;

#[derive(Serialize, Clone, Debug)]
pub struct AuthorityApplicationConfig {
    authority_host: Option<HostConfig>,
    database_config: DatabaseConfig,
    ssh_user: Option<String>,
    ssh_private_key_path: Option<String>,
    cert_path: String,
    ssi_wallet_config: SSIWalletConfig,
    client_config: ClientConfig,
}

impl Default for AuthorityApplicationConfig {
    fn default() -> Self {
        Self {
            authority_host: Some(HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1400".to_string(),
            }),
            database_config: DatabaseConfig {
                db_type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "1450".to_string(),
                user: "ds_authority".to_string(),
                password: "ds_authority".to_string(),
                name: "ds_authority".to_string(),
            },
            ssh_user: None,
            ssh_private_key_path: None,
            cert_path: ".".to_string(),
            ssi_wallet_config: SSIWalletConfig {
                wallet_portal_url: "".to_string(),
                wallet_portal_port: "".to_string(),
                wallet_type: "".to_string(),
                wallet_name: "".to_string(),
                wallet_email: "".to_string(),
                wallet_password: "".to_string(),
                wallet_id: None,
            },
            client_config: ClientConfig { self_client: "".to_string() },
        }
    }
}

impl ApplicationProviderConfigTrait for AuthorityApplicationConfig {
    fn ssh_user(&self) -> Option<String> {
        self.ssh_user.clone()
    }
    fn ssh_private_key_path(&self) -> Option<String> {
        self.ssh_private_key_path.clone()
    }
    fn is_datahub_as_catalog(&self) -> bool {
        false
    }
    fn get_role(&self) -> ConfigRoles {
        ConfigRoles::Auth
    }
    fn get_raw_transfer_process_host(&self) -> &Option<HostConfig> {
        &None
    }
    fn get_raw_business_system_host(&self) -> &Option<HostConfig> {
        &None
    }
    fn get_raw_catalog_host(&self) -> &Option<HostConfig> {
        &None
    }
    fn get_raw_datahub_host(&self) -> &Option<HostConfig> {
        &None
    }

    fn get_raw_ssi_wallet_config(&self) -> &SSIWalletConfig { &self.ssi_wallet_config }

    fn get_raw_contract_negotiation_host(&self) -> &Option<HostConfig> {
        &None
    }
    fn get_raw_auth_host(&self) -> &Option<HostConfig> {
        &None
    }
    fn get_raw_gateway_host(&self) -> &Option<HostConfig> {
        &None
    }
    fn get_raw_ssi_auth_host(&self) -> &Option<HostConfig> {
        &None
    }
    fn get_raw_database_config(&self) -> &DatabaseConfig {
        &self.database_config
    }

    fn get_raw_client_config(&self) -> &ClientConfig { &self.client_config }

    fn get_raw_cert_path(&self) -> &String {
        &self.cert_path
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
            ssh_user: env::var("SSH_USER").ok(),
            ssh_private_key_path: env::var("SSH_PKEY_PATH").ok(),
            cert_path: ".".to_string(),
            ssi_wallet_config: SSIWalletConfig {
                wallet_portal_url: "".to_string(),
                wallet_portal_port: "".to_string(),
                wallet_type: "".to_string(),
                wallet_name: "".to_string(),
                wallet_email: "".to_string(),
                wallet_password: "".to_string(),
                wallet_id: None,
            },
            client_config: ClientConfig { self_client: "".to_string() },
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

impl From<ApplicationProviderConfig> for AuthorityApplicationConfig {
    fn from(value: ApplicationProviderConfig) -> Self {
        Self {
            authority_host: value.auth_host,
            database_config: value.database_config,
            ssh_user: value.ssh_user,
            ssh_private_key_path: value.ssh_private_key_path,
            cert_path: value.cert_path,
            ssi_wallet_config: value.ssi_wallet_config,
            client_config: value.client_config,
        }
    }
}
