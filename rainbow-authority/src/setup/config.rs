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
use rainbow_common::config::global_config::{extract_env, DatabaseConfig, HostConfig};
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use rainbow_common::config::ConfigRoles;
use serde::Serialize;
use std::env;

#[derive(Serialize, Clone, Debug)]
pub struct AuthorityApplicationConfig {
    authority_host: Option<HostConfig>,
    database_config: DatabaseConfig,
    ssh_user: Option<String>,
    ssh_private_key_path: Option<String>,
}

impl Default for AuthorityApplicationConfig {
    fn default() -> Self {
        AuthorityApplicationConfig::from(ApplicationProviderConfig::default())
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
    fn merge_dotenv_configuration(&self) -> Self {
        dotenvy::dotenv().ok();
        let default = ApplicationProviderConfig::default();
        let compound_config = Self {
            authority_host: Some(HostConfig {
                protocol: extract_env(
                    "AUTHORITY_HOST_PROTOCOL",
                    default.auth_host.clone().unwrap().protocol,
                ),
                url: extract_env(
                    "AUTHORITY_HOST_URL",
                    default.auth_host.clone().unwrap().url,
                ),
                port: extract_env(
                    "AUTHORITY_HOST_PORT",
                    default.auth_host.clone().unwrap().port,
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
        };
        compound_config
    }
}

impl From<ApplicationProviderConfig> for AuthorityApplicationConfig {
    fn from(value: ApplicationProviderConfig) -> Self {
        Self {
            authority_host: value.auth_host,
            database_config: value.database_config,
            ssh_user: value.ssh_user,
            ssh_private_key_path: value.ssh_private_key_path,
        }
    }
}
