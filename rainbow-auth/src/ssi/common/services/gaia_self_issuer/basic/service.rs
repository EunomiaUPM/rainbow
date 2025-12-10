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

use super::super::GaiaSelfIssuerTrait;
use super::config::{GaiaGaiaSelfIssuerConfigTrait, GaiaSelfIssuerConfig};
use crate::ssi::common::types::enums::VcDataModelVersion;
use crate::ssi::common::types::vc_issuing::claims::{VCClaimsV1, VCClaimsV2, VCFromClaimsV1};
use crate::ssi::common::types::vc_issuing::cred_subject::{LegalPersonCredentialSubject, TermsAndConditionsCredSub};
use crate::ssi::common::types::vc_issuing::{
    AuthServerMetadata, GiveVC, IssuerMetadata, IssuingToken, VCCredOffer, VCIssuer, VcType,
};
use anyhow::bail;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rainbow_common::config::traits::ExtraHostsTrait;
use rainbow_common::config::types::HostType;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use serde_json::{json, Value};
use tracing::{error, info};

pub struct BasicGaiaSelfIssuer {
    config: GaiaSelfIssuerConfig,
}

impl BasicGaiaSelfIssuer {
    pub fn new(config: GaiaSelfIssuerConfig) -> BasicGaiaSelfIssuer {
        BasicGaiaSelfIssuer { config }
    }
}

impl GaiaSelfIssuerTrait for BasicGaiaSelfIssuer {
    fn get_cred_offer_data(&self) -> VCCredOffer {
        info!("Retrieving credential offer data");

        let issuer = format!(
            "{}{}/gaia",
            self.config.hosts().get_host(HostType::Http),
            self.config.get_api_path()
        );
        let issuer = match self.config.is_local() {
            true => issuer.replace("127.0.0.1", "host.docker.internal"),
            false => issuer,
        };

        VCCredOffer::new4gaia(&issuer)
    }
    fn get_issuer_data(&self) -> IssuerMetadata {
        info!("Retrieving issuer data");
        let host = format!(
            "{}{}/gaia",
            self.config.hosts().get_host(HostType::Http),
            self.config.get_api_path()
        );
        let host = match self.config.is_local() {
            true => host.replace("127.0.0.1", "host.docker.internal"),
            false => host,
        };
        IssuerMetadata::new(&host)
    }
    fn get_oauth_server_data(&self) -> AuthServerMetadata {
        info!("Retrieving oauth server data");

        let host = format!(
            "{}{}/gaia",
            self.config.hosts().get_host(HostType::Http),
            self.config.get_api_path()
        );
        let host = match self.config.is_local() {
            true => host.replace("127.0.0.1", "host.docker.internal"),
            false => host,
        };

        AuthServerMetadata::new(&host)
    }

    fn get_token(&self) -> IssuingToken {
        info!("Giving token");
        IssuingToken::new4gai()
    }
    fn generate_issuing_uri(&self, id: &str) -> String {
        let semi_host = format!(
            "{}{}/gaia",
            self.config.hosts().get_host(HostType::Http),
            self.config.get_api_path()
        );
        let host = format!(
            "{}{}/gaia",
            self.config.hosts().get_host(HostType::Http),
            self.config.get_api_path()
        );
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
            "openid-credential-offer://{}/?credential_offer_uri={}",
            semi_host, encoded_host
        );
        info!("Issuing uri: {}", uri);
        uri
    }

    fn issue_cred(&self, did: &str) -> anyhow::Result<Value> {
        info!("Issuing cred");

        let legal_id = uuid::Uuid::new_v4().to_string();
        let terms_id = uuid::Uuid::new_v4().to_string();

        let legal_person_subj = serde_json::to_value(LegalPersonCredentialSubject::default4gaia(did))?;
        let terms_subj = serde_json::to_value(TermsAndConditionsCredSub::new4gaia(did))?;
        let now = Utc::now();
        let person_vc = match self.config.get_data_model_version() {
            VcDataModelVersion::V1 => serde_json::to_value(VCClaimsV1 {
                exp: None,
                iat: None,
                iss: None,
                sub: None,
                vc: VCFromClaimsV1 {
                    context: vec!["https://www.w3.org/ns/credentials/v1".to_string()],
                    r#type: vec!["VerifiableCredential".to_string(), VcType::LegalPerson.to_string()],
                    id: legal_id,
                    credential_subject: legal_person_subj,
                    issuer: VCIssuer { id: did.to_string(), name: None },
                    valid_from: Some(now),
                    valid_until: Some(now + Duration::days(365)),
                },
            })?,
            VcDataModelVersion::V2 => serde_json::to_value(VCClaimsV2 {
                exp: None,
                iat: None,
                iss: None,
                sub: None,
                context: vec!["https://www.w3.org/ns/credentials/v2".to_string()],
                r#type: vec!["VerifiableCredential".to_string(), VcType::LegalPerson.to_string()],
                id: legal_id,
                credential_subject: legal_person_subj,
                issuer: VCIssuer { id: did.to_string(), name: None },
                valid_from: Some(now),
                valid_until: Some(now + Duration::days(365)),
            })?,
        };

        let terms_vc = match self.config.get_data_model_version() {
            VcDataModelVersion::V1 => serde_json::to_value(VCClaimsV1 {
                exp: None,
                iat: None,
                iss: None,
                sub: None,
                vc: VCFromClaimsV1 {
                    context: vec!["https://www.w3.org/ns/credentials/v1".to_string()],
                    r#type: vec!["VerifiableCredential".to_string(), VcType::TermsAndConditions.to_string()],
                    id: terms_id,
                    credential_subject: terms_subj,
                    issuer: VCIssuer { id: did.to_string(), name: None },
                    valid_from: Some(now),
                    valid_until: Some(now + Duration::days(365)),
                },
            })?,
            VcDataModelVersion::V2 => serde_json::to_value(VCClaimsV2 {
                exp: None,
                iat: None,
                iss: None,
                sub: None,
                context: vec!["https://www.w3.org/ns/credentials/v2".to_string()],
                r#type: vec!["VerifiableCredential".to_string(), VcType::TermsAndConditions.to_string()],
                id: terms_id,
                credential_subject: terms_subj,
                issuer: VCIssuer { id: did.to_string(), name: None },
                valid_from: Some(now),
                valid_until: Some(now + Duration::days(365)),
            })?,
        };

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(did.to_string());

        let key = match EncodingKey::from_rsa_pem(self.config.get_priv_key()?.as_bytes()) {
            Ok(data) => data,
            Err(e) => {
                let error = CommonErrors::format_new(
                    BadFormat::Unknown,
                    &format!("Error parsing private key: {}", e.to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        let person_vc_jwt = match encode(&header, &person_vc, &key) {
            Ok(data) => data,
            Err(e) => {
                let error = CommonErrors::format_new(
                    BadFormat::Unknown,
                    &format!("Error parsing private key: {}", e.to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };
        let terms_vc_jwt = match encode(&header, &terms_vc, &key) {
            Ok(data) => data,
            Err(e) => {
                let error = CommonErrors::format_new(
                    BadFormat::Unknown,
                    &format!("Error parsing private key: {}", e.to_string()),
                );
                error!("{}", error.log());
                bail!(error)
            }
        };

        Ok(json!({
            "credential_responses": vec![
                GiveVC {
                    format: "jwt_vc_json".to_string(),
                    credential: person_vc_jwt,
                },
                GiveVC {
                    format: "jwt_vc_json".to_string(),
                    credential: terms_vc_jwt,
                }
            ]
        }))
    }
}
