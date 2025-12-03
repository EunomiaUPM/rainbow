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
use super::super::WalletServiceTrait;
use super::config::{WaltIdConfig, WaltIdConfigTrait};
use crate::ssi::common::errors::AuthErrors;
use crate::ssi::common::services::client::ClientServiceTrait;
use crate::ssi::common::types::enums::request::Body;
use crate::ssi::common::types::jwt::AuthJwtClaims;
use crate::ssi::common::types::wallet::{
    CredentialOfferResponse, DidsInfo, KeyDefinition, MatchVCsRequest, MatchingVCs, OidcUri, RedirectResponse, Vpd,
    WalletInfo, WalletInfoResponse, WalletLoginResponse, WalletSession,
};
use crate::ssi::common::utils::get_query_param;
use anyhow::bail;
use axum::async_trait;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderMap;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rainbow_common::errors::helpers::{BadFormat, MissingAction};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use crate::ssi::common::data::entities::mates;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::log::warn;
use tracing::{debug, error, info};
use url::Url;
use urlencoding::decode;

pub struct WaltIdService {
    wallet_session: Arc<Mutex<WalletSession>>,
    key_data: Arc<Mutex<Vec<KeyDefinition>>>,
    client: Arc<dyn ClientServiceTrait>,
    config: WaltIdConfig,
}

impl WaltIdService {
    pub fn new(client: Arc<dyn ClientServiceTrait>, config: WaltIdConfig) -> WaltIdService {
        WaltIdService {
            wallet_session: Arc::new(Mutex::new(WalletSession {
                account_id: None,
                token: None,
                token_exp: None,
                wallets: vec![],
            })),
            key_data: Arc::new(Mutex::new(Vec::new())),
            config,
            client,
        }
    }
}

#[async_trait]
impl WalletServiceTrait for WaltIdService {
    // BASIC ----------------------------------------------------------------------------------------------->
    async fn register(&self) -> anyhow::Result<()> {
        info!("Registering in web wallet");
        let url = format!(
            "{}/wallet-api/auth/register",
            self.config.get_wallet_api_url()
        );
        let body = self.config.get_wallet_register_data();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(&url, Some(headers), Body::Json(body)).await?;

        match res.status().as_u16() {
            201 => {
                info!("Wallet account registration successful");
            }
            409 => {
                warn!("Wallet account has already registered");
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition to register Wallet failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
        Ok(())
    }

    async fn login(&self) -> anyhow::Result<()> {
        info!("Login into web wallet");
        let url = format!("{}/wallet-api/auth/login", self.config.get_wallet_api_url());

        let body = self.config.get_wallet_login_data();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(&url, Some(headers), Body::Json(body)).await?;

        match res.status().as_u16() {
            200 => {
                info!("Wallet login successful");

                let json_res: WalletLoginResponse = res.json().await?;

                let mut wallet_session = self.wallet_session.lock().await;
                wallet_session.account_id = Some(json_res.id);
                wallet_session.token = Some(json_res.token.clone());

                let jwt_parts: Vec<&str> = json_res.token.split('.').collect();
                if jwt_parts.len() != 3 {
                    let error = CommonErrors::format_new(BadFormat::Sent, "The jwt does not have the correct format");
                    error!("{}", error.log());
                    bail!(error);
                }

                let decoded = URL_SAFE_NO_PAD.decode(jwt_parts[1])?;
                let claims: AuthJwtClaims = serde_json::from_slice(&decoded)?;
                wallet_session.token_exp = Some(claims.exp);

                info!("Login data saved successfully");
                Ok(())
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition to login into Wallet failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn logout(&self) -> anyhow::Result<()> {
        info!("Login out of web wallet");
        let url = format!(
            "{}/wallet-api/auth/logout",
            self.config.get_wallet_api_url()
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(&url, Some(headers), Body::None).await?;

        match res.status().as_u16() {
            200 => {
                info!("Wallet logout successful");
                let mut wallet_session = self.wallet_session.lock().await;
                wallet_session.token = None;
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition to logout from Wallet failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
        Ok(())
    }

    async fn onboard(&self) -> anyhow::Result<mates::NewModel> {
        info!("Onboarding into web wallet");

        self.register().await?;
        self.login().await?;
        self.retrieve_wallet_info().await?;
        self.retrieve_keys().await?;
        self.retrieve_wallet_dids().await?;

        info!("---FIRST STEPS DONE---");
        let wallet = self.get_wallet().await?;
        let key_data = self.get_key().await?;
        let did_info = match wallet.dids.first() {
            Some(data) => data.clone(),
            None => {
                bail!("Something unexpected happened");
            }
        };

        info!("---SECOND STEPS DONE---");
        self.delete_did(&did_info).await?;
        self.delete_key(&key_data).await?;

        info!("---THIRD STEPS DONE---");
        self.register_key().await?;
        self.retrieve_keys().await?;

        info!("---FOURTH STEPS DONE---");
        self.register_did().await?;

        info!("---FIFTH STEPS DONE---");
        self.retrieve_wallet_info().await?;
        self.retrieve_wallet_dids().await?;
        self.set_default_did().await?;

        info!("---SIX STEPS DONE---");
        let wallet = self.get_wallet().await?;
        let did_info = match wallet.dids.first() {
            Some(data) => data.clone(),
            None => {
                bail!("Something unexpected happened");
            }
        };

        let participant_type = self.config.get_role().to_string();
        let base_url = self.config.get_host();
        let minion = mates::NewModel {
            participant_id: did_info.did,
            participant_slug: "Myself".to_string(),
            participant_type,
            base_url,
            is_me: true,
            token: None,
        };

        info!("Onboarding completed successfully");
        Ok(minion)
    }

    async fn partial_onboard(&self) -> anyhow::Result<()> {
        info!("Initializing partial onboarding");

        self.login().await?;
        self.retrieve_wallet_info().await?;
        self.retrieve_keys().await?;
        self.retrieve_wallet_dids().await?;

        info!("Initialization successful");
        Ok(())
    }

    // GET FROM STRUCT------------------------------------------------------------------------------------>
    async fn get_wallet(&self) -> anyhow::Result<WalletInfo> {
        info!("Getting wallet data");
        let wallet_session = self.wallet_session.lock().await;

        match wallet_session.wallets.first() {
            Some(data) => Ok(data.clone()),
            None => {
                let error = CommonErrors::missing_action_new(
                    MissingAction::Wallet,
                    "There is no wallet to retrieve dids from",
                );
                error!("{}", error.log());
                bail!(error)
            }
        }
    }

    async fn first_wallet_mut(&self) -> anyhow::Result<tokio::sync::MutexGuard<'_, WalletSession>> {
        let wallet_session = self.wallet_session.lock().await;

        if wallet_session.wallets.is_empty() {
            let error = CommonErrors::missing_action_new(MissingAction::Wallet, "No wallet available");
            error!("{}", error.log());
            bail!(error);
        }

        Ok(wallet_session)
    }

    async fn get_did(&self) -> anyhow::Result<String> {
        info!("Getting Did");
        let wallet = self.get_wallet().await?;

        match wallet.dids.first() {
            Some(did_entry) => Ok(did_entry.did.clone()),
            None => {
                let error = CommonErrors::missing_action_new(MissingAction::Did, "No DIDs found in wallet");
                error!("{}", error.log());
                bail!(error)
            }
        }
    }

    async fn get_token(&self) -> anyhow::Result<String> {
        info!("Getting token");

        let wallet_session = self.wallet_session.lock().await;
        match &wallet_session.token {
            Some(token) => Ok(token.clone()),
            None => {
                let error =
                    CommonErrors::missing_action_new(MissingAction::Token, "There is no token available for use");
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn get_did_doc(&self) -> anyhow::Result<Value> {
        info!("Getting Did Document");

        let wallet = self.get_wallet().await?;
        let did = match wallet.dids.first() {
            Some(did_entry) => did_entry.document.clone(),
            None => {
                let error = CommonErrors::missing_action_new(MissingAction::Did, "No DIDs found in wallet");
                error!("{}", error.log());
                bail!(error)
            }
        };

        let json: Value = serde_json::from_str(did.as_str())?;
        Ok(json)
    }

    async fn get_key(&self) -> anyhow::Result<KeyDefinition> {
        info!("Getting key data");

        let key_data = self.key_data.lock().await;
        match key_data.first() {
            Some(data) => Ok(data.clone()),
            None => {
                let error = CommonErrors::missing_action_new(MissingAction::Key, "Retrieve keys from wallet first");
                error!("{}", error.log());
                bail!(error)
            }
        }
    }

    // RETRIEVE FROM WALLET ------------------------------------------------------------------------------->
    async fn retrieve_wallet_info(&self) -> anyhow::Result<()> {
        info!("Retrieving wallet info from web wallet");
        let url = format!(
            "{}/wallet-api/wallet/accounts/wallets",
            self.config.get_wallet_api_url()
        );

        let token = self.get_token().await?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.get(&url, Some(headers)).await?;

        match res.status().as_u16() {
            200 => {
                let weird_wallets = res.json::<WalletInfoResponse>().await?.wallets;
                let mut wallets = Vec::<WalletInfo>::new();
                for wallet in weird_wallets {
                    let wallet = wallet.to_normal();
                    if !wallets.contains(&wallet) {
                        wallets.push(wallet);
                    }
                }
                let mut wallet_session = self.wallet_session.lock().await;
                for wallet in wallets {
                    if !wallet_session.wallets.contains(&wallet) {
                        wallet_session.wallets.push(wallet);
                    }
                }
                info!("Wallet data loaded successfully");
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "GET",
                    res.status().as_u16(),
                    "Petition to retrieve Wallet information failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
        Ok(())
    }

    async fn retrieve_keys(&self) -> anyhow::Result<()> {
        info!("Retrieving keys from web wallet");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/keys",
            self.config.get_wallet_api_url(),
            &wallet.id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/plain".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.get(&url, Some(headers)).await?;

        match res.status().as_u16() {
            200 => {
                info!("Keys retrieved successfully");
                let res = res.text().await?;
                let keys: Vec<KeyDefinition> = serde_json::from_str(&res)?;
                let mut key_data = self.key_data.lock().await;
                for key in keys {
                    if !key_data.contains(&key) {
                        key_data.push(key);
                    }
                }
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition to retrieve keys failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
        Ok(())
    }

    async fn retrieve_wallet_dids(&self) -> anyhow::Result<()> {
        info!("Retrieving dids from web wallet");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/dids",
            self.config.get_wallet_api_url(),
            &wallet.id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.get(&url, Some(headers)).await?;

        match res.status().as_u16() {
            200 => {
                let dids: Vec<DidsInfo> = res.json().await?;

                let mut wallet_session = self.first_wallet_mut().await?;
                let wallet = wallet_session.wallets.first_mut().unwrap();

                for did in dids {
                    if !wallet.dids.contains(&did) {
                        wallet.dids.push(did)
                    }
                }

                info!("Wallet Dids data loaded successfully");
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "GET",
                    res.status().as_u16(),
                    "Petition to retrieve Wallet DIDs failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
        Ok(())
    }

    // REGISTER STUFF IN WALLET ----------------------------------------------------------------------------->
    async fn register_key(&self) -> anyhow::Result<()> {
        info!("Registering key in web wallet");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;
        let body = self.config.get_priv_key()?;

        let url = format!(
            "{}/wallet-api/wallet/{}/keys/import",
            self.config.get_wallet_api_url(),
            &wallet.id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/plain".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.post(&url, Some(headers), Body::Raw(body)).await?;

        match res.status().as_u16() {
            201 => {
                info!("Key registered successfully");
                let res = res.text().await?;
                debug!("{}", res);
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition to register key failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
        Ok(())
    }

    async fn register_did(&self) -> anyhow::Result<()> {
        info!("Registering did in web wallet");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;
        let key_data = self.get_key().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/dids/create/jwk?keyId={}&alias=privatekey",
            self.config.get_wallet_api_url(),
            &wallet.id,
            key_data.key_id.id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.post(&url, Some(headers), Body::None).await?;

        match res.status().as_u16() {
            200 => {
                info!("Did registered successfully");
                let res = res.text().await?;
                debug!("{:#?}", res);
            }
            409 => {
                warn!("Did already exists");
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition to register key failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }

        Ok(())
    }

    async fn set_default_did(&self) -> anyhow::Result<()> {
        info!("Setting default did in web wallet");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;
        let did = self.get_did().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/dids/default?did={}",
            self.config.get_wallet_api_url(),
            &wallet.id,
            did
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.post(&url, Some(headers), Body::None).await?;

        match res.status().as_u16() {
            202 => {
                info!("Did has been set as default");
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition to set did as default failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }

        Ok(())
    }

    // DELETE STUFF FROM WALLET --------------------------------------------------------------------------->
    async fn delete_key(&self, key_id: &KeyDefinition) -> anyhow::Result<()> {
        info!("Deleting key in web wallet and from internal data");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/keys/{}",
            self.config.get_wallet_api_url(),
            &wallet.id,
            key_id.key_id.id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/plain".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.delete(&url, Some(headers), Body::None).await?;

        match res.status().as_u16() {
            202 => {
                info!("Key deleted successfully from web wallet");
                let mut keys_data = self.key_data.lock().await;
                keys_data.retain(|key| *key != *key_id);
                info!("Key deleted successfully from internal data");
                Ok(())
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "DELETE",
                    res.status().as_u16(),
                    "Petition to delete key failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn delete_did(&self, did_info: &DidsInfo) -> anyhow::Result<()> {
        info!("Deleting did from web wallet and from internal data");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/dids/{}",
            self.config.get_wallet_api_url(),
            &wallet.id,
            did_info.did
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/plain".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.delete(&url, Some(headers), Body::None).await?;

        match res.status().as_u16() {
            202 => {
                info!("Did deleted successfully from web wallet");
                let mut wallet_session = self.first_wallet_mut().await?;
                let wallet = wallet_session.wallets.first_mut().unwrap();

                wallet.dids.retain(|did| *did != *did_info);
                info!("Did deleted successfully from internal data");
                Ok(())
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "DELETE",
                    res.status().as_u16(),
                    "Petition to delete key failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn resolve_credential_offer(&self, payload: &OidcUri) -> anyhow::Result<CredentialOfferResponse> {
        info!("Resolving credential offer");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/resolveCredentialOffer",
            self.config.get_wallet_api_url(),
            &wallet.id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/plain;charset=UTF-8".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.post(&url, Some(headers), Body::Raw(payload.uri.clone())).await?;

        match res.status().as_u16() {
            200 => {
                let data: CredentialOfferResponse = res.json().await?;
                info!("Credential offer resolved successfully");
                Ok(data)
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition to resolve credential offer failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn resolve_credential_issuer(&self, cred_offer: &CredentialOfferResponse) -> anyhow::Result<Value> {
        info!("Resolving credential issuer metadata");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/resolveIssuerOpenIDMetadata?issuer={}",
            self.config.get_wallet_api_url(),
            &wallet.id,
            cred_offer.credential_issuer
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.get(&url, Some(headers)).await?;

        match res.status().as_u16() {
            200 => {
                let data: Value = res.json().await?;
                info!("Credential issuer resolved successfully");
                // debug!("{:#?}", data);
                Ok(data)
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "GET",
                    res.status().as_u16(),
                    "Petition resolve credential issuer failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn use_offer_req(&self, payload: &OidcUri, cred_offer: &CredentialOfferResponse) -> anyhow::Result<()> {
        info!("Accepting credential");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;
        let did = self.get_did().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/useOfferRequest?did={}&requireUserInput=false&pinOrTxCode={}",
            self.config.get_wallet_api_url(),
            &wallet.id,
            did,
            cred_offer.grants.pre_authorized_code.pre_authorized_code
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.post(&url, Some(headers), Body::Raw(payload.uri.clone())).await?;

        match res.status().as_u16() {
            200 => {
                let data: Value = res.json().await?;
                info!("Credential accepted successfully");
                debug!("{:#?}", data);
                Ok(())
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition accept credential issuer failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn get_vpd(&self, payload: &OidcUri) -> anyhow::Result<Vpd> {
        info!("Joining exchange");

        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/resolvePresentationRequest",
            self.config.get_wallet_api_url(),
            &wallet.id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/plain".parse()?);
        headers.insert(ACCEPT, "text/plain".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.post(&url, Some(headers), Body::Raw(payload.uri.clone())).await?;

        match res.status().as_u16() {
            200 => {
                info!("Joined the exchange successful");
                let vpd = res.text().await?;
                let vpd = self.parse_vpd(&vpd)?;
                Ok(vpd)
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Error joining the exchange",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    fn parse_vpd(&self, vpd_as_string: &str) -> anyhow::Result<Vpd> {
        info!("Parsing Vpd");

        let url = Url::parse(decode(&vpd_as_string)?.as_ref())?;

        let vpd_json = get_query_param(&url, "presentation_definition")?;

        let vpd: Vpd = match serde_json::from_str(&vpd_json) {
            Ok(data) => data,
            Err(err) => {
                let error = CommonErrors::format_new(
                    BadFormat::Received,
                    &format!("Error parsing the credential -> {}", err),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        debug!("{:#?}", vpd);
        Ok(vpd)
    }

    async fn get_matching_vcs(&self, vpd: &Vpd) -> anyhow::Result<Vec<String>> {
        info!("Matching Verifiable Credentials for OIDC4VP");
        let mut vcs_id = Vec::new();
        for descriptor in vpd.input_descriptors.clone() {
            let n_vpd = Vpd { id: "temporal_id".to_string(), input_descriptors: vec![descriptor] };
            let vcs = self.match_vc4vp(serde_json::to_value(n_vpd)?).await?;
            let vc_id = match vcs.first() {
                Some(vc) => vc.id.clone(),
                None => {
                    let error =
                        CommonErrors::forbidden_new("There are no VCs that match the specified input descriptor");
                    error!("{}", error.log());
                    bail!(error)
                }
            };
            vcs_id.push(vc_id);
        }
        Ok(vcs_id)
    }

    async fn match_vc4vp(&self, vp_def: Value) -> anyhow::Result<Vec<MatchingVCs>> {
        info!("Matching vcs for a specific descriptor");
        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/matchCredentialsForPresentationDefinition",
            self.config.get_wallet_api_url(),
            wallet.id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let res = self.client.post(&url, Some(headers), Body::Json(vp_def)).await?;
        match res.status().as_u16() {
            200 => {
                info!("Credentials matched successfully");
                let vc_json: Vec<MatchingVCs> = res.json().await?;
                Ok(vc_json)
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition to match credentials failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn present_vp(&self, payload: &OidcUri, vcs_id: Vec<String>) -> anyhow::Result<Option<String>> {
        info!("Presenting Verifiable Presentation");
        let wallet = self.get_wallet().await?;
        let token = self.get_token().await?;
        let did = self.get_did().await?;

        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/usePresentationRequest",
            self.config.get_wallet_api_url(),
            wallet.id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

        let body = MatchVCsRequest { did, presentation_request: payload.uri.clone(), selected_credentials: vcs_id };

        let res = self.client.post(&url, Some(headers), Body::Json(serde_json::to_value(body)?)).await?;
        match res.status().as_u16() {
            200 => {
                info!("Credentials presented successfully");
                // let data: RedirectResponse = res.json().await?;
                match res.json::<Option<RedirectResponse>>().await {
                    Ok(Some(data)) => Ok(Some(data.redirect_uri)),
                    _ => Ok(None),
                }
            }
            _ => {
                let error = AuthErrors::wallet_new(
                    &url,
                    "POST",
                    res.status().as_u16(),
                    "Petition to present credentials failed",
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }
}
