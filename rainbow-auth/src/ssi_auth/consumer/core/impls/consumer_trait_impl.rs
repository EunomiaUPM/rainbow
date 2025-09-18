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
use crate::ssi_auth::consumer::core::traits::consumer_trait::RainbowSSIAuthConsumerManagerTrait;
use crate::ssi_auth::consumer::core::Manager;
use crate::ssi_auth::errors::AuthErrors;
use crate::ssi_auth::types::entities::WhatEntity;
use crate::ssi_auth::types::gnap::RefBody;
use crate::ssi_auth::types::wallet::{MatchingVCs, RedirectResponse};
use crate::ssi_auth::utils::format::trim_4_base;
use anyhow::bail;
use axum::async_trait;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rainbow_common::auth::gnap::{AccessToken, GrantRequest, GrantResponse};
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_common::errors::helpers::{BadFormat, MissingAction};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_db::auth_consumer::entities::{
    auth_interaction, auth_request, auth_token_requirements, auth_verification, authority_request, mates,
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
        let client = self.config.get_pretty_client_config();
        let mut body = GrantRequest::default4oidc(client, "redirect".to_string(), Some(callback_uri));

        let new_request_model =
            auth_request::NewModel { id: id.clone(), provider_id, provider_slug, grant_endpoint: url.clone() };
        let mut request_model = match self.repo.request().create(new_request_model).await {
            Ok(model) => {
                info!("Authentication request saved successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let interact = body.interact.as_ref().unwrap(); // // EXPECTED ALWAYS

        let new_interaction_model = auth_interaction::NewModel {
            id: id.clone(),
            start: interact.start.clone(),
            method: interact.finish.method.clone(),
            uri: interact.finish.uri.as_ref().unwrap().clone(), // // EXPECTED ALWAYS
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
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
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
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
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
                let error = CommonErrors::petition_new(url, "POST".to_string(), http_code, e.to_string());
                error!("{}", error.log());
                bail!(error);
            }
        };

        let res: GrantResponse = match res.status().as_u16() {
            200 => {
                info!("Grant Response received successfully");
                res.json().await?
            }
            _ => {
                let http_code = Some(res.status().as_u16());
                let error_res: GrantResponse = res.json().await?;
                let error = CommonErrors::provider_new(
                    Some(url),
                    Some("POST".to_string()),
                    http_code,
                    error_res.error,
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let cont_data = match res.r#continue {
            Some(data) => data,
            None => {
                let error = CommonErrors::provider_new(
                    Some(url),
                    Some("POST".to_string()),
                    None,
                    Some("The expected 'continue' field was missing".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        request_model.status = "Pending".to_string();
        request_model.assigned_id = res.instance_id;

        let _ = match self.repo.request().update(request_model).await {
            Ok(model) => {
                info!("Authentication request updated successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let res_interact = match res.interact {
            Some(data) => data,
            None => {
                let error = CommonErrors::provider_new(
                    Some(url),
                    Some("POST".to_string()),
                    None,
                    Some("The expected 'interact' field was missing".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

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
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let uri = match res_interact.oidc4vp.as_ref() {
            Some(data) => data,
            None => {
                let error = CommonErrors::provider_new(
                    Some(url),
                    Some("POST".to_string()),
                    None,
                    Some("The expected 'oidc4vp' field was missing".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let fixed_uri = uri.replacen("openid4vp://", "https://", 1);
        let parsed_uri = Url::parse(&fixed_uri)?;

        let response_type =
            match parsed_uri.query_pairs().find(|(k, _)| k == "response_type").map(|(_, v)| v.into_owned()) {
                Some(data) => data,
                None => {
                    let error = CommonErrors::provider_new(
                        Some(url),
                        Some("POST".to_string()),
                        None,
                        Some("The expected 'response_type' field was missing in the oidc4vp uri".to_string()),
                    );
                    error!("{}", error.log());
                    bail!(error);
                }
            };

        let client_id = match parsed_uri.query_pairs().find(|(k, _)| k == "client_id").map(|(_, v)| v.into_owned()) {
            Some(data) => data,
            None => {
                let error = CommonErrors::provider_new(
                    Some(url),
                    Some("POST".to_string()),
                    None,
                    Some("The expected 'client_id' field was missing in the oidc4vp uri".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let response_mode =
            match parsed_uri.query_pairs().find(|(k, _)| k == "response_mode").map(|(_, v)| v.into_owned()) {
                Some(data) => data,
                None => {
                    let error = CommonErrors::provider_new(
                        Some(url),
                        Some("POST".to_string()),
                        None,
                        Some("The expected 'response_mode' field was missing in the oidc4vp uri".to_string()),
                    );
                    error!("{}", error.log());
                    bail!(error);
                }
            };

        let pd_uri = match parsed_uri
            .query_pairs()
            .find(|(k, _)| k == "presentation_definition_uri")
            .map(|(_, v)| v.into_owned())
        {
            Some(data) => data,
            None => {
                let error = CommonErrors::provider_new(
                    Some(url),
                    Some("POST".to_string()),
                    None,
                    Some("The expected 'presentation_definition_uri' field was missing in the oidc4vp uri".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let client_id_scheme =
            match parsed_uri.query_pairs().find(|(k, _)| k == "client_id_scheme").map(|(_, v)| v.into_owned()) {
                Some(data) => data,
                None => {
                    let error = CommonErrors::provider_new(
                        Some(url),
                        Some("POST".to_string()),
                        None,
                        Some("The expected 'client_id_scheme' field was missing in the oidc4vp uri".to_string()),
                    );
                    error!("{}", error.log());
                    bail!(error);
                }
            };

        let nonce = match parsed_uri.query_pairs().find(|(k, _)| k == "nonce").map(|(_, v)| v.into_owned()) {
            Some(data) => data,
            None => {
                let error = CommonErrors::provider_new(
                    Some(url),
                    Some("POST".to_string()),
                    None,
                    Some("The expected 'nonce' field was missing in the oidc4vp uri".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let response_uri =
            match parsed_uri.query_pairs().find(|(k, _)| k == "response_uri").map(|(_, v)| v.into_owned()) {
                Some(data) => data,
                None => {
                    let error = CommonErrors::provider_new(
                        Some(url),
                        Some("POST".to_string()),
                        None,
                        Some("The expected 'response_uri' field was missing in the oidc4vp uri".to_string()),
                    );
                    error!("{}", error.log());
                    bail!(error);
                }
            };

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

        let verification_model = match self.repo.verification().create(new_verification_model).await {
            Ok(model) => {
                info!("Verification data saved successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
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
                let error = CommonErrors::missing_resource_new(
                    id.clone(),
                    Some(format!("There is no process with id: {}", &id)),
                );
                error!("{}", error.log());
                bail!(error);
            }
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        interaction_model.interact_ref = Some(interact_ref);
        interaction_model.hash = Some(hash.clone());

        let upd_interaction_model = match self.repo.interaction().update(interaction_model).await {
            Ok(model) => model,
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        // let hash_method = upd_interaction_model.hash_method; // TODO
        let hash_input = format!(
            "{}\n{}\n{}\n{}",
            upd_interaction_model.client_nonce,
            upd_interaction_model.as_nonce.unwrap(),     // EXPECTED ALWAYS
            upd_interaction_model.interact_ref.unwrap(), // EXPECTED ALWAYS
            upd_interaction_model.grant_endpoint
        );

        let mut hasher = Sha256::new(); // TODO
        hasher.update(hash_input.as_bytes());
        let result = hasher.finalize();

        let calculated_hash = URL_SAFE_NO_PAD.encode(result);

        if calculated_hash != hash {
            let error = AuthErrors::security_new(Some("Hash does not match the calculated one".to_string()));
            error!("{}", error.log());
            bail!(error);
        }

        info!("Hash matches the calculated one");
        Ok(())
    }

    async fn continue_request(&self, id: String, interact_ref: String) -> anyhow::Result<Value> {
        // TODO WAIT 5 SECONDS

        let (who, request_model, authority_model) = self.who_is_it(id.clone()).await?;

        let interact_model = match self.repo.interaction().get_by_id(id.as_str()).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = CommonErrors::missing_resource_new(
                    id.clone(),
                    Some(format!("There is no process with id: {}", &id)),
                );
                error!("{}", error.log());
                bail!(error);
            }
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        match who {
            WhatEntity::Provider => {
                info!("Continuing provider request");

                let url = interact_model.continue_endpoint.unwrap(); // EXPECTED ALWAYS
                let token = format!("GNAP {}", interact_model.continue_token.unwrap()); // EXPECTED ALWAYS

                let mut headers = HeaderMap::new();
                headers.insert(CONTENT_TYPE, "application/json".parse()?);
                headers.insert(ACCEPT, "application/json".parse()?);
                headers.insert(AUTHORIZATION, token.parse()?);

                let body = RefBody { interact_ref };

                let res = self.client.post(&url).headers(headers).json(&body).send().await;

                let res = match res {
                    Ok(res) => res,
                    Err(e) => {
                        let http_code = match e.status() {
                            Some(code) => Some(code.as_u16()),
                            None => None,
                        };
                        let error = CommonErrors::petition_new(url, "POST".to_string(), http_code, e.to_string());
                        error!("{}", error.log());
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
                        let error = CommonErrors::provider_new(
                            Some(url),
                            Some("POST".to_string()),
                            http_code,
                            error_res.error,
                        );
                        error!("{}", error.log());
                        bail!(error);
                    }
                };

                let mut request_model = request_model.unwrap(); // EXPECTED ALWAYS
                request_model.status = "Approved".to_string();
                request_model.token = Some(res.value); // TODO Save al token data (pending)

                let upd_request_model = match self.repo.request().update(request_model).await {
                    Ok(model) => model,
                    Err(e) => {
                        let error = CommonErrors::database_new(Some(e.to_string()));
                        error!("{}", error.log());
                        bail!(error);
                    }
                };

                let base_url = trim_4_base(upd_request_model.grant_endpoint.as_str());
                let mate = mates::NewModel {
                    participant_id: upd_request_model.provider_id,
                    participant_slug: upd_request_model.provider_slug,
                    participant_type: "Provider".to_string(),
                    base_url,
                    token: upd_request_model.token,
                    is_me: false,
                };

                let mate = serde_json::to_value(self.save_mate(mate).await?)?;

                Ok(mate)
            }
            WhatEntity::Authority => {
                info!("Continuing authority request");

                let url = interact_model.continue_endpoint.unwrap(); // EXPECTED ALWAYS
                let token = format!("GNAP {}", interact_model.continue_token.unwrap()); // EXPECTED ALWAYS

                let mut headers = HeaderMap::new();
                headers.insert(CONTENT_TYPE, "application/json".parse()?);
                headers.insert(ACCEPT, "application/json".parse()?);
                headers.insert(AUTHORIZATION, token.parse()?);

                let body = RefBody { interact_ref };

                let res = self.client.post(&url).headers(headers).json(&body).send().await;

                let res = match res {
                    Ok(res) => res,
                    Err(e) => {
                        let http_code = match e.status() {
                            Some(code) => Some(code.as_u16()),
                            None => None,
                        };
                        let error = CommonErrors::petition_new(url, "POST".to_string(), http_code, e.to_string());
                        error!("{}", error.log());
                        bail!(error);
                    }
                };

                // TODO Is it worth putting "processing" as state??

                let res = match res.status().as_u16() {
                    200 => {
                        info!("Success retrieving the vc_uri");
                        res.text().await?
                    }
                    _ => {
                        let http_code = Some(res.status().as_u16());
                        let error_res: GrantResponse = res.json().await?;
                        let error = CommonErrors::authority_new(
                            Some(url),
                            Some("POST".to_string()),
                            http_code,
                            error_res.error,
                        );
                        error!("{}", error.log());
                        bail!(error);
                    }
                };

                info!("{}", res);
                let mut authority_model = authority_model.unwrap();
                authority_model.vc_uri = Some(res);
                authority_model.status = "Approved".to_string();

                let upd_request_model = match self.repo.authority().update(authority_model).await {
                    Ok(model) => model,
                    Err(e) => {
                        let error = CommonErrors::database_new(Some(e.to_string()));
                        error!("{}", error.log());
                        bail!(error);
                    }
                };

                let base_url = trim_4_base(upd_request_model.grant_endpoint.as_str());
                let mate = mates::NewModel {
                    participant_id: upd_request_model.authority_id,
                    participant_slug: upd_request_model.authority_slug,
                    participant_type: "Authority".to_string(),
                    base_url,
                    token: None,
                    is_me: false,
                };

                let mate = serde_json::to_value(self.save_mate(mate).await?)?;

                Ok(mate)
            }
        }
    }

    async fn save_mate(&self, mate: mates::NewModel) -> anyhow::Result<mates::Model> {
        match self.repo.mates().force_create(mate).await {
            Ok(model) => Ok(model),
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn beg_credential(&self, authority_id: String, authority_slug: String, url: String) -> anyhow::Result<()> {
        info!("Begging for a credential");
        let client = self.config.get_pretty_client_config();
        let id = uuid::Uuid::new_v4().to_string();
        let callback_uri = format!(
            "{}/api/v1/callback/{}",
            self.config.get_auth_host_url().unwrap(),
            &id
        );
        let mut grant_request = GrantRequest::default4cross_user(client, Some(callback_uri.clone()));

        let new_authority_request_model =
            authority_request::NewModel { id: id.clone(), authority_id, authority_slug, grant_endpoint: url.clone() };

        let mut authority_model = match self.repo.authority().create(new_authority_request_model).await {
            Ok(model) => model,
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let new_interact_model = auth_interaction::NewModel {
            id,
            start: vec!["await".to_string()],
            method: "push".to_string(),
            uri: callback_uri,
            hash_method: None,
            hints: None,
            grant_endpoint: url.clone(),
        };

        let mut interact_model = match self.repo.interaction().create(new_interact_model).await {
            Ok(model) => model,
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        info!("Sending Grant Petition to Authority");

        grant_request.update_nonce(interact_model.client_nonce.clone());

        let res = self.client.post(&url).headers(headers).json(&grant_request).send().await;

        let res = match res {
            Ok(res) => res,
            Err(e) => {
                let http_code = match e.status() {
                    Some(status) => Some(status.as_u16()),
                    None => None,
                };
                let error = CommonErrors::petition_new(url, "POST".to_string(), http_code, e.to_string());
                error!("{}", error.log());
                bail!(error);
            }
        };

        let res: GrantResponse = match res.status().as_u16() {
            200 => {
                info!("Grant Response received successfully");
                res.json().await?
            }
            _ => {
                let http_code = Some(res.status().as_u16());
                let error_res: GrantResponse = res.json().await?;
                let error = CommonErrors::authority_new(
                    Some(url),
                    Some("POST".to_string()),
                    http_code,
                    error_res.error,
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        authority_model.status = "Pending".to_string();
        authority_model.assigned_id = res.instance_id;

        let _ = match self.repo.authority().update(authority_model).await {
            Ok(model) => {
                info!("Authority request updated successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let res_interact = match res.interact {
            Some(data) => data,
            None => {
                let error = CommonErrors::format_new(
                    BadFormat::Sent,
                    Some("Missing field interact in the response".to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        let cont_data = match res.r#continue {
            Some(data) => data,
            None => {
                let error = CommonErrors::format_new(
                    BadFormat::Sent,
                    Some("Missing field continue in the response".to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        interact_model.as_nonce = res_interact.finish;
        interact_model.continue_token = Some(cont_data.access_token.value);
        interact_model.continue_endpoint = Some(cont_data.uri);
        interact_model.continue_wait = cont_data.wait;

        let _ = match self.repo.interaction().update(interact_model).await {
            Ok(model) => {
                info!("Authority interaction updated successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        Ok(())
    }

    async fn who_is_it(
        &self,
        id: String,
    ) -> anyhow::Result<(
        WhatEntity,
        Option<auth_request::Model>,
        Option<authority_request::Model>,
    )> {
        // TODO WAIT 5 SECONDS
        info!("Continuing request");

        match self.repo.request().get_by_id(id.as_str()).await {
            Ok(Some(model)) => {
                info!("It is a request 4 the provider");
                return Ok((WhatEntity::Provider, Some(model), None));
            }
            Ok(None) => info!("It is not a request 4 the provider"),
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        match self.repo.authority().get_by_id(id.as_str()).await {
            Ok(Some(model)) => {
                info!("It is a request 4 an authority");
                return Ok((WhatEntity::Authority, None, Some(model)));
            }
            Ok(None) => info!("It is not a request 4 an authority"),
            Err(e) => {
                let error = CommonErrors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let error = CommonErrors::missing_resource_new(
            id.clone(),
            Some(format!("Missing resource with id: {}", &id)),
        );
        error!("{}", error.log());
        bail!(error)
    }

    // EXTRAS ------------------------------------------------------------------------------------->

    async fn join_exchange(&self, exchange_url: String) -> anyhow::Result<String> {
        let wallet_session = self.wallet_session.lock().await;

        let wallet = match wallet_session.wallets.first() {
            Some(w) => w,
            None => {
                let error = CommonErrors::missing_action_new(
                    "There is no wallet associated to this session".to_string(),
                    MissingAction::Wallet,
                    Some("There is no wallet to retrieve dids from".to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        let url = format!(
            "{}/wallet-api/wallet/{}/exchange/resolvePresentationRequest",
            self.config.get_wallet_portal_url(),
            &wallet.id
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
