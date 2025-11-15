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
use super::super::IssuerTrait;
use super::config::{BasicIssuerConfig, BasicIssuerConfigTrait};
use crate::data::entities::{issuing, request};
use crate::errors::{ErrorLogTrait, Errors};
use crate::types::enums::errors::BadFormat;
use crate::types::enums::vc_type::VcType;
use crate::types::issuing::{AuthServerMetadata, CredentialRequest, DidPossession, GiveVC, IssuerMetadata, IssuingToken, VCCredOffer};
use crate::types::vcs::cred_subject::{CredentialSubject4DataSpace, CredentialSubject4Identity};
use crate::types::vcs::{VCClaimsV1, VCClaimsV2, VCFromClaimsV1, VCIssuer};
use crate::utils::{create_opaque_token, get_from_opt, has_expired, is_active, validate_token};
use anyhow::bail;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{error, info};
use urlencoding;

pub struct BasicIssuerService {
    config: BasicIssuerConfig,
}

impl BasicIssuerService {
    pub fn new(config: BasicIssuerConfig) -> BasicIssuerService {
        BasicIssuerService { config }
    }
}

impl IssuerTrait for BasicIssuerService {
    fn start_vci(&self, model: &request::Model) -> issuing::NewModel {
        info!("Starting OIDC4VCI");
        let uri = model.vc_uri.clone().unwrap(); // EXPECTED ALWAYS
        let host = format!("{}/api/v1", self.config.get_host());
        let aud = match self.config.is_local() {
            true => host.replace("127.0.0.1", "host.docker.internal"),
            false => host,
        };

        issuing::NewModel {
            id: model.id.clone(),
            name: model.participant_slug.clone(),
            vc_type: model.vc_type.clone(),
            uri,
            aud,
        }
    }

    fn generate_issuing_uri(&self, id: &str) -> String {
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
        let h_host = format!("{}/credentialOffer?id={}", host, &id);
        let encoded_host = urlencoding::encode(h_host.as_str());
        let uri = format!(
            "openid-credential-offer://{}/authority/?credential_offer_uri={}",
            semi_host, encoded_host
        );
        info!("Issuing uri: {}", uri);
        uri
    }

    fn get_cred_offer_data(&self, model: &issuing::Model) -> anyhow::Result<VCCredOffer> {
        info!("Retrieving credential offer data");

        let issuer = format!("{}/api/v1", self.config.get_host());
        let issuer = match self.config.is_local() {
            true => issuer.replace("127.0.0.1", "host.docker.internal"),
            false => issuer,
        };

        let vc_type = VcType::from_str(&model.vc_type)?;

        let offer = match model.step {
            true => VCCredOffer::new(issuer, model.tx_code.clone(), vc_type),
            false => VCCredOffer::new(issuer, model.pre_auth_code.clone(), vc_type),
        };

        Ok(offer)
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

    fn get_token(&self, model: &issuing::Model) -> IssuingToken {
        info!("Giving token");
        IssuingToken::new(model.token.clone())
    }
    fn validate_token_req(&self, model: &issuing::Model, tx_code: &str, pre_auth_code: &str) -> anyhow::Result<()> {
        info!("Validating token request");

        if model.tx_code != tx_code {
            let error = Errors::forbidden_new("tx_code does not match");
            error!("{}", error.log());
            bail!(error)
        }
        if model.pre_auth_code != pre_auth_code {
            let error = Errors::forbidden_new("pre_auth_code does not match");
            error!("{}", error.log());
            bail!(error)
        }

        Ok(())
    }

    fn issue_cred(&self, model: &mut issuing::Model, did: &str) -> anyhow::Result<GiveVC> {
        info!("Issuing cred");

        let credential_subject = match VcType::from_str(&model.vc_type)? {
            VcType::DataSpaceParticipant => serde_json::to_value(CredentialSubject4DataSpace::new(
                get_from_opt(&model.did, "did")?,
                model.name.clone(),
            ))?,
            VcType::Identity => serde_json::to_value(CredentialSubject4Identity::new(
                get_from_opt(&model.did, "did")?,
                model.name.clone(),
            ))?,
        };

        let now = Utc::now();
        let claims = VCClaimsV1 {
            exp: None,
            iat: None,
            iss: None,
            sub: None,
            vc: VCFromClaimsV1 {
                context: vec!["https://www.w3.org/ns/credentials/v2".to_string()],
                r#type: vec!["VerifiableCredential".to_string(), model.vc_type.clone()],
                id: model.credential_id.clone(),
                credential_subject,
                issuer: VCIssuer {
                    id: did.to_string(),
                    name: "RainbowAuthority".to_string(),
                },
                valid_from: Some(now),
                valid_until: Some(now + Duration::days(365)),
            },
        };
        // let claims = VCClaimsV2 {
        //     exp: None,
        //     iat: None,
        //     iss: None,
        //     sub: None,
        //     context: vec!["https://www.w3.org/ns/credentials/v2".to_string()],
        //     r#type: vec!["VerifiableCredential".to_string(), model.vc_type.clone()],
        //     id: model.credential_id.clone(),
        //     credential_subject,
        //     issuer: VCIssuer {
        //         id: did.to_string(),
        //         name: "RainbowAuthority".to_string(),
        //     },
        //     valid_from: Some(now),
        //     valid_until: Some(now + Duration::days(365)),
        // };

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(did.to_string());

        let key = match EncodingKey::from_rsa_pem(self.config.get_priv_key()?.as_bytes()) {
            Ok(data) => data,
            Err(e) => {
                let error = Errors::format_new(
                    BadFormat::Unknown,
                    &format!("Error parsing private key: {}", e.to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };
        let vc_jwt = match encode(&header, &claims, &key) {
            Ok(data) => data,
            Err(e) => {
                let error = Errors::format_new(
                    BadFormat::Unknown,
                    &format!("Error parsing private key: {}", e.to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        model.credential = Some(vc_jwt.clone());
        Ok(GiveVC {
            format: "jwt_vc_json".to_string(),
            credential: vc_jwt,
        })


    }

    fn validate_cred_req(
        &self,
        model: &mut issuing::Model,
        cred_req: &CredentialRequest,
        token: &str,
    ) -> anyhow::Result<()> {
        info!("Validating credential request");

        if model.token != token {
            let error = Errors::forbidden_new("tx_code does not match");
            error!("{}", error.log());
            bail!(error)
        }

        if cred_req.format != "jwt_vc_json" {
            let error = Errors::format_new(
                BadFormat::Received,
                &format!("Cannot issue a credentia with format: {}", cred_req.format),
            );
            error!("{}", error.log());
            bail!(error)
        }

        if cred_req.proof.proof_type != "jwt" {
            let error = Errors::format_new(
                BadFormat::Received,
                &format!(
                    "Cannot validate proof with type: {}",
                    cred_req.proof.proof_type
                ),
            );
            error!("{}", error.log());
            bail!(error)
        }

        let (token, kid) = validate_token::<DidPossession>(&cred_req.proof.jwt, Some(&model.aud))?;
        self.validate_did_possession(&token, &kid)?;
        model.did = Some(kid);
        is_active(token.claims.iat)?;
        has_expired(token.claims.exp)?;
        Ok(())
    }

    fn validate_did_possession(&self, token: &TokenData<DidPossession>, kid: &str) -> anyhow::Result<()> {
        info!("Validating did possession");
        if token.claims.iss != token.claims.sub && token.claims.sub != kid {
            let error = Errors::forbidden_new("Invalid proof of did possession");
            error!("{}", error.log());
            bail!(error)
        }
        Ok(())
    }
}
