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

use crate::config::services::{CommonConfig, MinKnownConfig};
use crate::config::traits::{CacheConfigTrait, CommonConfigTrait, ConfigLoader};
use crate::config::types::cache::CacheConfig;
use crate::errors::{CommonErrors, ErrorLog};
use crate::utils::get_host_helper;
use serde::{Deserialize, Serialize};
use tracing::error;
use ymir::config::types::HostConfig;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CatalogConfig {
    common: CommonConfig,
    cache: CacheConfig,
    is_datahub: bool,
    policy_templates_folder: Option<String>,
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
    pub fn cache(&self) -> CacheConfig {
        self.cache.clone()
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
    pub fn get_policy_templates_folder(&self) -> String {
        self.policy_templates_folder.clone().unwrap_or("/".to_string())
    }
}

impl ConfigLoader for CatalogConfig {
    fn load(env_file: String) -> Self {
        match Self::global_load(env_file.clone()) {
            Ok(data) => data.catalog(),
            Err(_) => Self::local_load(env_file).expect("Unable to load catalog config"),
        }
    }
}

impl CommonConfigTrait for CatalogConfig {
    fn common(&self) -> &CommonConfig {
        &self.common
    }
}

impl CacheConfigTrait for CatalogConfig {
    fn cache_config(&self) -> &CacheConfig {
        &self.cache
    }
}

impl CatalogConfig {
    pub fn is_datahub(&self) -> bool {
        self.is_datahub
    }
}
