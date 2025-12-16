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

use crate::config::min_know_services::MinKnownConfig;
use crate::config::types::HostType;
use crate::errors::helpers::{BadFormat, MissingAction};
use crate::errors::{CommonErrors, ErrorLog};
use crate::facades::ssi_auth_facade::SSIAuthFacadeTrait;
use crate::mates::mates::VerifyTokenRequest;
use crate::mates::Mates;
use anyhow::bail;
use axum::async_trait;
use reqwest::Client;
use std::time::Duration;
use tracing::error;

pub struct SSIAuthFacadeService {
    config: MinKnownConfig,
    client: Client,
}

impl SSIAuthFacadeService {
    pub fn new(config: MinKnownConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { config, client }
    }
}

#[async_trait]
impl SSIAuthFacadeTrait for SSIAuthFacadeService {
    async fn verify_token(&self, token: String) -> anyhow::Result<Mates> {
        let base_url = self.config.get_host(HostType::Http);
        let url = format!("{}{}/mates/token", base_url, self.config.get_api_version());
        let response = self.client.post(&url).json(&VerifyTokenRequest { token }).send().await;
        let response = match response {
            Ok(response) => response,
            Err(_e) => {
                let e = CommonErrors::missing_action_new(MissingAction::Token, "Not able to verify token");
                error!("{}", e.log());
                bail!(e);
            }
        };
        if response.status().is_success() == false {
            let e = CommonErrors::missing_action_new(MissingAction::Token, "Not able to verify token");
            error!("{}", e.log());
            bail!(e);
        }
        let mate = match response.json::<Mates>().await {
            Ok(mate) => mate,
            Err(e) => {
                let e_ = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Not able to deserialize entity: {}", e.to_string()),
                );
                error!("{}", e_.log());
                bail!(e_);
            }
        };
        Ok(mate)
    }
}
