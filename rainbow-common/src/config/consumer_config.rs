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
use crate::config::database::DbType;
use crate::config::global_config::{extract_env, DatabaseConfig, HostConfig};
use crate::config::ConfigRoles;
use serde::Serialize;
use std::env;
use std::fmt::Display;

#[derive(Serialize, Clone)]
pub struct ApplicationConfig {
    pub transfer_process_host: Option<HostConfig>,
    pub business_system_host: Option<HostConfig>,
    pub catalog_host: Option<HostConfig>,
    pub catalog_as_datahub: bool,
    pub datahub_host: Option<HostConfig>,
    pub contract_negotiation_host: Option<HostConfig>,
    pub auth_host: Option<HostConfig>,
    pub ssi_auth_host: Option<HostConfig>,
    pub database_config: DatabaseConfig,
    pub ssh_user: Option<String>,
    pub ssh_private_key_path: Option<String>,
    pub role: ConfigRoles,
}

impl Default for ApplicationConfig {
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
            datahub_host: None,
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
            database_config: DatabaseConfig {
                db_type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "1300".to_string(),
                user: "ds_transfer_provider".to_string(),
                password: "ds_transfer_provider".to_string(),
                name: "ds_transfer_provider".to_string(),
            },
            ssh_user: None,
            ssh_private_key_path: None,
            role: ConfigRoles::Provider,
        }
    }
}

impl ApplicationConfig {
    pub fn merge_dotenv_configuration(&self) -> Self {
        dotenvy::dotenv().ok();
        let default_config = ApplicationConfig::default();

        let catalog_as_datahub: bool = extract_env(
            "CATALOG_AS_DATAHUB",
            default_config.catalog_as_datahub.to_string(),
        )
            .parse()
            .unwrap_or(default_config.catalog_as_datahub);

        let datahub_host_config = if catalog_as_datahub {
            Some(HostConfig {
                protocol: extract_env("DATAHUB_PROTOCOL", "http".to_string()), // Default explícito
                url: extract_env("DATAHUB_URL", "127.0.0.1".to_string()), // Default explícito
                port: extract_env("DATAHUB_PORT", "1205".to_string()), // Default explícito para puerto datahub
            })
        } else {
            None
        };

        let default_transfer_host = default_config.transfer_process_host.unwrap();
        let default_business_host = default_config.business_system_host.unwrap();
        let default_catalog_host = default_config.catalog_host.unwrap();
        let default_contract_host = default_config.contract_negotiation_host.unwrap();
        let default_auth_host = default_config.auth_host.unwrap();
        let default_ssi_auth_host = default_config.ssi_auth_host.unwrap();

        Self {
            transfer_process_host: Some(HostConfig {
                protocol: extract_env("TRANSFER_PROTOCOL", default_transfer_host.protocol),
                url: extract_env("TRANSFER_URL", default_transfer_host.url),
                port: extract_env("TRANSFER_PORT", default_transfer_host.port),
            }),
            business_system_host: Some(HostConfig {
                protocol: extract_env("BUSINESS_SYSTEM_PROTOCOL", default_business_host.protocol),
                url: extract_env("BUSINESS_SYSTEM_URL", default_business_host.url),
                port: extract_env("BUSINESS_SYSTEM_PORT", default_business_host.port), // Corregido: _PORT
            }),
            catalog_host: Some(HostConfig {
                protocol: extract_env("CATALOG_PROTOCOL", default_catalog_host.protocol),
                url: extract_env("CATALOG_URL", default_catalog_host.url),
                port: extract_env("CATALOG_PORT", default_catalog_host.port),
            }),
            catalog_as_datahub,
            datahub_host: datahub_host_config,
            contract_negotiation_host: Some(HostConfig {
                protocol: extract_env("CONTRACT_NEGOTIATION_PROTOCOL", default_contract_host.protocol),
                url: extract_env("CONTRACT_NEGOTIATION_URL", default_contract_host.url),
                port: extract_env("CONTRACT_NEGOTIATION_PORT", default_contract_host.port),
            }),
            auth_host: Some(HostConfig {
                protocol: extract_env("AUTH_PROTOCOL", default_auth_host.protocol),
                url: extract_env("AUTH_URL", default_auth_host.url),
                port: extract_env("AUTH_PORT", default_auth_host.port),
            }),
            ssi_auth_host: Some(HostConfig {
                protocol: extract_env("SSI_AUTH_PROTOCOL", default_ssi_auth_host.protocol),
                url: extract_env("SSI_AUTH_URL", default_ssi_auth_host.url),
                port: extract_env("SSI_AUTH_PORT", default_ssi_auth_host.port),
            }),
            database_config: DatabaseConfig {
                db_type: extract_env("DB_TYPE", default_config.database_config.db_type.to_string())
                    .parse()
                    .unwrap_or(default_config.database_config.db_type), // Asume que DbType implementa FromStr
                url: extract_env("DB_URL", default_config.database_config.url),
                port: extract_env("DB_PORT", default_config.database_config.port),
                user: extract_env("DB_USER", default_config.database_config.user),
                password: extract_env("DB_PASSWORD", default_config.database_config.password),
                name: extract_env("DB_DATABASE", default_config.database_config.name),
            },
            ssh_user: env::var("SSH_USER").ok(), // Usar env::var para variables de entorno en tiempo de ejecución
            ssh_private_key_path: env::var("SSH_PKEY_PATH").ok(),
            role: extract_env("CONFIG_ROLE", default_config.role.to_string())
                .parse()
                .unwrap_or(default_config.role), // Asume que ConfigRoles implementa FromStr
        }
    }
}

pub trait ApplicationConfigTrait {
    fn ssh_user(&self) -> Option<String> {
        None
    }
    fn ssh_private_key_path(&self) -> Option<String> {
        None
    }
    fn get_transfer_host_url(&self) -> Option<String> {
        None
    }
    fn get_contract_negotiation_host_url(&self) -> Option<String> {
        None
    }
    fn is_datahub_as_catalog(&self) -> bool {
        false
    }
    fn get_catalog_host_url(&self) -> Option<String> {
        None
    }
    fn get_datahub_host_url(&self) -> Option<String> {
        None
    }
    fn get_business_system_host_url(&self) -> Option<String> {
        None
    }
    fn get_auth_host_url(&self) -> Option<String> {
        None
    }
    fn get_ssi_auth_host_url(&self) -> Option<String> {
        None
    }
    fn get_role(&self) -> ConfigRoles;
    fn merge_dotenv_configuration(&self) -> Self
    where
        Self: Sized;
    fn get_full_db_url(&self) -> String;
}