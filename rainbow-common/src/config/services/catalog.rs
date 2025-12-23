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
use crate::config::types::HostConfig;
use crate::errors::{CommonErrors, ErrorLog};
use crate::utils::get_host_helper;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CatalogConfig {
    common: CommonConfig,
    is_datahub: bool,
    datahub_host: Option<HostConfig>,
    datahub_token: Option<String>,
    ssi_auth: MinKnownConfig,
    contracts: MinKnownConfig,
}

impl CatalogConfig {
    pub fn contracts(&self) -> MinKnownConfig {
        self.contracts.clone()
    }

    pub fn ssi_auth(&self) -> MinKnownConfig {
        self.ssi_auth.clone()
    }
    pub fn get_datahub_host(&self) -> String {
        let host = get_host_helper(self.datahub_host.as_ref(), "datahub");
        host.expect("datahub_host not found")
    }
    pub fn get_datahub_token(&self) -> String {
        let token = match self.datahub_token.clone() {
            Some(datahub_token) => Some(datahub_token),
            None => {
                let error = CommonErrors::module_new("datahub token");
                error!("{}", error.log());
                None
            }
        };
        token.expect("datahub_token not found")
    }
}

impl ConfigLoader for CatalogConfig {
    fn default(common_config: CommonConfig) -> Self {
        Self {
            common: common_config.clone(),
            is_datahub: false,
            datahub_host: None,
            datahub_token: None,
            ssi_auth: MinKnownConfig {
                hosts: common_config.hosts.clone(),
                api_version: common_config.api.openapi_path.clone(),
            },
            contracts: MinKnownConfig {
                hosts: common_config.hosts.clone(),
                api_version: common_config.api.openapi_path.clone(),
            },
        }
    }

    fn load(role: RoleConfig, env_file: Option<String>) -> Self {
        match Self::global_load(role, env_file.clone()) {
            Ok(data) => data.catalog(),
            Err(_) => Self::local_load(role, env_file).expect("Unable to load catalog config"),
        }
    }
}

impl CommonConfigTrait for CatalogConfig {
    fn common(&self) -> &CommonConfig {
        &self.common
    }
}

impl HostConfigTrait for CatalogConfig {}

impl DatabaseConfigTrait for CatalogConfig {}

impl IsLocalTrait for CatalogConfig {}

impl KeysPathTrait for CatalogConfig {}

impl RoleTrait for CatalogConfig {}
impl ApiConfigTrait for CatalogConfig {}

impl CatalogConfig {
    pub fn is_datahub(&self) -> bool {
        self.is_datahub
    }
}
