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
use crate::config::types::roles::RoleConfig;
use crate::config::ApplicationConfig;
use crate::errors::{CommonErrors, ErrorLog};
use serde::de::DeserializeOwned;
use std::fs;
use std::path::PathBuf;
use tracing::error;

pub trait ConfigLoader: Sized + DeserializeOwned {
    fn default(common_config: CommonConfig) -> Self;
    fn load(role: RoleConfig, env_file: Option<String>) -> Self;
    fn global_load(role: RoleConfig, env_file: Option<String>) -> anyhow::Result<ApplicationConfig> {
        ApplicationConfig::load(role, env_file)
    }

    fn local_load(role: RoleConfig, env_file: Option<String>) -> anyhow::Result<Self> {
        if let Some(file) = env_file {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(file);
            let data = fs::read_to_string(&path).expect("Cannot read local config");
            let config: Self = serde_norway::from_str(&data).map_err(|e| {
                let error = CommonErrors::parse_new(&format!("Unable to load local config: {}", e));
                error!("{}", error.log());
                error
            })?;
            Ok(config)
        } else {
            Ok(Self::default(ApplicationConfig::common(role)))
        }
    }
}
