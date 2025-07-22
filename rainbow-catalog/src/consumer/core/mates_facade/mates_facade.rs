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

use crate::consumer::core::mates_facade::MatesFacadeTrait;
use anyhow::{anyhow, bail};
use axum::async_trait;
use rainbow_common::config::global_config::{format_host_config_to_url_string, ApplicationGlobalConfig};
use rainbow_common::mates::Mates;
use reqwest::Client;
use std::time::Duration;

pub struct MatesFacadeService {
    config: ApplicationGlobalConfig,
    client: Client,
}

impl MatesFacadeService {
    pub fn new(config: ApplicationGlobalConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { config, client }
    }
}

#[async_trait]
impl MatesFacadeTrait for MatesFacadeService {
    async fn get_mate_by_id(&self, mate_id: String) -> anyhow::Result<Mates> {
        let ssi_auth_url = format_host_config_to_url_string(&self.config.ssi_auth_host.clone().expect("Auth host not configured"));
        let mates_url = format!("{}/api/v1/mates/{}", ssi_auth_url, mate_id);
        let response = self.client
            .get(mates_url)
            .send()
            .await
            .map_err(|e| anyhow!("not able to connect with ssi-auth server"))?;
        if response.status().is_success() == false {
            bail!("response failed");
        }
        let mates = match response.json::<Mates>().await {
            Ok(mates) => mates,
            Err(_) => bail!("response not serializable")
        };
        Ok(mates)
    }

    async fn get_mate_by_slug(&self, mate_slug: String) -> anyhow::Result<Mates> {
        let ssi_auth_url = format_host_config_to_url_string(&self.config.ssi_auth_host.clone().expect("Auth host not configured"));
        let mates_url = format!("{}/api/v1/mates/slug/{}", ssi_auth_url, mate_slug);
        let response = self.client
            .get(mates_url)
            .send()
            .await
            .map_err(|e| anyhow!("not able to connect with ssi-auth server"))?;
        if response.status().is_success() == false {
            bail!("response failed");
        }
        let mates = match response.json::<Mates>().await {
            Ok(mates) => mates,
            Err(_) => bail!("response not serializable")
        };
        Ok(mates)
    }

    async fn get_me_mate(&self) -> anyhow::Result<Mates> {
        let ssi_auth_url = format_host_config_to_url_string(&self.config.ssi_auth_host.clone().expect("Auth host not configured"));
        let mates_url = format!("{}/api/v1/mates/me", ssi_auth_url);
        let response = self.client
            .get(mates_url)
            .send()
            .await
            .map_err(|e| anyhow!("not able to connect with ssi-auth server"))?;
        if response.status().is_success() == false {
            bail!("response failed");
        }
        let mates = match response.json::<Mates>().await {
            Ok(mates) => mates,
            Err(_) => bail!("response not serializable")
        };
        Ok(mates)
    }
}
