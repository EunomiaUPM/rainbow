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
use std::collections::HashSet;
use anyhow::bail;
use crate::data::entities::{auth_interaction, auth_request, auth_verification, minions};
use crate::types::gnap::{GrantRequest, GrantResponse};
use crate::types::manager::VcManager;
use axum::async_trait;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::Validation;
use serde_json::Value;
use crate::errors::Errors;
use crate::errors::helpers::BadFormat;
use crate::utils::split_did;

#[async_trait]
pub trait AuthorityTrait: Send + Sync {
    async fn manage_access(&self, payload: GrantRequest) -> anyhow::Result<GrantResponse>;
    async fn save_minion(&self, mate: minions::NewModel) -> anyhow::Result<minions::Model>;
    async fn generate_verification_uri(&self, ver_model: auth_verification::Model) -> anyhow::Result<String>;
    async fn generate_issuing_uri(
        &self,
        id: String,
        name: String,
        website: String,
        fake: bool,
    ) -> anyhow::Result<String>;
    async fn validate_continue_request(
        &self,
        cont_id: String,
        interact_ref: String,
        token: String,
    ) -> anyhow::Result<auth_interaction::Model>;
    async fn continue_req(&self, int_model: auth_interaction::Model) -> anyhow::Result<auth_request::Model>;
    async fn retrieve_data(&self, req_model: auth_request::Model, vc_token: String) -> anyhow::Result<minions::NewModel>;
    async fn generate_vp_def(&self, state: String) -> anyhow::Result<Value>;
    async fn verify_all(&self, state: String, vp_token: String) -> anyhow::Result<String>;
    async fn verify_vp(
        &self,
        model: auth_verification::Model,
        vp_token: String,
    ) -> anyhow::Result<(Vec<String>, String)>;
    async fn verify_vc(&self, vc_token: String, vp_holder: String) -> anyhow::Result<()>;
    async fn end_verification(&self, id: String) -> anyhow::Result<Option<String>>;
    async fn manage_vc_request(&self, id: String, payload: VcManager) -> anyhow::Result<()>;
    async fn manage_callback(&self, id: String, payload: Value) -> anyhow::Result<()>;
    async fn retrieve_holder(&self, vc_token: String) -> anyhow::Result<String>;
}
