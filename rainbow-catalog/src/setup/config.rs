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

use rainbow_common::config::config::ConfigRoles;
use rainbow_common::config::database::DbType;
use serde::Serialize;

#[derive(Serialize, Clone)]
struct HostConfig {
    protocol: String,
    url: String,
    port: String,
}

#[derive(Serialize, Clone)]
struct DatabaseConfig {
    db_type: DbType,
    url: String,
    port: String,
    user: String,
    password: String,
    name: String,
}

#[derive(Serialize, Clone)]
pub struct CatalogApplicationConfig {
    catalog_host: HostConfig,
    auth_host: HostConfig,
    database_config: DatabaseConfig,
    role: ConfigRoles,
}

impl Default for CatalogApplicationConfig {
    fn default() -> Self {
        CatalogApplicationConfig {
            catalog_host: HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1236".to_string(),
            },
            auth_host: HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1231".to_string(),
            },
            database_config: DatabaseConfig {
                db_type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "5435".to_string(),
                user: "ds-protocol-catalog".to_string(),
                password: "ds-protocol-catalog".to_string(),
                name: "ds-protocol-catalog".to_string(),
            },
            role: ConfigRoles::Catalog,
        }
    }
}

impl CatalogApplicationConfig {
    pub fn get_full_host_url(&self) -> String {
        format!(
            "{}://{}:{}",
            self.catalog_host.protocol, self.catalog_host.url, self.catalog_host.port
        )
    }
    pub fn get_full_db_url(&self) -> String {
        match self.database_config.db_type {
            DbType::Memory => ":memory:".to_string(),
            _ => format!(
                "{}://{}:{}@{}:{}/{}",
                self.database_config.db_type,
                self.database_config.user,
                self.database_config.password,
                self.database_config.url,
                self.database_config.port,
                self.database_config.name
            ),
        }
    }
    pub fn get_auth_url(&self) -> String {
        format!(
            "{}://{}:{}",
            self.catalog_host.protocol, self.catalog_host.url, self.catalog_host.port
        )
    }
    pub fn get_role(&self) -> ConfigRoles {
        self.role.clone()
    }
    pub fn merge_dotenv_configuration(&self) -> anyhow::Result<Self> {
        dotenvy::dotenv()?;
        let compound_config = Self {
            catalog_host: HostConfig {
                protocol: option_env!("HOST_PROTOCOL")
                    .unwrap_or(self.catalog_host.protocol.as_str())
                    .to_string(),
                url: option_env!("HOST_URL").unwrap_or(self.catalog_host.url.as_str()).to_string(),
                port: option_env!("HOST_PORT").unwrap_or(self.catalog_host.port.as_str()).to_string(),
            },
            auth_host: HostConfig {
                protocol: option_env!("AUTH_PROTOCOL").unwrap_or(self.auth_host.protocol.as_str()).to_string(),
                url: option_env!("AUTH_URL").unwrap_or(self.auth_host.url.as_str()).to_string(),
                port: option_env!("AUTH_PORT").unwrap_or(self.auth_host.port.as_str()).to_string(),
            },
            database_config: DatabaseConfig {
                db_type: option_env!("DB_TYPE")
                    .unwrap_or(self.database_config.db_type.to_string().as_str())
                    .parse()
                    .expect("Db type error"),
                url: option_env!("DB_URL").unwrap_or(self.database_config.url.as_str()).to_string(),
                port: option_env!("DB_PORT").unwrap_or(self.database_config.port.as_str()).to_string(),
                user: option_env!("DB_USER").unwrap_or(self.database_config.user.as_str()).to_string(),
                password: option_env!("DB_PASSWORD").unwrap_or(self.database_config.password.as_str()).to_string(),
                name: option_env!("DB_DATABASE").unwrap_or(self.database_config.name.as_str()).to_string(),
            },
            role: ConfigRoles::Consumer,
        };
        Ok(compound_config)
    }
    pub fn get_host_url(&self) -> String {
        self.catalog_host.url.clone()
    }
    pub fn get_host_port(&self) -> String {
        self.catalog_host.port.clone()
    }
}
