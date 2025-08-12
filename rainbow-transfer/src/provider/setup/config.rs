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

use rainbow_common::config::global_config::{DatabaseConfig, HostConfig};
use rainbow_common::config::provider_config::{ApplicationProviderConfig, ApplicationProviderConfigTrait};
use rainbow_common::config::ConfigRoles;
use rainbow_common::ssi_wallet::{ClientConfig, SSIWalletConfig};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct TransferProviderApplicationConfig {
    transfer_process_host: Option<HostConfig>,
    business_system_host: Option<HostConfig>,
    catalog_host: Option<HostConfig>,
    catalog_as_datahub: bool,
    datahub_host: Option<HostConfig>,
    contract_negotiation_host: Option<HostConfig>,
    auth_host: Option<HostConfig>,
    ssi_auth_host: Option<HostConfig>,
    database_config: DatabaseConfig,
    ssh_user: Option<String>,
    ssh_private_key_path: Option<String>,
    ssi_wallet_config: SSIWalletConfig,
    client_config: ClientConfig,
    role: ConfigRoles,
}

impl Default for TransferProviderApplicationConfig {
    fn default() -> Self {
        TransferProviderApplicationConfig::from(ApplicationProviderConfig::default())
    }
}

impl ApplicationProviderConfigTrait for TransferProviderApplicationConfig {
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

    fn get_raw_ssi_wallet_config(&self) -> &SSIWalletConfig {
        &self.ssi_wallet_config
    }

    fn get_raw_datahub_token(&self) -> &String {
        todo!()
    }

    fn get_raw_contract_negotiation_host(&self) -> &Option<HostConfig> {
        &self.contract_negotiation_host
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

    fn get_raw_client_config(&self) -> &ClientConfig {
        &self.client_config
    }

    fn merge_dotenv_configuration(&self) -> Self {
        let app_config = ApplicationProviderConfig::default().merge_dotenv_configuration();
        TransferProviderApplicationConfig::from(app_config)
    }
}

impl From<ApplicationProviderConfig> for TransferProviderApplicationConfig {
    fn from(value: ApplicationProviderConfig) -> Self {
        Self {
            transfer_process_host: value.transfer_process_host,
            business_system_host: value.business_system_host,
            catalog_host: value.catalog_host,
            catalog_as_datahub: value.catalog_as_datahub,
            datahub_host: value.datahub_host,
            contract_negotiation_host: value.contract_negotiation_host,
            auth_host: value.auth_host,
            ssi_auth_host: value.ssi_auth_host,
            database_config: value.database_config,
            ssh_user: value.ssh_user,
            ssh_private_key_path: value.ssh_private_key_path,
            ssi_wallet_config: value.ssi_wallet_config,
            client_config: value.client_config,
            role: value.role,
        }
    }
}

impl Into<ApplicationProviderConfig> for TransferProviderApplicationConfig {
    fn into(self) -> ApplicationProviderConfig {
        ApplicationProviderConfig {
            transfer_process_host: self.transfer_process_host,
            business_system_host: self.business_system_host,
            catalog_host: self.catalog_host,
            catalog_as_datahub: self.catalog_as_datahub,
            datahub_host: self.datahub_host,
            datahub_token: "".to_string(),
            contract_negotiation_host: self.contract_negotiation_host,
            auth_host: self.auth_host,
            ssi_auth_host: self.ssi_auth_host,
            gateway_host: None,
            database_config: self.database_config,
            ssh_user: self.ssh_user,
            ssh_private_key_path: self.ssh_private_key_path,
            ssi_wallet_config: self.ssi_wallet_config,
            client_config: self.client_config,
            role: self.role,
        }
    }
}
