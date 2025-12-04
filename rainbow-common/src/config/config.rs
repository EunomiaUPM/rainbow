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

use crate::config::services::{
    CatalogConfig, CommonConfig, ContractsConfig, MonolithConfig, SsiAuthConfig, TransferConfig,
};
use crate::config::traits::{ConfigLoader, DatabaseConfigTrait, HostConfigTrait, IsLocalTrait, MonoConfigTrait};
use crate::config::types::HostType;
use crate::errors::{CommonErrors, ErrorLog};
use serde::{Deserialize, Serialize};
use tracing::error;

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
        let module = match self.ssi_auth.as_ref() {
            None => {
                let error = CommonErrors::module_new("ssi_auth");
                error!("{}", error.log());
                None
            }
            Some(data) => Some(data.clone()),
        };
        module.expect("Trying to access core mode without it being defined")
    }
    pub fn transfer(&self) -> TransferConfig {
        let module = match self.transfer.as_ref() {
            None => {
                let error = CommonErrors::module_new("transfer");
                error!("{}", error.log());
                None
            }
            Some(data) => Some(data.clone()),
        };
        module.expect("Trying to access core mode without it being defined")
    }
    pub fn contracts(&self) -> ContractsConfig {
        let module = match self.contracts.as_ref() {
            None => {
                let error = CommonErrors::module_new("contracts");
                error!("{}", error.log());
                None
            }
            Some(data) => Some(data.clone()),
        };
        module.expect("Trying to access core mode without it being defined")
    }
    pub fn catalog(&self) -> CatalogConfig {
        let module = match self.catalog.as_ref() {
            None => {
                let error = CommonErrors::module_new("catalog");
                error!("{}", error.log());
                None
            }
            Some(data) => Some(data.clone()),
        };
        module.expect("Trying to access core mode without it being defined")
    }
}
