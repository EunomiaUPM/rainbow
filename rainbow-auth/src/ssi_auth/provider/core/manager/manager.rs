/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under terms of the GNU General Public License as published by
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

use crate::ssi_auth::consumer::core::types::{AuthJwtClaims, WalletInfoResponse, WalletLoginResponse};
use crate::ssi_auth::provider::core::manager::RainbowSSIAuthProviderManagerTrait;
use crate::ssi_auth::provider::setup::config::SSIAuthProviderApplicationConfig;
use crate::ssi_auth::provider::utils::{compare_with_margin, create_opaque_token, create_token, split_did};
use anyhow::bail;
use axum::async_trait;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderMap;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use chrono::format::Fixed::TimezoneName;
use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken;
use jsonwebtoken::jwk::{Jwk, KeyAlgorithm};
use jsonwebtoken::{Algorithm, Validation};
use log::error;
use rainbow_common::auth::gnap::{GrantRequest, GrantResponse};
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use rainbow_common::mates::{BusMates, Mates};
use rainbow_common::ssi_wallet::{DidsInfo, RainbowSSIAuthWalletTrait, WalletSession};
use rainbow_db::auth_consumer::repo_factory::AuthConsumerRepoTrait;
use rainbow_db::auth_provider::repo::AuthProviderRepoTrait;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tracing::field::debug;
use tracing::{debug, info};
use url::Url;
use urlencoding::{decode, encode};

#[derive(Debug)]
pub struct Manager<T>
where
    T: AuthProviderRepoTrait + Send + Sync + Clone + 'static,
{
    pub wallet_session: Mutex<WalletSession>,
    pub wallet_onboard: bool,
    pub auth_repo: Arc<T>,
    client: Client,
    config: SSIAuthProviderApplicationConfig,
    didweb: Value,
}

impl<T> Manager<T>
where
    T: AuthProviderRepoTrait + Send + Sync + Clone + 'static,
{
    pub fn new(auth_repo: Arc<T>, config: SSIAuthProviderApplicationConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self {
            wallet_session: Mutex::new(WalletSession {
                account_id: None,
                token: None,
                token_exp: None,
                wallets: Vec::new(),
            }),
            wallet_onboard: false,
            auth_repo,
            client,
            config,
            didweb: Value::Null,
        }
    }
}

#[async_trait]
impl<T> RainbowSSIAuthProviderManagerTrait for Manager<T>
where
    T: AuthProviderRepoTrait + Send + Sync + Clone + 'static,
{
    async fn generate_exchange_uri(&self, payload: GrantRequest) -> anyhow::Result<(String, String, String)> {
        info!("Generating exchange URI");

        let interact = payload.interact.unwrap();
        let start = interact.clone().start;

        if !start.contains(&"oidc4vp".to_string()) {
            error!("Interact Method {} Not supported ", start.first().unwrap());
            bail!("Interact Method {} Not supported ", start.first().unwrap());
        }
        let mut provider_url = self.config.get_ssi_auth_host_url().unwrap(); // TODO fix docker internal
        provider_url = provider_url.replace("127.0.0.1", "host.docker.internal");
        provider_url = format!("{}/api/v1", provider_url);

        let client_id = format!("{}/verify", &provider_url);

        let actions = payload.access_token.access.actions.unwrap_or_else(|| String::from("talk"));
        let grant_endpoint = format!(
            "{}/api/v1/access",
            self.config.get_ssi_auth_host_url().unwrap()
        );

        let (auth_model, interaction_model, verification_model) = match self
            .auth_repo
            .create_auth(
                payload.client.to_string(),
                client_id.clone(),
                grant_endpoint,
                actions,
                interact,
            )
            .await
        {
            Ok(model) => {
                info!("exchange saved successfully");
                model
            }
            Err(e) => bail!("Unable to save exchange in db: {}", e),
        };

        let base_url = "openid4vp://authorize";

        let encoded_client_id = encode(&verification_model.audience);

        let state = verification_model.state;
        let nonce = verification_model.nonce;

        let presentation_definition_uri = format!("{}/pd/{}", &provider_url, state);
        let encoded_presentation_definition_uri = encode(&presentation_definition_uri);

        let response_uri = format!("{}/verify/{}", &provider_url, state);
        let encoded_response_uri = encode(&response_uri);

        let response_type = "vp_token";
        let response_mode = "direct_post";
        let client_id_scheme = "redirect_uri";

        // TODO let client_metadata = r#"{"authorization_encrypted_response_alg":"ECDH-ES","authorization_encrypted_response_enc":"A256GCM"}"#;

        let uri = format!("{}?response_type={}&client_id={}&response_mode={}&presentation_definition_uri={}&client_id_scheme={}&nonce={}&response_uri={}", base_url, response_type, encoded_client_id, response_mode, encoded_presentation_definition_uri, client_id_scheme, nonce, encoded_response_uri);
        info!("uri generated successfully: {}", uri);
        Ok((auth_model.id, uri, interaction_model.as_nonce))
    }

    async fn generate_vp_def(&self, state: String) -> anyhow::Result<Value> {
        // json!({
        //     "vp_policies": [
        //         {
        //             "policy": "minimum-credentials",
        //             "args": 1
        //         },
        //         {
        //             "policy": "maximum-credentials",
        //             "args": 100
        //         }
        //     ],
        //     "vc_policies": [
        //         "signature",
        //         "expired",
        //         "not-before",
        //         "revoked-status-list"
        //     ],
        //     "request_credentials": [
        //         {
        //             "format": "jwt_vc_json",
        //             "type": "VerifiableId"
        //         }
        //     ]
        // })

        let auth = match self.auth_repo.get_auth_by_state(state.clone()).await {
            Ok(auth) => auth,
            Err(e) => bail!("No exchange for state {}", state),
        };

        Ok(json!({
          "id": auth.id,
          "input_descriptors": [
            {
              "id": "DataspaceParticipantCredential",
              "format": {
                "jwt_vc_json": {
                  "alg": [
                    "EdDSA"
                  ]
                }
              },
              "constraints": {
                "fields": [
                  {
                    "path": [
                      "$.vc.type"
                    ],
                    "filter": {
                      "type": "string",
                      "pattern": "DataspaceParticipantCredential"
                    }
                  }
                ]
              }
            }
          ]
        }))
    }

    async fn verify_all(&self, state: String, vp_token: String) -> anyhow::Result<Option<String>> {
        let exchange_model = match self.auth_repo.get_auth_by_state(state.clone()).await {
            Ok(auth) => auth,
            Err(e) => bail!("No exchange for state {}", state),
        };
        let exchange = exchange_model.id;

        let (vcts, holder) = match self.verify_vp(exchange.clone(), state, vp_token).await {
            Ok(v) => v,
            Err(e) => {
                match self.auth_repo.update_verification_result(exchange, false).await {
                    Ok(_) => {}
                    Err(e) => {
                        bail!("{}", e)
                    }
                }
                bail!("{}", e)
            }
        };
        for cred in vcts {
            match self.verify_vc(cred, holder.clone()).await {
                Ok(()) => {}
                Err(e) => {
                    match self.auth_repo.update_verification_result(exchange, false).await {
                        Ok(_) => {}
                        Err(e) => {
                            bail!("{}", e)
                        }
                    }
                    bail!("{}", e)
                }
            }
        }
        info!("VP & VP Validated successfully");

        let state = match self.auth_repo.update_verification_result(exchange.clone(), true).await {
            Ok(model) => model.state,
            Err(e) => {
                bail!("{}", e)
            }
        };

        // TODO

        let interact = match self.auth_repo.get_interaction_by_id(exchange.clone()).await {
            Ok(interact) => interact,
            Err(e) => {
                if e.to_string().contains("No Interaction from authentication with id") {
                    let url = format!(
                        "{}/api/v1/busmates",
                        self.config.get_ssi_auth_host_url().unwrap()
                    ); // TODO fix 4 microservices

                    let mut headers = HeaderMap::new();
                    headers.insert(CONTENT_TYPE, "application/json".parse()?);
                    headers.insert(ACCEPT, "application/json".parse()?);

                    let token: String = create_opaque_token();

                    let body = BusMates::default4provider(
                        state,
                        Some(exchange_model.holder.unwrap()),
                        Some(token),
                    ); // TODO

                    println!("{:#?}", body);

                    let res = self.client.post(url).headers(headers).json(&body).send().await;

                    let res = match res {
                        Ok(res) => res,
                        Err(e) => bail!("Error sending request: {}", e),
                    };

                    match res.status().as_u16() {
                        200 => {
                            info!("Business Mate saved successfully");
                        }
                        _ => {
                            error!("Business Mate saving failed: {}", res.status());
                            bail!("Mate saving failed: {}", res.status());
                        }
                    }
                    return Ok(None); // TODO CARLOS VISTA
                } else {
                    bail!("{}", e)
                }
            }
        };

        match interact.method.as_str() {
            "redirect" => match interact.uri {
                Some(uri) => {
                    let redirect_uri = format!(
                        "{}?hash={}&interact_ref={}",
                        uri, interact.hash, interact.interact_ref
                    );
                    Ok(Some(redirect_uri))
                }
                None => Ok(None),
            },
            "push" => {
                match interact.uri {
                    // TODO ESTO DE MOMENTO NO ESTA SOPORTADO PRO LA WALLET
                    Some(uri) => {
                        // let redirect_uri = uri + "?nonce=" + &interact.nonce;
                        let redirect_uri = uri;
                        Ok(Some(redirect_uri))
                    }
                    None => Ok(None),
                }
            }
            _ => {
                bail!("Interact method not supported")
            }
        }
    }

    async fn verify_vp(
        &self,
        exchange: String,
        state: String,
        vp_token: String,
    ) -> anyhow::Result<(Vec<String>, String)> {
        info!("Verifying VP");
        let header = jsonwebtoken::decode_header(&vp_token)?;
        let kid_str = header.kid.as_ref().unwrap();
        let (kid, kid_id) = split_did(kid_str.as_str()); // TODO KID_ID
        let alg = header.alg;

        let vec = URL_SAFE_NO_PAD.decode(&(kid.replace("did:jwk:", "")))?;
        let mut jwk: Jwk = serde_json::from_slice(&vec)?;

        let key = jsonwebtoken::DecodingKey::from_jwk(&jwk)?;
        let mut audience = format!(
            "{}/api/v1/verify/{}",
            self.config.get_ssi_auth_host_url().unwrap(),
            state
        );
        audience = audience.replace("127.0.0.1", "host.docker.internal"); // TODO fix docker

        let mut val = Validation::new(alg);

        val.required_spec_claims = HashSet::new();
        val.validate_aud = true;
        val.set_audience(&[&(audience)]);
        val.validate_exp = false;
        val.validate_nbf = true;

        let token = match jsonwebtoken::decode::<Value>(&vp_token, &key, &val) {
            Ok(token) => token,
            Err(e) => {
                error!("VPT token signature is incorrect");
                bail!("VPT token signature is incorrect {}", e)
            }
        };

        debug!("{:#?}", token);
        info!("VPT token signature is correct");

        let id = token.claims["jti"].as_str().unwrap();
        let nonce = token.claims["nonce"].as_str().unwrap();

        if token.claims["sub"].as_str().unwrap() != token.claims["iss"].as_str().unwrap()
            || token.claims["iss"].as_str().unwrap() != kid
        {
            // VALIDATE HOLDER 1
            error!("VPT token issuer, subject & kid does not match");
            bail!("VPT token issuer, subject & kid does not match");
        }
        info!("VPT issuer, subject & kid matches");

        let auth_ver = match self
            .auth_repo
            .get_av_by_id_update_holder(
                id.to_string(),
                vp_token,
                token.claims["sub"].as_str().unwrap().to_string(),
            )
            .await
        {
            Ok(model) => model,
            Err(e) => {
                error!("No verification expected for id: {}", id);
                bail!("No verification expected for id: {}", id)
            }
        };

        if auth_ver.nonce != nonce {
            // VALIDATE NONCE
            error!("Invalid nonce");
            bail!("Invalid nonce");
        }
        info!("VPT Nonce matches");

        if auth_ver.id != exchange || exchange != token.claims["vp"]["id"].as_str().unwrap() {
            // VALIDATE ID MATCHES JTI
            error!("Invalid exchange");
            bail!("Invalid exchange");
        }
        info!("Exchange is valid");

        if auth_ver.holder.unwrap() != token.claims["vp"]["holder"].as_str().unwrap() {
            error!("VP id does not match sub & issuer");
            bail!("VP id does not match sub & issuer");
        }
        info!("vp holder matches vpt subject & issuer");
        info!("VP Verification successful");

        let vct: Vec<String> = match serde_json::from_value(token.claims["vp"]["verifiableCredential"].clone()) {
            Ok(vc) => vc,
            Err(_) => {
                error!("VPresentation is based on a nonexistent credential");
                bail!("VPresentation is based on a nonexistent credential")
            }
        };
        Ok((vct, kid.to_string()))
    }

    async fn verify_vc(&self, vc_token: String, vp_holder: String) -> anyhow::Result<()> {
        info!("Verifying VC");
        let header = jsonwebtoken::decode_header(&vc_token)?;
        let kid_str = header.kid.as_ref().unwrap();
        let (kid, kid_id) = split_did(kid_str.as_str()); // TODO KID_ID
        let alg = header.alg;

        let vec = URL_SAFE_NO_PAD.decode(&(kid.replace("did:jwk:", "")))?;
        let mut jwk: Jwk = serde_json::from_slice(&vec)?;

        let key = jsonwebtoken::DecodingKey::from_jwk(&jwk)?;

        let mut val = Validation::new(alg);
        val.required_spec_claims = HashSet::new();
        val.validate_aud = false;
        val.validate_exp = false; // TODO de momemnto las VCs no caducan
        val.validate_nbf = true;

        let token = match jsonwebtoken::decode::<Value>(&vc_token, &key, &val) {
            Ok(token) => token,
            Err(e) => {
                error!("VCT token signature is incorrect");
                bail!("VCT token signature is incorrect {}", e)
            }
        };

        info!("VCT token signature is correct");
        debug!("{:#?}", token);

        if token.claims["iss"].as_str().unwrap() != kid || kid != token.claims["vc"]["issuer"]["id"].as_str().unwrap() {
            // VALIDATE IF ISSUER IS THE SAME AS KID
            error!("VCT token issuer & kid does not match");
            bail!("VCT token issuer & kid does not match");
        }
        info!("VCT issuer & kid matches");

        // if issuers_list.contains(kid) {
        //     // TODO
        //     error!("VCT issuer is not on the trusted issuers list");
        //     bail!("VCT issuer is not on the trusted issuers list");
        // }
        // info!("VCT issuer is on the trusted issuers list");

        if token.claims["sub"].as_str().unwrap() != &vp_holder
            || &vp_holder != token.claims["vc"]["credentialSubject"]["id"].as_str().unwrap()
        {
            error!("VCT token sub, credential subject & VP Holder do not match");
            bail!("VCT token sub, credential subject & VP Holder do not match");
        }
        info!("VC Holder Data is Correct");

        if token.claims["jti"].as_str().unwrap() != token.claims["vc"]["id"].as_str().unwrap() {
            error!("VCT jti & VC id do not match");
            bail!("VCT jti & VC id do not match");
        }
        info!("VCT jti & VC id match");

        let (keep, message) = compare_with_margin(
            token.claims["iat"].as_i64().unwrap(),
            token.claims["vc"]["issuanceDate"].as_str().unwrap(),
            2,
        );
        if keep {
            error!("{}", &message);
            bail!("{}", &message);
        }
        info!("VC IssuanceDate and iat field match");

        match DateTime::parse_from_rfc3339(token.claims["vc"]["validFrom"].as_str().unwrap()) {
            Ok(parsed_date) => parsed_date <= Utc::now(),
            Err(_) => {
                error!("VC iat and issuanceDate do not match");
                bail!("VC iat and issuanceDate do not match");
            }
        };
        info!("VC validFrom is correct");
        info!("VC Verification successful");
        Ok(())
    }

    async fn continue_req(&self, interact_ref: String) -> anyhow::Result<(Value, String, String)> {
        let auth_interact = match self.auth_repo.get_auth_by_interact_ref(interact_ref).await {
            Ok(auth_interact) => auth_interact,
            Err(e) => bail!("No interact reference expected"),
        };

        let auth_interact_id = auth_interact.id;
        let model = match self.auth_repo.get_auth_by_id(auth_interact_id.clone()).await {
            Ok(model) => model,
            Err(e) => {
                bail!("Not expected")
            }
        };

        // if model.status != "pending" {
        //     bail!("Too many attempts"); // TODO
        // }

        let token: String = create_opaque_token();
        let base_url = auth_interact.uri.unwrap_or("".to_string());
        let base_url = match Url::parse(base_url.as_str()) {
            Ok(parsed_url) => {
                format!(
                    "{}://{}:{}",
                    parsed_url.scheme(),
                    parsed_url.host_str().unwrap_or_default(),
                    parsed_url.port().unwrap_or_default()
                )
            }
            Err(e) => bail!("not able to parse url: {}", e.to_string()),
        };

        let model = match self.auth_repo.save_token(auth_interact_id, base_url.clone(), token.clone()).await {
            Ok(model) => model,
            Err(e) => {
                bail!("Unable to create token")
            }
        };

        let holder = match self.auth_repo.get_auth_ver_by_id(model.id.clone()).await {
            Ok(ver_model) => ver_model.holder.unwrap(),
            Err(e) => {
                bail!("Unexpected error")
            }
        };

        Ok((serde_json::to_value(&model)?, base_url, holder))
    }

    async fn save_mate(
        &self,
        global_id: Option<String>,
        slug: String,
        token: String,
        base_url: String,
        token_actions: String,
    ) -> anyhow::Result<()> {
        let url = format!(
            "{}/api/v1/mates",
            self.config.get_ssi_auth_host_url().unwrap()
        ); // TODO fix 4 microservices

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let body = Mates::default4provider(
            global_id,
            slug,
            base_url,
            Some(token),
            Some(token_actions),
            false,
        ); // TODO

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

        Ok(())
    }

    // async fn save_busmate(&self, global_id: Option<String>, token: String) -> anyhow::Result<()> {
    //     let url = format!(
    //         "{}/api/v1/busmates",
    //         self.config.get_ssi_auth_host_url().unwrap()
    //     ); // TODO fix 4 microservices
    //
    //     let mut headers = HeaderMap::new();
    //     headers.insert(CONTENT_TYPE, "application/json".parse()?);
    //     headers.insert(ACCEPT, "application/json".parse()?);
    //
    //     let body = BusMates::default4provider(global_id, Some(token), false); // TODO
    //
    //     let res = self.client.post(url).headers(headers).json(&body).send().await;
    //
    //     let res = match res {
    //         Ok(res) => res,
    //         Err(e) => bail!("Error sending request: {}", e),
    //     };
    //
    //     match res.status().as_u16() {
    //         200 => {
    //             info!("BusMate saved successfully");
    //         }
    //         _ => {
    //             error!("Mate saving failed: {}", res.status());
    //             bail!("Mate saving failed: {}", res.status());
    //         }
    //     }
    //
    //     Ok(())
    // }

    fn get_continue_uri(&self) -> anyhow::Result<String> {
        Ok(format!(
            "{}/api/v1/continue",
            self.config.get_auth_host_url().unwrap()
        ))
    }

    async fn generate_uri(&self, state: String) -> anyhow::Result<String> {
        info!("Generating exchange URI");

        let mut provider_url = self.config.get_ssi_auth_host_url().unwrap(); // TODO fix docker internal
        provider_url = provider_url.replace("127.0.0.1", "host.docker.internal");
        provider_url = format!("{}/api/v1", provider_url);

        let client_id = format!("{}/verify", &provider_url);

        let actions = String::from("talk");
        let grant_endpoint = format!(
            "{}/api/v1/access",
            self.config.get_ssi_auth_host_url().unwrap()
        );

        let (auth_model, verification_model) =
            match self.auth_repo.create_truncated_auth(client_id.clone(), state).await {
                Ok(model) => {
                    info!("exchange saved successfully");
                    model
                }
                Err(e) => bail!("Unable to save exchange in db: {}", e),
            };

        let base_url = "openid4vp://authorize";

        let encoded_client_id = encode(&verification_model.audience);

        let state = verification_model.state;
        let nonce = verification_model.nonce;

        let presentation_definition_uri = format!("{}/pd/{}", &provider_url, state);
        let encoded_presentation_definition_uri = encode(&presentation_definition_uri);

        let response_uri = format!("{}/verify/{}", &provider_url, state);
        let encoded_response_uri = encode(&response_uri);

        let response_type = "vp_token";
        let response_mode = "direct_post";
        let client_id_scheme = "redirect_uri";

        // TODO let client_metadata = r#"{"authorization_encrypted_response_alg":"ECDH-ES","authorization_encrypted_response_enc":"A256GCM"}"#;

        let uri = format!("{}?response_type={}&client_id={}&response_mode={}&presentation_definition_uri={}&client_id_scheme={}&nonce={}&response_uri={}", base_url, response_type, encoded_client_id, response_mode, encoded_presentation_definition_uri, client_id_scheme, nonce, encoded_response_uri);
        info!("uri generated successfully: {}", uri);
        Ok(uri)
    }
}

#[async_trait]
impl<T> RainbowSSIAuthWalletTrait for Manager<T>
where
    T: AuthProviderRepoTrait + Send + Sync + Clone + 'static,
{
    async fn register_wallet(&self) -> anyhow::Result<()> {
        let url = format!(
            "{}/wallet-api/auth/register",
            self.config.get_wallet_portal_url()
        );
        let wallet_data = self.config.get_wallet_data();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(url).headers(headers).json(&wallet_data).send().await;

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
                tracing::error!("WaltId account registration failed: {}", res.status());
                bail!("WaltId account registration failed: {}", res.status());
            }
        }
        Ok(())
    }

    async fn login_wallet(&self) -> anyhow::Result<()> {
        let url = format!(
            "{}/wallet-api/auth/login",
            self.config.get_wallet_portal_url()
        );

        let mut wallet_data = self.config.get_wallet_data();
        wallet_data.as_object_mut().map(|obj| obj.remove("name"));

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(url).headers(headers).json(&wallet_data).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("WaltId account login successful");

                let json_res: WalletLoginResponse = res.json().await?;

                let mut wallet_session = self.wallet_session.lock().await;
                wallet_session.account_id = Some(json_res.id);
                wallet_session.token = Some(json_res.token.clone());

                let jwt_parts: Vec<&str> = json_res.token.split('.').collect();

                if jwt_parts.len() != 3 {
                    bail!("JWT token does not have the correct format");
                }

                let decoded = URL_SAFE_NO_PAD.decode(jwt_parts[1])?;

                let claims: AuthJwtClaims = serde_json::from_slice(&decoded)?;

                wallet_session.token_exp = Some(claims.exp);

                Ok(())
            }
            _ => {
                tracing::error!("WaltId account login failed: {}", res.status());
                bail!("WaltId account login failed: {}", res.status())
            }
        }
    }

    async fn logout_wallet(&self) -> anyhow::Result<()> {
        let url = format!(
            "{}/wallet-api/auth/logout",
            self.config.get_wallet_portal_url()
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(url).headers(headers).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                info!("WaltId account logout successful");
                let mut wallet_session = self.wallet_session.lock().await;
                wallet_session.token = None;
            }
            _ => {
                tracing::error!("WaltId account logout failed: {}", res.status());
                bail!("WaltId account logout failed: {}", res.status())
            }
        }

        Ok(())
    }

    async fn get_wallet_info(&self) -> anyhow::Result<()> {
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
            None => bail!("No token available for wallet authentication"),
        };

        let res = self.client.get(url).headers(headers).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        match res.status().as_u16() {
            200 => {
                let wallets = res.json::<WalletInfoResponse>().await?.wallets;
                for wallet in wallets {
                    if wallet_session.wallets.contains(&wallet) {
                        info!("Wallet {} already exists", wallet.id);
                    } else {
                        wallet_session.wallets.push(wallet);
                    }
                }

                info!("Wallet data loaded successfully");
            }
            _ => {
                tracing::error!("Wallet data loading failed: {}", res.status());
                bail!("Wallet data loading failed: {}", res.status())
            }
        }

        Ok(())
    }

    async fn get_wallet_dids(&self) -> anyhow::Result<()> {
        let mut wallet_session = self.wallet_session.lock().await;

        if wallet_session.wallets.first().is_none() {
            bail!("There is not a wallet registered")
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
            None => bail!("No token available for authentication"),
        };

        let res = self.client.get(url).headers(headers).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
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
                tracing::error!("Wallet Dids data loading failed: {}", res.status());
                bail!("Wallet Dids data loading failed: {}", res.status())
            }
        }

        Ok(())
    }

    async fn onboard(&self) -> anyhow::Result<()> {
        if !self.wallet_onboard {
            self.register_wallet().await?
        }
        self.login_wallet().await?;
        self.get_wallet_info().await?;
        self.get_wallet_dids().await?;

        let url = format!(
            "{}/api/v1/mates",
            self.config.get_ssi_auth_host_url().unwrap()
        ); // TODO fix 4 microservices

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let mut wallet_session = self.wallet_session.lock().await;

        let did = wallet_session.wallets.first().unwrap().dids.clone().unwrap().first().unwrap().did.clone();
        let body = Mates::default4consumer(
            Some(did),
            "Myself".to_string(),
            self.config.get_auth_host_url().unwrap(),
            None,
            None,
            true,
        ); // TODO

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
                tracing::error!("Mate saving failed: {}", res.status());
                bail!("Mate saving failed: {}", res.status());
            }
        }

        // let url2 = format!(
        //     "{}/api/v1/busmates",
        //     self.config.get_ssi_auth_host_url().unwrap()
        // ); // TODO fix 4 microservices
        //
        // let mut headers = HeaderMap::new();
        // headers.insert(CONTENT_TYPE, "application/json".parse()?);
        // headers.insert(ACCEPT, "application/json".parse()?);

        // let mut wallet_session = self.wallet_session.lock().await;

        // let did = wallet_session.wallets.first().unwrap().dids.clone().unwrap().first().unwrap().did.clone();
        // let body = BusMates::default4consumer(Some(did), None, true);

        // let res = self.client.post(url2).headers(headers).json(&body).send().await;
        //
        // let res = match res {
        //     Ok(res) => res,
        //     Err(e) => bail!("Error sending request: {}", e),
        // };
        //
        // match res.status().as_u16() {
        //     200 => {
        //         info!("Mate saved successfully");
        //     }
        //     _ => {
        //         tracing::error!("Mate saving failed: {}", res.status());
        //         bail!("Mate saving failed: {}", res.status());
        //     }
        // }

        Ok(())
    }

    async fn token_expired(&self) -> anyhow::Result<bool> {
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
                bail!("No token available for authentication")
            }
        }
    }

    async fn update_token(&self) -> anyhow::Result<()> {
        match self.login_wallet().await {
            Ok(()) => {
                info!("Token updated successfully");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Token update failed: {}", e);
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
                bail!("No wallets available")
            }
        }
    }
}
