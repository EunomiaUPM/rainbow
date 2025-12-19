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

use super::MatesFacadeTrait;
use crate::config::min_know_services::MinKnownConfig;
use crate::config::types::HostType;
use crate::errors::helpers::BadFormat;
use crate::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use crate::config::global_config::{format_host_config_to_url_string, ApplicationGlobalConfig};
use crate::http_client::HttpClient;
use crate::mates::Mates;
use anyhow::bail;
use crate::mates_facade::MatesFacadeTrait;
use axum::async_trait;
use reqwest::Client;
use std::time::Duration;
use tracing::error;

pub struct MatesFacadeService {
    config: MinKnownConfig,
    client: Arc<HttpClient>,
}

impl MatesFacadeService {
    pub fn new(config: MinKnownConfig, client: Arc<HttpClient>) -> Self {
        Self { config, client }
    }
}

#[async_trait]
impl MatesFacadeTrait for MatesFacadeService {
    async fn get_mate_by_id(&self, mate_id: String) -> anyhow::Result<Mates> {
        let ssi_auth_url =
            format_host_config_to_url_string(&self.config.ssi_auth_host.clone().expect("Auth host not configured"));
        let mates_url = format!("{}/api/v1/mates/{}", ssi_auth_url, mate_id);
        let mates = self.client.get_json::<Mates>(mates_url.as_str()).await?;
        Ok(mates)
    }

    async fn get_mate_by_slug(&self, mate_slug: String) -> anyhow::Result<Mates> {
        let ssi_auth_url =
            format_host_config_to_url_string(&self.config.ssi_auth_host.clone().expect("Auth host not configured"));
        let mates_url = format!("{}/api/v1/mates/slug/{}", ssi_auth_url, mate_slug);
        let mates = self.client.get_json::<Mates>(mates_url.as_str()).await?;
        Ok(mates)
    }

    async fn get_me_mate(&self) -> anyhow::Result<Mates> {
        let ssi_auth_url =
            format_host_config_to_url_string(&self.config.ssi_auth_host.clone().expect("Auth host not configured"));
        let mates_url = format!("{}/api/v1/mates/myself", ssi_auth_url);
        let mates = self.client.get_json::<Mates>(mates_url.as_str()).await?;
        Ok(mates)
    }
}
