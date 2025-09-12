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

use crate::data::entities::{auth_interaction, auth_request, auth_verification, minions};
use crate::types::gnap::{GrantRequest, GrantResponse};
use axum::async_trait;
use serde_json::Value;

#[async_trait]
pub trait AuthorityTrait: Send + Sync {
    async fn manage_access(&self, payload: GrantRequest) -> anyhow::Result<GrantResponse>;
    async fn save_minion(&self, mate: minions::NewModel) -> anyhow::Result<minions::Model>;
    async fn generate_uri(&self, ver_model: auth_verification::Model) -> anyhow::Result<String>;
    async fn validate_continue_request(
        &self,
        cont_id: String,
        interact_ref: String,
        token: String,
    ) -> anyhow::Result<auth_interaction::Model>;
    async fn continue_req(&self, int_model: auth_interaction::Model) -> anyhow::Result<auth_request::Model>;
    async fn retrieve_data(
        &self,
        req_model: auth_request::Model,
        int_model: auth_interaction::Model,
    ) -> anyhow::Result<minions::NewModel>;
    async fn generate_vp_def(&self, state: String) -> anyhow::Result<Value>;
    async fn verify_all(&self, state: String, vp_token: String) -> anyhow::Result<String>;
    async fn verify_vp(
        &self,
        model: auth_verification::Model,
        vp_token: String,
    ) -> anyhow::Result<(Vec<String>, String)>;
    async fn verify_vc(&self, vc_token: String, vp_holder: String) -> anyhow::Result<()>;
    async fn end_verification(&self, id: String) -> anyhow::Result<Option<String>>;
}
