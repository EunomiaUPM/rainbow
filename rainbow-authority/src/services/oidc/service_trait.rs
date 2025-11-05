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
use std::format;
use anyhow::bail;
use chrono::{DateTime, Utc};
use crate::data::entities::{auth_request, auth_verification};
use crate::types::enums::vc_type::VcType;
use crate::types::vcs::VPDef;
use jsonwebtoken::TokenData;
use serde_json::Value;
use tracing::{error, info};
use crate::errors::Errors;
use crate::types::enums::errors::BadFormat;
use crate::types::oidc::{AuthServerMetadata, IssuerMetadata, VCCredOffer};
use crate::utils::create_opaque_token;

pub trait OidcServiceTrait {
    fn start_vp(&self, id: &str, vc_type: VcType) -> anyhow::Result<auth_verification::NewModel>;
    fn generate_verification_uri(&self, model: auth_verification::Model) -> String;
    fn generate_issuing_uri(&self, id: String) -> anyhow::Result<String>;
    fn generate_vpd(&self, ver_model: auth_verification::Model) -> VPDef;
    fn verify_all(&self, ver_model: &mut auth_verification::Model, vp_token: String) -> anyhow::Result<()>;
    fn verify_vp(&self, model: &mut auth_verification::Model, vp_token: &str) -> anyhow::Result<(Vec<String>, String)>;
    fn verify_vc(&self, vc_token: &str, holder: &str) -> anyhow::Result<()>;
    fn validate_token(
        &self,
        vp_token: &str,
        audience: Option<&str>,
    ) -> anyhow::Result<(TokenData<Value>, String)>;
    fn validate_nonce(&self, model: &auth_verification::Model, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn validate_sub(
        &self,
        model: &mut auth_verification::Model,
        token: &TokenData<Value>,
        kid: &str,
    ) -> anyhow::Result<()>;
    fn validate_vc_sub(&self, token: &TokenData<Value>, holder: &str) -> anyhow::Result<()>;
    fn validate_vp_id(&self, model: &auth_verification::Model, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn validate_holder(&self, model: &auth_verification::Model, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn validate_issuer(&self, token: &TokenData<Value>, kid: &str) -> anyhow::Result<()>;
    fn validate_vc_id(&self, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn validate_valid_from(&self, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn validate_valid_until(&self, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn retrieve_vcs(&self, token: TokenData<Value>) -> anyhow::Result<Vec<String>>;
    fn get_cred_offer_data(&self, model: auth_request::Model) -> anyhow::Result<VCCredOffer>;
    fn get_issuer_data(&self) -> IssuerMetadata;
    fn get_oauth_server_data(&self) -> AuthServerMetadata;
    fn get_token(&self) -> anyhow::Result<Value>;
    fn issue_cred(&self) -> anyhow::Result<Value>;
    // async fn generate_fake_issuing_uri(&self, vc_data: VCIData) -> anyhow::Result<String>;
}
