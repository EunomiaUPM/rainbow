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
use crate::ssi::provider::types::vcs::{VPDef, VerifyPayload};
use jsonwebtoken::TokenData;
use rainbow_db::auth::provider::entities::{recv_interaction, recv_verification};
use serde_json::Value;
use axum::async_trait;

#[async_trait]
pub trait VerifierTrait: Send + Sync + 'static {
    fn start(&self, id: &str) -> recv_verification::NewModel;
    fn generate_uri(&self, ver_model: &recv_verification::Model) -> String;
    fn get_vpd(&self, ver_model: &recv_verification::Model) -> VPDef;
    fn verify_all(&self, ver_model: &mut recv_verification::Model, payload: &VerifyPayload) -> anyhow::Result<()>;
    fn verify_vp(&self, model: &mut recv_verification::Model, vp_token: &str) -> anyhow::Result<(Vec<String>, String)>;
    fn verify_vc(&self, vc_token: &str, holder: &str) -> anyhow::Result<()>;
    fn validate_token(&self, vp_token: &str, audience: Option<&str>) -> anyhow::Result<(TokenData<Value>, String)>;
    fn validate_nonce(&self, model: &recv_verification::Model, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn validate_vp_subject(
        &self,
        model: &mut recv_verification::Model,
        token: &TokenData<Value>,
        kid: &str,
    ) -> anyhow::Result<()>;
    fn validate_vc_sub(&self, token: &TokenData<Value>, holder: &str) -> anyhow::Result<()>;
    fn validate_vp_id(&self, model: &recv_verification::Model, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn validate_holder(&self, model: &recv_verification::Model, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn validate_issuer(&self, token: &TokenData<Value>, kid: &str) -> anyhow::Result<()>;
    fn validate_vc_id(&self, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn validate_valid_from(&self, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn validate_valid_until(&self, token: &TokenData<Value>) -> anyhow::Result<()>;
    fn retrieve_vcs(&self, token: TokenData<Value>) -> anyhow::Result<Vec<String>>;
    async fn end_verification(&self, model: &recv_interaction::Model) -> anyhow::Result<(Option<String>)>;
}
