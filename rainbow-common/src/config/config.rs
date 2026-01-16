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
    CatalogConfig, CommonConfig, ContractsConfig, GatewayConfig, MonolithConfig, SsiAuthConfig, TransferConfig,
};
use crate::config::traits::MonoConfigTrait;
use crate::config::types::database::{DatabaseConfig, DbType};
use crate::config::types::{ApiConfig, CommonHostsConfig, HostConfig};
use crate::errors::{CommonErrors, ErrorLog};
use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{debug, error};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationConfig {
    monolith: Option<MonolithConfig>,
    transfer: Option<TransferConfig>,
    contracts: Option<ContractsConfig>,
    catalog: Option<CatalogConfig>,
    ssi_auth: Option<SsiAuthConfig>,
    gateway: Option<GatewayConfig>,
}

impl ApplicationConfig {
    pub fn new(common_config: CommonConfig) -> Self {
        Self {
            monolith: Some(MonolithConfig::new(common_config)),
            transfer: None,
            contracts: None,
            catalog: None,
            ssi_auth: None,
            gateway: None,
        }
    }
}

impl MonoConfigTrait for ApplicationConfig {
    fn mono(&self) -> &MonolithConfig {
        self.monolith.as_ref().expect("Trying to access core mode without it being defined")
    }

    fn is_mono_catalog_datahub(&self) -> bool {
        match &self.catalog {
            Some(catalog) => catalog.is_datahub(),
            None => false,
        }
    }
}

impl ApplicationConfig {
    pub fn ssi_auth(&self) -> SsiAuthConfig {
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
    pub fn gateway(&self) -> GatewayConfig {
        let module = match self.gateway.as_ref() {
            None => {
                let error = CommonErrors::module_new("gateway");
                error!("{}", error.log());
                None
            }
            Some(data) => Some(data.clone()),
        };
        module.expect("Trying to access core mode without it being defined")
    }
    pub fn load(env_file: Option<String>) -> anyhow::Result<Self> {
        if let Some(env_file) = env_file {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(env_file);
            debug!("Config file path: {}", path.display());

            let data = fs::read_to_string(&path).expect("Unable to read config file");
            let config = match serde_norway::from_str(&data) {
                Ok(config) => config,
                Err(e) => {
                    let error = CommonErrors::parse_new(&format!("Unable to parse config file: {}", e));
                    error!("{}", error.log());
                    bail!(error)
                }
            };

            Ok(config)
        } else {
            Ok(Self::new(Self::common()))
        }
    }

    pub fn common() -> CommonConfig {
        let host =
            HostConfig { protocol: "http".to_string(), url: "127.0.0.1".to_string(), port: Some("1200".to_string()) };
        let hosts = CommonHostsConfig { http: host, grpc: None, graphql: None };
        let db = DatabaseConfig { db_type: DbType::Postgres, url: "127.0.0.1".to_string(), port: "1400".to_string() };
        let keys_path = "static/certificates/".to_string();
        let api = ApiConfig { version: "v1".to_string(), openapi_path: "/static/specs/openapi/auth".to_string() };
        CommonConfig { hosts, db, api, keys_path, is_local: true }
    }
}
