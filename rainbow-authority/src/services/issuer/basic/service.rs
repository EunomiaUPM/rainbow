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

use super::super::IssuerServiceTrait;
use super::config::{BasicIssuerConfig, BasicIssuerConfigTrait};
use crate::data::entities::{issuing, request};
use crate::types::enums::vc_type::VcType;
use crate::types::issuing::{AuthServerMetadata, IssuerMetadata, IssuingToken, VCCredOffer};
use crate::utils::create_opaque_token;
use serde_json::Value;
use tracing::info;
use urlencoding;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

pub struct BasicIssuerService {
    config: BasicIssuerConfig,
}

impl BasicIssuerService {
    pub fn new(config: BasicIssuerConfig) -> BasicIssuerService {
        BasicIssuerService { config }
    }
}

impl IssuerServiceTrait for BasicIssuerService {
    fn start_vci(&self, model: &request::Model) -> issuing::NewModel {
        info!("Starting OIDC4VCI");
        issuing::NewModel {
            id: model.id.clone(),
            vc_type: model.vc_type.clone(),
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

        let token = create_opaque_token();
        println!("{}", token);
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

    fn get_token(&self) -> IssuingToken {
        info!("Giving token");
        IssuingToken::new()
    }

    fn issue_cred(&self) -> anyhow::Result<Value> {
        info!("Issuing cred");

        let vc_jwt = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJodHRwOi8vbG9jYWxob3N0OjcwMDIvZHJhZnQxMyIsInN1YiI6ImRpZDpleGFtcGxlOmFiY2RlZjEyMzQ1NiIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIl0sImlkIjoidXJuOnV1aWQ6MTIzNDU2NzgtMTIzNC0xMjM0LTEyMzQtMTIzNDU2Nzg5YWJjIiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIklkZW50aXR5Q3JlZGVudGlhbCJdLCJpc3VlciI6Imh0dHA6Ly9sb2NhbGhvc3Q6NzAwMi9kcmFmdDEzIiwiaXNzdWFuY2VEYXRlIjoiMjAyNS0xMC0yM1QxNDowMDowMFoiLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpleGFtcGxlOmFiY2RlZjEyMzQ1NiIsIm5hbWUiOiJKb2huIERvZSJ9fX0.MOCK_SIGNATURE";
        let response = serde_json::json!({
            "format": "jwt_vc_json",
            "credential": vc_jwt,
        });

        Ok(response)
    }
}
