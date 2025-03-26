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

use crate::ssi_auth::consumer::types::{Jwtclaims, WalletInfo, WalletLoginResponse};
use crate::ssi_auth::consumer::SSI_AUTH_HTTP_CLIENT;
use anyhow::bail;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use log::error;
use once_cell::sync::Lazy;
use rainbow_common::config::config::{get_consumer_wallet_data, get_consumer_wallet_portal_url};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;
use tracing::info;

pub struct WalletSessionManager {
    pub token: Option<String>,
    pub token_exp: Option<u64>,
    pub account_id: Option<String>,
    pub wallets: Vec<WalletInfo>,
}

impl WalletSessionManager {
    pub fn new() -> Self {
        WalletSessionManager { token: None, account_id: None, wallets: Vec::new(), token_exp: None }
    }

    async fn register(&self) -> anyhow::Result<()> {
        let wallet_portal_url = get_consumer_wallet_portal_url()? + "/wallet-api/auth/register";
        let wallet_data = get_consumer_wallet_data()?;
        let headers = Headers4WalletPetitions::build();

        let res = SSI_AUTH_HTTP_CLIENT
            .post(wallet_portal_url)
            .headers(headers.headers)
            .json(&wallet_data)
            .send()
            .await;
        let res = match res {
            // Este match es necesario si esta el de despues??
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            201 => {
                info!("WaltId account registration successful");
            }
            409 => {
                info!("This WaltId account has already registered");
            }
            _ => {
                error!("WaltId account registration failed: {}", res.status());
                bail!("WaltId account registration failed: {}", res.status());
            }
        }

        Ok(())
    }

    async fn login(&mut self) -> anyhow::Result<()> {
        let wallet_portal_url = get_consumer_wallet_portal_url()? + "/wallet-api/auth/login";
        let mut wallet_data = get_consumer_wallet_data()?;
        wallet_data.as_object_mut().map(|obj| obj.remove("name"));
        let headers = Headers4WalletPetitions::build();

        let res = SSI_AUTH_HTTP_CLIENT
            .post(wallet_portal_url)
            .headers(headers.headers)
            .json(&wallet_data)
            .send()
            .await;
        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("WaltId account login successful");
                let json_res: WalletLoginResponse = res.json().await?;
                self.account_id = Some(json_res.id);
                self.token = Some(json_res.token.clone());

                let jwtparts: Vec<&str> = json_res.token.as_str().split('.').collect();
                if jwtparts.len() != 3 {
                    bail!("JWT token does not have the correct format");
                }

                let claims = serde_json::from_slice::<Jwtclaims>(&STANDARD.decode(jwtparts[1])?)?;
                self.token_exp = Some(claims.exp);
            }
            _ => {
                error!("WaltId account login failed: {}", res.status());
                bail!("WaltId account login failed: {}", res.status())
            }
        }

        Ok(())
    }

    pub async fn logout(&mut self) -> anyhow::Result<()> {
        let wallet_portal_url = get_consumer_wallet_portal_url()? + "/wallet-api/auth/logout";
        let headers = Headers4WalletPetitions::build();

        let res =
            SSI_AUTH_HTTP_CLIENT.post(wallet_portal_url).headers(headers.headers).send().await;
        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("WaltId account logout successful");
                self.token = None;
            }
            _ => {
                error!("WaltId account logout failed: {}", res.status());
                bail!("WaltId account logout failed: {}", res.status())
            }
        }

        Ok(())
    }

    async fn get_wallet_info(&mut self) -> anyhow::Result<()> {
        let wallet_portal_url = get_consumer_wallet_portal_url()? + "/wallet-api/accounts/wallets";
        let mut headers = Headers4WalletPetitions::build();

        if let Some(token) = &self.token {
            headers.addheader(AUTHORIZATION, &format!("Bearer {}", token));
        } else {
            bail!("No token available for authentication");
        }

        let res = SSI_AUTH_HTTP_CLIENT.get(wallet_portal_url).headers(headers.headers).send().await;
        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("Wallet data loaded successfully");
            }
            _ => {
                error!("Wallet data loading failed: {}", res.status());
                bail!("Wallet data loading failed: {}", res.status())
            }
        }

        Ok(())
    }

    pub async fn token_expired(&mut self) -> anyhow::Result<()> {
        match self.token_exp {
            Some(expiration_time) => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs();

                if now >= expiration_time {
                    info!("Token expired");
                    self.update_token().await
                }
                Ok(())
            }
            None => {
                bail!("No token available for authentication")
            }
        }
    }

    async fn update_token(&mut self) {
        match self.login().await {
            Ok(_) => {
                info!("Token updated successfully");
            }
            Err(e) => {
                error!("Token update failed: {}", e);
            }
        }
    }

    pub async fn access(&mut self) -> () {
        if self.account_id.is_none() {
            self.register().await.unwrap();
        }
        self.login().await.unwrap();
        self.get_wallet_info().await.unwrap();
    }

    pub async fn joinexchange(&self, exchange_url: &str) -> anyhow::Result<()> {
        if !self.wallets.first() {
            bail!("There is not a wallet registered")
        };
        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/resolvePresentationRequest",
            get_consumer_wallet_portal_url()?,
            self.wallets.first().unwrap().id
        );

        let mut headers = Headers4WalletPetitions::build();
        if let Some(token) = &self.token {
            headers.addheader(AUTHORIZATION, &format!("Bearer {}", token));
        } else {
            bail!("No token available for authentication");
        }

        let res =
            SSI_AUTH_HTTP_CLIENT.post(url).headers(headers.headers).json(exchange_url).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("Joined the exchange successful");
            }
            _ => {
                error!("Error joining the exchange: {}", res.status());
                bail!("Error joining the exchange: {}", res.status())
            }
        }

        Ok(())
    }

    pub async fn match_vc4vp(&self, vpdef: Value) -> anyhow::Result<()> {
        if !self.wallets.first() {
            bail!("There is not a wallet registered")
        };
        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/matchCredentialsForPresentationDefinition",
            get_consumer_wallet_portal_url()?,
            self.wallets.first().unwrap().id
        );

        let mut headers = Headers4WalletPetitions::build();
        if let Some(token) = &self.token {
            headers.addheader(AUTHORIZATION, &format!("Bearer {}", token));
        } else {
            bail!("No token available for authentication");
        }

        let res =
            SSI_AUTH_HTTP_CLIENT.post(url).headers(headers.headers).json(&vpdef).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("Credentials matched successfully");
            }
            _ => {
                error!("Error matching credentials: {}", res.status());
                bail!("Error matching credentials: {}", res.status())
            }
        }

        Ok(())
    }

    pub async fn present_vp(&self, vpdef: Value) -> anyhow::Result<()> {
        
        Ok(())
    }
}

pub static SESSION_MANAGER: Lazy<Arc<Mutex<WalletSessionManager>>> =
    Lazy::new(|| Arc::new(Mutex::new(WalletSessionManager::new())));

struct Headers4WalletPetitions {
    headers: HeaderMap,
}

impl Headers4WalletPetitions {
    pub fn build() -> Self {
        let mut builder = Headers4WalletPetitions { headers: HeaderMap::new() };

        builder.addheader(CONTENT_TYPE, "application/json");
        builder.addheader(ACCEPT, "application/json");
        builder
    }

    pub fn addheader(&mut self, key: HeaderName, value: &str) -> () {
        self.headers.insert(key, HeaderValue::from_str(value).unwrap());
    }
}
