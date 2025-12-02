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
    BusinessConfig, CatalogConfig, ContractsConfig, GatewayConfig, MonolithConfig, SsiAuthConfig, TransferConfig,
};
use crate::config::traits::{GlobalConfigTrait, HostConfigTrait};
use crate::config::types::database::{DatabaseConfig, DbType};
use crate::config::types::roles::RoleConfig;
use crate::config::types::{ApiConfig, CommonHostsConfig, HostConfig};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::debug;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationConfig {
    monolith: Option<MonolithConfig>,
    transfer: Option<TransferConfig>,
    business: Option<BusinessConfig>,
    contract: Option<ContractsConfig>,
    catalog: Option<CatalogConfig>,
    ssi_auth: Option<SsiAuthConfig>,
    gateway: Option<GatewayConfig>,
}

impl ApplicationConfig {
    fn new(role: RoleConfig) -> Self {
        match role {
            RoleConfig::Consumer => {
                let host = HostConfig {
                    protocol: "http".to_string(),
                    url: "127.0.0.1".to_string(),
                    port: Some("1100".to_string()),
                };
                let hosts = CommonHostsConfig { http: host, grpc: None, graphql: None };
                let db = DatabaseConfig {
                    db_type: DbType::Postgres,
                    url: "127.0.0.1".to_string(),
                    port: "1300".to_string(),
                    user: "ds_consumer".to_string(),
                    password: "ds_consumer".to_string(),
                    name: "ds_consumer".to_string(),
                };
                let keys_path = "static/certificates/consumer".to_string();
                Self {
                    monolith: Some(MonolithConfig::new(hosts, db, role, true, keys_path)),
                    transfer: None,
                    business: None,
                    contract: None,
                    catalog: None,
                    ssi_auth: None,
                    gateway: None,
                }
            }
            RoleConfig::Provider => {
                let host = HostConfig {
                    protocol: "http".to_string(),
                    url: "127.0.0.1".to_string(),
                    port: Some("1200".to_string()),
                };
                let hosts = CommonHostsConfig { http: host, grpc: None, graphql: None };
                let db = DatabaseConfig {
                    db_type: DbType::Postgres,
                    url: "127.0.0.1".to_string(),
                    port: "1400".to_string(),
                    user: "ds_provider".to_string(),
                    password: "ds_provider".to_string(),
                    name: "ds_provider".to_string(),
                };
                let keys_path = "static/certificates/consumer".to_string();
                Self {
                    monolith: Some(MonolithConfig::new(hosts, db, role, true, keys_path)),
                    transfer: None,
                    business: None,
                    contract: None,
                    catalog: None,
                    ssi_auth: None,
                    gateway: None,
                }
            }
        }
    }
}

impl GlobalConfigTrait for ApplicationConfig {
    fn load(role: RoleConfig, env_file: Option<String>) -> Self {
        if let Some(env_file) = env_file {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(env_file);
            debug!("Config file path: {}", path.display());

            let data = fs::read_to_string(&path).expect("Unable to read config file");
            serde_norway::from_str(&data).expect("Unable to parse config file")
        } else {
            ApplicationConfig::new(role)
        }
    }

    // MONOLITH FUNCTIONS
    fn get_mono_host(&self) -> String {
        let mono = self.monolith.as_ref().expect("Trying to access core mode without it being defined");
        mono.get_host()
    }
    fn get_mono_port(&self) -> String {
        let mono = self.monolith.as_ref().expect("Trying to access core mode without it being defined");
        mono.get_port().unwrap_or("".to_string())
    }
    fn is_mono_local(&self) -> bool {
        self.monolith
    }
    fn is_catalog_datahub(&self) -> bool {
        match &self.catalog {
            Some(catalog) => catalog.is_datahub(),
            None => false,
        }
    }
}
