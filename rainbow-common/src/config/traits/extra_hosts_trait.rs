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
use crate::config::types::{HostConfig, HostType};
use crate::utils::get_host_helper;

pub trait ExtraHostsTrait {
    fn http(&self) -> &HostConfig;
    fn grpc(&self) -> Option<&HostConfig>;
    fn graphql(&self) -> Option<&HostConfig>;
    fn get_host(&self, host_type: HostType) -> String {
        let host = match host_type {
            HostType::Http => Self::get_host_helper(Some(self.http()), &host_type.to_string()),
            HostType::Grpc => Self::get_host_helper(self.grpc(), &host_type.to_string()),
            HostType::Graphql => Self::get_host_helper(self.graphql(), &host_type.to_string()),
        };
        host.expect("Failed to get host")
    }
    fn get_host_helper(host: Option<&HostConfig>, module: &str) -> anyhow::Result<String> {
        get_host_helper(host, module)
    }
}
