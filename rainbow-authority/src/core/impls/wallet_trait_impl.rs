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

use crate::core::traits::{AuthorityTrait, RainbowSSIAuthWalletTrait};
use crate::core::Authority;
use crate::data::entities::minions;
use crate::data::repo_factory::factory_trait::AuthRepoFactoryTrait;
use crate::errors::helpers::{BadFormat, MissingAction};
use crate::errors::{ErrorLog, Errors};
use crate::setup::config::AuthorityFunctions;
use crate::setup::AuthorityApplicationConfigTrait;
use crate::types::jwt::AuthJwtClaims;
use crate::types::wallet::{DidsInfo, WalletInfoResponse, WalletLoginResponse};
use anyhow::bail;
use axum::async_trait;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderMap;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info, warn};

#[async_trait]
impl<T> RainbowSSIAuthWalletTrait for Authority<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    async fn register_wallet(&self) -> anyhow::Result<()> {
        info!("Registering wallet");
        let url = format!(
            "{}/wallet-api/auth/register",
            self.config.get_wallet_portal_url()
        );
        let wallet_data = self.config.get_wallet_data();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(&url).headers(headers).json(&wallet_data).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => {
                let http_code = match e.status() {
                    Some(status) => Some(status.as_u16()),
                    None => None,
                };
                let error = Errors::petition_new(url, "POST".to_string(), http_code, e.to_string());
                error!("{}", error.log());
                bail!(error);
            }
        };

        match res.status().as_u16() {
            201 => {
                info!("Wallet account registration successful");
            }
            409 => {
                warn!("Wallet account has already registered");
            }
            _ => {
                let error = Errors::wallet_new(
                    url,
                    "POST".to_string(),
                    res.status().as_u16(),
                    Some("Petition to register Wallet failed".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
        Ok(())
    }

    async fn login_wallet(&self) -> anyhow::Result<()> {
        info!("Login into wallet");
        let url = format!(
            "{}/wallet-api/auth/login",
            self.config.get_wallet_portal_url()
        );

        let mut wallet_data = self.config.get_wallet_data();
        wallet_data.as_object_mut().map(|obj| obj.remove("name"));

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(&url).headers(headers).json(&wallet_data).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => {
                let http_code = match e.status() {
                    Some(status) => Some(status.as_u16()),
                    None => None,
                };
                let error = Errors::petition_new(url, "POST".to_string(), http_code, e.to_string());
                error!("{}", error.log());
                bail!(error);
            }
        };

        match res.status().as_u16() {
            200 => {
                info!("Wallet login successful");

                let json_res: WalletLoginResponse = res.json().await?;

                let mut wallet_session = self.wallet_session.lock().await;
                wallet_session.account_id = Some(json_res.id);
                wallet_session.token = Some(json_res.token.clone());

                let jwt_parts: Vec<&str> = json_res.token.split('.').collect();

                if jwt_parts.len() != 3 {
                    let error = Errors::format_new(
                        BadFormat::Sent,
                        Some("The jwt does not have the correct format".to_string()),
                    );
                    error!("{}", error.log());
                    bail!(error);
                }

                let decoded = URL_SAFE_NO_PAD.decode(jwt_parts[1])?;

                let claims: AuthJwtClaims = serde_json::from_slice(&decoded)?;

                wallet_session.token_exp = Some(claims.exp);

                Ok(())
            }
            _ => {
                let error = Errors::wallet_new(
                    url,
                    "POST".to_string(),
                    res.status().as_u16(),
                    Some("Petition to login into Wallet failed".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn logout_wallet(&self) -> anyhow::Result<()> {
        info!("Login out of wallet");
        let url = format!(
            "{}/wallet-api/auth/logout",
            self.config.get_wallet_portal_url()
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(&url).headers(headers).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => {
                let http_code = match e.status() {
                    Some(status) => Some(status.as_u16()),
                    None => None,
                };
                let error = Errors::petition_new(url, "POST".to_string(), http_code, e.to_string());
                error!("{}", error.log());
                bail!(error);
            }
        };

        match res.status().as_u16() {
            200 => {
                info!("Wallet logout successful");
                let mut wallet_session = self.wallet_session.lock().await;
                wallet_session.token = None;
            }
            _ => {
                let error = Errors::wallet_new(
                    url,
                    "POST".to_string(),
                    res.status().as_u16(),
                    Some("Petition to logout from Wallet failed".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        }

        Ok(())
    }

    async fn get_wallet_info(&self) -> anyhow::Result<()> {
        info!("Retrieving wallet info");
        let url = format!(
            "{}/wallet-api/wallet/accounts/wallets",
            self.config.get_wallet_portal_url()
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        let mut wallet_session = self.wallet_session.lock().await;
        match &wallet_session.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => {
                let error = Errors::missing_action_new(
                    "Login is needed".to_string(),
                    MissingAction::Token,
                    Some("No token available for use into the wallet".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let res = self.client.get(&url).headers(headers).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => {
                let http_code = match e.status() {
                    Some(status) => Some(status.as_u16()),
                    None => None,
                };
                let error = Errors::petition_new(url, "GET".to_string(), http_code, e.to_string());
                error!("{}", error.log());
                bail!(error);
            }
        };

        match res.status().as_u16() {
            200 => {
                let wallets = res.json::<WalletInfoResponse>().await?.wallets;
                for wallet in wallets {
                    if wallet_session.wallets.contains(&wallet) {
                        warn!("Wallet {} already exists", wallet.id);
                    } else {
                        wallet_session.wallets.push(wallet);
                    }
                }
                info!("Wallet data loaded successfully");
            }
            _ => {
                let error = Errors::wallet_new(
                    url,
                    "GET".to_string(),
                    res.status().as_u16(),
                    Some("Petition to retrieve Wallet information failed".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        }

        Ok(())
    }

    async fn get_wallet_dids(&self) -> anyhow::Result<()> {
        info!("Retrieving dids from Wallet");
        let mut wallet_session = self.wallet_session.lock().await;

        let wallet = match wallet_session.wallets.first() {
            Some(w) => w,
            None => {
                let error = Errors::missing_action_new(
                    "There is no wallet associated to this session".to_string(),
                    MissingAction::Wallet,
                    Some("There is no wallet to retrieve dids from".to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        let url = format!(
            "{}/wallet-api/wallet/{}/dids",
            self.config.get_wallet_portal_url(),
            &wallet.id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &wallet_session.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => {
                let error = Errors::missing_action_new(
                    "There is no token associated to this session".to_string(),
                    MissingAction::Token,
                    Some("There is no token available for use".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let res = self.client.get(&url).headers(headers).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => {
                let http_code = match e.status() {
                    Some(status) => Some(status.as_u16()),
                    None => None,
                };
                let error = Errors::petition_new(url, "GET".to_string(), http_code, e.to_string());
                error!("{}", error.log());
                bail!(error);
            }
        };

        match res.status().as_u16() {
            200 => {
                let dids: Vec<DidsInfo> = res.json().await?;

                for did in dids {
                    if let Some(wallet) = wallet_session.wallets.first_mut() {
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
                let error = Errors::wallet_new(
                    url,
                    "GET".to_string(),
                    res.status().as_u16(),
                    Some("Petition to retrieve Wallet DIDs failed".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        }

        Ok(())
    }

    async fn onboard(&self) -> anyhow::Result<()> {
        info!("Onboarding into wallet");
        if !self.wallet_onboard {
            self.register_wallet().await?
        }
        self.login_wallet().await?;
        self.get_wallet_info().await?;
        self.get_wallet_dids().await?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let wallet_session = self.wallet_session.lock().await;

        let wallet = match wallet_session.wallets.first() {
            Some(w) => w,
            None => {
                let error = Errors::missing_action_new(
                    "There is no wallet associated to this session".to_string(),
                    MissingAction::Wallet,
                    Some("There is no wallet to retrieve dids from".to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        let did = match wallet.dids.as_ref().and_then(|d| d.first()) {
            Some(did_entry) => did_entry.did.clone(),
            None => {
                let error = Errors::missing_action_new(
                    "A DID is needed".to_string(),
                    MissingAction::Did,
                    Some("No DIDs found in wallet".to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        let model = minions::NewModel {
            participant_id: did.clone(),
            participant_slug: "Myself".to_string(),
            participant_type: "Provider".to_string(),
            base_url: Some(self.config.get_host()),
            vc_uri: None,
            is_vc_issued: false,
            is_me: true,
        };
        self.save_minion(model).await?;

        Ok(())
    }

    async fn token_expired(&self) -> anyhow::Result<bool> {
        info!("Checking if token is expired");
        let wallet_session = self.wallet_session.lock().await;

        match wallet_session.token_exp {
            Some(expiration_time) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

                if now >= expiration_time {
                    info!("Token expired");
                    return Ok(true);
                };
                Ok(false)
            }
            None => {
                let error = Errors::unauthorized_new(Some("There is no token".to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn update_token(&self) -> anyhow::Result<()> {
        info!("Updating token");
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

    async fn ok(&self) -> anyhow::Result<()> {
        if self.token_expired().await? {
            self.update_token().await?;
        }
        Ok(())
    }

    async fn didweb(&self) -> anyhow::Result<Value> {
        info!("Retrieving did");
        let wallet_session = self.wallet_session.lock().await;

        let wallet = match wallet_session.wallets.first() {
            Some(w) => w,
            None => {
                let error = Errors::missing_action_new(
                    "There is no wallet associated to this session".to_string(),
                    MissingAction::Wallet,
                    Some("There is no wallet to retrieve dids from".to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        let did = match wallet.dids.as_ref().and_then(|d| d.first()) {
            Some(did_entry) => did_entry.clone().document,
            None => {
                let error = Errors::missing_action_new(
                    "A DID is needed".to_string(),
                    MissingAction::Did,
                    Some("No DIDs found in wallet".to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        let json: Value = serde_json::from_str(did.as_str())?;
        Ok(json)
    }
}
