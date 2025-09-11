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
use super::super::traits::AuthorityTrait;
use super::super::Authority;
use crate::data::entities::{auth_interaction, auth_request, auth_verification, minions};
use crate::data::repo_factory::factory_trait::AuthRepoFactoryTrait;
use crate::errors::helpers::BadFormat;
use crate::errors::Errors;
use crate::setup::config::client_config::ClientConfig;
use crate::setup::config::AuthorityFunctions;
use crate::types::gnap::{GrantRequest, GrantResponse};
use crate::utils::create_opaque_token;
use anyhow::bail;
use axum::async_trait;
use tracing::info;
use urlencoding::{decode, encode};

#[async_trait]
impl<T> AuthorityTrait for Authority<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    async fn manage_access(&self, payload: GrantRequest) -> anyhow::Result<GrantResponse> {
        info!("Managing access");

        println!("{:#?}", payload);
        let interact = match payload.interact {
            Some(model) => model,
            None => {
                let error = Errors::not_impl_new(
                    "Only petitions with an 'interact field' are supported right now".to_string(),
                    Some("Only petitions with an 'interact field' are supported right now".to_string()),
                );
                error.log();
                bail!(error);
            }
        };

        let start = interact.start;
        if !&start.contains(&"await".to_string()) && !&start.contains(&"oidc4vp".to_string()) {
            let error = Errors::not_impl_new(
                "Interact method not supported yet".to_string(),
                Some("Interact method not supported yet".to_string()),
            );
            error.log();
            bail!(error);
        }

        let host_url = self.config.get_host(); //  EXPECTED ALWAYS TODO fix docker internal
        let host_url = format!("{}/api/v1", host_url);
        let docker_host_url = host_url.clone().replace("127.0.0.1", "host.docker.internal");

        // TODO OIDC
        let id = uuid::Uuid::new_v4().to_string();

        let client = payload.client;
        let class_id = match client["class_id"].as_str() {
            Some(data) => data.to_string(),
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("Missing field class_id in the petition".to_string()),
                );
                error.log();
                bail!(error);
            }
        };
        // let client: ClientConfig = serde_json::from_value(payload.client)?;

        let new_request_model = auth_request::NewModel { id: id.clone(), participant_slug: class_id };

        let _ = match self.repo.request().create(new_request_model).await {
            Ok(model) => {
                info!("Authentication request saved successfully");
                model
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error.log();
                bail!(error);
            }
        };

        let continue_endpoint = format!("{}/continue", &host_url);
        let grant_endpoint = format!("{}/credential/request", &host_url);
        let continue_token = create_opaque_token();
        let new_interaction_model = auth_interaction::NewModel {
            id: id.clone(),
            start,
            method: interact.finish.method,
            uri: interact.finish.uri.unwrap(), // EXPECTED ALWAYS
            client_nonce: interact.finish.nonce,
            hash_method: interact.finish.hash_method,
            hints: interact.hints,
            grant_endpoint,
            continue_endpoint,
            continue_token,
        };

        let interaction_model = match self.repo.interaction().create(new_interaction_model).await {
            Ok(model) => {
                info!("Authentication interaction saved successfully");
                model
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error.log();
                bail!(error);
            }
        };

        // ----------OIDC4VP---------------------------------------------------------------
        if interaction_model.start.contains(&"oidc4vp".to_string()) {
            let client_id = format!("{}/verify", &docker_host_url);

            let new_verification_model = auth_verification::NewModel { id: id.clone(), audience: client_id };

            let verification_model = match self.repo.verification().create(new_verification_model).await {
                Ok(model) => {
                    info!("Verification data saved successfully");
                    model
                }
                Err(e) => {
                    let error = Errors::database_new(Some(e.to_string()));
                    error.log();
                    bail!(error);
                }
            };

            let uri = self.generate_uri(verification_model).await?;

            let response = GrantResponse::default4oidc4vp(
                interaction_model.id,
                interaction_model.continue_endpoint,
                interaction_model.continue_token,
                interaction_model.as_nonce,
                uri,
            );

            return Ok(response);
        }

        // ----------AWAIT---------------------------------------------------------------
        let response = GrantResponse::default4cross_user(
            interaction_model.id,
            interaction_model.continue_endpoint,
            interaction_model.continue_token,
            interaction_model.as_nonce,
        );
        Ok(response)
    }

    async fn save_minion(&self, minion: minions::NewModel) -> anyhow::Result<minions::Model> {
        match self.repo.minions().force_create(minion).await {
            Ok(model) => Ok(model),
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error.log();
                bail!(error);
            }
        }
    }

    async fn generate_uri(&self, ver_model: auth_verification::Model) -> anyhow::Result<String> {
        info!("Generating verification exchange URI");

        let host_url = self.config.get_host();
        let host_url = format!("{}/api/v1", host_url);
        let docker_host_url = host_url.replace("127.0.0.1", "host.docker.internal");

        let base_url = "openid4vp://authorize";

        let encoded_client_id = encode(&ver_model.audience);

        let presentation_definition_uri = format!("{}/pd/{}", &docker_host_url, ver_model.state);
        let encoded_presentation_definition_uri = encode(&presentation_definition_uri);

        let response_uri = format!("{}/verify/{}", &docker_host_url, ver_model.state);
        let encoded_response_uri = encode(&response_uri);

        let response_type = "vp_token";
        let response_mode = "direct_post";
        let client_id_scheme = "redirect_uri";

        // TODO let client_metadata = r#"{"authorization_encrypted_response_alg":"ECDH-ES","authorization_encrypted_response_enc":"A256GCM"}"#;

        let uri = format!("{}?response_type={}&client_id={}&response_mode={}&presentation_definition_uri={}&client_id_scheme={}&nonce={}&response_uri={}", base_url, response_type, encoded_client_id, response_mode, encoded_presentation_definition_uri, client_id_scheme, ver_model.nonce, encoded_response_uri);
        info!("Uri generated successfully: {}", uri);

        Ok(uri)
    }
}
