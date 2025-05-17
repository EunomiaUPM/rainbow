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
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ContractNegotiationApplicationProviderConfig {
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
    role: ConfigRoles,
}

impl Default for ContractNegotiationApplicationProviderConfig {
    fn default() -> Self {
        ContractNegotiationApplicationProviderConfig::from(ApplicationProviderConfig::default())
    }
}


impl ApplicationProviderConfigTrait for ContractNegotiationApplicationProviderConfig {
    fn ssh_user(&self) -> Option<String> { self.ssh_user.clone() }
    fn ssh_private_key_path(&self) -> Option<String> { self.ssh_private_key_path.clone() }
    fn is_datahub_as_catalog(&self) -> bool { self.catalog_as_datahub }
    fn get_role(&self) -> ConfigRoles { self.role }
    fn get_raw_transfer_process_host(&self) -> &Option<HostConfig> { &None }
    fn get_raw_business_system_host(&self) -> &Option<HostConfig> { &self.business_system_host }
    fn get_raw_catalog_host(&self) -> &Option<HostConfig> { &self.catalog_host }
    fn get_raw_datahub_host(&self) -> &Option<HostConfig> { &self.datahub_host }
    fn get_raw_contract_negotiation_host(&self) -> &Option<HostConfig> { &self.contract_negotiation_host }
    fn get_raw_auth_host(&self) -> &Option<HostConfig> { &self.auth_host }
    fn get_raw_ssi_auth_host(&self) -> &Option<HostConfig> { &self.ssi_auth_host }
    fn get_raw_database_config(&self) -> &DatabaseConfig { &self.database_config }
    fn merge_dotenv_configuration(&self) -> Self {
        let app_config = ApplicationProviderConfig::default().merge_dotenv_configuration();
        ContractNegotiationApplicationProviderConfig::from(app_config)
    }
}

impl From<ApplicationProviderConfig> for ContractNegotiationApplicationProviderConfig {
    fn from(value: ApplicationProviderConfig) -> Self {
        Self {
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
            role: value.role,
        }
    }
}

impl Into<ApplicationProviderConfig> for ContractNegotiationApplicationProviderConfig {
    fn into(self) -> ApplicationProviderConfig {
        ApplicationProviderConfig {
            transfer_process_host: None,
            business_system_host: self.business_system_host,
            catalog_host: self.catalog_host,
            catalog_as_datahub: self.catalog_as_datahub,
            datahub_host: self.datahub_host,
            contract_negotiation_host: self.contract_negotiation_host,
            auth_host: self.auth_host,
            ssi_auth_host: self.ssi_auth_host,
            database_config: self.database_config,
            ssh_user: self.ssh_user,
            ssh_private_key_path: self.ssh_private_key_path,
            role: self.role,
        }
    }
}
