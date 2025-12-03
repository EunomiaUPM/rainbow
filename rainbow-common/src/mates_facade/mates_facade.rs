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

use crate::config::services::SsiAuthConfig;
use crate::config::traits::{ApiConfigTrait, HostConfigTrait};
use crate::config::types::HostType;
use crate::errors::helpers::BadFormat;
use crate::errors::{CommonErrors, ErrorLog};
use crate::mates::Mates;
use crate::mates_facade::MatesFacadeTrait;
use anyhow::bail;
use axum::async_trait;
use reqwest::Client;
use std::time::Duration;
use tracing::error;

pub struct MatesFacadeService {
    config: SsiAuthConfig,
    client: Client,
}

impl MatesFacadeService {
    pub fn new(config: SsiAuthConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { config, client }
    }
}

#[async_trait]
impl MatesFacadeTrait for MatesFacadeService {
    async fn get_mate_by_id(&self, mate_id: String) -> anyhow::Result<Mates> {
        let ssi_auth_url = self.config.get_host(HostType::Http);
        let mates_url = format!(
            "{}{}/mates/{}",
            ssi_auth_url,
            self.config.get_api_version(),
            mate_id
        );
        let response = self.client.get(mates_url).send().await.map_err(|_e| {
            let e = CommonErrors::missing_resource_new(&mate_id, "Not able to connect with ssi-auth server");
            error!("{}", e.log());
            return e;
        })?;

        if response.status().is_success() == false {
            let e = CommonErrors::missing_resource_new(&mate_id, "Mate not resolvable");
            error!("{}", e.log());
            bail!(e);
        }
        let mates = match response.json::<Mates>().await {
            Ok(mates) => mates,
            Err(e_) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Mate not serializable: {}", e_.to_string()),
                );
                error!("{}", e.log());
                bail!(e);
            }
        };
        Ok(mates)
    }

    async fn get_mate_by_slug(&self, mate_slug: String) -> anyhow::Result<Mates> {
        let ssi_auth_url = self.config.get_host(HostType::Http);
        let mates_url = format!(
            "{}{}/mates/slug/{}",
            ssi_auth_url,
            self.config.get_api_version(),
            mate_slug
        );
        let response = self.client.get(mates_url).send().await.map_err(|_e| {
            let e = CommonErrors::missing_resource_new(&mate_slug, "Not able to connect with ssi-auth server");
            error!("{}", e.log());
            return e;
        })?;
        if response.status().is_success() == false {
            let e = CommonErrors::missing_resource_new(&mate_slug, "Mate not resolvable");
            error!("{}", e.log());
            bail!(e);
        }
        let mates = match response.json::<Mates>().await {
            Ok(mates) => mates,
            Err(e_) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Mate not serializable: {}", e_.to_string()),
                );
                error!("{}", e.log());
                bail!(e);
            }
        };
        Ok(mates)
    }

    async fn get_me_mate(&self) -> anyhow::Result<Mates> {
        let ssi_auth_url = self.config.get_host(HostType::Http);
        let mates_url = format!("{}{}/mates/me", ssi_auth_url, self.config.get_api_version());
        let response = self.client.get(mates_url).send().await.map_err(|_e| {
            let e = CommonErrors::missing_resource_new("Me", "Not able to connect with ssi-auth server");
            error!("{}", e.log());
            return e;
        })?;
        if response.status().is_success() == false {
            let e = CommonErrors::missing_resource_new("Me", "Mate not resolvable");
            error!("{}", e.log());
            bail!(e);
        }
        let mates = match response.json::<Mates>().await {
            Ok(mates) => mates,
            Err(e_) => {
                let e = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Mate not serializable: {}", e_.to_string()),
                );
                error!("{}", e.log());
                bail!(e);
            }
        };
        Ok(mates)
    }
}
