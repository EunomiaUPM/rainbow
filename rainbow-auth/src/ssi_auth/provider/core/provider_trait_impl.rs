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
use crate::ssi_auth::errors::AuthErrors;
use crate::ssi_auth::provider::core::provider_trait::RainbowSSIAuthProviderManagerTrait;
use crate::ssi_auth::provider::utils::{compare_with_margin, create_opaque_token, split_did};
use anyhow::bail;
use axum::async_trait;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::Validation;
use rainbow_common::auth::gnap::{GrantRequest, GrantResponse};
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use rainbow_common::errors::{CommonErrors, ErrorInfo};
use rainbow_common::ssi_wallet::ClientConfig;
use rainbow_db::auth_provider::entities::{
    auth_interaction, auth_request, auth_token_requirements, auth_verification, business_mates, mates,
};
use rainbow_db::auth_provider::repo_factory::factory_trait::AuthRepoFactoryTrait;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::HashSet;
use tracing::info;
use url::Url;
use urlencoding::{decode, encode};

#[async_trait]
impl<T> RainbowSSIAuthProviderManagerTrait for Manager<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    async fn generate_uri(&self, ver_model: auth_verification::Model) -> anyhow::Result<String> {
        info!("Generating verification exchange URI");

        let provider_url = self.config.get_ssi_auth_host_url().unwrap(); // TODO fix docker internal
        let provider_url = provider_url.replace("127.0.0.1", "host.docker.internal");
        let provider_url = format!("{}/api/v1", provider_url);

        let base_url = "openid4vp://authorize";

        let encoded_client_id = encode(&ver_model.audience);

        let presentation_definition_uri = format!("{}/pd/{}", &provider_url, ver_model.state);
        let encoded_presentation_definition_uri = encode(&presentation_definition_uri);

        let response_uri = format!("{}/verify/{}", &provider_url, ver_model.state);
        let encoded_response_uri = encode(&response_uri);

        let response_type = "vp_token";
        let response_mode = "direct_post";
        let client_id_scheme = "redirect_uri";

        // TODO let client_metadata = r#"{"authorization_encrypted_response_alg":"ECDH-ES","authorization_encrypted_response_enc":"A256GCM"}"#;

        let uri = format!("{}?response_type={}&client_id={}&response_mode={}&presentation_definition_uri={}&client_id_scheme={}&nonce={}&response_uri={}", base_url, response_type, encoded_client_id, response_mode, encoded_presentation_definition_uri, client_id_scheme, ver_model.nonce, encoded_response_uri);
        info!("Uri generated successfully: {}", uri);

        Ok(uri)
    }
    async fn manage_access(&self, payload: GrantRequest) -> anyhow::Result<GrantResponse> {
        info!("Managing access");
        let interact = match payload.interact {
            Some(model) => model,
            None => {
                let error = CommonErrors::FormatError {
                    // TODO
                    info: ErrorInfo {
                        message: "There is no interact method in the request".to_string(),
                        error_code: 1200,
                        details: None,
                    },
                    cause: None,
                };
                error.log();
                bail!(error);
            }
        };

        if !&interact.start.contains(&"oidc4vp".to_string()) {
            let error = CommonErrors::FormatError {
                info: ErrorInfo {
                    message: "Interact method not supported".to_string(),
                    error_code: 1200,
                    details: None,
                },
                cause: None,
            };
            error.log();
            bail!(error);
        }

        let mut provider_url = self.config.get_ssi_auth_host_url().unwrap(); // TODO fix docker internal
        let provider_url = provider_url.replace("127.0.0.1", "host.docker.internal");
        let provider_url = format!("{}/api/v1", provider_url);

        let client_id = format!("{}/verify", &provider_url);

        let grant_endpoint = format!(
            "{}/api/v1/access",
            self.config.get_ssi_auth_host_url().unwrap()
        );

        let id = uuid::Uuid::new_v4().to_string();

        let client: ClientConfig = serde_json::from_value(payload.client)?;

        let new_request_model = auth_request::NewModel { id: id.clone(), consumer_id: client.class_id };

        let _ = match self.repo.request().create(new_request_model).await {
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

        let continue_endpoint = format!(
            "{}/api/v1/continue",
            self.config.get_ssi_auth_host_url().unwrap()
        );
        let continue_token = create_opaque_token();
        let new_interaction_model = auth_interaction::NewModel {
            id: id.clone(),
            start: interact.start,
            method: interact.finish.method,
            uri: interact.finish.uri.unwrap(),
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
            r#type: payload.access_token.access.r#type,
            actions: payload.access_token.access.actions.unwrap_or(vec![String::from("talk")]),
            locations: payload.access_token.access.locations,
            datatypes: payload.access_token.access.datatypes,
            identifier: payload.access_token.access.identifier,
            privileges: payload.access_token.access.privileges,
            label: payload.access_token.label,
            flags: payload.access_token.flags,
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

        let new_verification_model = auth_verification::NewModel { id: id.clone(), audience: client_id };

        let verification_model = match self.repo.verification().create(new_verification_model).await {
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

        let uri = self.generate_uri(verification_model).await?;

        let response = GrantResponse::default4oidc4vp(
            interaction_model.id,
            interaction_model.continue_endpoint,
            interaction_model.continue_token,
            interaction_model.as_nonce,
            uri,
        );
        Ok(response)
    }

    async fn validate_continue_request(
        &self,
        cont_id: String,
        interact_ref: String,
        token: String,
    ) -> anyhow::Result<auth_interaction::Model> {
        info!("Validating continue request");
        let int_model = match self.repo.interaction().get_by_cont_id(cont_id.as_str()).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = CommonErrors::MissingError {
                    info: ErrorInfo {
                        message: format!("There is no process with cont_id: {}", &cont_id),
                        error_code: 1600,
                        details: None,
                    },
                    id: cont_id,
                    cause: None,
                };
                error.log();
                bail!(error);
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: format!("Error retrieving the process with cont_id: {}", &cont_id),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        if interact_ref != int_model.interact_ref {
            let error = CommonErrors::InvalidError {
                info: ErrorInfo { message: "Invalid petition".to_string(), error_code: 1300, details: None },
                cause: Some(format!(
                    "Interact reference '{}' does not match '{}'",
                    interact_ref, int_model.interact_ref
                )),
            };
            error.log();
            bail!(error);
        }

        if token != int_model.continue_token {
            let error = CommonErrors::InvalidError {
                info: ErrorInfo { message: "Invalid petition".to_string(), error_code: 1300, details: None },
                cause: Some(format!(
                    "Token '{}' does not match '{}'",
                    token, int_model.continue_token
                )),
            };
            error.log();
            bail!(error);
        }
        Ok(int_model)
    }

    async fn continue_req(&self, int_model: auth_interaction::Model) -> anyhow::Result<auth_request::Model> {
        let id = int_model.clone().id;
        let mut request_model = match self.repo.request().get_by_id(id.as_str()).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = CommonErrors::MissingError {
                    info: ErrorInfo {
                        message: format!("There is no process with id: {}", &id),
                        error_code: 1600,
                        details: None,
                    },
                    id,
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

        let token = create_opaque_token();
        request_model.token = Some(token);
        request_model.status = "Approved".to_string();

        let new_request_model = match self.repo.request().update(request_model).await {
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

        // if model.status != "pending" {
        //     bail!("Too many attempts"); // TODO
        // }

        let base_url = int_model.uri;
        match Url::parse(base_url.as_str()) {
            Ok(parsed_url) => match parsed_url.port() {
                Some(port) => {
                    format!(
                        "{}://{}:{}",
                        parsed_url.scheme(),
                        parsed_url.host_str().unwrap(),
                        port
                    )
                }
                None => {
                    format!(
                        "{}://{}",
                        parsed_url.scheme(),
                        parsed_url.host_str().unwrap()
                    )
                }
            },
            Err(e) => {
                let error = CommonErrors::FormatError {
                    info: ErrorInfo { message: "Error parsing the url".to_string(), error_code: 1200, details: None },
                    cause: Some(format!("Error parsing the url -> {}", e)),
                };
                error.log();
                bail!(error);
            }
        };

        Ok(new_request_model)
    }

    async fn retrieve_data(
        &self,
        req_model: auth_request::Model,
        int_model: auth_interaction::Model,
    ) -> anyhow::Result<mates::NewModel> {
        let id = int_model.id;
        let ver_model = match self.repo.verification().get_by_id(id.as_str()).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = CommonErrors::MissingError {
                    info: ErrorInfo {
                        message: format!("There is no process with id: {}", &id),
                        error_code: 1600,
                        details: None,
                    },
                    id: id,
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

        let mate = mates::NewModel {
            participant_id: ver_model.holder.unwrap(),
            participant_slug: req_model.consumer_id,
            participant_type: "Consumer".to_string(),
            base_url: Some(int_model.uri),
            token: req_model.token,
            is_me: false,
        };
        Ok(mate)
    }

    async fn save_mate(&self, mate: mates::NewModel) -> anyhow::Result<mates::Model> {
        match self.repo.mates().create(mate).await {
            Ok(model) => Ok(model),
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo { message: format!("Error saving mate"), error_code: 1300, details: None },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        }
    }

    async fn generate_vp_def(&self, state: String) -> anyhow::Result<Value> {
        let model = match self.repo.verification().get_by_state(state.as_str()).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = CommonErrors::MissingError {
                    info: ErrorInfo {
                        message: format!("There is no process with state: {}", &state),
                        error_code: 1600,
                        details: None,
                    },
                    id: state,
                    cause: None,
                };
                error.log();
                bail!(error);
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: format!("Error retrieving the process with state: {}", &state),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        Ok(json!({
          "id": model.id,
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

    async fn verify_all(&self, state: String, vp_token: String) -> anyhow::Result<String> {
        let verification_model = match self.repo.verification().get_by_state(state.as_str()).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = CommonErrors::MissingError {
                    info: ErrorInfo {
                        message: format!("There is no process with state: {}", &state),
                        error_code: 1600,
                        details: None,
                    },
                    id: state,
                    cause: None,
                };
                error.log();
                bail!(error);
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: format!("Error retrieving the process with state: {}", &state),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        let (vcts, holder) = match self.verify_vp(verification_model.clone(), vp_token).await {
            Ok((vcts, holder)) => (vcts, holder),
            Err(e) => {
                let mut new_model = self.repo.verification().get_by_id(verification_model.id.as_str()).await?.unwrap();
                new_model.success = Some(false);
                new_model.ended_at = Some(Utc::now().naive_utc());
                self.repo.verification().update(new_model).await?;
                bail!(e)
            }
        };

        for cred in vcts {
            match self.verify_vc(cred, holder.clone()).await {
                Ok(()) => {}
                Err(e) => {
                    let mut new_model =
                        self.repo.verification().get_by_id(verification_model.id.as_str()).await?.unwrap();
                    new_model.success = Some(false);
                    new_model.ended_at = Some(Utc::now().naive_utc());
                    self.repo.verification().update(new_model).await?;
                    bail!(e)
                }
            }
        }
        info!("VP & VC Validated successfully");

        let mut new_request_model = self.repo.request().get_by_id(verification_model.id.as_str()).await?.unwrap();
        let mut new_ver_model = self.repo.verification().get_by_id(verification_model.id.as_str()).await?.unwrap();

        new_ver_model.ended_at = Some(Utc::now().naive_utc());
        new_request_model.status = "Processing".to_string();

        match self.repo.request().update(new_request_model).await {
            Ok(model) => model,
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: format!("Error updating process with id: {}", &verification_model.id),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };
        match self.repo.verification().update(new_ver_model).await {
            Ok(model) => model,
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: format!("Error updating process with id: {}", &verification_model.id),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        Ok(verification_model.id)
    }

    async fn verify_vp(
        &self,
        model: auth_verification::Model,
        vp_token: String,
    ) -> anyhow::Result<(Vec<String>, String)> {
        info!("Verifying VP");
        let header = jsonwebtoken::decode_header(&vp_token)?;
        let kid_str = header.kid.as_ref().unwrap();
        // let (kid, kid_id) = split_did(kid_str.as_str()); // TODO KID_ID
        let (kid, _) = split_did(kid_str.as_str()); // TODO KID_ID
        let alg = header.alg;

        let vec = URL_SAFE_NO_PAD.decode(&(kid.replace("did:jwk:", "")))?;
        let mut jwk: Jwk = serde_json::from_slice(&vec)?;

        let key = jsonwebtoken::DecodingKey::from_jwk(&jwk)?;
        let mut audience = format!(
            "{}/api/v1/verify/{}",
            self.config.get_ssi_auth_host_url().unwrap(),
            &model.state
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
                let error = CommonErrors::InvalidError {
                    info: ErrorInfo {
                        message: "VPT token signature is incorrect".to_string(),
                        error_code: 1700,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        info!("VPT token signature is correct");

        let id = token.claims["jti"].as_str().unwrap();
        let nonce = token.claims["nonce"].as_str().unwrap();

        if token.claims["sub"].as_str().unwrap() != token.claims["iss"].as_str().unwrap()
            || token.claims["iss"].as_str().unwrap() != kid
        {
            // VALIDATE HOLDER 1
            let error = CommonErrors::InvalidError {
                info: ErrorInfo { message: "VPT token  is incorrect".to_string(), error_code: 1700, details: None },
                cause: Some("VPT token issuer, subject & kid does not match".to_string()),
            };
            error.log();
            bail!(error);
        }
        info!("VPT issuer, subject & kid matches");

        let mut model = model.clone();
        model.holder = Some(token.claims["sub"].as_str().unwrap().to_string());
        model.vpt = Some(vp_token);
        println!("{:#?}", model);

        let new_model = match self.repo.verification().update(model).await {
            Ok(model) => {
                println!("{:#?}", model);
                model
            },
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

        if new_model.nonce != nonce {
            // VALIDATE NONCE
            let error = CommonErrors::InvalidError {
                info: ErrorInfo {
                    message: "VPT token signature is incorrect".to_string(),
                    error_code: 1700,
                    details: None,
                },
                cause: Some("Invalid nonce, it does not match".to_string()),
            };
            error.log();
            bail!(error);
        }
        info!("VPT Nonce matches");

        if new_model.id != token.claims["vp"]["id"].as_str().unwrap() {
            // VALIDATE ID MATCHES JTI
            let error = CommonErrors::InvalidError {
                info: ErrorInfo {
                    message: "VPT token signature is incorrect".to_string(),
                    error_code: 1700,
                    details: None,
                },
                cause: Some("Invalid id, it does not match".to_string()),
            };
            error.log();
            bail!(error);
        }
        info!("Exchange is valid");

        if new_model.holder.unwrap() != token.claims["vp"]["holder"].as_str().unwrap() {
            let error = CommonErrors::InvalidError {
                info: ErrorInfo {
                    message: "VPT token signature is incorrect".to_string(),
                    error_code: 1700,
                    details: None,
                },
                cause: Some("VP id does not match sub & issuer".to_string()),
            };
            error.log();
            bail!(error);
        }
        info!("vp holder matches vpt subject & issuer");
        info!("VP Verification successful");

        let vct: Vec<String> = match serde_json::from_value(token.claims["vp"]["verifiableCredential"].clone()) {
            Ok(vc) => vc,
            Err(e) => {
                let error = CommonErrors::InvalidError {
                    info: ErrorInfo {
                        message: "VPT token signature is incorrect".to_string(),
                        error_code: 1700,
                        details: None,
                    },
                    cause: Some(format!(
                        "VPresentation is based on a nonexistent credential -> {}",
                        e.to_string()
                    )),
                };
                error.log();
                bail!(error);
            }
        };
        Ok((vct, kid.to_string()))
    }

    async fn verify_vc(&self, vc_token: String, vp_holder: String) -> anyhow::Result<()> {
        info!("Verifying VC");
        let header = jsonwebtoken::decode_header(&vc_token)?;
        let kid_str = header.kid.as_ref().unwrap();
        // let (kid, kid_id) = split_did(kid_str.as_str()); // TODO KID_ID
        let (kid, _) = split_did(kid_str.as_str()); // TODO KID_ID
        let alg = header.alg;

        let vec = URL_SAFE_NO_PAD.decode(&(kid.replace("did:jwk:", "")))?; // TODO
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
                let error = CommonErrors::InvalidError {
                    info: ErrorInfo {
                        message: "VPT token signature is incorrect".to_string(),
                        error_code: 1700,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };

        info!("VCT token signature is correct");

        if token.claims["iss"].as_str().unwrap() != kid || kid != token.claims["vc"]["issuer"]["id"].as_str().unwrap() {
            // VALIDATE IF ISSUER IS THE SAME AS KID
            let error = CommonErrors::InvalidError {
                info: ErrorInfo {
                    message: "VPT token signature is incorrect".to_string(),
                    error_code: 1700,
                    details: None,
                },
                cause: Some("VCT token issuer & kid does not match".to_string()),
            };
            error.log();
            bail!(error);
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
            let error = CommonErrors::InvalidError {
                info: ErrorInfo {
                    message: "VPT token signature is incorrect".to_string(),
                    error_code: 1700,
                    details: None,
                },
                cause: Some("VCT token sub, credential subject & VP Holder do not match".to_string()),
            };
            error.log();
            bail!(error);
        }
        info!("VC Holder Data is Correct");

        if token.claims["jti"].as_str().unwrap() != token.claims["vc"]["id"].as_str().unwrap() {
            let error = CommonErrors::InvalidError {
                info: ErrorInfo {
                    message: "VPT token signature is incorrect".to_string(),
                    error_code: 1700,
                    details: None,
                },
                cause: Some("VCT jti & VC id do not match".to_string()),
            };
            error.log();
            bail!(error);
        }
        info!("VCT jti & VC id match");

        let (keep, message) = compare_with_margin(
            token.claims["iat"].as_i64().unwrap(),
            token.claims["vc"]["issuanceDate"].as_str().unwrap(),
            2,
        );
        if keep {
            let error = CommonErrors::InvalidError {
                info: ErrorInfo {
                    message: "VPT token signature is incorrect".to_string(),
                    error_code: 1700,
                    details: None,
                },
                cause: Some(message.to_string()),
            };
            error.log();
            bail!(error);
        }
        info!("VC IssuanceDate and iat field match");

        match DateTime::parse_from_rfc3339(token.claims["vc"]["validFrom"].as_str().unwrap()) {
            Ok(parsed_date) => parsed_date <= Utc::now(),
            Err(e) => {
                let error = CommonErrors::InvalidError {
                    info: ErrorInfo {
                        message: "VPT token signature is incorrect".to_string(),
                        error_code: 1700,
                        details: None,
                    },
                    cause: Some(format!("VC iat and issuanceDate do not match -> {}", e)),
                };
                error.log();
                bail!(error);
            }
        };
        info!("VC validFrom is correct");
        info!("VC Verification successful");
        Ok(())
    }

    async fn end_verification(&self, id: String) -> anyhow::Result<Option<String>> {
        match self.repo.interaction().get_by_id(id.as_str()).await {
            Ok(Some(model)) => {
                if model.method == "redirect" {
                    let redirect_uri = format!(
                        "{}?hash={}&interact_ref={}",
                        model.uri, model.hash, model.interact_ref
                    );
                    Ok(Some(redirect_uri))
                } else if model.method == "else" {
                    // TODO
                    return Ok(None);
                } else {
                    let error = CommonErrors::InvalidError {
                        info: ErrorInfo {
                            message: "Interact method not supported".to_string(),
                            error_code: 1700,
                            details: None,
                        },
                        cause: Some(format!("Interact method {} not supported", model.method)),
                    };
                    error.log();
                    bail!(error);
                }
            }
            Ok(None) => {
                let verification_model = self.repo.verification().get_by_id(id.as_str()).await?.unwrap();
                let token = create_opaque_token();
                let bus_mate_model = business_mates::NewModel {
                    id: verification_model.state,
                    participant_id: verification_model.holder.unwrap(),
                    token: Some(token),
                };
                self.repo.business_mates().create(bus_mate_model).await?;
                Ok(None)
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
        }
    }

    async fn fast_login(&self, state: String) -> anyhow::Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();
        let provider_url = self.config.get_ssi_auth_host_url().unwrap(); // TODO fix docker internal
        let provider_url = provider_url.replace("127.0.0.1", "host.docker.internal");
        let provider_url = format!("{}/api/v1", provider_url);

        let client_id = format!("{}/verify", &provider_url);
        let audience = format!("{}/{}", client_id, &state);
        let new_ver_model = auth_verification::Model {
            id,
            state,
            nonce,
            audience,
            holder: None,
            vpt: None,
            success: None,
            status: "Pending".to_string(),
            created_at: Utc::now().naive_utc(),
            ended_at: None,
        };

        let ver_model = match self.repo.verification().create_extra(new_ver_model).await {
            Ok(model) => {
                info!("Verification model saved successfully");
                model
            }
            Err(e) => {
                let error = CommonErrors::DatabaseError {
                    info: ErrorInfo {
                        message: "Error saving the verification model into the database".to_string(),
                        error_code: 1300,
                        details: None,
                    },
                    cause: Some(e.to_string()),
                };
                error.log();
                bail!(error);
            }
        };
        let uri = self.generate_uri(ver_model).await?;
        Ok(uri)
    }
}
