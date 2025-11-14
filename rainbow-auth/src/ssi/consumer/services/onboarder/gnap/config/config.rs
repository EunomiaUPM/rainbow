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
use crate::ssi::common::types::entities::SelfClient;
use crate::ssi::common::utils::read;
use crate::ssi::consumer::config::AuthConsumerConfig;
use crate::ssi::consumer::services::onboarder::gnap::config::GnapConfigTrait;
use rainbow_common::config::global_config::HostConfig;
use serde_json::{json, Value};

pub struct ConsumerGnapConfig {
    host: HostConfig,
    client: SelfClient,
    keys_path: String,
}

impl From<AuthConsumerConfig> for ConsumerGnapConfig {
    fn from(value: AuthConsumerConfig) -> Self {
        ConsumerGnapConfig {
            host: value.common_config.host,
            client: value.common_config.client,
            keys_path: value.common_config.keys_path,
        }
    }
}

impl GnapConfigTrait for ConsumerGnapConfig {
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
            "class_id" : self.client.class_id,
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
