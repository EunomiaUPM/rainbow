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

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::config::traits::ExtraHostsTrait;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HostConfig {
    pub protocol: String,
    pub url: String,
    pub port: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommonHostsConfig {
    pub http: HostConfig,
    pub grpc: Option<HostConfig>,
    pub graphql: Option<HostConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum HostType {
    Http,
    Grpc,
    Graphql,
}

impl Display for HostType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HostType::Http => "http".to_string(),
            HostType::Grpc => "grpc".to_string(),
            HostType::Graphql => "graphql".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl ExtraHostsTrait for CommonHostsConfig {
    fn http(&self) -> &HostConfig {
        &self.http
    }

    fn grpc(&self) -> Option<&HostConfig> {
        self.grpc.as_ref()
    }

    fn graphql(&self) -> Option<&HostConfig> {
        self.graphql.as_ref()
    }
}