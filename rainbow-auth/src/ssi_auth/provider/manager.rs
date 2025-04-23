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

use anyhow::bail;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use chrono::format::Fixed::TimezoneName;
use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken;
use jsonwebtoken::jwk::{Jwk, KeyAlgorithm};
use jsonwebtoken::{Algorithm, Validation};
use log::error;
use rainbow_common::auth::GrantRequest;
use rainbow_common::config::config::{get_provider_audience, get_provider_portal_url};
use rainbow_db::auth_provider::repo::AUTH_PROVIDER_REPO;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::{json, Value};
use std::collections::HashSet;
use tracing::field::debug;
use tracing::{debug, info};
use urlencoding::{decode, encode};

pub struct Manager {}
impl Manager {
    pub fn new() -> Manager {
        Manager {}
    }
    pub async fn generate_exchange_uri(
        &self,
        payload: GrantRequest,
    ) -> anyhow::Result<(String, String, String)> {
        info!("Generating exchange URI");

        if !payload.interact.start.contains(&"oidc4vp".to_string()) {
            bail!(
                "Interact Method {} Not supported ",
                payload.interact.start.first().unwrap()
            );
        }
        let actions =
            payload.access_token.access.actions.unwrap_or_else(|| vec![String::from("talk")]);
        let (auth_model, interaction_model, verification_model) =
            match AUTH_PROVIDER_REPO.create_auth(payload.client, actions, payload.interact).await {
                Ok(model) => {
                    info!("exchange saved successfully");
                    model
                }
                Err(e) => bail!("Unable to save exchange in db: {}", e),
            };

        let base_url = "openid4vp://authorize";
        let provider_url = get_provider_portal_url().unwrap();

        let client_id = format!("{}/verify", &provider_url);
        let encoded_client_id = encode(&client_id);

        let state = verification_model.state;
        let nonce = verification_model.nonce;

        // COMPLETAR
        let presentation_definition_uri = format!("{}/pd/{}", &provider_url, state);
        let encoded_presentation_definition_uri = encode(&presentation_definition_uri);

        // COMPLETAR
        let response_uri = format!("{}/verify/{}", &provider_url, state);
        let encoded_response_uri = encode(&response_uri);

        let response_type = "vp_token";
        let response_mode = "direct_post";
        let clientid_scheme = "redirect_uri";

        // let client_metadata = r#"{"authorization_encrypted_response_alg":"ECDH-ES","authorization_encrypted_response_enc":"A256GCM"}"#;

        let uri = format!("{}?response_type={}&client_id={}&response_mode={}&presentation_definition_uri={}&client_id_scheme={}&nonce={}&response_uri={}", base_url, response_type, encoded_client_id, response_mode, encoded_presentation_definition_uri, clientid_scheme, nonce, encoded_response_uri);
        info!("uri generated successfully: {}", uri);
        Ok((auth_model.id, uri, interaction_model.nonce))
    }

    pub async fn generate_vp_def(state: String) -> anyhow::Result<Value> {
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

        let id = match AUTH_PROVIDER_REPO.get_auth_by_state(state.clone()).await {
            Ok(id) => id,
            Err(e) => bail!("No exchange for state {}", state),
        };

        Ok(json!({
          "id": id,
          "input_descriptors": [
            {
              "id": "VerifiableId",
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
                      "pattern": "VerifiableId"
                    }
                  }
                ]
              }
            }
          ]
        }))
    }

    pub async fn verifyAll(&self, state: String, vptoken: String) -> anyhow::Result<()> {
        let exchange = match AUTH_PROVIDER_REPO.get_auth_by_state(state.clone()).await {
            Ok(auth) => auth,
            Err(e) => bail!("No exchange for state {}", state),
        };

        let (vcts, holder) = match self.verifyVP(exchange.clone(), vptoken).await {
            Ok(v) => v,
            Err(e) => {
                match AUTH_PROVIDER_REPO.update_verification_result(exchange, false).await {
                    Ok(_) => {}
                    Err(e) => {
                        bail!("{}", e)
                    }
                }
                bail!("{}", e)
            }
        };
        for cred in vcts {
            match self.verifyVC(cred, holder.clone()).await {
                Ok(()) => {}
                Err(e) => {
                    match AUTH_PROVIDER_REPO.update_verification_result(exchange, false).await {
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

        match AUTH_PROVIDER_REPO.update_verification_result(exchange, true).await {
            Ok(_) => {}
            Err(e) => {
                bail!("{}", e)
            }
        }
        Ok(())
    }

    pub async fn verifyVP(
        &self,
        exchange: String,
        vptoken: String,
    ) -> anyhow::Result<(Vec<String>, String)> {
        info!("Verifying VP");
        let header = jsonwebtoken::decode_header(&vptoken)?;
        let kid_str = header.kid.as_ref().unwrap();
        let (kid, kid_id) = split_did(kid_str.as_str()); // COMPLETAR KIDID

        let vec = URL_SAFE_NO_PAD.decode(&(kid.replace("did:jwk:", "")))?;
        let mut jwk: Jwk = serde_json::from_slice(&vec)?;

        let key = jsonwebtoken::DecodingKey::from_jwk(&jwk)?;

        let mut val = Validation::new(Algorithm::EdDSA); // AJUSTAR
        val.required_spec_claims = HashSet::new();
        val.validate_aud = true; // VALIDATE AUDIENCE
        val.set_audience(&[&(get_provider_audience()?)]);
        val.validate_exp = false;
        val.validate_nbf = true; // VALIDATE NBF

        let token = match jsonwebtoken::decode::<Value>(&vptoken, &key, &val) {
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

        let auth_ver = match AUTH_PROVIDER_REPO
            .get_av_by_id_update_holder(
                id.to_string(),
                vptoken,
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
            bail!("Invalid exchange");
            error!("Invalid exchange");
        }
        info!("Exchange is valid");

        if auth_ver.holder.unwrap() != token.claims["vp"]["holder"].as_str().unwrap() {
            error!("VP id does not match sub & issuer");
            bail!("VP id does not match sub & issuer");
        }
        info!("vp holder matches vpt subject & issuer");
        info!("VP Verification successful");

        let vct: Vec<String> =
            match serde_json::from_value(token.claims["vp"]["verifiableCredential"].clone()) {
                Ok(vc) => vc,
                Err(_) => {
                    error!("VPresentation is based on a nonexistent credential");
                    bail!("VPresentation is based on a nonexistent credential")
                }
            };
        Ok((vct, kid.to_string()))
    }

    pub async fn verifyVC(&self, vctoken: String, VP_holder: String) -> anyhow::Result<()> {
        info!("Verifying VC");
        let header = jsonwebtoken::decode_header(&vctoken)?;
        let kid_str = header.kid.as_ref().unwrap();
        let (kid, kid_id) = split_did(kid_str.as_str()); // COMPLETAR KIDID

        let vec = URL_SAFE_NO_PAD.decode(&(kid.replace("did:jwk:", "")))?;
        let mut jwk: Jwk = serde_json::from_slice(&vec)?;

        let key = jsonwebtoken::DecodingKey::from_jwk(&jwk)?;

        let mut val = Validation::new(Algorithm::ES256); // AJUSTAR
        val.required_spec_claims = HashSet::new();
        val.validate_aud = false;
        val.set_audience(&[&(get_provider_audience()?)]);
        val.validate_exp = false; // SI CREDS EXPIRAN COMPLETAR
        val.validate_nbf = true; // VALIDATE NBF

        let token = match jsonwebtoken::decode::<Value>(&vctoken, &key, &val) {
            Ok(token) => token,
            Err(e) => {
                error!("VCT token signature is incorrect");
                bail!("VCT token signature is incorrect {}", e)
            }
        };

        info!("VCT token signature is correct");
        debug!("{:#?}", token);

        if token.claims["iss"].as_str().unwrap() != kid
            || kid != token.claims["vc"]["issuer"].as_str().unwrap()
        {
            // VALIDATE IF ISSUER IS THE SAME AS KID
            error!("VCT token issuer & kid does not match");
            bail!("VCT token issuer & kid does not match");
        }
        info!("VCT issuer & kid matches");

        // if issuers_list.contains(kid) {
        //     // COMPLETAR
        //     error!("VCT issuer is not on the trusted issuers list");
        //     bail!("VCT issuer is not on the trusted issuers list");
        // }
        // info!("VCT issuer is on the trusted issuers list");

        if token.claims["sub"].as_str().unwrap() != &VP_holder
            || &VP_holder != token.claims["vc"]["credentialSubject"]["id"].as_str().unwrap()
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
}

fn split_did(did: &str) -> (&str, Option<&str>) {
    match did.split_once('#') {
        Some((didkid, id)) => (didkid, Some(id)),
        None => (did, None),
    }
}

fn compare_with_margin(iat: i64, issuance_date: &str, margin_seconds: i64) -> (bool, String) {
    let datetime = match DateTime::from_timestamp(iat, 0) {
        Some(dt) => dt,
        None => return (true, "Invalid iat field".to_string()),
    };

    let parsed_date = match DateTime::parse_from_rfc3339(issuance_date) {
        Ok(dt) => dt,
        Err(_) => {
            return (
                true,
                "IssuanceDate is not with the correct format".to_string(),
            )
        }
    };
    let parsed_date_utc = parsed_date.with_timezone(&Utc);

    if parsed_date_utc > Utc::now() {
        return (true, "Issuance date has not reached yet".to_string());
    }

    if (datetime - parsed_date_utc).num_seconds().abs() > margin_seconds {
        return (true, "IssuanceDate & iat field do not match".to_string());
    }

    (false, "Ignore this".to_string())
}
