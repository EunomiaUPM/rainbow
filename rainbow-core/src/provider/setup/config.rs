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
pub struct CoreProviderApplicationConfig {
    core_host: HostConfig,
    database_config: DatabaseConfig,
    role: ConfigRoles,
}

impl Default for CoreProviderApplicationConfig {
    fn default() -> Self {
        CoreProviderApplicationConfig {
            core_host: HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: "1234".to_string(),
            },
            database_config: DatabaseConfig {
                db_type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "5437".to_string(),
                user: "ds_core_provider_db".to_string(),
                password: "ds_core_provider_db".to_string(),
                name: "ds_core_provider_db".to_string(),
            },
            role: ConfigRoles::Provider,
        }
    }
}

impl CoreProviderApplicationConfig {
    pub fn get_full_host_url(&self) -> String {
        format!(
            "{}://{}:{}",
            self.core_host.protocol, self.core_host.url, self.core_host.port
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

    pub fn get_role(&self) -> ConfigRoles {
        self.role.clone()
    }

    pub fn merge_dotenv_configuration(&self) -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let compound_config = Self {
            core_host: HostConfig {
                protocol: std::env::var("HOST_PROTOCOL").unwrap_or(self.core_host.protocol.to_string()),
                url: std::env::var("HOST_URL").unwrap_or(self.core_host.url.to_string()),
                port: std::env::var("HOST_PORT").unwrap_or(self.core_host.port.to_string()),
            },
            database_config: DatabaseConfig {
                db_type: std::env::var("DB_TYPE")
                    .unwrap_or(self.database_config.db_type.to_string())
                    .parse()
                    .expect("Db type error"),
                url: std::env::var("DB_URL").unwrap_or(self.database_config.url.to_string()),
                port: std::env::var("DB_PORT").unwrap_or(self.database_config.port.to_string()),
                user: std::env::var("DB_USER").unwrap_or(self.database_config.user.to_string()),
                password: std::env::var("DB_PASSWORD").unwrap_or(self.database_config.password.to_string()),
                name: std::env::var("DB_DATABASE").unwrap_or(self.database_config.name.to_string()),
            },
            role: ConfigRoles::Provider,
        };
        Ok(compound_config)
    }
    pub fn get_host_url(&self) -> String {
        self.core_host.url.clone()
    }
    pub fn get_host_port(&self) -> String {
        self.core_host.port.clone()
    }
}
