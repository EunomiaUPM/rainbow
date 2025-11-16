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

use super::super::VcRequesterTrait;
use super::config::{VCRequesterConfig, VCRequesterConfigTrait};
use crate::ssi::common::services::client::ClientServiceTrait;
use crate::ssi::common::types::entities::{ReachAuthority, ReachMethod};
use crate::ssi::common::types::enums::request::Body;
use crate::ssi::common::types::gnap::{GrantRequest, GrantResponse};
use crate::ssi::common::utils::{get_query_param, trim_4_base};
use anyhow::bail;
use axum::async_trait;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::utils::get_from_opt;
use rainbow_db::auth::common::entities::{mates, req_interaction, req_vc, req_verification};
use rainbow_db::auth::consumer::entities::req_request;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Response;
use std::sync::Arc;
use tracing::{error, info};
use url::Url;

pub struct VCReqService {
    client: Arc<dyn ClientServiceTrait>,
    config: VCRequesterConfig,
}

impl VCReqService {
    pub fn new(client: Arc<dyn ClientServiceTrait>, config: VCRequesterConfig) -> Self {
        VCReqService { client, config }
    }
}

#[async_trait]
impl VcRequesterTrait for VCReqService {
    fn start(&self, payload: ReachAuthority) -> (req_vc::NewModel, req_interaction::NewModel) {
        info!("Begging for a credential");

        let id = uuid::Uuid::new_v4().to_string();
        let callback_uri = format!(
            "{}/api/v1/vc-request/callback/{}",
            self.config.get_host(),
            &id
        );

        let vc_model = req_vc::NewModel {
            id: id.clone(),
            authority_id: payload.id.clone(),
            authority_slug: payload.slug.clone(),
            grant_endpoint: payload.url.clone(),
            vc_type: payload.vc_type.clone(),
        };

        let int_model = req_interaction::NewModel {
            id,
            start: vec!["await".to_string()],
            method: "push".to_string(),
            uri: callback_uri,
            hash_method: None,
            hints: None,
            grant_endpoint: payload.url.clone(),
        };

        (vc_model, int_model)
    }

    async fn send_req(
        &self,
        vc_model: &mut req_vc::Model,
        int_model: &mut req_interaction::Model,
        method: ReachMethod,
    ) -> anyhow::Result<Option<String>> {
        info!("Sending grant request request to authority");

        let client = self.config.get_pretty_client_config()?;

        let grant_request = match method {
            ReachMethod::Oidc => GrantRequest::vc_oidc(
                client,
                int_model.method.clone(),
                Some(int_model.uri.clone()),
                vc_model.vc_type.clone(),
                Some(int_model.client_nonce.clone()),
            ),
            ReachMethod::CrossUser => GrantRequest::vc_cross_user(
                client,
                Some(int_model.uri.clone()),
                vc_model.vc_type.clone(),
                Some(int_model.client_nonce.clone()),
            ),
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self
            .client
            .post(
                &vc_model.grant_endpoint,
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
                let error = CommonErrors::authority_new(
                    &vc_model.grant_endpoint,
                    "POST",
                    http_code,
                    &error_res.error.unwrap_or("Unknown error".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        vc_model.status = "Pending".to_string();
        vc_model.assigned_id = res.instance_id;

        let res_interact = get_from_opt(&res.interact, "interact")?;
        let cont_data = get_from_opt(&res.r#continue, "continue")?;

        int_model.as_nonce = res_interact.finish;
        int_model.continue_token = Some(cont_data.access_token.value);
        int_model.continue_endpoint = Some(cont_data.uri);
        int_model.continue_wait = cont_data.wait;

        Ok(res_interact.oidc4vp)
    }

    fn save_ver_data(&self, uri: &str, id: &str) -> anyhow::Result<req_verification::NewModel> {
        info!("Saving verification data");

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
            id: id.to_string(),
            uri: uri.to_string(),
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

    async fn manage_res(&self, vc_req_model: &mut req_vc::Model, res: Response) -> anyhow::Result<mates::NewModel> {
        info!("Managing response");
        let res = match res.status().as_u16() {
            200 => {
                info!("Success retrieving the vc_uri");
                res.text().await?
            }
            _ => {
                let http_code = Some(res.status().as_u16());
                let error_res: GrantResponse = res.json().await?;
                let error = CommonErrors::authority_new(
                    "authority/continue",
                    "POST",
                    http_code,
                    &error_res.error.unwrap_or("Error with provider continue request".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        vc_req_model.vc_uri = Some(res);
        vc_req_model.status = "Approved".to_string();

        let base_url = trim_4_base(&vc_req_model.grant_endpoint);
        let mate = mates::NewModel {
            participant_id: vc_req_model.authority_id.clone(),
            participant_slug: vc_req_model.authority_slug.clone(),
            participant_type: "Authority".to_string(),
            base_url,
            token: None,
            is_me: false,
        };

        Ok(mate)
    }
}
