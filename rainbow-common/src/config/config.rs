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

use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum ConfigRoles {
    Catalog,
    Contracts,
    Consumer,
    Provider,
    Auth,
}

impl FromStr for ConfigRoles {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Catalog" => Ok(ConfigRoles::Catalog),
            "Contracts" => Ok(ConfigRoles::Contracts),
            "Consumer" => Ok(ConfigRoles::Consumer),
            "Provider" => Ok(ConfigRoles::Provider),
            "Auth" => Ok(ConfigRoles::Auth),
            _ => bail!("Invalid config role: {}", s),
        }
    }
}

impl ToString for ConfigRoles {
    fn to_string(&self) -> String {
        match self {
            ConfigRoles::Catalog => "Catalog".to_string(),
            ConfigRoles::Contracts => "Contracts".to_string(),
            ConfigRoles::Consumer => "Consumer".to_string(),
            ConfigRoles::Provider => "Provider".to_string(),
            ConfigRoles::Auth => "Auth".to_string(),
        }
    }
}
