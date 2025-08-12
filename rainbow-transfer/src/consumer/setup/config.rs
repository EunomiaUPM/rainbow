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
pub struct TransferConsumerApplicationConfig {
    pub transfer_process_host: Option<HostConfig>,
    pub business_system_host: Option<HostConfig>,
    pub contract_negotiation_host: Option<HostConfig>,
    pub catalog_bypass_host: Option<HostConfig>,
    pub auth_host: Option<HostConfig>,
    pub ssi_auth_host: Option<HostConfig>,
    pub database_config: DatabaseConfig,
    pub ssh_user: Option<String>,
    pub ssh_private_key_path: Option<String>,
    pub ssi_wallet_config: SSIWalletConfig,
    pub client_config: ClientConfig,
    pub role: ConfigRoles,
}

impl Default for TransferConsumerApplicationConfig {
    fn default() -> Self {
        TransferConsumerApplicationConfig::from(ApplicationConsumerConfig::default())
    }
}

impl ApplicationConsumerConfigTrait for TransferConsumerApplicationConfig {
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
        &None
    }
    fn get_raw_ssi_auth_host(&self) -> &Option<HostConfig> {
        &self.ssi_auth_host
    }
    fn get_raw_database_config(&self) -> &DatabaseConfig {
        &self.database_config
    }
    fn get_raw_ssi_wallet_config(&self) -> &SSIWalletConfig {
        &self.ssi_wallet_config
    }

    fn merge_dotenv_configuration(&self) -> Self
    where
        Self: Sized,
    {
        let app_config = ApplicationConsumerConfig::default().merge_dotenv_configuration();
        TransferConsumerApplicationConfig::from(app_config)
    }

    fn get_raw_client_config(&self) -> &ClientConfig {
        &self.client_config
    }
}

impl From<ApplicationConsumerConfig> for TransferConsumerApplicationConfig {
    fn from(value: ApplicationConsumerConfig) -> Self {
        Self {
            transfer_process_host: value.transfer_process_host,
            business_system_host: value.business_system_host,
            contract_negotiation_host: value.contract_negotiation_host,
            catalog_bypass_host: value.catalog_bypass_host,
            auth_host: value.auth_host,
            ssi_auth_host: value.ssi_auth_host,
            database_config: value.database_config,
            ssh_user: value.ssh_user,
            ssh_private_key_path: value.ssh_private_key_path,
            ssi_wallet_config: value.ssi_wallet_config,
            role: value.role,
            client_config: value.client_config,
        }
    }
}

impl Into<ApplicationConsumerConfig> for TransferConsumerApplicationConfig {
    fn into(self) -> ApplicationConsumerConfig {
        ApplicationConsumerConfig {
            transfer_process_host: self.transfer_process_host,
            business_system_host: self.business_system_host,
            contract_negotiation_host: self.contract_negotiation_host,
            catalog_bypass_host: self.catalog_bypass_host,
            auth_host: self.auth_host,
            ssi_auth_host: self.ssi_auth_host,
            gateway_host: None,
            database_config: self.database_config,
            ssh_user: self.ssh_user,
            ssh_private_key_path: self.ssh_private_key_path,
            ssi_wallet_config: self.ssi_wallet_config,
            role: self.role,
            client_config: self.client_config,
        }
    }
}
