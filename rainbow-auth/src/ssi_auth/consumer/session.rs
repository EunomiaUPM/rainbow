/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use crate::ssi_auth::consumer::SSI_AUTH_HTTP_CLIENT;
use anyhow::bail;
use log::error;
use rainbow_common::config::config::{get_consumer_wallet_data, get_consumer_wallet_portal_url};
use tracing::info;

pub struct SessionManager {
    pub token: Option<String>,
    pub wallet_id: Option<String>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager { token: None, wallet_id: None }
    }

    pub async fn register() -> anyhow::Result<()> {
        let wallet_portal_url = get_consumer_wallet_portal_url()? + "/wallet-api/auth/register";
        let wallet_data = get_consumer_wallet_data()?;

        let res = SSI_AUTH_HTTP_CLIENT.post(wallet_portal_url).json(&wallet_data).send().await;
        let res = match res {
            // Este match es necesario si esta el de despues??
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            201 => {
                info!("Wallet registration successful");
            }
            400 => {
                error!("Wallet registration failed");
                bail!("Wallet registration failed");
            }
            409 => {
                info!("This wallet has already registered");
            }
            _ => {
                error!("Unexpected status: {}", res.status());
                bail!("Unexpected status: {}", res.status())
            }
        }

        Ok(())
    }

    pub async fn login(&mut self) -> anyhow::Result<()> {
        let wallet_portal_url = get_consumer_wallet_portal_url()? + "/wallet-api/auth/login";
        let mut wallet_data = get_consumer_wallet_data()?;
        wallet_data.as_object_mut().map(|obj| obj.remove("name"));

        let res = SSI_AUTH_HTTP_CLIENT.post(wallet_portal_url).json(&wallet_data).send().await;
        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("Wallet login successful");
                let json_res = res.json().await?; // TEST 
                self.wallet_id = Some(json_res.wallet_id); // TEST
                self.token = Some(json_res.token); //TEST
            }
            _ => {
                error!("Wallet login failed: {}", res.status());
                bail!("Wallet login failed: {}", res.status())
            }
        }

        Ok(())
    }

    pub async fn logout(&mut self) -> anyhow::Result<()> {
        let wallet_portal_url = get_consumer_wallet_portal_url()? + "/wallet-api/auth/logout";

        let res = SSI_AUTH_HTTP_CLIENT.post(wallet_portal_url).send().await;
        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("Wallet login successful");
                self.token = None; //TEST
            }
            _ => {
                error!("Wallet login failed: {}", res.status());
                bail!("Wallet login failed: {}", res.status())
            }
        }

        Ok(())
    }
}
