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

use super::{RainbowSSIAuthConsumerManagerTrait, RainbowSSIAuthConsumerWalletTrait};
use crate::ssi_auth::consumer::core::types::{
    AuthJwtClaims, DidsInfo, MatchingVCs, RedirectResponse, WalletInfo, WalletInfoResponse, WalletLoginResponse,
};
use crate::ssi_auth::consumer::setup::config::SSIAuthConsumerApplicationConfig;
use anyhow::bail;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{async_trait, Json};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use once_cell::sync::Lazy;
use rainbow_common::auth::gnap::{AccessToken, GrantRequest, GrantResponse};
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_common::mates::Mates;
use rainbow_db::auth_consumer::entities::auth;
use rainbow_db::auth_consumer::entities::auth_verification::Model;
use rainbow_db::auth_consumer::repo::AuthConsumerRepoTrait;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Response};
use sea_orm_migration::cli::Cli;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Serializer, Value};
use sha2::{Digest, Sha256};
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
    config: SSIAuthConsumerApplicationConfig,
    didweb: Value,
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
    pub fn new(auth_repo: Arc<T>, config: SSIAuthConsumerApplicationConfig) -> Self {
        info!("Manager created");
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Manager {
            wallet_session: WalletSession { account_id: None, token: None, token_exp: None, wallets: Vec::new() },
            wallet_onboard: false,
            auth_repo,
            client,
            config,
            didweb: Value::Null,
        }
    }
}

#[async_trait]
impl<T> RainbowSSIAuthConsumerWalletTrait for Manager<T>
where
    T: AuthConsumerRepoTrait + Send + Sync + Clone + 'static,
{
    async fn register_wallet(&self) -> anyhow::Result<()> {
        let wallet_portal_url = format!("http://{}",self.config.get_consumer_wallet_portal_url() + "/wallet-api/auth/register");
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

    async fn login_wallet(&mut self) -> anyhow::Result<()> {
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

                let jwt_parts: Vec<&str> = json_res.token.split('.').collect();

                if jwt_parts.len() != 3 {
                    bail!("JWT token does not have the correct format");
                }

                let decoded = URL_SAFE_NO_PAD.decode(jwt_parts[1])?;

                let claims: AuthJwtClaims = serde_json::from_slice(&decoded)?;

                self.wallet_session.token_exp = Some(claims.exp);

                Ok(())
            }
            _ => {
                error!("WaltId account login failed: {}", res.status());
                bail!("WaltId account login failed: {}", res.status())
            }
        }
    }

    async fn logout_wallet(&mut self) -> anyhow::Result<()> {
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
                let dids: Vec<DidsInfo> = res.json().await?;

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

    async fn onboard(&mut self) -> anyhow::Result<()> {
        if !self.wallet_onboard {
            self.register_wallet().await?;
        }
        self.login_wallet().await?;
        self.get_wallet_info().await?;
        self.get_wallet_dids().await?;

        Ok(())
    }

    async fn token_expired(&self) -> anyhow::Result<bool> {
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

    async fn ok(&mut self) -> anyhow::Result<()> {
        if self.token_expired().await? {
            self.update_token().await?;
        }
        Ok(())
    }

    async fn didweb(&mut self) -> anyhow::Result<Value> {
        Ok(json!({
          "@context": [
            "https://www.w3.org/ns/did/v1",
            "https://w3id.org/security/suites/jws-2020/v1"
          ],
          "id": "did:web:host.docker.internal%3A1235",
          "verificationMethod": [
            {
              "id": "did:web:host.docker.internal%3A1235#vfn_5i43O2Rs7vFjrpIBUln9LwG2-ALMyfQ-G4_F_8U",
              "type": "JsonWebKey2020",
              "controller": "did:web:host.docker.internal%3A1235",
              "publicKeyJwk": {
                "kty": "OKP",
                "crv": "Ed25519",
                "kid": "vfn_5i43O2Rs7vFjrpIBUln9LwG2-ALMyfQ-G4_F_8U",
                "x": "6OmRPulFw3MX44mbZ0bedcULKSrPdZlqqsIrwgBhyLs"
              }
            }
          ],
          "assertionMethod": [
            "did:web:host.docker.internal%3A1235#vfn_5i43O2Rs7vFjrpIBUln9LwG2-ALMyfQ-G4_F_8U"
          ],
          "authentication": [
            "did:web:host.docker.internal%3A1235#vfn_5i43O2Rs7vFjrpIBUln9LwG2-ALMyfQ-G4_F_8U"
          ],
          "capabilityInvocation": [
            "did:web:host.docker.internal%3A1235#vfn_5i43O2Rs7vFjrpIBUln9LwG2-ALMyfQ-G4_F_8U"
          ],
          "capabilityDelegation": [
            "did:web:host.docker.internal%3A1235#vfn_5i43O2Rs7vFjrpIBUln9LwG2-ALMyfQ-G4_F_8U"
          ],
          "keyAgreement": [
            "did:web:host.docker.internal%3A1235#vfn_5i43O2Rs7vFjrpIBUln9LwG2-ALMyfQ-G4_F_8U"
          ]
        }))

        // Ok(self.didweb.clone())
    }
}

#[async_trait]
impl<T> RainbowSSIAuthConsumerManagerTrait for Manager<T>
where
    T: AuthConsumerRepoTrait + Send + Sync + Clone + 'static,
{
    async fn request_access(&self, url: String, provider: String, actions: String) -> anyhow::Result<Model> {
        let mut body = GrantRequest::default4oidc(
            self.config.ssi_consumer_client.consumer_client.clone(), // TODO change with did:web
            "push".to_string(),
        );

        let id = uuid::Uuid::new_v4().to_string();
        body.update_actions(actions.clone());
        let auth_url = self.config.get_auth_host_url().unwrap();
        let callback = format!("{}/callback/{}", auth_url, id.clone());
        body.update_callback(callback);

        let interact = body.clone().interact.unwrap();
        let model = match self.auth_repo.create_auth(id, url.clone(), provider, actions, interact).await {
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

        let mut res: GrantResponse = match res.status().as_u16() {
            200 => {
                info!("Grant Response received successfully");
                res.json().await?
            }
            _ => {
                error!("Grant Response failed: {}", res.status());
                bail!("Grant Response failed: {}", res.status())
            }
        };

        let interact = res.interact.unwrap();

        match self
            .auth_repo
            .auth_pending(
                model.id.clone(),
                res.instance_id.unwrap(),
                interact.finish.unwrap(),
            )
            .await
        {
            Ok(model) => {
                info!("Assigned id updated successfully");
            }
            Err(e) => bail!("Unable to update assigned id in db: {}", e),
        };

        let model = match self.auth_repo.create_auth_verification(model.id.clone(), interact.oidc4vp.unwrap()).await {
            Ok(model) => {
                info!("Verification data stored successfully");
                model
            }
            Err(e) => bail!("Unable to save verification in db: {}", e),
        };

        Ok(model)
    }

    async fn manual_request_access(&self, url: String, provider: String, actions: String) -> anyhow::Result<Model> {
        let _ = match self.auth_repo.create_prov(provider.clone(), url.clone()).await {
            Ok(model) => {
                info!("Provider saved successfully");
                model
            }
            Err(e) => bail!("Unable to save provider in db: {}", e),
        };

        let mut body = GrantRequest::default4oidc(
            self.config.ssi_consumer_client.consumer_client.clone(), // TODO change with did:web
            "redirect".to_string(),
        );

        let id = uuid::Uuid::new_v4().to_string();
        body.update_actions(actions.clone());
        let auth_url = self.config.get_auth_host_url().unwrap();
        let callback = format!("{}/api/v1/callback/manual/{}", auth_url, id.clone());
        body.update_callback(callback);

        let interact = body.clone().interact.unwrap();
        let model = match self.auth_repo.create_auth(id, url.clone(), provider, actions, interact).await {
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

        let mut res: GrantResponse = match res.status().as_u16() {
            200 => {
                info!("Grant Response received successfully");
                res.json().await?
            }
            _ => {
                error!("Grant Response failed: {}", res.status());
                bail!("Grant Response failed: {}", res.status())
            }
        };

        let interact = res.interact.unwrap();

        match self
            .auth_repo
            .auth_pending(
                model.id.clone(),
                res.instance_id.unwrap(),
                interact.finish.unwrap(),
            )
            .await
        {
            Ok(model) => {
                info!("Assigned id updated successfully");
            }
            Err(e) => bail!("Unable to update assigned id in db: {}", e),
        };

        let model = match self.auth_repo.create_auth_verification(model.id.clone(), interact.oidc4vp.unwrap()).await {
            Ok(model) => {
                info!("Verification data stored successfully");
                model
            }
            Err(e) => bail!("Unable to save verification in db: {}", e),
        };

        Ok(model)
    }

    async fn join_exchange(&self, exchange_url: String) -> anyhow::Result<String> {
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

    async fn parse_vpd(&self, vpd_as_string: String) -> anyhow::Result<Value> {
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

    async fn match_vc4vp(&self, vp_def: Value) -> anyhow::Result<Vec<MatchingVCs>> {
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

        let res = self.client.post(url).headers(headers).json(&vp_def).send().await;

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

    async fn present_vp(&self, preq: String, creds: Vec<String>) -> anyhow::Result<RedirectResponse> {
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
        let body: RedirectResponse = res.json().await?;

        Ok(body)
    }

    async fn do_callback(&self, uri: String) -> anyhow::Result<()> {
        // TODO
        Ok(())
    }
    async fn check_callback(&self, id: String, interact_ref: String, hash: String) -> anyhow::Result<String> {
        let model = match self.auth_repo.update_interaction_by_id(id, interact_ref, hash.clone()).await {
            Ok(model) => model,
            Err(e) => bail!("Error getting interaction by id: {}", e),
        };

        let hash_method = model.hash_method.unwrap_or_else(|| "sha-256".to_string());
        let hash_input = format!(
            "{}\n{}\n{}\n{}",
            model.client_nonce,
            model.as_nonce.unwrap(),
            model.interact_ref.unwrap(),
            model.grant_endpoint
        );

        let mut hasher = Sha256::new();
        hasher.update(hash_input.as_bytes());
        let result = hasher.finalize();

        let calculated_hash = URL_SAFE_NO_PAD.encode(result);

        if calculated_hash != hash {
            bail!("Incorrect hash")
        }

        info!("Hash matches the calculated one");
        Ok(model.grant_endpoint)
    }

    async fn continue_request(&self, id: String, interact_ref: String, uri: String) -> anyhow::Result<auth::Model> {
        // TODO WAIT 5 SECONDS
        info!("Continuing request");
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let body = json!({
            "interact_ref": interact_ref
        });

        let mut url = uri;
        url = url.replace("access", "continue");

        let res = self.client.post(url).headers(headers).json(&body).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        // TODO MERECE LA PENA PONER ESTADO PROCCESING??

        let res: AccessToken = match res.status().as_u16() {
            200 => {
                info!("Success retrieving the token");
                res.json().await?
            }
            _ => {
                error!("Error retrieving the token: {}", res.status());
                bail!("Error retrieving the token: {}", res.status());
            }
        };

        let model = match self.auth_repo.grant_req_approved(id, res.value).await {
            Ok(model) => model,
            Err(e) => bail!("Error saving data: {}", e),
        };

        Ok(model)
    }

    async fn save_mate(&self, id: String, base_url: String, token: String, token_actions: String) -> anyhow::Result<Response> {
        let url = format!("{}/api/v1/mates", self.config.get_ssi_auth_host_url().unwrap()); // TODO fix 4 microservices

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let body = Mates::default4consumer(id, base_url, token, token_actions);

        let res = self.client.post(url).headers(headers).json(&body).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("Mate saved successfully");
            }
            _ => {
                error!("Mate saving failed: {}", res.status());
                bail!("Mate saving failed: {}", res.status());
            }
        }

        Ok(res)
    }

    async fn beg4credential(&self, url: String) -> anyhow::Result<()> {
        let body = GrantRequest::default4async(self.config.ssi_consumer_client.consumer_client.clone());

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(url).headers(headers).json(&body).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        let mut res: GrantResponse = match res.status().as_u16() {
            200 => {
                info!("Grant Response received successfully");
                res.json().await?
            }
            _ => {
                error!("Grant Response failed: {}", res.status());
                bail!("Grant Response failed: {}", res.status())
            }
        };

        Ok(())
    }
}

// pub static MANAGER: Lazy<Arc<Mutex<Manager>>> = Lazy::new(|| Arc::new(Mutex::new(Manager::new())));
