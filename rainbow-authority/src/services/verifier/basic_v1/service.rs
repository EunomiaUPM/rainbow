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

use super::super::VerifierTrait;
use super::config::{BasicVerifierConfig, BasicVerifierConfigTrait};
use crate::data::entities::verification;
use crate::errors::{ErrorLogTrait, Errors};
use crate::types::enums::errors::BadFormat;
use crate::types::enums::vc_type::VcType;
use crate::types::vcs::VPDef;
use crate::utils::{get_claim, get_opt_claim, split_did};
use anyhow::bail;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{DateTime, Utc};
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::{TokenData, Validation};
use serde_json::Value;
use std::collections::HashSet;
use tracing::{error, info};
use urlencoding::encode;

pub struct BasicVerifierService {
    config: BasicVerifierConfig,
}

impl BasicVerifierService {
    pub fn new(config: BasicVerifierConfig) -> BasicVerifierService {
        BasicVerifierService { config }
    }
}

impl VerifierTrait for BasicVerifierService {
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
        let host_url = format!("{}{}/verifier", host_url, self.config.get_api_path());
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
        self.validate_vp_subject(model, &token, &kid)?;
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
                let error = Errors::format_new(BadFormat::Received, "Jwt does not contain a token");
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
                let audience = format!(
                    "{}{}/verifier/verify/{}",
                    self.config.get_host(),
                    self.config.get_api_path(),
                    data
                );
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
                let error = Errors::security_new(&format!("VPT signature is incorrect -> {}", e.to_string()));
                error!("{}", error.log());
                bail!(error);
            }
        };

        info!("Token signature is correct");
        Ok((token, kid.to_string()))
    }

    fn validate_nonce(&self, model: &verification::Model, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating nonce");

        let nonce = get_claim(&token.claims, vec!["nonce"])?;

        if model.nonce != nonce {
            let error = Errors::security_new("Invalid nonce, it does not match");
            error!("{}", error.log());
            bail!(error);
        }
        info!("VPT Nonce matches");
        Ok(())
    }

    fn validate_vp_subject(
        &self,
        model: &mut verification::Model,
        token: &TokenData<Value>,
        kid: &str,
    ) -> anyhow::Result<()> {
        info!("Validating subject");

        let sub = get_opt_claim(&token.claims, vec!["sub"])?;
        let iss = get_opt_claim(&token.claims, vec!["iss"])?;

        match sub {
            Some(sub) => {
                if sub != kid {
                    let error = Errors::security_new("VPT token subject & kid does not match");
                    error!("{}", error.log());
                    bail!(error);
                }
                info!("VPT subject & kid matches");
            }
            None => {}
        }
        match iss {
            Some(iss) => {
                if iss != kid {
                    let error = Errors::security_new("VPT token issuer & kid does not match");
                    error!("{}", error.log());
                    bail!(error);
                }
                info!("VPT issuer & kid matches");
            }
            None => {}
        }

        model.holder = Some(kid.to_string());
        Ok(())
    }

    fn validate_vc_sub(&self, token: &TokenData<Value>, holder: &str) -> anyhow::Result<()> {
        info!("Validating VC subject");

        let sub = get_opt_claim(&token.claims, vec!["sub"])?;
        let cred_sub_id = get_claim(&token.claims, vec!["vc", "CredentialSubject", "id"])?;

        match sub {
            Some(sub) => {
                if sub != holder {
                    let error = Errors::security_new("VCT token sub, credential subject & VP Holder do not match");
                    error!("{}", error.log());
                    bail!(error);
                }
                info!("Sub & Holder match");
            }
            None => {}
        }

        if holder != cred_sub_id {
            let error = Errors::security_new("VCT token sub, credential subject & VP Holder do not match");
            error!("{}", error.log());
            bail!(error);
        }
        info!("Vc Holder & Holder match");
        Ok(())
    }

    fn validate_vp_id(&self, model: &verification::Model, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating vp id");

        let vp_id = get_claim(&token.claims, vec!["vp", "id"])?;

        if model.id != vp_id {
            // VALIDATE ID MATCHES JTI
            let error = Errors::security_new("Invalid id, it does not match");
            error!("{}", error.log());
            bail!(error);
        }
        info!("Exchange is valid");
        Ok(())
    }

    fn validate_holder(&self, model: &verification::Model, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating holder");

        let vp_holder = get_claim(&token.claims, vec!["vp", "holder"])?;

        if model.holder.clone().unwrap() != vp_holder {
            // EXPECTED ALWAYS
            let error = Errors::security_new("Invalid holder, it does not match");
            error!("{}", error.log());
            bail!(error);
        }
        info!("vp holder matches vpt subject & issuer");
        Ok(())
    }

    fn validate_issuer(&self, token: &TokenData<Value>, kid: &str) -> anyhow::Result<()> {
        info!("Validating issuer");

        let iss = get_opt_claim(&token.claims, vec!["iss"])?;
        let vc_iss_id = get_claim(&token.claims, vec!["vc", "issuer", "id"])?;

        match iss {
            Some(iss) => {
                if iss != kid {
                    // VALIDATE IF ISSUER IS THE SAME AS KID
                    let error = Errors::security_new("VCT token issuer & kid does not match");
                    error!("{}", error.log());
                    bail!(error);
                }
                info!("VC iss and kid matches")
            }
            None => {}
        }

        if vc_iss_id != kid {
            // VALIDATE IF ISSUER IS THE SAME AS KID
            let error = Errors::security_new("VCT token issuer & kid does not match");
            error!("{}", error.log());
            bail!(error);
        }
        info!("VC issuer & kid matches");
        Ok(())
    }

    fn validate_vc_id(&self, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating VC id & JTI");

        let vc_id = get_claim(&token.claims, vec!["vc", "id"])?;
        let jti = get_opt_claim(&token.claims, vec!["jti"])?;

        match jti {
            Some(jti) => {
                if jti != vc_id {
                    // VALIDATE ID MATCHES JTI
                    let error = Errors::security_new("Invalid id, it does not match");
                    error!("{}", error.log());
                    bail!(error);
                }
                info!("VCT jti & VC id match");
            }
            None => {}
        }

        Ok(())
    }

    fn validate_valid_from(&self, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating issuance date");

        let valid_from = get_opt_claim(&token.claims, vec!["vc", "validFrom"])?;

        match valid_from {
            Some(valid_from) => {
                match DateTime::parse_from_rfc3339(&valid_from) {
                    Ok(parsed_date) => {
                        if parsed_date > Utc::now() {
                            let error = Errors::security_new("VC is not valid yet");
                            error!("{}", error.log());
                            bail!(error)
                        }
                    }
                    Err(e) => {
                        let error = Errors::security_new(&format!("VC iat and issuanceDate do not match -> {}", e));
                        error!("{}", error.log());
                        bail!(error);
                    }
                };
                info!("VC has started its validity period correct");
            }
            None => {}
        }

        Ok(())
    }

    fn validate_valid_until(&self, token: &TokenData<Value>) -> anyhow::Result<()> {
        info!("Validating expiration date");

        let valid_until = get_opt_claim(&token.claims, vec!["vc", "validUntil"])?;
        // let valid_until = get_claim(&token.claims, vec!["vc", "validUntil"])?;

        match valid_until {
            Some(valid_until) => {
                match DateTime::parse_from_rfc3339(&valid_until) {
                    Ok(parsed_date) => {
                        if Utc::now() > parsed_date {
                            let error = Errors::security_new("VC has expired");
                            error!("{}", error.log());
                            bail!(error)
                        }
                    }
                    Err(e) => {
                        let error = Errors::security_new(&format!("VC validUntil has invalid format -> {}", e));
                        error!("{}", error.log());
                        bail!(error);
                    }
                }
                info!("VC has not expired yet");
            }
            None => {}
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
                    &format!(
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
}
