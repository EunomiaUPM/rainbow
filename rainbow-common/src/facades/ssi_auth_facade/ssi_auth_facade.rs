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

use crate::config::global_config::{format_host_config_to_url_string, ApplicationGlobalConfig};
use crate::facades::ssi_auth_facade::SSIAuthFacadeTrait;
use crate::mates::mates::VerifyTokenRequest;
use crate::mates::Mates;
use anyhow::bail;
use axum::async_trait;
use reqwest::Client;
use std::time::Duration;

pub struct SSIAuthFacadeService {
    config: ApplicationGlobalConfig,
    client: Client,
}
impl SSIAuthFacadeService {
    pub fn new(config: ApplicationGlobalConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { config, client }
    }
}

#[async_trait]
impl SSIAuthFacadeTrait for SSIAuthFacadeService {
    async fn verify_token(&self, token: String) -> anyhow::Result<Mates> {
        let base_url = format_host_config_to_url_string(&self.config.ssi_auth_host.clone().unwrap());
        let url = format!("{}/api/v1/verify/mate/token", base_url);
        let response = self.client
            .post(url)
            .json(&VerifyTokenRequest {
                token: token
            })
            .send()
            .await;
        let response = match response {
            Ok(response) => response,
            Err(e) => bail!("Not able to verify token: {}", e.to_string())
        };
        if response.status().is_success() == false {
            bail!("Not able to verify token, request not accepted")
        }
        let mate = match response.json::<Mates>().await {
            Ok(mate) => mate,
            Err(e) => bail!("Not able to deserialize mate: {}", e.to_string())
        };
        Ok(mate)
    }
}
