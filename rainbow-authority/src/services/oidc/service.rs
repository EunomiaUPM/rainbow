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
use super::config::{OidcServiceConfig, OidcServiceConfigTrait};
use super::OidcServiceTrait;
use crate::data::entities::request;
use crate::data::entities::verification;
use crate::errors::{ErrorLogTrait, Errors};
use crate::types::enums::errors::BadFormat;
use crate::types::enums::vc_type::VcType;
use crate::types::oidc::{AuthServerMetadata, IssuerMetadata, VCCredOffer, WellKnownJwks};
use crate::types::vcs::VPDef;
use crate::utils::{create_opaque_token, split_did};
use anyhow::bail;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use chrono::{DateTime, Utc};
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::{TokenData, Validation};
use serde_json::Value;
use std::collections::HashSet;
use tracing::{error, info};
use urlencoding::encode;

pub struct OidcService {
    config: OidcServiceConfig,
}

impl OidcService {
    pub fn new(config: OidcServiceConfig) -> Self {
        OidcService { config }
    }
}

impl OidcServiceTrait for OidcService {
    fn start_vp(&self, id: &str, vc_type: VcType) -> anyhow::Result<verification::NewModel> {
        info!("Managing OIDC4VP");
        let host_url = self.config.get_host();
        let host_url = match self.config.is_local() {
            true => host_url.replace("127.0.0.1", "host.docker.internal"),
            false => host_url,
        };

        let client_id = format!("{}/verify", &host_url);
        let new_verification_model =
            verification::NewModel { id: id.to_string(), audience: client_id, vc_type: vc_type.to_string() };

        Ok(new_verification_model)
    }

    fn generate_verification_uri(&self, model: verification::Model) -> String {
        info!("Generating verification exchange URI");

        let host_url = self.config.get_host();
        let host_url = format!("{}/api/v1", host_url);
        let host_url = match self.config.is_local() {
            true => host_url.replace("127.0.0.1", "host.docker.internal"),
            false => host_url,
        };

        let base_url = "openid4vp://authorize";
        let encoded_client_id = encode(&model.audience);
        let presentation_definition_uri = format!("{}/pd/{}", &host_url, model.state);
        let encoded_presentation_definition_uri = encode(&presentation_definition_uri);
        let response_uri = format!("{}/verify/{}", &host_url, model.state);
        let encoded_response_uri = encode(&response_uri);
        let response_type = "vp_token";
        let response_mode = "direct_post";
        let client_id_scheme = "redirect_uri";

        // TODO let client_metadata = r#"{"authorization_encrypted_response_alg":"ECDH-ES","authorization_encrypted_response_enc":"A256GCM"}"#;

        let uri = format!("{}?response_type={}&client_id={}&response_mode={}&presentation_definition_uri={}&client_id_scheme={}&nonce={}&response_uri={}",
                          base_url,
                          response_type,
                          encoded_client_id,
                          response_mode,
                          encoded_presentation_definition_uri,
                          client_id_scheme,
                          model.nonce,
                          encoded_response_uri);
        info!("Uri generated successfully: {}", uri);

        uri
    }

    fn generate_issuing_uri(&self, id: String) -> anyhow::Result<String> {
        let semi_host = format!("{}/api/v1", self.config.get_host_without_protocol());
        let host = format!("{}/api/v1", self.config.get_host());
        let (semi_host, host) = match self.config.is_local() {
            true => {
                let a = semi_host.replace("127.0.0.1", "host.docker.internal");
                let b = host.replace("127.0.0.1", "host.docker.internal");
                (a, b)
            }
            false => (semi_host, host),
        };
        let h_host = format!("{}/credentialOffer?id={}", host, id);
        let encoded_host = encode(h_host.as_str());
        let oidc4vci_uri = format!(
            "openid-credential-offer://{}/authority/?credential_offer_uri={}",
            semi_host, encoded_host
        );
        Ok(oidc4vci_uri)
    }

    fn generate_vpd(&self, ver_model: verification::Model) -> VPDef {
        info!("Generating an vp definition");
        VPDef::new(ver_model.id, ver_model.vc_type)
    }

    fn verify_all(&self, ver_model: &mut verification::Model, vp_token: String) -> anyhow::Result<()> {
        info!("Verifying all");

        let (vcs, holder) = self.verify_vp(ver_model, &vp_token)?;
        for vc in vcs {
            self.verify_vc(&vc, &holder)?;
        }
        info!("VP & VC Validated successfully");

        Ok(())
    }

    fn verify_vp(&self, model: &mut verification::Model, vp_token: &str) -> anyhow::Result<(Vec<String>, String)> {
        info!("Verifying vp");

        model.vpt = Some(vp_token.to_string());
        let (token, kid) = self.validate_token(vp_token, Some(&model.state))?;
        self.validate_nonce(model, &token)?;
        self.validate_sub(model, &token, &kid)?;
        self.validate_vp_id(model, &token)?;
        self.validate_holder(model, &token)?;
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

        info!("VP Verification successful");
        let vcs = self.retrieve_vcs(token)?;

        Ok((vcs, kid))
    }

    fn verify_vc(&self, vc_token: &str, holder: &str) -> anyhow::Result<()> {
        info!("Verifying vc");

        let (token, kid) = self.validate_token(vc_token, None)?;
        self.validate_issuer(&token, &kid)?;
        self.validate_vc_id(&token)?;
        self.validate_vc_sub(&token, holder)?;

        // if issuers_list.contains(kid) {
        //     // TODO
        //     error!("VCT issuer is not on the trusted issuers list");
        //     bail!("VCT issuer is not on the trusted issuers list");
        // }
        // info!("VCT issuer is on the trusted issuers list");

        self.validate_valid_from(&token)?;
        self.validate_valid_until(&token)?;

        info!("VC Verification successful");

        Ok(())
    }

    fn validate_token(&self, vp_token: &str, audience: Option<&str>) -> anyhow::Result<(TokenData<Value>, String)> {
        info!("Validating token");
        let header = jsonwebtoken::decode_header(&vp_token)?;
        let kid_str = match header.kid.as_ref() {
            Some(data) => data,
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    "Jwt does not contain a token".to_string(),
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

        let mut val = Validation::new(alg);

        val.required_spec_claims = HashSet::new();
        val.validate_exp = false;
        val.validate_nbf = true;

        match audience {
            Some(data) => {
                let audience = format!("{}/api/v1/verify/{}", self.config.get_host(), data);
                let audience = match self.config.is_local() {
                    true => audience.replace("127.0.0.1", "host.docker.internal"),
                    false => audience,
                };
                val.validate_aud = true;
                val.set_audience(&[&(audience)]);
            }
            None => {
                val.validate_aud = false;
            }
        };

        let token = match jsonwebtoken::decode::<Value>(&vp_token, &key, &val) {
            Ok(token) => token,
            Err(e) => {
                let error = Errors::security_new(format!("VPT signature is incorrect -> {}", e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        info!("Token signature is correct");
        Ok((token, kid.to_string()))
    }

    fn validate_nonce(&self, model: &verification::Model, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating nonce");

        let nonce = self.get_claim(&token.claims, vec!["nonce"])?;

        if model.nonce != nonce {
            let error = Errors::security_new("Invalid nonce, it does not match".to_string());
            error!("{}", error.log());
            bail!(error);
        }
        info!("VPT Nonce matches");
        Ok(())
    }

    fn validate_sub(
        &self,
        model: &mut verification::Model,
        token: &TokenData<Value>,
        kid: &str,
    ) -> anyhow::Result<()> {
        info!("Validating sub");

        let sub = self.get_claim(&token.claims, vec!["sub"])?;
        let iss = self.get_claim(&token.claims, vec!["iss"])?;

        if sub != iss || iss != kid {
            // VALIDATE HOLDER 1
            let error = Errors::security_new("VPT token issuer, subject & kid does not match".to_string());
            error!("{}", error.log());
            bail!(error);
        }
        info!("VPT issuer, subject & kid matches");

        model.holder = Some(sub.to_string());
        Ok(())
    }

    fn validate_vc_sub(&self, token: &TokenData<Value>, holder: &str) -> anyhow::Result<()> {
        info!("Validating VC subject");

        let sub = self.get_claim(&token.claims, vec!["sub"])?;
        let cred_sub_id = self.get_claim(&token.claims, vec!["vc", "credentialSubject", "id"])?;

        if sub != holder || holder != cred_sub_id {
            let error = Errors::security_new("VCT token sub, credential subject & VP Holder do not match".to_string());
            error!("{}", error.log());
            bail!(error);
        }
        info!("VC Holder Data is Correct");
        Ok(())
    }

    fn validate_vp_id(&self, model: &verification::Model, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating vp id");

        let vp_id = self.get_claim(&token.claims, vec!["vp", "id"])?;

        if model.id != vp_id {
            // VALIDATE ID MATCHES JTI
            let error = Errors::security_new("Invalid id, it does not match".to_string());
            error!("{}", error.log());
            bail!(error);
        }
        info!("Exchange is valid");
        Ok(())
    }

    fn validate_holder(&self, model: &verification::Model, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating holder");

        let vp_holder = self.get_claim(&token.claims, vec!["vp", "holder"])?;

        if model.holder.clone().unwrap() != vp_holder {
            // EXPECTED ALWAYS
            let error = Errors::security_new("Invalid holder, it does not match".to_string());
            error!("{}", error.log());
            bail!(error);
        }
        info!("vp holder matches vpt subject & issuer");
        Ok(())
    }

    fn validate_issuer(&self, token: &TokenData<Value>, kid: &str) -> anyhow::Result<()> {
        info!("Validating issuer");

        let iss = self.get_claim(&token.claims, vec!["iss"])?;
        let vc_iss_id = self.get_claim(&token.claims, vec!["vc", "issuer", "id"])?;

        if iss != kid || kid != vc_iss_id {
            // VALIDATE IF ISSUER IS THE SAME AS KID
            let error = Errors::security_new("VCT token issuer & kid does not match".to_string());
            error!("{}", error.log());
            bail!(error);
        }
        info!("VCT issuer & kid matches");
        Ok(())
    }

    fn validate_vc_id(&self, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating VC id & JTI");

        let jti = self.get_claim(&token.claims, vec!["jti"])?;
        let vc_id = self.get_claim(&token.claims, vec!["vc", "id"])?;

        if jti != vc_id {
            let error = Errors::security_new("VCT jti & VC id do not match".to_string());
            error!("{}", error.log());
            bail!(error);
        }
        info!("VCT jti & VC id match");
        Ok(())
    }

    fn validate_valid_from(&self, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating issuance date");

        let valid_from = self.get_claim(&token.claims, vec!["vc", "validFrom"])?;

        match DateTime::parse_from_rfc3339(&valid_from) {
            Ok(parsed_date) => {
                if parsed_date > Utc::now() {
                    let error = Errors::security_new("VC is not valid yet".to_string());
                    error!("{}", error.log());
                    bail!(error)
                }
            }
            Err(e) => {
                let error = Errors::security_new(format!("VC iat and issuanceDate do not match -> {}", e));
                error!("{}", error.log());
                bail!(error);
            }
        };

        info!("VC validFrom is correct");
        Ok(())
    }

    fn validate_valid_until(&self, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating expiration date");

        let valid_until = self.get_claim(&token.claims, vec!["vc", "validUntil"])?;

        match DateTime::parse_from_rfc3339(&valid_until) {
            Ok(parsed_date) => {
                if Utc::now() > parsed_date {
                    let error = Errors::security_new("VC has expired".to_string());
                    error!("{}", error.log());
                    bail!(error)
                }
            }
            Err(e) => {
                let error = Errors::security_new(format!("VC validUntil has invalid format -> {}", e));
                error!("{}", error.log());
                bail!(error);
            }
        }
        Ok(())
    }

    fn retrieve_vcs(&self, token: TokenData<Value>) -> anyhow::Result<Vec<String>> {
        info!("Retrieving VCs");
        let vcs: Vec<String> = match serde_json::from_value(token.claims["vp"]["verifiableCredential"].clone()) {
            Ok(data) => data,
            Err(e) => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    format!(
                        "VPT does not contain the 'verifiableCredential' field -> {}",
                        e.to_string()
                    ),
                );
                error!("{}", error.log());
                bail!(error);
            }
        };
        Ok(vcs)
    }

    fn get_cred_offer_data(&self, model: request::Model) -> anyhow::Result<VCCredOffer> {
        info!("Retrieving credential offer data");

        let issuer = format!("{}/api/v1", self.config.get_host());
        let issuer = match self.config.is_local() {
            true => issuer.replace("127.0.0.1", "host.docker.internal"),
            false => issuer,
        };

        let token = create_opaque_token();
        let vc_type = VcType::from_str(&model.vc_type)?;

        Ok(VCCredOffer::new(issuer, token, vc_type))
    }

    fn get_issuer_data(&self) -> IssuerMetadata {
        info!("Retrieving issuer data");
        let host = format!("{}/api/v1", self.config.get_host());
        let host = match self.config.is_local() {
            true => host.replace("127.0.0.1", "host.docker.internal"),
            false => host,
        };
        IssuerMetadata::new(&host)
    }

    fn get_oauth_server_data(&self) -> AuthServerMetadata {
        info!("Retrieving oauth server data");

        let host = format!("{}/api/v1", self.config.get_host());
        let host = match self.config.is_local() {
            true => host.replace("127.0.0.1", "host.docker.internal"),
            false => host,
        };

        AuthServerMetadata::new(&host)
    }

    fn get_token(&self) -> anyhow::Result<Value> {
        info!("Giving token");
        // TODO
        let resp = serde_json::json!({
            "access_token": "MOCK_TOKEN_123",
            "token_type": "Bearer",
            "expires_in": 3600
        }
        );

        Ok(resp)
    }

    fn issue_cred(&self) -> anyhow::Result<Value> {
        info!("Issuing cred");
        // TODO
        let vc_jwt = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJodHRwOi8vbG9jYWxob3N0OjcwMDIvZHJhZnQxMyIsInN1YiI6ImRpZDpleGFtcGxlOmFiY2RlZjEyMzQ1NiIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIl0sImlkIjoidXJuOnV1aWQ6MTIzNDU2NzgtMTIzNC0xMjM0LTEyMzQtMTIzNDU2Nzg5YWJjIiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIklkZW50aXR5Q3JlZGVudGlhbCJdLCJpc3VlciI6Imh0dHA6Ly9sb2NhbGhvc3Q6NzAwMi9kcmFmdDEzIiwiaXNzdWFuY2VEYXRlIjoiMjAyNS0xMC0yM1QxNDowMDowMFoiLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpleGFtcGxlOmFiY2RlZjEyMzQ1NiIsIm5hbWUiOiJKb2huIERvZSJ9fX0.MOCK_SIGNATURE";
        let response = serde_json::json!({
            "format": "jwt_vc_json",
            "credential": vc_jwt,
        });

        Ok(response)
    }
}

impl OidcService {
    fn get_claim(&self, claims: &Value, path: Vec<&str>) -> anyhow::Result<String> {
        let mut node = claims;
        let field = path.last().unwrap_or(&"unknown");
        for key in path.iter() {
            node = match node.get(key) {
                Some(data) => data,
                None => {
                    let error = Errors::format_new(BadFormat::Received, format!("Missing field '{}'", key));
                    error!("{}", error.log());
                    bail!(error)
                }
            };
        }
        let data = match node.as_str() {
            Some(data) => data.to_string(),
            None => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    format!("Field '{}' not a string", field),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };
        Ok(data)
    }
}
