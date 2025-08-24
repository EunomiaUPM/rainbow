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

use super::Manager;
use crate::ssi_auth::provider::core::provider_trait::RainbowSSIAuthProviderManagerTrait;
use crate::ssi_auth::types::{AuthJwtClaims, WalletInfoResponse, WalletLoginResponse};
use anyhow::bail;
use axum::async_trait;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderMap;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use rainbow_common::errors::{CommonErrors, ErrorInfo};
use rainbow_common::ssi_wallet::{DidsInfo, RainbowSSIAuthWalletTrait};
use rainbow_db::auth_provider::entities::{business_mates, mates};
use rainbow_db::auth_provider::repo_factory::factory_trait::AuthRepoFactoryTrait;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info, warn};

#[async_trait]
impl<T> RainbowSSIAuthWalletTrait for Manager<T>
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
                let error = CommonErrors::PetitionError {
                    info: ErrorInfo {
                        message: "Error contacting the wallet for registration".to_string(),
                        error_code: 1000,
                        details: None,
                    },
                    http_code,
                    url,
                    method: "POST".to_string(),
                    cause: e.to_string(),
                };
                error.log();
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
                let error = CommonErrors::WalletError {
                    info: ErrorInfo {
                        message: "Wallet account registration failed".to_string(),
                        error_code: 1100,
                        details: None,
                    },
                    http_code: Some(res.status().as_u16()),
                    cause: None,
                };
                error.log();
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
                let error = CommonErrors::PetitionError {
                    info: ErrorInfo {
                        message: "Error contacting the wallet for login".to_string(),
                        error_code: 1000,
                        details: None,
                    },
                    http_code,
                    url,
                    method: "POST".to_string(),
                    cause: e.to_string(),
                };
                error.log();
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
                    let error = CommonErrors::FormatError {
                        info: ErrorInfo {
                            message: "The jwt does not have the correct format".to_string(),
                            error_code: 1200,
                            details: None,
                        },
                        cause: None,
                    };
                    error.log();
                    bail!(error);
                }

                let decoded = URL_SAFE_NO_PAD.decode(jwt_parts[1])?;

                let claims: AuthJwtClaims = serde_json::from_slice(&decoded)?;

                wallet_session.token_exp = Some(claims.exp);

                Ok(())
            }
            _ => {
                let error = CommonErrors::WalletError {
                    info: ErrorInfo { message: "Wallet login failed".to_string(), error_code: 1100, details: None },
                    http_code: Some(res.status().as_u16()),
                    cause: None,
                };
                error.log();
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
                let error = CommonErrors::PetitionError {
                    info: ErrorInfo {
                        message: "Error contacting the wallet for logout".to_string(),
                        error_code: 1000,
                        details: None,
                    },
                    http_code,
                    url,
                    method: "POST".to_string(),
                    cause: e.to_string(),
                };
                error.log();
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
                let error = CommonErrors::WalletError {
                    info: ErrorInfo { message: "Wallet logout failed".to_string(), error_code: 1100, details: None },
                    http_code: Some(res.status().as_u16()),
                    cause: None,
                };
                error.log();
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
                let error = CommonErrors::WalletError {
                    info: ErrorInfo {
                        message: "No token available for use into the wallet".to_string(),
                        error_code: 1100,
                        details: None,
                    },
                    http_code: None,
                    cause: None,
                };
                error.log();
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
                let error = CommonErrors::PetitionError {
                    info: ErrorInfo {
                        message: "Error contacting the wallet for retrieving information".to_string(),
                        error_code: 1000,
                        details: None,
                    },
                    http_code,
                    url,
                    method: "GET".to_string(),
                    cause: e.to_string(),
                };
                error.log();
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
                let error = CommonErrors::WalletError {
                    info: ErrorInfo {
                        message: "Wallet information acquisition failed".to_string(),
                        error_code: 1100,
                        details: None,
                    },
                    http_code: Some(res.status().as_u16()),
                    cause: None,
                };
                error.log();
                bail!(error);
            }
        }

        Ok(())
    }

    async fn get_wallet_dids(&self) -> anyhow::Result<()> {
        info!("Retrieving dids from Wallet");
        let mut wallet_session = self.wallet_session.lock().await;

        if wallet_session.wallets.first().is_none() {
            let error = CommonErrors::WalletError {
                info: ErrorInfo {
                    message: "There is no wallet to retrieve dids from".to_string(),
                    error_code: 1100,
                    details: None,
                },
                http_code: None,
                cause: None,
            };
            error.log();
            bail!(error);
        };

        let url = format!(
            "{}/wallet-api/wallet/{}/dids",
            self.config.get_wallet_portal_url(),
            &wallet_session.wallets.first().unwrap().id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &wallet_session.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => {
                let error = CommonErrors::WalletError {
                    info: ErrorInfo {
                        message: "No token available for use into the wallet".to_string(),
                        error_code: 1100,
                        details: None,
                    },
                    http_code: None,
                    cause: None,
                };
                error.log();
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
                let error = CommonErrors::PetitionError {
                    info: ErrorInfo {
                        message: "Error contacting the wallet for retrieving dids".to_string(),
                        error_code: 1000,
                        details: None,
                    },
                    http_code,
                    url,
                    method: "GET".to_string(),
                    cause: e.to_string(),
                };
                error.log();
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
                let error = CommonErrors::WalletError {
                    info: ErrorInfo {
                        message: "Wallet dids acquisition failed".to_string(),
                        error_code: 1100,
                        details: None,
                    },
                    http_code: Some(res.status().as_u16()),
                    cause: None,
                };
                error.log();
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

        let mut wallet_session = self.wallet_session.lock().await;

        let did = wallet_session.wallets.first().unwrap().dids.clone().unwrap().first().unwrap().did.clone();

        let model = mates::NewModel {
            participant_id: did.clone(),
            participant_slug: "Myself".to_string(),
            participant_type: "Provider".to_string(),
            base_url: self.config.get_auth_host_url(),
            token: None,
            is_me: true,
        };
        self.save_mate(model).await?;

        Ok(())
    }

    async fn token_expired(&self) -> anyhow::Result<bool> {
        info!("Checking if token is expired");
        let mut wallet_session = self.wallet_session.lock().await;

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
                let error = CommonErrors::WalletError {
                    info: ErrorInfo {
                        message: "No token available for use into the wallet".to_string(),
                        error_code: 1100,
                        details: None,
                    },
                    http_code: None,
                    cause: None,
                };
                error.log();
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

        match wallet_session.wallets.first() {
            Some(wallet) => {
                let dids = wallet.clone().dids.unwrap();
                let did = dids.first().unwrap();
                let did_doc = did.clone().document;
                let json: Value = serde_json::from_str(&did_doc)?;
                Ok(json)
            }
            None => {
                let error = CommonErrors::WalletError {
                    info: ErrorInfo {
                        message: "There is no wallet to retrieve dids from".to_string(),
                        error_code: 1100,
                        details: None,
                    },
                    http_code: None,
                    cause: None,
                };
                error.log();
                bail!(error);
            }
        }
    }
}
