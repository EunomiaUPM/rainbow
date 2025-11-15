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
use super::super::OnboarderTrait;
use crate::ssi::common::errors::AuthErrors;
use crate::ssi::common::services::client::ClientServiceTrait;
use crate::ssi::common::types::enums::request::Body;
use crate::ssi::common::types::gnap::grant_request::{AccessTokenRequirements4GR, Finish4Interact, Interact4GR};
use crate::ssi::common::types::gnap::{AccessToken, CallbackBody, GrantRequest, GrantResponse, RefBody};
use crate::ssi::common::utils::get_query_param;
use crate::ssi::common::utils::trim_4_base;
use crate::ssi::consumer::services::onboarder::gnap::config::{GnapOnboarderConfig, GnapOnboarderConfigTrait};
use crate::ssi::consumer::types::ReachProvider;
use anyhow::bail;
use axum::async_trait;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use jsonwebtoken::TokenData;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::utils::get_from_opt;
use rainbow_db::auth::common::entities::{mates, req_interaction, req_verification, token_requirements};
use rainbow_db::auth::consumer::entities::req_request;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tracing::{error, info};
use url::Url;

pub struct GnapOnboarderService {
    client: Arc<dyn ClientServiceTrait>,
    config: GnapOnboarderConfig,
}

impl GnapOnboarderService {
    pub fn new(client: Arc<dyn ClientServiceTrait>, config: GnapOnboarderConfig) -> GnapOnboarderService {
        GnapOnboarderService { client, config }
    }
}

#[async_trait]
impl OnboarderTrait for GnapOnboarderService {
    fn start(
        &self,
        payload: &ReachProvider,
    ) -> (
        req_request::NewModel,
        req_interaction::NewModel,
        token_requirements::Model,
    ) {
        info!("Starting process to request consumer onboarding");

        let id = uuid::Uuid::new_v4().to_string();
        let callback_uri = format!("{}/api/v1/onboard/callback/{}", self.config.get_host(), &id);

        let req_model = req_request::NewModel {
            id: id.clone(),
            provider_id: payload.id.clone(),
            provider_slug: payload.slug.clone(),
            grant_endpoint: payload.url.clone(),
        };

        let int_model = req_interaction::NewModel {
            id: id.clone(),
            start: vec!["oidc4vp".to_string()],
            method: "push".to_string(),
            uri: callback_uri.clone(),
            hash_method: Some("sha-256".to_string()),
            hints: None,
            grant_endpoint: payload.url.clone(),
        };

        let token_model = token_requirements::Model {
            id,
            r#type: "provider-api".to_string(),
            actions: vec!["talk".to_string()],
            locations: None,
            datatypes: None,
            identifier: None,
            privileges: None,
            label: None,
            flags: None,
        };

        (req_model, int_model, token_model)
    }

    async fn send_req(
        &self,
        req_model: &mut req_request::Model,
        int_model: &mut req_interaction::Model,
    ) -> anyhow::Result<()> {
        info!("Sending onboarding request");

        let client = self.config.get_pretty_client_config()?;
        let grant_request = GrantRequest::prov_oidc(client, int_model);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self
            .client
            .post(
                &int_model.grant_endpoint,
                Some(headers),
                Body::Json(serde_json::to_value(grant_request)?),
            )
            .await?;

        let res: GrantResponse = match res.status().as_u16() {
            200 => {
                info!("Grant Response received successfully");
                res.json().await?
            }
            _ => {
                let http_code = Some(res.status().as_u16());
                let error_res: GrantResponse = res.json().await?;
                let error = CommonErrors::provider_new(
                    &int_model.grant_endpoint,
                    "POST",
                    http_code,
                    &error_res.error.unwrap(),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        req_model.status = "Pending".to_string();
        req_model.assigned_id = res.instance_id;

        let interact = get_from_opt(&res.interact, "interact")?;
        let cont_data = get_from_opt(&res.r#continue, "continue")?;

        int_model.as_nonce = interact.finish;
        int_model.oidc_vp_uri = interact.oidc4vp;
        int_model.continue_token = Some(cont_data.access_token.value);
        int_model.continue_endpoint = Some(cont_data.uri);
        int_model.continue_wait = cont_data.wait;
        Ok(())
    }

    fn save_verification(&self, int_model: &req_interaction::Model) -> anyhow::Result<req_verification::NewModel> {
        info!("Saving verification data");

        let uri = get_from_opt(&int_model.oidc_vp_uri, "oidc4vp")?;
        let fixed_uri = uri.replacen("openid4vp://", "https://", 1);
        let parsed_uri = Url::parse(&fixed_uri)?;

        let response_type = get_query_param(&parsed_uri, "response_type")?;
        let client_id = get_query_param(&parsed_uri, "client_id")?;
        let response_mode = get_query_param(&parsed_uri, "response_mode")?;
        let pd_uri = get_query_param(&parsed_uri, "presentation_definition_uri")?;
        let client_id_scheme = get_query_param(&parsed_uri, "client_id_scheme")?;
        let nonce = get_query_param(&parsed_uri, "nonce")?;
        let response_uri = get_query_param(&parsed_uri, "response_uri")?;

        Ok(req_verification::NewModel {
            id: int_model.id.clone(),
            uri,
            scheme: "openid4vp".to_string(),
            response_type,
            client_id,
            response_mode,
            pd_uri,
            client_id_scheme,
            nonce,
            response_uri,
        })
    }

    fn check_callback(&self, int_model: &mut req_interaction::Model, payload: &CallbackBody) -> anyhow::Result<()> {
        info!("Checking callback");

        int_model.interact_ref = Some(payload.interact_ref.clone());
        int_model.hash = Some(payload.hash.clone());
        let nonce = get_from_opt(&int_model.as_nonce, "as_nonce")?;
        let interact_ref = get_from_opt(&int_model.interact_ref, "interact_ref")?;
        let hash_input = format!(
            "{}\n{}\n{}\n{}",
            int_model.client_nonce, nonce, interact_ref, int_model.grant_endpoint
        );

        let mut hasher = Sha256::new(); // TODO
        hasher.update(hash_input.as_bytes());
        let result = hasher.finalize();

        let calculated_hash = URL_SAFE_NO_PAD.encode(result);

        let hash = get_from_opt(&int_model.hash, "hash")?;
        if calculated_hash != hash {
            let error = AuthErrors::security_new("Hash does not match the calculated one");
            error!("{}", error.log());
            bail!(error);
        }

        info!("Hash matches the calculated one");
        Ok(())
    }

    async fn continue_req(&self, int_model: &req_interaction::Model) -> anyhow::Result<AccessToken> {
        info!("Continuing request");

        let url = get_from_opt(&int_model.continue_endpoint, "continue-endpoint")?;
        let base_token = get_from_opt(&int_model.continue_token, "continue token")?;
        let token = format!("GNAP {}", base_token);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);
        headers.insert(AUTHORIZATION, token.parse()?);

        let interact_ref = get_from_opt(&int_model.interact_ref, "interact_ref")?;
        let body = RefBody { interact_ref };

        let res = self.client.post(&url, Some(headers), Body::Json(serde_json::to_value(body)?)).await?;

        match res.status().as_u16() {
            200 => {
                info!("Success retrieving the token");
                let token: AccessToken = res.json().await?;
                Ok(token)
            }
            _ => {
                let http_code = Some(res.status().as_u16());
                let error_res: GrantResponse = res.json().await?;
                let error = CommonErrors::provider_new(&url, "POST", http_code, &error_res.error.unwrap());
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    fn end_req(&self, req_model: &mut req_request::Model, token: &AccessToken) -> mates::NewModel {
        info!("Ending request");

        req_model.status = "Approved".to_string();
        req_model.token = Some(token.value.clone());

        let base_url = trim_4_base(&req_model.grant_endpoint);
        mates::NewModel {
            participant_id: req_model.provider_id.clone(),
            participant_slug: req_model.provider_slug.clone(),
            participant_type: "Provider".to_string(),
            base_url,
            token: req_model.token.clone(),
            is_me: false,
        }
    }
}
