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
use crate::core::traits::RainbowSSIAuthWalletTrait;
use crate::data::entities::{auth_interaction, auth_request, auth_verification, minions};
use crate::data::repo_factory::factory_trait::AuthRepoFactoryTrait;
use crate::errors::helpers::BadFormat;
use crate::errors::{ErrorLog, Errors};
use crate::setup::config::AuthorityFunctions;
use crate::setup::AuthorityApplicationConfigTrait;
use crate::types::gnap::{CallbackBody, GrantRequest, GrantResponse, RejectedCallbackBody};
use crate::types::manager::VcManager;
use crate::utils::{compare_with_margin, create_opaque_token, split_did, trim_4_base, trim_path};
use anyhow::bail;
use axum::async_trait;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::HeaderMap;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use chrono::{DateTime, Utc};
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::Validation;
use rsa::pkcs8::DecodePrivateKey;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use rsa::RsaPrivateKey;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use tracing::{debug, error, info};
use urlencoding::encode;
use x509_parser::parse_x509_certificate;

#[async_trait]
impl<T> AuthorityTrait for Authority<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    async fn manage_access(&self, payload: GrantRequest) -> anyhow::Result<GrantResponse> {
        info!("Managing access");

        let interact = match payload.interact {
            Some(model) => model,
            None => {
                let error = Errors::not_impl_new(
                    "Only petitions with an 'interact field' are supported right now".to_string(),
                    Some("Only petitions with an 'interact field' are supported right now".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let start = interact.start;
        if !&start.contains(&"await".to_string()) && !&start.contains(&"oidc4vp".to_string()) {
            let error = Errors::not_impl_new(
                "Interact method not supported yet".to_string(),
                Some("Interact method not supported yet".to_string()),
            );
            error!("{}", error.log());
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
                error!("{}", error.log());
                bail!(error);
            }
        };

        let cert: Option<String> = match serde_json::from_value(client["key"]["cert"].clone()) {
            Ok(data) => Some(data),
            Err(_) => None,
        };
        // let client: ClientConfig = serde_json::from_value(payload.client)?;

        let new_request_model = auth_request::NewModel { id: id.clone(), participant_slug: class_id, cert };

        let _ = match self.repo.request().create(new_request_model).await {
            Ok(model) => {
                info!("Authentication request saved successfully");
                model
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let continue_endpoint = format!("{}/continue", &host_url);
        let grant_endpoint = format!("{}/request/credential", &host_url);
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
                error!("{}", error.log());
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
                    error!("{}", error.log());
                    bail!(error);
                }
            };

            let uri = self.generate_verification_uri(verification_model).await?;

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
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn generate_verification_uri(&self, ver_model: auth_verification::Model) -> anyhow::Result<String> {
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

    async fn generate_issuing_uri(
        &self,
        callback_id: String,
        name: String,
        website: String,
        real: bool,
    ) -> anyhow::Result<String> {
        info!("Generating an issuing uri");

        match real {
            true => {
                // TODO THE MODULE SHOULD BE ABLE TO DO IT ITSELF
                let error = Errors::not_impl_new("REAL URI".to_string(), None);
                error!("{}", error.log());
                bail!(error)
            }
            false => {
                let url = "http://127.0.0.1:7002/openid4vc/jwt/issue".to_string();
                // let issuer_id = match self.get_did_doc["id"].as_str() {
                //     Some(data) => data,
                //     None => {
                //         bail!("Error parsing the DID identifier")
                //     }
                // };
                let path = trim_path(self.config.get_raw_client_config().cert_path.as_str());
                let pkey_path = format!("{}/private_key.pem", path);

                let pkey = fs::read_to_string(pkey_path)?;

                let key = RsaPrivateKey::from_pkcs8_pem(pkey.as_str())?;

                let jwk = json!({
                    "kty" : "RSA",
                    "n" : URL_SAFE_NO_PAD.encode(key.n().to_bytes_be()),
                    "e" : URL_SAFE_NO_PAD.encode(key.e().to_bytes_be()),
                    "d" : URL_SAFE_NO_PAD.encode(key.d().to_bytes_be()),
                    "kid" : "0"
                });

                let did = self.get_did().await?;

                let client = self.config.get_raw_client_config();
                let issuer = client.class_id.clone();

                let body = json!({
                    "issuerKey": { // TODO
                        "type": "jwk",
                        "jwk": jwk
                    },
                    "credentialConfigurationId": "DataspaceParticipantCredential_jwt_vc_json",
                    "credentialData": {
                        "@context": [
                            "https://www.w3.org/2018/credentials/v1"
                        ],
                    "id": "https://example.gov/credentials/3732", // DISAPPEARS
                    "type": [
                        "VerifiableCredential",
                        "DataspaceParticipantCredential"
                    ],
                    "issuer": {
                        "id": "did:web:vc.transmute.world", // DISAPPEARS
                        "name": issuer,
                    },
                    "issuanceDate": "2020-03-10T04:24:12.164Z", // DISAPPEARS
                    "credentialSubject": {
                        "id": "did:example:ebfeb1f712ebc6f1c276e12ec21", // DISAPPEARS
                        "type": "DataspaceParticipant",
                        "dataspaceId": "Rainbow DataSpace",
                        "legalName": name,
                        "website": website,
                        }
                    },
                    "mapping": {
                    "id": "<uuid>",
                    "issuer": {
                        "id": "<issuerDid>"
                    },
                    "credentialSubject": {
                        "id": "<subjectDid>"
                    },
                    "issuanceDate": "<timestamp>",
                    "expirationDate": "<timestamp-in:365d>"
                    },
                    "issuerDid": did,
                });

                let host_url = self.config.get_host();
                let docker_host_url = host_url.clone().replace("127.0.0.1", "host.docker.internal"); // TODO FIX 4 MICROSERICES
                let callback_uri = format!("{}/api/v1/callback/{}", docker_host_url, callback_id);
                let mut headers = HeaderMap::new();
                headers.insert(CONTENT_TYPE, "application/json".parse()?);
                headers.insert(ACCEPT, "application/json".parse()?);
                headers.insert("statusCallbackUri", callback_uri.parse()?);

                let res = self.client.post(&url).headers(headers).json(&body).send().await;

                let res = match res {
                    Ok(data) => data,
                    Err(e) => {
                        let http_code = match e.status() {
                            Some(status) => Some(status.as_u16()),
                            None => None,
                        };
                        let error = Errors::petition_new(url, "POST".to_string(), http_code, e.to_string());
                        error!("{}", error.log());
                        bail!(error);
                    }
                };

                match res.status().as_u16() {
                    200 => {
                        let response = res.text().await?;
                        info!("URI generated successfully");
                        info!("{}", response);
                        Ok(response)
                    }
                    _ => {
                        let error = Errors::wallet_new(
                            url,
                            "POST".to_string(),
                            res.status().as_u16(),
                            Some("Petition to register Wallet failed".to_string()),
                        );
                        error!("{}", error.log());
                        bail!(error);
                    }
                }
            }
        }
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
                let error = Errors::missing_resource_new(
                    cont_id.to_string(),
                    Some(format!("There is no process with cont_id: {}", &cont_id)),
                );
                error!("{}", error.log());
                bail!(error);
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        if interact_ref != int_model.interact_ref {
            let error = Errors::security_new(Some(format!(
                "Interact reference '{}' does not match '{}'",
                interact_ref, int_model.interact_ref
            )));
            error!("{}", error.log());
            bail!(error);
        }

        if token != int_model.continue_token {
            let error = Errors::security_new(Some(format!(
                "Token '{}' does not match '{}'",
                token, int_model.continue_token
            )));
            error!("{}", error.log());
            bail!(error);
        }

        Ok(int_model)
    }

    async fn continue_req(&self, int_model: auth_interaction::Model) -> anyhow::Result<auth_request::Model> {
        info!("Continuing request");
        let id = int_model.clone().id;
        let mut request_model = match self.repo.request().get_by_id(id.as_str()).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = Errors::missing_resource_new(
                    id.clone(),
                    Some(format!("There is no process with cont_id: {}", &id)),
                );
                error!("{}", error.log());
                bail!(error);
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let name = request_model.participant_slug.clone();
        let mut website = "null".to_string();
        for starts in int_model.start {
            // --------------AWAIT------------------------------------------------------------------
            if starts.contains("await") {
                let base_cert = match request_model.cert.clone() {
                    Some(data) => data,
                    None => {
                        let error = Errors::format_new(
                            BadFormat::Received,
                            Some("There was no cert in the Grant Request".to_string()),
                        );
                        error!("{}", error.log());
                        bail!(error)
                    }
                };

                let cert_bytes = STANDARD.decode(base_cert)?;
                let (_, cert) = parse_x509_certificate(&cert_bytes)?;
                let test = cert.subject.to_string();
                let clean = test.strip_prefix("CN=").unwrap_or(test.as_str());
                website = format!("http://{}", clean.to_string());
                break;
            }
            // -------------OIDC4VP-----------------------------------------------------------------
            if starts.contains("oidc4vp") {
                let ver_request = match self.repo.verification().get_by_id(id.as_str()).await {
                    Ok(Some(data)) => data,
                    Ok(None) => {
                        let error = Errors::missing_resource_new(id.clone(), None);
                        error!("{}", error.log());
                        bail!(error)
                    }
                    Err(e) => {
                        let error = Errors::database_new(Some(e.to_string()));
                        error!("{}", error.log());
                        bail!(error)
                    }
                };

                // TODO COMPROBAR WEBSITE BIEN
                website = ver_request.holder.unwrap(); // EXPECTED ALWAYS
                break;
            }
            let error = Errors::format_new(BadFormat::Received, None);
            error!("{}", error.log());
            bail!(error)
        }

        let vc_uri = self.generate_issuing_uri(id, name, website, false).await?;
        request_model.status = "Approved".to_string();
        request_model.vc_uri = Some(vc_uri);

        let new_request_model = match self.repo.request().update(request_model).await {
            Ok(model) => model,
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        // if model.status != "pending" {
        //     bail!("Too many attempts"); // TODO
        // }

        Ok(new_request_model)
    }

    async fn retrieve_data(
        &self,
        req_model: auth_request::Model,
        vc_token: String,
    ) -> anyhow::Result<minions::NewModel> {
        let id = req_model.id.as_str();

        let int_model = match self.repo.interaction().get_by_id(id).await {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new(
                    id.to_string(),
                    Some(format!("Missing process with id: {}", id)),
                );
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error)
            }
        };
        let id = int_model.id;

        for starts in int_model.start {
            // --------------AWAIT------------------------------------------------------------------
            if starts.contains("await") {
                let base_cert = match req_model.cert {
                    Some(data) => data,
                    None => {
                        let error = Errors::format_new(
                            BadFormat::Received,
                            Some("There was no cert in the Grant Request".to_string()),
                        );
                        error!("{}", error.log());
                        bail!(error)
                    }
                };

                let cert_bytes = STANDARD.decode(base_cert)?;
                let (_, cert) = parse_x509_certificate(&cert_bytes)?;

                let holder = self.retrieve_holder(vc_token).await?;
                let base_url = Some(trim_4_base(int_model.uri.as_str()));
                let subject = cert.subject.to_string();
                let subject = subject.strip_prefix("CN=").unwrap_or(subject.as_str());
                let minion = minions::NewModel {
                    participant_id: holder,
                    participant_slug: subject.to_string(),
                    participant_type: "Minion".to_string(),
                    base_url,
                    vc_uri: req_model.vc_uri,
                    is_vc_issued: true,
                    is_me: false,
                };
                return Ok(minion);
            }
            // -------------OIDC4VP-----------------------------------------------------------------
            if starts.contains("oidc4vp") {
                let ver_model = match self.repo.verification().get_by_id(id.as_str()).await {
                    Ok(Some(model)) => model,
                    Ok(None) => {
                        let error = Errors::missing_resource_new(
                            id.clone(),
                            Some(format!("There is no process with id: {}", &id)),
                        );
                        error!("{}", error.log());
                        bail!(error);
                    }
                    Err(e) => {
                        let error = Errors::database_new(Some(e.to_string()));
                        error!("{}", error.log());
                        bail!(error);
                    }
                };

                let base_url = Some(trim_4_base(int_model.uri.as_str()));
                let minion = minions::NewModel {
                    participant_id: ver_model.holder.unwrap(), // EXPECTED ALWAYS
                    participant_slug: req_model.participant_slug,
                    participant_type: "Consumer".to_string(),
                    base_url,
                    vc_uri: req_model.vc_uri,
                    is_vc_issued: true,
                    is_me: false,
                };
                return Ok(minion);
            }
        }
        let error = Errors::format_new(BadFormat::Received, None);
        error!("{}", error.log());
        bail!(error)
    }

    async fn generate_vp_def(&self, state: String) -> anyhow::Result<Value> {
        let model = match self.repo.verification().get_by_state(state.as_str()).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                let error = Errors::missing_resource_new(
                    state.clone(),
                    Some(format!("There is no process with state: {}", &state)),
                );
                error!("{}", error.log());
                bail!(error);
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        Ok(json!({
            "id": model.id,
            "input_descriptors": [
                {
                "id": "IdentityCredential",
                "format": {
                    "jwt_vc_json": {
                    "alg": [
                        "RSA"
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
                        "pattern": "IdentityCredential"
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
                let error = Errors::missing_resource_new(
                    state.clone(),
                    Some(format!("There is no process with state: {}", &state)),
                );
                error!("{}", error.log());
                bail!(error);
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let (vcts, holder) = match self.verify_vp(verification_model.clone(), vp_token).await {
            Ok((vcts, holder)) => (vcts, holder),
            Err(e) => {
                let mut new_model = match self.repo.verification().get_by_id(verification_model.id.as_str()).await {
                    Ok(Some(model)) => model,
                    Ok(None) => {
                        let error = Errors::missing_resource_new(
                            verification_model.id.clone(),
                            Some(format!(
                                "There is no process with id: {}",
                                &verification_model.id
                            )),
                        );
                        error!("{}", error.log());
                        bail!(error);
                    }
                    Err(e) => {
                        let error = Errors::database_new(Some(e.to_string()));
                        error!("{}", error.log());
                        bail!(error);
                    }
                };

                new_model.success = Some(false);
                new_model.ended_at = Some(Utc::now().naive_utc());
                match self.repo.verification().update(new_model).await {
                    Ok(_) => {}
                    Err(e) => {
                        let error = Errors::database_new(Some(e.to_string()));
                        error!("{}", error.log());
                        bail!(error);
                    }
                };
                bail!(e)
            }
        };

        for cred in vcts {
            match self.verify_vc(cred, holder.clone()).await {
                Ok(()) => {}
                Err(e) => {
                    let mut new_model = match self.repo.verification().get_by_id(verification_model.id.as_str()).await {
                        Ok(Some(model)) => model,
                        Ok(None) => {
                            let error = Errors::missing_resource_new(
                                verification_model.id.clone(),
                                Some(format!(
                                    "There is no process with id: {}",
                                    &verification_model.id
                                )),
                            );
                            error!("{}", error.log());
                            bail!(error);
                        }
                        Err(e) => {
                            let error = Errors::database_new(Some(e.to_string()));
                            error!("{}", error.log());
                            bail!(error);
                        }
                    };
                    new_model.success = Some(false);
                    new_model.ended_at = Some(Utc::now().naive_utc());
                    match self.repo.verification().update(new_model).await {
                        Ok(_) => {}
                        Err(e) => {
                            let error = Errors::database_new(Some(e.to_string()));
                            error!("{}", error.log());
                            bail!(error);
                        }
                    };
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
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };
        match self.repo.verification().update(new_ver_model).await {
            Ok(model) => model,
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
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
        let kid_str = match header.kid.as_ref() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("Jwt does not contain a token".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        // let (kid, kid_id) = split_did(kid_str.as_str()); // TODO KID_ID
        let (kid, _) = split_did(kid_str.as_str()); // TODO KID_ID
        let alg = header.alg;

        let vec = URL_SAFE_NO_PAD.decode(&(kid.replace("did:jwk:", "")))?;
        let jwk: Jwk = serde_json::from_slice(&vec)?;

        let key = jsonwebtoken::DecodingKey::from_jwk(&jwk)?;
        let mut audience = format!("{}/api/v1/verify/{}", self.config.get_host(), &model.state);
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
                let error = Errors::security_new(Some(format!(
                    "VPT signature is incorrect -> {}",
                    e.to_string()
                )));
                error!("{}", error.log());
                bail!(error);
            }
        };

        info!("VPT token signature is correct");

        // let id = match token.claims["jti"].as_str() {
        //     Some(data) => data,
        //     None => {
        //         let error = CommonErrors::format_new(
        //             BadFormat::Received,
        //             Some("VPT does not contain the 'jti' field".to_string()),
        //         );
        //         error!("{}", error.log());
        //         bail!(error);
        //     }
        // };
        let nonce = match token.claims["nonce"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("VPT does not contain the 'nonce' field".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let sub = match token.claims["sub"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("VPT does not contain the 'sub' field".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        let iss = match token.claims["iss"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("VPT does not contain the 'iss' field".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        if sub != iss || iss != kid {
            // VALIDATE HOLDER 1
            let error = Errors::security_new(Some(
                "VPT token issuer, subject & kid does not match".to_string(),
            ));
            error!("{}", error.log());
            bail!(error);
        }
        info!("VPT issuer, subject & kid matches");

        let mut model = model.clone();
        model.holder = Some(sub.to_string());
        model.vpt = Some(vp_token);

        let new_model = match self.repo.verification().update(model).await {
            Ok(model) => model,
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        if new_model.nonce != nonce {
            // VALIDATE NONCE
            let error = Errors::security_new(Some("Invalid nonce, it does not match".to_string()));
            error!("{}", error.log());
            bail!(error);
        }
        info!("VPT Nonce matches");

        let vp_id = match token.claims["vp"]["id"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("VPT does not contain the 'vp_id' field".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        if new_model.id != vp_id {
            // VALIDATE ID MATCHES JTI
            let error = Errors::security_new(Some("Invalid id, it does not match".to_string()));
            error!("{}", error.log());
            bail!(error);
        }
        info!("Exchange is valid");

        let vp_holder = match token.claims["vp"]["holder"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("VPT does not contain the 'vp_holder' field".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        if new_model.holder.unwrap() != vp_holder {
            // EXPECTED ALWAYS
            let error = Errors::security_new(Some("Invalid holder, it does not match".to_string()));
            error!("{}", error.log());
            bail!(error);
        }
        info!("vp holder matches vpt subject & issuer");
        info!("VP Verification successful");

        let vct: Vec<String> = match serde_json::from_value(token.claims["vp"]["verifiableCredential"].clone()) {
            Ok(data) => data,
            Err(e) => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some(format!(
                        "VPT does not contain the 'verifiableCredential' field -> {}",
                        e.to_string()
                    )),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        Ok((vct, kid.to_string()))
    }

    async fn verify_vc(&self, vc_token: String, vp_holder: String) -> anyhow::Result<()> {
        info!("Verifying VC");
        let header = jsonwebtoken::decode_header(&vc_token)?;
        let kid_str = match header.kid.as_ref() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("Jwt does not contain a token".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        // let (kid, kid_id) = split_did(kid_str.as_str()); // TODO KID_ID
        let (kid, _) = split_did(kid_str.as_str()); // TODO KID_ID
        let alg = header.alg;

        let vec = URL_SAFE_NO_PAD.decode(&(kid.replace("did:jwk:", "")))?; // TODO
        let jwk: Jwk = serde_json::from_slice(&vec)?;

        let key = jsonwebtoken::DecodingKey::from_jwk(&jwk)?;

        let mut val = Validation::new(alg);
        val.required_spec_claims = HashSet::new();
        val.validate_aud = false;
        val.validate_exp = false; // TODO de momemnto las VCs no caducan
        val.validate_nbf = true;

        let token = match jsonwebtoken::decode::<Value>(&vc_token, &key, &val) {
            Ok(token) => token,
            Err(e) => {
                let error = Errors::format_new(BadFormat::Received, Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        info!("VCT token signature is correct");

        let iss = match token.claims["iss"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(BadFormat::Received, Some("No issuer in the vc".to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };
        let vc_iss_id = match token.claims["vc"]["issuer"]["id"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("No issuer id in the vc".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        if iss != kid || kid != vc_iss_id {
            // VALIDATE IF ISSUER IS THE SAME AS KID
            let error = Errors::security_new(Some("VCT token issuer & kid does not match".to_string()));
            error!("{}", error.log());
            bail!(error);
        }
        info!("VCT issuer & kid matches");

        // if issuers_list.contains(kid) {
        //     // TODO
        //     error!("VCT issuer is not on the trusted issuers list");
        //     bail!("VCT issuer is not on the trusted issuers list");
        // }
        // info!("VCT issuer is on the trusted issuers list");

        let sub = match token.claims["sub"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("No sub field in the vc".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let cred_sub_id = match token.claims["vc"]["credentialSubject"]["id"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("No credentialSubject id field in the vc".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        if sub != &vp_holder || &vp_holder != cred_sub_id {
            let error = Errors::security_new(Some(
                "VCT token sub, credential subject & VP Holder do not match".to_string(),
            ));
            error!("{}", error.log());
            bail!(error);
        }
        info!("VC Holder Data is Correct");

        let jti = match token.claims["jti"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("No jti id field in the vc".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let vc_id = match token.claims["vc"]["id"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("No vc_id id field in the vc".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        if jti != vc_id {
            let error = Errors::security_new(Some("VCT jti & VC id do not match".to_string()));
            error!("{}", error.log());
            bail!(error);
        }
        info!("VCT jti & VC id match");

        let iat = match token.claims["iat"].as_i64() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("No credentialSubject id field in the vc".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };

        let iss_date = match token.claims["vc"]["issuanceDate"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("No credentialSubject id field in the vc".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        let (keep, message) = compare_with_margin(iat, iss_date, 2);
        if keep {
            let error = Errors::security_new(Some(message.to_string()));
            error!("{}", error.log());
            bail!(error);
        }
        info!("VC IssuanceDate and iat field match");

        let valid_from = match token.claims["vc"]["validFrom"].as_str() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("No validFrom id field in the vc".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        match DateTime::parse_from_rfc3339(valid_from) {
            Ok(parsed_date) => parsed_date <= Utc::now(),
            Err(e) => {
                let error = Errors::security_new(Some(format!(
                    "VC iat and issuanceDate do not match -> {}",
                    e
                )));
                error!("{}", error.log());
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
                    let error = Errors::not_impl_new(
                        "Interact method not supported".to_string(),
                        Some(format!("Interact method {} not supported", model.method)),
                    );
                    error!("{}", error.log());
                    bail!(error);
                }
            }
            Ok(None) => Ok(None),
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        }
    }

    async fn manage_vc_request(&self, id: String, payload: VcManager) -> anyhow::Result<()> {
        info!("Managing vc request");
        let mut req_model = match self.repo.request().get_by_id(id.as_str()).await {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new(id.clone(), Some(format!("Missing request with id: {}", id)));
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error)
            }
        };

        let int_model = match self.repo.interaction().get_by_id(id.as_str()).await {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new(id.clone(), Some(format!("Missing request with id: {}", id)));
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error)
            }
        };

        let body = match payload.approve {
            true => {
                info!("Approving petition to obtain a VC");
                req_model.status = "Approved".to_string();
                let _ = match self.repo.request().update(req_model).await {
                    Ok(model) => model,
                    Err(e) => {
                        let error = Errors::database_new(Some(e.to_string()));
                        error!("{}", error.log());
                        bail!(error);
                    }
                };

                let body = CallbackBody { interact_ref: int_model.interact_ref, hash: int_model.hash };
                serde_json::to_value(body)?
            }
            false => {
                info!("Rejecting petition to obtain a VC");
                req_model.status = "Finalized".to_string();
                let _ = match self.repo.request().update(req_model).await {
                    Ok(model) => model,
                    Err(e) => {
                        let error = Errors::database_new(Some(e.to_string()));
                        error!("{}", error.log());
                        bail!(error);
                    }
                };

                let body = RejectedCallbackBody { rejected: "Petition was rejected".to_string() };
                serde_json::to_value(body)?
            }
        };
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self.client.post(&int_model.uri).headers(headers).json(&body).send().await;

        let res = match res {
            Ok(data) => data,
            Err(e) => {
                let http_code = match e.status() {
                    Some(status) => Some(status.as_u16()),
                    None => None,
                };
                let error = Errors::petition_new(int_model.uri, "POST".to_string(), http_code, e.to_string());
                error!("{}", error.log());
                bail!(error);
            }
        };

        match res.status().as_u16() {
            200 => {
                info!("Minion received callback received successfully");
            }
            _ => {
                let error = Errors::consumer_new(
                    Some(int_model.uri),
                    Some("POST".to_string()),
                    Some(res.status().as_u16()),
                    None,
                );
                error!("{}", error.log());
                bail!(error);
            }
        }

        Ok(())
    }

    async fn manage_callback(&self, id: String, payload: Value) -> anyhow::Result<()> {
        info!("Managing callback");

        let jwt = if let Some(event_type) = payload.get("type").and_then(|t| t.as_str()) {
            match event_type {
                "jwt_issue" => {
                    let jwt =
                        payload.get("data").and_then(|d| d.get("jwt")).and_then(|j| j.as_str()).ok_or_else(|| {
                            let error = Errors::format_new(
                                BadFormat::Received,
                                Some("There was no field jwt".to_string()),
                            );
                            error!("{}", error.log());
                            error
                        })?;
                    info!("Credential issued successfully");
                    debug!("Issued JWT: {}", jwt);
                    jwt.to_string()
                }
                other => {
                    info!("Received another type of callback: {}", other);
                    return Ok(());
                }
            }
        } else {
            let error = Errors::format_new(
                BadFormat::Received,
                Some("There was no field type".to_string()),
            );
            error!("{}", error.log());
            bail!(error)
        };

        let mut req_model = match self.repo.request().get_by_id(id.as_str()).await {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new(id.clone(), Some(format!("Missing procces with id: {}", id)));
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error)
            }
        };

        let minion = self.retrieve_data(req_model.clone(), jwt).await?;
        let minion = self.save_minion(minion).await?;

        req_model.is_vc_issued = true;
        req_model.vc_uri = minion.vc_uri;

        let _ = match self.repo.request().update(req_model).await {
            Ok(data) => data,
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                bail!(error)
            }
        };

        Ok(())
    }

    async fn retrieve_holder(&self, vc_token: String) -> anyhow::Result<String> {
        let parts: Vec<&str> = vc_token.split('.').collect();
        if parts.len() != 3 {
            let error = Errors::format_new(
                BadFormat::Received,
                Some("JWT does not have 3 parts".to_string()),
            );
            error!("{}", error.log());
            bail!(error);
        }

        let decoded_payload = match URL_SAFE_NO_PAD.decode(parts[1]) {
            Ok(data) => data,
            Err(e) => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some(format!("Base64 decode error: {}", e)),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        let json: Value = match serde_json::from_slice(&decoded_payload) {
            Ok(data) => data,
            Err(e) => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some(format!("JSON parse error: {}", e)),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        match json["vc"]["credentialSubject"]["id"].as_str() {
            Some(data) => {
                let holder = data.to_string();
                info!("Holder: {}", holder);
                Ok(holder)
            }
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("No credentialSubject id field in the vc".to_string()),
                );
                error!("{}", error.log());
                bail!(error);
            }
        }
    }
}
