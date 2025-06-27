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

use rainbow_common::config::consumer_config::{ApplicationConsumerConfig, ApplicationConsumerConfigTrait};
use rainbow_common::config::global_config::{DatabaseConfig, HostConfig};
use rainbow_common::config::ConfigRoles;
use rainbow_common::ssi_wallet::{ClientConfig, SSIWalletConfig};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct CoreApplicationConsumerConfig {
    pub core_host: Option<HostConfig>,
    pub business_system_host: Option<HostConfig>,
    pub database_config: DatabaseConfig,
    pub ssh_user: Option<String>,
    pub ssh_private_key_path: Option<String>,
    pub ssi_wallet_config: SSIWalletConfig,
    pub client_config: ClientConfig,
    pub role: ConfigRoles,
    pub cert_path: String,
}

impl Default for CoreApplicationConsumerConfig {
    fn default() -> Self {
        CoreApplicationConsumerConfig::from(ApplicationConsumerConfig::default())
    }
}

impl ApplicationConsumerConfigTrait for CoreApplicationConsumerConfig {
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
        &self.core_host
    }
    fn get_raw_business_system_host(&self) -> &Option<HostConfig> {
        &self.business_system_host
    }
    fn get_raw_contract_negotiation_host(&self) -> &Option<HostConfig> {
        &self.core_host
    }
    fn get_raw_catalog_bypass_host(&self) -> &Option<HostConfig> {
        &self.core_host
    }
    fn get_raw_auth_host(&self) -> &Option<HostConfig> {
        &self.core_host
    }
    fn get_raw_gateway_host(&self) -> &Option<HostConfig> {
        &None
    }
    fn get_raw_ssi_auth_host(&self) -> &Option<HostConfig> {
        &self.core_host
    }
    fn get_raw_database_config(&self) -> &DatabaseConfig {
        &self.database_config
    }

    fn get_raw_ssi_wallet_config(&self) -> &SSIWalletConfig {
        &self.ssi_wallet_config
    }

    fn get_raw_cert_path(&self) -> &String {
        &self.cert_path
    }

    fn merge_dotenv_configuration(&self) -> Self
    where
        Self: Sized,
    {
        let app_config = ApplicationConsumerConfig::default().merge_dotenv_configuration();
        CoreApplicationConsumerConfig::from(app_config)
    }

    fn get_raw_client_config(&self) -> &ClientConfig {
        &self.client_config
    }
}

impl From<ApplicationConsumerConfig> for CoreApplicationConsumerConfig {
    fn from(value: ApplicationConsumerConfig) -> Self {
        Self {
            core_host: value.transfer_process_host,
            business_system_host: value.business_system_host,
            database_config: value.database_config,
            ssh_user: value.ssh_user,
            ssh_private_key_path: value.ssh_private_key_path,
            ssi_wallet_config: value.ssi_wallet_config,
            role: value.role,
            cert_path: value.cert_path,
            client_config: value.client_config,
        }
    }
}

impl Into<ApplicationConsumerConfig> for CoreApplicationConsumerConfig {
    fn into(self) -> ApplicationConsumerConfig {
        ApplicationConsumerConfig {
            transfer_process_host: self.core_host.clone(),
            business_system_host: self.business_system_host,
            contract_negotiation_host: self.core_host.clone(),
            catalog_bypass_host: self.core_host.clone(),
            auth_host: self.core_host.clone(),
            ssi_auth_host: self.core_host.clone(),
            gateway_host: None,
            database_config: self.database_config,
            ssh_user: self.ssh_user,
            ssh_private_key_path: self.ssh_private_key_path,
            ssi_wallet_config: self.ssi_wallet_config,
            role: self.role,
            client_config: self.client_config,
            cert_path: self.cert_path,
        }
    }
}
