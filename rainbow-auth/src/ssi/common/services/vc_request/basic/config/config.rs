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
use super::VCRequesterConfigTrait;
use crate::ssi::common::types::entities::SelfClient;
use crate::ssi::utils::read;
use serde_json::{json, Value};
use rainbow_common::config::global_config::HostConfig;

pub struct VCRequesterConfig {
    host: HostConfig,
    client: SelfClient,
    keys_path: String,
}

impl VCRequesterConfigTrait for VCRequesterConfig {
    fn get_pretty_client_config(&self) -> anyhow::Result<Value> {
        let path = format!("{}/cert.pem", self.keys_path);
        let cert = read(&path)?;

        let clean_cert = cert.lines().filter(|line| !line.starts_with("-----")).collect::<String>();

        let key = json!({
            "proof": "httpsig",
            "cert": clean_cert
        });
        Ok(json!({
            "key" : key,
            "class_id" : self.client.name,
            "display" : self.client.display,
        }))
    }
    fn get_host(&self) -> String {
        let host = self.host.clone();
        match host.port.is_empty() {
            true => {
                format!("{}://{}", host.protocol, host.url)
            }
            false => {
                format!("{}://{}:{}", host.protocol, host.url, host.port)
            }
        }
    }
}
