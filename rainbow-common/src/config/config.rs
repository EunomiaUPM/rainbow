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

use crate::config::services::traits::MonoConfigTrait;
use crate::config::services::{
    CatalogConfig, CommonConfig, ContractsConfig, MonolithConfig, SsiAuthConfig, TransferConfig,
};
use crate::config::traits::{ConfigLoader, DatabaseConfigTrait, HostConfigTrait, IsLocalTrait};
use crate::config::types::HostType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationConfig {
    monolith: Option<MonolithConfig>,
    transfer: Option<TransferConfig>,
    contracts: Option<ContractsConfig>,
    catalog: Option<CatalogConfig>,
    ssi_auth: Option<SsiAuthConfig>,
}

impl ApplicationConfig {
    fn new(common_config: CommonConfig) -> Self {
        Self {
            monolith: Some(MonolithConfig::new(common_config)),
            transfer: None,
            contracts: None,
            catalog: None,
            ssi_auth: None,
        }
    }
}

impl MonoConfigTrait for ApplicationConfig {
    fn get_mono_host(&self) -> String {
        let mono = self.monolith.as_ref().expect("Trying to access core mode without it being defined");
        mono.get_host(HostType::Http)
    }
    fn get_weird_mono_port(&self) -> String {
        let mono = self.monolith.as_ref().expect("Trying to access core mode without it being defined");
        mono.get_weird_port()
    }
    fn get_mono_db(&self) -> String {
        let mono = self.monolith.as_ref().expect("Trying to access core mode without it being defined");
        mono.get_full_db_url()
    }
    fn is_mono_local(&self) -> bool {
        let mono = self.monolith.as_ref().expect("Trying to access core mode without it being defined");
        mono.is_local()
    }

    fn is_mono_catalog_datahub(&self) -> bool {
        match &self.catalog {
            Some(catalog) => catalog.is_datahub(),
            None => false,
        }
    }
}
impl ConfigLoader for ApplicationConfig {
    fn default_with_config(config: CommonConfig) -> Self {
        ApplicationConfig::new(config)
    }
}

impl ApplicationConfig {
    pub fn ssi_auth_config(&self) -> SsiAuthConfig {
        self.ssi_auth.clone().expect("Trying to access core mode without it being defined")
    }
}
