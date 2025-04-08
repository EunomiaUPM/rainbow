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

#[derive(Serialize, Copy, Clone)]
struct HostConfig<'a> {
    protocol: &'a str,
    url: &'a str,
    port: &'a str,
}

#[derive(Serialize, Copy, Clone)]
struct DatabaseConfig<'a> {
    db_type: &'a DbType,
    url: &'a str,
    port: &'a str,
    user: &'a str,
    password: &'a str,
    name: &'a str,
}

#[derive(Serialize, Copy, Clone)]
pub struct CoreProviderApplicationConfig<'a> {
    core_host: HostConfig<'a>,
    database_config: DatabaseConfig<'a>,
    role: ConfigRoles,
}


impl<'a> Default for CoreProviderApplicationConfig<'a> {
    fn default() -> Self {
        CoreProviderApplicationConfig {
            core_host: HostConfig { protocol: "http", url: "127.0.0.1", port: "1234" },
            database_config: DatabaseConfig {
                db_type: &DbType::Postgres,
                url: "127.0.0.1",
                port: "5437",
                user: "ds_core_provider_db",
                password: "ds_core_provider_db",
                name: "ds_core_provider_db",
            },
            role: ConfigRoles::Provider,
        }
    }
}

impl<'a> CoreProviderApplicationConfig<'a> {
    pub fn get_full_host_url(&self) -> String {
        format!(
            "{}://{}:{}",
            self.core_host.protocol,
            self.core_host.url,
            self.core_host.port
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
        dotenvy::dotenv()?;
        let compound_config = Self {
            core_host: HostConfig {
                protocol: option_env!("HOST_PROTOCOL")
                    .unwrap_or(self.core_host.protocol),
                url: option_env!("HOST_URL").unwrap_or(self.core_host.url),
                port: option_env!("HOST_PORT").unwrap_or(self.core_host.port),
            },
            database_config: DatabaseConfig {
                db_type: option_env!("DB_TYPE")
                    .unwrap_or(self.database_config.db_type.to_string().as_str())
                    .parse()
                    .expect("Db type error"),
                url: option_env!("DB_URL").unwrap_or(self.database_config.url),
                port: option_env!("DB_PORT").unwrap_or(self.database_config.port),
                user: option_env!("DB_USER").unwrap_or(self.database_config.user),
                password: option_env!("DB_PASSWORD").unwrap_or(self.database_config.password),
                name: option_env!("DB_DATABASE").unwrap_or(self.database_config.name),
            },
            role: ConfigRoles::Provider,
        };
        Ok(compound_config)
    }
    pub fn get_host_url(&self) -> &'a str {
        self.core_host.url
    }
    pub fn get_host_port(&self) -> &'a str {
        self.core_host.port
    }
}
