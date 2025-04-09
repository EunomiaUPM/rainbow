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

use crate::ssi_auth::consumer::types::{
    Didsinfo, Jwtclaims, MatchingVCs, VPexchange, WalletInfo, WalletInfoResponse,
    WalletLoginResponse,
};
use crate::ssi_auth::consumer::SSI_AUTH_HTTP_CLIENT;
use anyhow::bail;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use log::error;
use once_cell::sync::Lazy;
use rainbow_common::config::config::{get_consumer_wallet_data, get_consumer_wallet_portal_url};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
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
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = SSI_AUTH_HTTP_CLIENT
            .post(wallet_portal_url)
            .headers(headers)
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

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = SSI_AUTH_HTTP_CLIENT
            .post(wallet_portal_url)
            .headers(headers)
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

                let jwtparts: Vec<&str> = json_res.token.split('.').collect();

                if jwtparts.len() != 3 {
                    bail!("JWT token does not have the correct format");
                }

                let decoded = URL_SAFE_NO_PAD.decode(jwtparts[1]).unwrap();

                let claims: Jwtclaims = serde_json::from_slice(&decoded).unwrap();

                self.token_exp = Some(claims.exp);

                Ok(())
            }
            _ => {
                error!("WaltId account login failed: {}", res.status());
                bail!("WaltId account login failed: {}", res.status())
            }
        }
    }

    pub async fn logout(&mut self) -> anyhow::Result<()> {
        let wallet_portal_url = get_consumer_wallet_portal_url()? + "/wallet-api/auth/logout";
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = SSI_AUTH_HTTP_CLIENT.post(wallet_portal_url).headers(headers).send().await;
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
        let wallet_portal_url =
            get_consumer_wallet_portal_url()? + "/wallet-api/wallet/accounts/wallets";

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &self.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for authentication"),
        };

        let res = SSI_AUTH_HTTP_CLIENT.get(wallet_portal_url).headers(headers).send().await;
        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                let wallets = res.json::<WalletInfoResponse>().await.unwrap().wallets;
                for wallet in wallets {
                    if self.wallets.contains(&wallet) {
                        info!("Wallet {} already exists", wallet.id);
                    } else {
                        self.wallets.push(wallet);
                    }
                }

                info!("Wallet data loaded successfully");
            }
            _ => {
                error!("Wallet data loading failed: {}", res.status());
                bail!("Wallet data loaading failed: {}", res.status())
            }
        }

        Ok(())
    }

    async fn get_wallet_dids(&mut self) -> anyhow::Result<()> {
        if self.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };

        let wallet_portal_url = get_consumer_wallet_portal_url()?
            + "/wallet-api/wallet/"
            + &self.wallets.first().unwrap().id
            + "/dids";

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &self.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for authentication"),
        };

        let res = SSI_AUTH_HTTP_CLIENT.get(wallet_portal_url).headers(headers).send().await;
        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                let dids: Vec<Didsinfo> = res.json().await?;

                for did in dids {
                    if let Some(wallet) = self.wallets.first_mut() {
                        if let Some(dids) = &mut wallet.dids {
                            if dids.contains(&did) {
                                info!("Did {} already exists", did.did);
                            } else {
                                dids.push(did);
                            }
                        } else {
                            wallet.dids = Some(vec![did]);
                        }
                    }
                }

                info!("Dids data loaded successfully");
            }
            _ => {
                error!("Dids data loading failed: {}", res.status());
                bail!("Dids data loaading failed: {}", res.status())
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

    pub async fn access(&mut self) -> anyhow::Result<()> {
        if self.account_id.is_none() {
            self.register().await.unwrap();
        }

        self.login().await.unwrap();
        self.get_wallet_info().await.unwrap();
        self.get_wallet_dids().await.unwrap();

        Ok(())
    }

    pub async fn joinexchange(&self, exchange_url: String) -> anyhow::Result<String> {
        let kk = self.wallets.first();

        if self.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };

        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/resolvePresentationRequest",
            get_consumer_wallet_portal_url()?,
            self.wallets.first().unwrap().id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/plain".parse()?);
        headers.insert(ACCEPT, "text/plain".parse()?);

        match &self.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for authentication"),
        };

        let res = SSI_AUTH_HTTP_CLIENT.post(url).headers(headers).body(exchange_url).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("Joined the exchange successful");
                Ok(res.text().await?)
            }
            _ => {
                error!("Error joining the exchange: {}", res.status());
                bail!("Error joining the exchange: {}", res.status())
            }
        }
    }

    pub async fn match_vc4vp(&self, vpdef: Value) -> anyhow::Result<Vec<MatchingVCs>> {
        if self.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };
        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/matchCredentialsForPresentationDefinition",
            get_consumer_wallet_portal_url()?,
            self.wallets.first().unwrap().id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &self.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for authentication"),
        };

        let res = SSI_AUTH_HTTP_CLIENT.post(url).headers(headers).json(&vpdef).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("Credentials matched successfully");
                println!("{:?}", res);
                let vc_json: Vec<MatchingVCs> = res.json().await?;

                Ok(vc_json)
            }
            _ => {
                error!("Error matching credentials: {}", res.status());
                bail!("Error matching credentials: {}", res.status())
            }
        }
    }

    pub async fn present_vp(&self, preq: String, creds: Vec<String>) -> anyhow::Result<()> {
        if self.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };
        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/usePresentationRequest",
            get_consumer_wallet_portal_url()?,
            self.wallets.first().unwrap().id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &self.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for authentication"),
        };

        let did = &self.wallets.first().unwrap().dids.as_ref().unwrap().first().unwrap().did;
        let did = self.wallets.first().unwrap().dids.as_ref().unwrap().first().unwrap().did.clone();

        let body =
            json!({ "did": null, "presentationRequest": preq, "selectedCredentials": creds });


        println!();
        println!("{}", serde_json::to_string_pretty(&body)?);
        println!();

        let res = SSI_AUTH_HTTP_CLIENT.post(url).headers(headers).json(&body).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        println!("{:?}", res);
        let status = res.status();
        let body_text = res.text().await?;

        println!("Status: {}", status);
        println!("Body: {}", body_text);
        Ok(())
    }
}

pub static SESSION_MANAGER: Lazy<Arc<Mutex<WalletSessionManager>>> =
    Lazy::new(|| Arc::new(Mutex::new(WalletSessionManager::new())));
