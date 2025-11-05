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
use crate::types::gnap::{GrantRequest, GrantResponse, RefBody};
use crate::types::wallet::{DidsInfo, KeyDefinition};
use axum::async_trait;
use serde_json::Value;
use crate::data::entities::auth_request;
use crate::types::oidc::{AuthServerMetadata, IssuerMetadata, VCCredOffer, WellKnownJwks};
use crate::types::vcs::{VPDef, VcDecisionApproval};

#[async_trait]
pub trait AuthorityTrait {
    async fn wallet_register(&self) -> anyhow::Result<()>;
    async fn wallet_login(&self) -> anyhow::Result<()>;
    async fn wallet_logout(&self) -> anyhow::Result<()>;
    async fn wallet_onboard(&self) -> anyhow::Result<()>;
    async fn wallet_partial_onboard(&self) -> anyhow::Result<()>;
    async fn register_key(&self) -> anyhow::Result<()>;
    async fn register_did(&self) -> anyhow::Result<()>;
    async fn delete_key(&self, key_definition: KeyDefinition) -> anyhow::Result<()>;
    async fn delete_did(&self, dids_info: DidsInfo) -> anyhow::Result<()>;
    async fn did_json(&self) -> anyhow::Result<Value>;
    async fn vc_access_request(&self, payload: GrantRequest) -> anyhow::Result<GrantResponse>;
    async fn vc_continue_request(&self, cont_id: String, payload: RefBody, token: String) -> anyhow::Result<String>;
    async fn generate_vp_def(&self, state: String) -> anyhow::Result<VPDef>;
    async fn verify(&self, state: String, vp_token: String) -> anyhow::Result<Option<String>>;
    async fn get_cred_offer_data(&self, id: String) -> anyhow::Result<VCCredOffer>;
    fn issuer(&self) -> IssuerMetadata;
    fn oauth_server(&self) -> AuthServerMetadata;
    fn jwks(&self) -> anyhow::Result<WellKnownJwks>;
    fn token(&self) -> Value;
    fn credential(&self) -> Value;
    async fn get_all_req(&self) -> anyhow::Result<Vec<auth_request::Model>>;
    async fn get_one_req(&self, id: String) -> anyhow::Result<auth_request::Model>;
    async fn manage_req(&self, id: String, payload: VcDecisionApproval) -> anyhow::Result<()>;
}
