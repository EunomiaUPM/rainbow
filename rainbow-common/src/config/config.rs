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
    BusinessConfig, ContractsConfig, GatewayConfig, SsiAuthConfig, TransferConfig,
    CatalogConfig,
};
use crate::config::types::database::{DatabaseConfig, DbType};
use crate::config::types::roles::RoleConfig;
use crate::config::types::ApiConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationProviderConfig {
    transfer: Option<TransferConfig>,
    business: Option<BusinessConfig>,
    contract: Option<ContractsConfig>,
    catalog: Option<CatalogConfig>,
    ssi_auth: Option<SsiAuthConfig>,
    gateway: Option<GatewayConfig>,
    database: DatabaseConfig,
    api: ApiConfig,
    role: RoleConfig,
    is_local: bool,
}

impl Default for ApplicationProviderConfig {
    fn default() -> Self {
        Self {
            transfer: None,
            business: None,
            contract: None,
            catalog: None,
            ssi_auth: None,
            gateway: None,
            database: DatabaseConfig {
                db_type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "1300".to_string(),
                user: "ds_provider".to_string(),
                password: "ds_provider".to_string(),
                name: "ds_provider".to_string(),
            },
            api: ApiConfig { version: "v1".to_string(), openapi_path: "static/specs/openapi".to_string() },
            role: RoleConfig::Consumer,
            is_local: false,
        }
    }
}
