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

use crate::config::consumer_config::ApplicationConsumerConfig;
use crate::config::database::DbType;
use crate::config::provider_config::ApplicationProviderConfig;
use crate::config::ConfigRoles;
use crate::ssi::{ClientConfig, SSIWalletConfig};
use serde::Serialize;
use std::env;

pub fn extract_env(env_var_name: &str, default: String) -> String {
    env::var(env_var_name).unwrap_or(default)
}

pub fn option_extract_env(env_var_name: &str) -> Option<String> {
    match env::var(env_var_name) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

pub fn format_host_config_to_url_string(hc: &HostConfig) -> String {
    if hc.port.is_empty() {
        format!("{}://{}", hc.protocol, hc.url)
    } else {
        format!("{}://{}:{}", hc.protocol, hc.url, hc.port)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct HostConfig {
    pub protocol: String,
    pub url: String,
    pub port: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub db_type: DbType,
    pub url: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ApplicationGlobalConfig {
    pub transfer_process_host: Option<HostConfig>,
    pub business_system_host: Option<HostConfig>,
    pub catalog_host: Option<HostConfig>,
    pub catalog_as_datahub: bool,
    pub catalog_bypass_host: Option<HostConfig>,
    pub datahub_host: Option<HostConfig>,
    pub datahub_token: String,
    pub contract_negotiation_host: Option<HostConfig>,
    pub auth_host: Option<HostConfig>,
    pub ssi_auth_host: Option<HostConfig>,
    pub ssi_wallet_config: Option<SSIWalletConfig>,
    pub client_config: Option<ClientConfig>,
    pub gateway_host: Option<HostConfig>,
    pub is_gateway_in_production: bool,
    pub database_config: DatabaseConfig,
    pub ssh_user: Option<String>,
    pub ssh_private_key_path: Option<String>,
    pub role: ConfigRoles,
    pub is_local: bool,
}

impl From<ApplicationGlobalConfig> for ApplicationProviderConfig {
    fn from(value: ApplicationGlobalConfig) -> Self {
        Self {
            transfer_process_host: value.transfer_process_host,
            business_system_host: value.business_system_host,
            catalog_host: value.catalog_host,
            catalog_as_datahub: value.catalog_as_datahub,
            datahub_host: value.datahub_host,
            datahub_token: value.datahub_token,
            contract_negotiation_host: value.contract_negotiation_host,
            auth_host: value.auth_host,
            ssi_auth_host: value.ssi_auth_host,
            gateway_host: value.gateway_host,
            is_gateway_in_production: value.is_gateway_in_production,
            database_config: value.database_config,
            ssh_user: value.ssh_user,
            ssh_private_key_path: value.ssh_private_key_path,
            ssi_wallet_config: SSIWalletConfig {
                wallet_api_protocol: value.ssi_wallet_config.clone().unwrap().wallet_api_protocol,
                wallet_api_url: value.ssi_wallet_config.clone().unwrap().wallet_api_url,
                wallet_api_port: value.ssi_wallet_config.clone().unwrap().wallet_api_port,
                wallet_type: value.ssi_wallet_config.clone().unwrap().wallet_type,
                wallet_name: value.ssi_wallet_config.clone().unwrap().wallet_name,
                wallet_email: value.ssi_wallet_config.clone().unwrap().wallet_email,
                wallet_password: value.ssi_wallet_config.clone().unwrap().wallet_password,
                wallet_id: None,
            },
            client_config: value.client_config.unwrap(),
            role: value.role,
            is_local: value.is_local,
        }
    }
}

impl Into<ApplicationGlobalConfig> for ApplicationProviderConfig {
    fn into(self) -> ApplicationGlobalConfig {
        ApplicationGlobalConfig {
            transfer_process_host: self.transfer_process_host,
            business_system_host: self.business_system_host,
            catalog_host: self.catalog_host,
            catalog_as_datahub: self.catalog_as_datahub,
            catalog_bypass_host: None,
            datahub_host: self.datahub_host,
            datahub_token: self.datahub_token,
            contract_negotiation_host: self.contract_negotiation_host,
            auth_host: self.auth_host,
            ssi_auth_host: self.ssi_auth_host,
            ssi_wallet_config: Option::from(self.ssi_wallet_config),
            client_config: Option::from(self.client_config),
            gateway_host: self.gateway_host,
            is_gateway_in_production: self.is_gateway_in_production,
            database_config: self.database_config,
            ssh_user: self.ssh_user,
            ssh_private_key_path: self.ssh_private_key_path,
            role: self.role,
            is_local: self.is_local,
        }
    }
}

impl From<ApplicationGlobalConfig> for ApplicationConsumerConfig {
    fn from(value: ApplicationGlobalConfig) -> Self {
        Self {
            transfer_process_host: value.transfer_process_host,
            business_system_host: value.business_system_host,
            contract_negotiation_host: value.contract_negotiation_host,
            catalog_bypass_host: value.catalog_bypass_host,
            auth_host: value.auth_host,
            ssi_auth_host: value.ssi_auth_host,
            gateway_host: value.gateway_host,
            is_gateway_in_production: false,
            database_config: value.database_config,
            ssh_user: value.ssh_user,
            ssh_private_key_path: value.ssh_private_key_path,
            ssi_wallet_config: SSIWalletConfig {
                wallet_api_protocol: value.ssi_wallet_config.clone().unwrap().wallet_api_protocol,
                wallet_api_url: value.ssi_wallet_config.clone().unwrap().wallet_api_url,
                wallet_api_port: value.ssi_wallet_config.clone().unwrap().wallet_api_port,
                wallet_type: value.ssi_wallet_config.clone().unwrap().wallet_type,
                wallet_name: value.ssi_wallet_config.clone().unwrap().wallet_name,
                wallet_email: value.ssi_wallet_config.clone().unwrap().wallet_email,
                wallet_password: value.ssi_wallet_config.clone().unwrap().wallet_password,
                wallet_id: None,
            },
            role: value.role,
            client_config: value.client_config.unwrap(),
            is_local: value.is_local,
        }
    }
}

impl Into<ApplicationGlobalConfig> for ApplicationConsumerConfig {
    fn into(self) -> ApplicationGlobalConfig {
        ApplicationGlobalConfig {
            transfer_process_host: self.transfer_process_host,
            business_system_host: self.business_system_host,
            catalog_host: None,
            catalog_as_datahub: false,
            catalog_bypass_host: self.catalog_bypass_host,
            datahub_host: None,
            datahub_token: "".to_string(),
            contract_negotiation_host: self.contract_negotiation_host,
            auth_host: self.auth_host,
            ssi_auth_host: self.ssi_auth_host,
            ssi_wallet_config: Option::from(self.ssi_wallet_config),
            client_config: Option::from(self.client_config),
            gateway_host: self.gateway_host,
            is_gateway_in_production: self.is_gateway_in_production,
            database_config: self.database_config,
            ssh_user: self.ssh_user,
            ssh_private_key_path: self.ssh_private_key_path,
            role: self.role,
            is_local: self.is_local,
        }
    }
}
