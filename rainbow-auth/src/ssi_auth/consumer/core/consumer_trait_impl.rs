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
use crate::ssi_auth::consumer::core::consumer_trait::RainbowSSIAuthConsumerManagerTrait;
use crate::ssi_auth::errors::AuthErrors;
use crate::ssi_auth::types::{MatchingVCs, RedirectResponse};
use anyhow::bail;
use axum::async_trait;
use axum::http::StatusCode;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rainbow_common::auth::gnap::{AccessToken, GrantRequest, GrantResponse};
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_common::errors::{CommonErrors, ErrorInfo};
use rainbow_db::auth_consumer::entities::{
    auth_interaction, auth_request, auth_token_requirements, auth_verification, mates,
};
use rainbow_db::auth_consumer::repo_factory::factory_trait::AuthRepoFactoryTrait;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tracing::{error, info};
use url::Url;
use urlencoding::decode;

#[async_trait]
impl<T> RainbowSSIAuthConsumerManagerTrait for Manager<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    async fn request_onboard_provider(
        &self,
        url: String,
        provider_id: String,
        provider_slug: String,
    ) -> anyhow::Result<String> {
        info!("Requesting access to the provider");
        let id = uuid::Uuid::new_v4().to_string();
        let callback_uri = format!(
            "{}/api/v1/callback/{}",
            self.config.get_auth_host_url().unwrap(),
            &id
        );
        let client = serde_json::to_value(self.config.get_raw_client_config())?;
        let mut body = GrantRequest::default4oidc(client, "redirect".to_string(), Some(callback_uri));

        let new_request_model =
            auth_request::NewModel { id: id.clone(), provider_id, provider_slug, grant_endpoint: url.clone() };
        let mut request_model = match self.repo.request().create(new_request_model).await {
            Ok(model) => {
                info!("Authentication request saved successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: "Error saving the authentication request into the database".to_string(),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        let interact = body.interact.as_ref().unwrap();

        let new_interaction_model = auth_interaction::NewModel {
            id: id.clone(),
            start: interact.start.clone(),
            method: interact.finish.method.clone(),
            uri: interact.finish.uri.as_ref().unwrap().clone(),
            hash_method: interact.finish.hash_method.clone(),
            hints: interact.hints.clone(),
            grant_endpoint: url.clone(),
        };

        let mut interaction_model = match self.repo.interaction().create(new_interaction_model).await {
            Ok(model) => {
                info!("Authentication interaction saved successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: "Error saving the authentication interaction into the database".to_string(),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        let new_token_req_model = auth_token_requirements::Model {
            id: id.clone(),
            r#type: "provider-api".to_string(),
            actions: vec!["talk".to_string()],
            locations: None,
            datatypes: None,
            identifier: None,
            privileges: None,
            label: None,
            flags: None,
        };

        let _ = match self.repo.token_requirements().create(new_token_req_model).await {
            Ok(model) => {
                info!("Token requirements saved successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: "Error saving the token requirements into the database".to_string(),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        info!("Sending Grant Petition to Provider");

        body.update_nonce(interaction_model.client_nonce.clone());

        let res = self.client.post(&url).headers(headers).json(&body).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => {
                let http_code = match e.status() {
                    Some(status) => Some(status.as_u16()),
                    None => None,
                };
                let error = CommonErrors::PetitionError {
                    info: ErrorInfo {
                        message: "Error contacting the provider".to_string(),
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

        let mut res: GrantResponse = match res.status().as_u16() {
            201 => {
                info!("Grant Response received successfully");
                res.json().await?
            }
            _ => {
                let http_code = Some(res.status().as_u16());
                let error_res: GrantResponse = res.json().await?;
                let error = AuthErrors::ProviderError {
                    info: ErrorInfo { message: "Grant response failed".to_string(), error_code: 1400, details: None },
                    http_code,
                    url,
                    method: "POST".to_string(),
                    cause: error_res.error,
                };

                error.log();
                bail!(error);
            }
        };

        let cont_data = res.r#continue.unwrap();
        request_model.status = "Pending".to_string();
        request_model.assigned_id = res.instance_id;

        let _ = match self.repo.request().update(request_model).await {
            Ok(model) => {
                info!("Authentication request updated successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: "Error updating the authentication request into the database".to_string(),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        let res_interact = res.interact.unwrap();

        interaction_model.as_nonce = res_interact.finish;
        interaction_model.continue_token = Some(cont_data.access_token.value);
        interaction_model.continue_endpoint = Some(cont_data.uri);
        interaction_model.continue_wait = cont_data.wait;

        let _ = match self.repo.interaction().update(interaction_model).await {
            Ok(model) => {
                info!("Interaction information updated successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: "Error updating the interaction information into the database".to_string(),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        let uri = res_interact.oidc4vp.as_ref().unwrap();
        let fixed_uri = uri.replacen("openid4vp://", "https://", 1);
        let parsed_uri = Url::parse(&fixed_uri)?;

        let response_type =
            parsed_uri.query_pairs().find(|(k, _)| k == "response_type").map(|(_, v)| v.into_owned()).unwrap();
        let client_id = parsed_uri.query_pairs().find(|(k, _)| k == "client_id").map(|(_, v)| v.into_owned()).unwrap();
        let response_mode =
            parsed_uri.query_pairs().find(|(k, _)| k == "response_mode").map(|(_, v)| v.into_owned()).unwrap();
        let pd_uri = parsed_uri
            .query_pairs()
            .find(|(k, _)| k == "presentation_definition_uri")
            .map(|(_, v)| v.into_owned())
            .unwrap();
        let client_id_scheme =
            parsed_uri.query_pairs().find(|(k, _)| k == "client_id_scheme").map(|(_, v)| v.into_owned()).unwrap();
        let nonce = parsed_uri.query_pairs().find(|(k, _)| k == "nonce").map(|(_, v)| v.into_owned()).unwrap();
        let response_uri =
            parsed_uri.query_pairs().find(|(k, _)| k == "response_uri").map(|(_, v)| v.into_owned()).unwrap();

        let new_verification_model = auth_verification::NewModel {
            id: id.clone(),
            uri: uri.clone(),
            scheme: "openid4vp".to_string(),
            response_type,
            client_id,
            response_mode,
            pd_uri,
            client_id_scheme,
            nonce,
            response_uri,
        };

        let mut verification_model = match self.repo.verification().create(new_verification_model).await {
            Ok(model) => {
                info!("Verification data saved successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: "Error saving the verification data into the database".to_string(),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        Ok(verification_model.uri.clone())
    }

    async fn check_callback(&self, id: String, interact_ref: String, hash: String) -> anyhow::Result<()> {
        info!("Checking callback");
        let mut interaction_model = match self.repo.interaction().get_by_id(&id).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = CommonErrors::MissingError {
                    info: ErrorInfo {
                        message: format!("There is no process with id: {}", &id),
                        error_code: 1600,
                        details: None,
                    },
                    id: id.clone(),
                    cause: None,
                };
                error.log();
                bail!(error);
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: format!("Error retrieving the process with id: {}", &id),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        interaction_model.interact_ref = Some(interact_ref);
        interaction_model.hash = Some(hash.clone());

        let upd_interaction_model = match self.repo.interaction().update(interaction_model).await {
            Ok(model) => model,
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: format!("Error updating the process with id: {}", &id),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        // let hash_method = upd_interaction_model.hash_method; // TODO
        let hash_input = format!(
            "{}\n{}\n{}\n{}",
            upd_interaction_model.client_nonce,
            upd_interaction_model.as_nonce.unwrap(),
            upd_interaction_model.interact_ref.unwrap(),
            upd_interaction_model.grant_endpoint
        );

        let mut hasher = Sha256::new(); // TODO
        hasher.update(hash_input.as_bytes());
        let result = hasher.finalize();

        let calculated_hash = URL_SAFE_NO_PAD.encode(result);

        println!("{}", calculated_hash);
        println!("{}", hash);

        if calculated_hash != hash {
            let error = CommonErrors::InvalidError {
                info: ErrorInfo { message: "Invalid hash".to_string(), error_code: 1700, details: None },
                cause: Some("Hash does not match the calculated one".to_string()),
            };
            error.log();
            bail!(error);
        }

        info!("Hash matches the calculated one");
        Ok(())
    }

    async fn continue_request(&self, id: String, interact_ref: String) -> anyhow::Result<auth_request::Model> {
        // TODO WAIT 5 SECONDS
        info!("Continuing request");

        let mut request_model = match self.repo.request().get_by_id(id.as_str()).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = CommonErrors::MissingError {
                    info: ErrorInfo {
                        message: format!("There is no process with id: {}", &id),
                        error_code: 1600,
                        details: None,
                    },
                    id: id.clone(),
                    cause: None,
                };
                error.log();
                bail!(error);
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: format!("Error retrieving the process with id: {}", &id),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        let interact_model = match self.repo.interaction().get_by_id(id.as_str()).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = CommonErrors::MissingError {
                    info: ErrorInfo {
                        message: format!("There is no process with id: {}", &id),
                        error_code: 1600,
                        details: None,
                    },
                    id: id.clone(),
                    cause: None,
                };
                error.log();
                bail!(error);
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: format!("Error retrieving the process with id: {}", &id),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        let url = interact_model.continue_endpoint.unwrap();
        let token = format!("GNAP {}", interact_model.continue_token.unwrap());

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, token.parse()?);

        let body = json!({
            "interact_ref": interact_ref
        });

        let res = self.client.post(&url).headers(headers).json(&body).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => {
                let http_code = match e.status() {
                    Some(code) => Some(code.as_u16()),
                    None => None,
                };
                let error = CommonErrors::PetitionError {
                    info: ErrorInfo {
                        message: "Error contacting the Provider".to_string(),
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

        // TODO Is it worth putting "processing" as state??

        let res: AccessToken = match res.status().as_u16() {
            200 => {
                info!("Success retrieving the token");
                res.json().await?
            }
            _ => {
                let http_code = Some(res.status().as_u16());
                let error_res: GrantResponse = res.json().await?;
                let error = AuthErrors::ProviderError {
                    info: ErrorInfo { message: "Continue request failed".to_string(), error_code: 1400, details: None },
                    http_code,
                    url,
                    method: "POST".to_string(),
                    cause: error_res.error,
                };
                error.log();
                bail!(error);
            }
        };

        request_model.status = "Approved".to_string();
        request_model.token = Some(res.value); // TODO Save al token data (pending)

        let upd_request_model = match self.repo.request().update(request_model).await {
            Ok(model) => model,
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: format!("Error updating process with id: {}", &id),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        Ok(upd_request_model)
    }

    async fn save_mate(&self, mate: mates::NewModel) -> anyhow::Result<mates::Model> {
        match self.repo.mates().create(mate).await {
            Ok(model) => Ok(model),
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo { message: "Error saving mate".to_string(), error_code: 1300, details: None },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        }
    }

    // EXTRAS ------------------------------------------------------------------------------------->

    async fn join_exchange(&self, exchange_url: String) -> anyhow::Result<String> {
        let wallet_session = self.wallet_session.lock().await;
        if wallet_session.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };

        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/resolvePresentationRequest",
            self.config.get_wallet_portal_url(),
            wallet_session.wallets.first().unwrap().id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/plain".parse()?);
        headers.insert(ACCEPT, "text/plain".parse()?);

        match &wallet_session.token {
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
                    let log = format!("Error parsing the credential -> {}", err.to_string());
                    error!(log);
                    bail!("Error parsing the credential")
                }
            }
        } else {
            error!("Invalid Presentation Definition");
            bail!("Invalid Presentation Definition")
        }
    }

    async fn match_vc4vp(&self, vp_def: Value) -> anyhow::Result<Vec<MatchingVCs>> {
        let wallet_session = self.wallet_session.lock().await;
        if wallet_session.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };
        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/matchCredentialsForPresentationDefinition",
            self.config.get_wallet_portal_url(),
            wallet_session.wallets.first().unwrap().id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &wallet_session.token {
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
        let wallet_session = self.wallet_session.lock().await;
        if wallet_session.wallets.first().is_none() {
            bail!("There is not a wallet registered")
        };
        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/usePresentationRequest",
            self.config.get_wallet_portal_url(),
            wallet_session.wallets.first().unwrap().id
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        match &wallet_session.token {
            Some(token) => headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?),
            None => bail!("No token available for authentication"),
        };

        // TODO Check
        // let did = &wallet_session.wallets.first().unwrap().dids.as_ref().unwrap().first().unwrap().did;
        let did = wallet_session.wallets.first().unwrap().dids.as_ref().unwrap().first().unwrap().did.clone();

        let body = json!({ "did": did, "presentationRequest": preq, "selectedCredentials": creds });

        let res = self.client.post(url).headers(headers).json(&body).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => bail!("Error sending request: {}", e),
        };

        let body: RedirectResponse = res.json().await?;

        Ok(body)
    }
}
