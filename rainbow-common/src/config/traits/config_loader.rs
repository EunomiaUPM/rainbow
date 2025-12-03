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

use crate::config::services::CommonConfig;
use crate::config::types::database::{DatabaseConfig, DbType};
use crate::config::types::roles::RoleConfig;
use crate::config::types::{ApiConfig, CommonHostsConfig, HostConfig};
use serde::de::DeserializeOwned;
use std::{fs, path::PathBuf};
use tracing::debug;

pub trait ConfigLoader: Sized + DeserializeOwned {
    fn default_with_config(common_config: CommonConfig) -> Self;
    fn load(role: RoleConfig, env_file: Option<String>) -> Self {
        if let Some(env_file) = env_file {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(env_file);
            debug!("Config file path: {}", path.display());

            let data = fs::read_to_string(&path).expect("Unable to read config file");
            serde_norway::from_str(&data).expect("Unable to parse config file")
        } else {
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
            let keys_path = "static/certificates/".to_string();
            let api = ApiConfig { version: "v1".to_string(), openapi_path: "/static/specs/openapi/auth".to_string() };
            let config = CommonConfig { hosts, db, role, api, keys_path, is_local: true };
            Self::default_with_config(config)
        }
    }
}
