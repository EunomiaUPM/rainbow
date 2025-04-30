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

use crate::setup::consumer::AuthConsumerApplicationConfig;
use crate::ssi_auth::consumer::core::types::{
    AuthJwtclaims, Didsinfo, MatchingVCs, WalletInfo, WalletInfoResponse, WalletLoginResponse,
};
use anyhow::bail;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use once_cell::sync::Lazy;
use rainbow_common::auth::{GrantRequest, GrantRequestResponse};
use rainbow_db::auth_consumer::entities::auth_verification::Model;
use rainbow_db::auth_consumer::repo::AuthConsumerRepoTrait;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use sea_orm_migration::cli::Cli;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Serializer, Value};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tracing::{error, info};
use url::Url;
use urlencoding::decode;

#[derive(Debug)]
pub struct Manager<T>
where
    T: AuthConsumerRepoTrait + Send + Sync + Clone + 'static,
{
    pub wallet_session: WalletSession,
    pub wallet_onboard: bool,
    pub auth_repo: Arc<T>,
    client: Client,
    config: AuthConsumerApplicationConfig,
}

#[derive(Serialize, Deserialize, Debug)]
struct WalletSession {
    pub account_id: Option<String>,
    pub token: Option<String>,
    pub token_exp: Option<u64>,
    pub wallets: Vec<WalletInfo>,
}

impl<T> Manager<T>
where
    T: AuthConsumerRepoTrait + Send + Sync + Clone + 'static,
{
    pub fn new(auth_repo: Arc<T>, config: AuthConsumerApplicationConfig) -> Self {
        info!("Manager created");
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build reqwest client");
        Manager {
            wallet_session: WalletSession { account_id: None, token: None, token_exp: None, wallets: Vec::new() },
            wallet_onboard: false,
            auth_repo,
            client,
            config,
        }
    }

    pub async fn register_wallet(&self) -> anyhow::Result<()> {
        let wallet_portal_url = self.config.get_consumer_wallet_portal_url() + "/wallet-api/auth/register";
        let wallet_data = self.config.get_consumer_wallet_data();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(wallet_portal_url).headers(headers).json(&wallet_data).send().await;

        let res = match res {
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

    pub async fn login_wallet(&mut self) -> anyhow::Result<()> {
        let wallet_portal_url = self.config.get_consumer_wallet_portal_url() + "/wallet-api/auth/login";
        let mut wallet_data = self.config.get_consumer_wallet_data();
        wallet_data.as_object_mut().map(|obj| obj.remove("name"));

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(wallet_portal_url).headers(headers).json(&wallet_data).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("WaltId account login successful");

                let json_res: WalletLoginResponse = res.json().await?;

                self.wallet_session.account_id = Some(json_res.id);
                self.wallet_session.token = Some(json_res.token.clone());

                let jwtparts: Vec<&str> = json_res.token.split('.').collect();

                if jwtparts.len() != 3 {
                    bail!("JWT token does not have the correct format");
                }

                let decoded = URL_SAFE_NO_PAD.decode(jwtparts[1])?;

                let claims: AuthJwtclaims = serde_json::from_slice(&decoded)?;

                self.wallet_session.token_exp = Some(claims.exp);

                Ok(())
            }
            _ => {
                error!("WaltId account login failed: {}", res.status());
                bail!("WaltId account login failed: {}", res.status())
            }
        }
    }

    pub async fn logout_wallet(&mut self) -> anyhow::Result<()> {
        let wallet_portal_url = self.config.get_consumer_wallet_portal_url() + "/wallet-api/auth/logout";

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(wallet_portal_url).headers(headers).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("WaltId account logout successful");
                self.wallet_session.token = None;
            }
            _ => {
                error!("WaltId account logout failed: {}", res.status());
                bail!("WaltId account logout failed: {}", res.status())
            }
        }

        Ok(())
    }

    async fn get_wallet_info(&mut self) -> anyhow::Result<()> {
        let wallet_portal_url = self.config.get_consumer_wallet_portal_url() + "/wallet-api/wallet/accounts/wallets";

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &self.wallet_session.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for wallet authentication"),
        };

        let res = self.client.get(wallet_portal_url).headers(headers).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                let wallets = res.json::<WalletInfoResponse>().await?.wallets;
                for wallet in wallets {
                    if self.wallet_session.wallets.contains(&wallet) {
                        info!("Wallet {} already exists", wallet.id);
                    } else {
                        self.wallet_session.wallets.push(wallet);
                    }
                }

                info!("Wallet data loaded successfully");
            }
            _ => {
                error!("Wallet data loading failed: {}", res.status());
                bail!("Wallet data loading failed: {}", res.status())
            }
        }

        Ok(())
    }

    async fn get_wallet_dids(&mut self) -> anyhow::Result<()> {
        if self.wallet_session.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };

        let wallet_portal_url = self.config.get_consumer_wallet_portal_url()
            + "/wallet-api/wallet/"
            + &self.wallet_session.wallets.first().unwrap().id
            + "/dids";

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &self.wallet_session.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for authentication"),
        };

        let res = self.client.get(wallet_portal_url).headers(headers).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                let dids: Vec<Didsinfo> = res.json().await?;

                for did in dids {
                    if let Some(wallet) = self.wallet_session.wallets.first_mut() {
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

                info!("Wallet Dids data loaded successfully");
            }
            _ => {
                error!("Wallet Dids data loading failed: {}", res.status());
                bail!("Wallet Dids data loading failed: {}", res.status())
            }
        }

        Ok(())
    }

    pub async fn onboard(&mut self) -> anyhow::Result<()> {
        if !self.wallet_onboard {
            self.register_wallet().await?;
        }

        self.login_wallet().await?;
        self.get_wallet_info().await?;
        self.get_wallet_dids().await?;

        Ok(())
    }

    pub fn token_expired(&self) -> anyhow::Result<bool> {
        match self.wallet_session.token_exp {
            Some(expiration_time) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

                if now >= expiration_time {
                    info!("Token expired");
                    return Ok(true);
                };
                Ok(false)
            }
            None => {
                bail!("No token available for authentication")
            }
        }
    }

    async fn update_token(&mut self) -> anyhow::Result<()> {
        match self.login_wallet().await {
            Ok(()) => {
                info!("Token updated successfully");
                Ok(())
            }
            Err(e) => {
                error!("Token update failed: {}", e);
                bail!("Error updating token: {}", e);
            }
        }
    }

    pub async fn ok(&mut self) -> anyhow::Result<()> {
        if self.token_expired()? {
            self.update_token().await?;
        }
        Ok(())
    }

    pub async fn request_access(&self, url: String, provider: String, actions: Vec<String>) -> anyhow::Result<Model> {
        let body = GrantRequest::default4oidc();

        let model = match self.auth_repo.create_auth(provider, actions, body.interact.clone()).await {
            Ok(model) => {
                info!("exchange saved successfully");
                model
            }
            Err(e) => bail!("Unable to save exchange in db: {}", e),
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        info!("Sending Grant Petition to Provider");

        let res = self.client.post(url).headers(headers).json(&body).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        let mut res: GrantRequestResponse = match res.status().as_u16() {
            200 => {
                info!("Grant Response received successfully");
                res.json().await?
            }
            _ => {
                error!("Grant Response failed: {}", res.status());
                bail!("Grant Response failed: {}", res.status())
            }
        };

        match self.auth_repo.auth_accepted(model.id.clone(), res.instance_id.unwrap()).await {
            Ok(model) => {
                info!("Assigned id updated successfully");
            }
            Err(e) => bail!("Unable to update assigned id in db: {}", e),
        };

        let model = match self
            .auth_repo
            .create_auth_verification(model.id.clone(), res.interact.unwrap().oidc4vp.unwrap())
            .await
        {
            Ok(model) => {
                info!("Verification data stored successfully");
                model
            }
            Err(e) => bail!("Unable to save verification in db: {}", e),
        };

        Ok(model)
    }

    pub async fn join_exchange(&self, exchange_url: String) -> anyhow::Result<String> {
        if self.wallet_session.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };

        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/resolvePresentationRequest",
            self.config.get_consumer_wallet_portal_url(),
            self.wallet_session.wallets.first().unwrap().id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/plain".parse()?);
        headers.insert(ACCEPT, "text/plain".parse()?);

        match &self.wallet_session.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for authentication"),
        };

        let res = self.client.post(url).headers(headers).body(exchange_url).send().await;

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

    pub async fn parse_vpd(&self, vpd_as_string: String) -> anyhow::Result<Value> {
        let url = Url::parse(decode(&vpd_as_string).unwrap().as_ref())?;

        if let Some((_, vpd_json)) = url.query_pairs().find(|(key, _)| key == "presentation_definition") {
            match serde_json::from_str::<Value>(&vpd_json) {
                Ok(json) => Ok(json),
                Err(err) => {
                    error!("Error parsing the credential");
                    bail!("Error parsing the credential")
                }
            }
        } else {
            error!("Invalid Presentation Definition");
            bail!("Invalid Presentation Definition")
        }
    }

    pub async fn match_vc4vp(&self, vpdef: Value) -> anyhow::Result<Vec<MatchingVCs>> {
        if self.wallet_session.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };
        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/matchCredentialsForPresentationDefinition",
            self.config.get_consumer_wallet_portal_url(),
            self.wallet_session.wallets.first().unwrap().id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &self.wallet_session.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for authentication"),
        };

        let res = self.client.post(url).headers(headers).json(&vpdef).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("Credentials matched successfully");
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
        if self.wallet_session.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };
        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/usePresentationRequest",
            self.config.get_consumer_wallet_portal_url(),
            self.wallet_session.wallets.first().unwrap().id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &self.wallet_session.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for authentication"),
        };

        let did = &self.wallet_session.wallets.first().unwrap().dids.as_ref().unwrap().first().unwrap().did;
        let did = self.wallet_session.wallets.first().unwrap().dids.as_ref().unwrap().first().unwrap().did.clone();

        let body = json!({ "did": null, "presentationRequest": preq, "selectedCredentials": creds });

        let res = self.client.post(url).headers(headers).json(&body).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        let status = res.status();
        let body_text = res.text().await?;

        println!("Status: {}", status);
        println!("Body: {}", body_text);
        Ok(())
    }

    pub async fn continue_request(&self, id: String, nonce: String) -> anyhow::Result<()> {
        let model = match self.auth_repo.get_interaction_by_id(id).await {
            Ok(interaction) => { interaction }
            Err(e) => bail!("Error retrieving interaction: {}", e),
        };

        if model.nonce != nonce {
            bail!("Invalid nonce");
        }

        Ok(())
    }
}

// pub static MANAGER: Lazy<Arc<Mutex<Manager>>> = Lazy::new(|| Arc::new(Mutex::new(Manager::new())));
