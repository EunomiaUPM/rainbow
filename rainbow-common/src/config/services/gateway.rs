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
use crate::config::min_know_services::MinKnownConfig;
use crate::config::services::CommonConfig;
use crate::config::traits::{
    ApiConfigTrait, CommonConfigTrait, ConfigLoader, DatabaseConfigTrait, HostConfigTrait, IsLocalTrait, KeysPathTrait,
    RoleTrait,
};
use crate::config::types::roles::RoleConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayConfig {
    common: CommonConfig,
    is_production: bool,
    transfer: MinKnownConfig,
    contracts: MinKnownConfig,
    catalog: MinKnownConfig,
    is_catalog_datahub: bool,
    ssi_auth: MinKnownConfig,
}

impl GatewayConfig {
    pub fn ssi_auth(&self) -> MinKnownConfig {
        self.ssi_auth.clone()
    }
    pub fn transfer(&self) -> MinKnownConfig {
        self.transfer.clone()
    }
    pub fn contracts(&self) -> MinKnownConfig {
        self.contracts.clone()
    }
    pub fn catalog(&self) -> MinKnownConfig {
        self.catalog.clone()
    }
    pub fn is_production(&self) -> bool {
        self.is_production
    }
    pub fn is_catalog_datahub(&self) -> bool {
        self.is_catalog_datahub
    }
}
impl ConfigLoader for GatewayConfig {
    fn default(common_config: CommonConfig) -> Self {
        let min_known_config =
            MinKnownConfig { hosts: common_config.hosts.clone(), api_version: common_config.api.openapi_path.clone() };

        Self {
            common: common_config.clone(),
            is_production: false,
            transfer: min_known_config.clone(),
            contracts: min_known_config.clone(),
            catalog: min_known_config.clone(),
            is_catalog_datahub: false,
            ssi_auth: min_known_config,
        }
    }

    fn load(role: RoleConfig, env_file: Option<String>) -> Self {
        match Self::global_load(role, env_file.clone()) {
            Ok(data) => data.gateway(),
            Err(_) => Self::local_load(role, env_file).expect("Unable to load catalog config"),
        }
    }
}

impl CommonConfigTrait for GatewayConfig {
    fn common(&self) -> &CommonConfig {
        &self.common
    }
}

impl HostConfigTrait for GatewayConfig {}

impl DatabaseConfigTrait for GatewayConfig {}

impl IsLocalTrait for GatewayConfig {}

impl KeysPathTrait for GatewayConfig {}

impl RoleTrait for GatewayConfig {}
impl ApiConfigTrait for GatewayConfig {}
