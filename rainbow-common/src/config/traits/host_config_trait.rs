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
use crate::config::traits::CommonConfigTrait;
use crate::config::types::HostConfig;
use crate::errors::{CommonErrors, ErrorLog};
use anyhow::bail;
use tracing::error;

pub trait HostConfigTrait: CommonConfigTrait {
    fn get_host(&self) -> String {
        Self::get_host_helper(Some(&self.common().hosts().http)).expect("Http host is invalid")
    }
    fn get_graphql_host(&self) -> anyhow::Result<String> {
        Self::get_host_helper(self.common().hosts().graphql.as_ref())
    }
    fn get_grpc_host(&self) -> anyhow::Result<String> {
        Self::get_host_helper(self.common().hosts().grpc.as_ref())
    }

    fn get_weird_port(&self) -> String {
        match self.common().hosts.http.port.as_ref() {
            Some(port) => format!(":{}", port),
            None => "".to_string(),
        }
    }

    // HELPERS

    fn get_host_helper(host: Option<&HostConfig>) -> anyhow::Result<String> {
        match host {
            Some(host) => match host.port.as_ref() {
                Some(port) => Ok(format!("{}://{}:{}", host.protocol, host.url, port)),
                None => Ok(format!("{}://{}", host.protocol, host.url)),
            },
            None => {
                let error = CommonErrors::module_new("grpc");
                error!("{}", error.log());
                bail!(error)
            }
        }
    }
}
