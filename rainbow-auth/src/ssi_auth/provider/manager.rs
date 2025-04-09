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

use anyhow::bail;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use jsonwebtoken;
use jsonwebtoken::jwk::{Jwk, KeyAlgorithm};
use jsonwebtoken::{Algorithm, Validation};
use log::error;
use rainbow_common::config::config::get_provider_portal_url;
use rainbow_db::ssi_auth_provider::repo::SSI_AUTH_PR_REPO;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::{json, Value};
use std::collections::HashSet;
use tracing::info;
use urlencoding::{decode, encode};

pub struct Manager {}
impl Manager {
    pub fn new() -> Manager {
        Manager {}
    }
    pub async fn generate_exchange_uri() -> anyhow::Result<String> {
        info!("Generating exchange URI");

        let model = match SSI_AUTH_PR_REPO.create_ssi_auth_data().await {
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

        let state = model.state.to_string();
        let nonce: String = model.nonce.to_string();

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

        Ok(uri)
    }

    pub fn gererate_vp_def() -> Value {
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
        json!({
          "id": "tDpde2Fy81Ls",
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
        })
    }

    pub async fn verify(&self, vptoken: String) -> anyhow::Result<()> {
        let header = jsonwebtoken::decode_header(&vptoken)?;
        println!("{:#?}", header);
        let kid = header.kid.unwrap().replace("did:jwk:", "").replace("#0", ""); // # significa el primero del diddoc

        println!();
        println!("vpt: {}", vptoken);
        println!();
        let kk = URL_SAFE_NO_PAD.decode(kid).unwrap();
        let mut jwk: Jwk = serde_json::from_slice(&kk).unwrap();
        println!();
        println!("ANTES {:#?}", jwk);
        println!();
        // jwk.common.key_algorithm = Some(KeyAlgorithm::EdDSA);
        println!();
        println!("DESPUES {:#?}", jwk);
        println!();

        let key = jsonwebtoken::DecodingKey::from_jwk(&jwk).unwrap();
        // let qq = vptoken + "as";

        let mut val = Validation::new(Algorithm::EdDSA);
        val.required_spec_claims = HashSet::new();
        val.validate_aud = true;
        val.set_audience(&["http://host.docker.internal:1234/verify"]);
        val.validate_exp = false;
        val.validate_nbf = true;
        // val.set_issuer(&["did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5Iiwia2lkIjoidmZuXzVpNDNPMlJzN3ZGanJwSUJVbG45THdHMi1BTE15ZlEtRzRfRl84VSIsIngiOiI2T21SUHVsRnczTVg0NG1iWjBiZWRjVUxLU3JQZFpscXFzSXJ3Z0JoeUxzIn0"]);
        println!();
        println!("VAL {:#?}", val);
        println!();

        let token = jsonwebtoken::decode::<Value>(&vptoken, &key, &val).unwrap();
        println!("{:#?}", token);

        Ok(())
    }

    pub fn verifyAll(&self, vptoken: String) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn verifyVP(&self, vptoken: String) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn verifyVC(&self, vptoken: String) -> anyhow::Result<()> {
        Ok(())
    }
}
