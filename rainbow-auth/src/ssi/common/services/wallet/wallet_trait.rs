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
use std::{format, vec};
use anyhow::bail;
use rainbow_db::auth::common::entities::mates;
use crate::ssi::common::types::wallet::{CredentialOfferResponse, DidsInfo, KeyDefinition, MatchVCsRequest, MatchingVCs, OidcUri, RedirectResponse, Vpd, WalletInfo, WalletSession};
use axum::async_trait;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderMap;
use serde_json::Value;
use tracing::{debug, error, info};
use url::Url;
use urlencoding::decode;
use rainbow_common::errors::CommonErrors;
use rainbow_common::errors::helpers::BadFormat;
use crate::ssi::common::errors::AuthErrors;
use crate::ssi::common::types::enums::request::Body;
use crate::ssi::common::utils::get_query_param;

#[async_trait]
pub trait WalletServiceTrait: Send + Sync + 'static {
    // BASIC
    async fn register(&self) -> anyhow::Result<()>;
    async fn login(&self) -> anyhow::Result<()>;
    async fn logout(&self) -> anyhow::Result<()>;
    async fn onboard(&self) -> anyhow::Result<mates::NewModel>;
    async fn partial_onboard(&self) -> anyhow::Result<()>;
    // GET FROM MANAGER (It gives a cloned Value, not a reference)
    async fn get_wallet(&self) -> anyhow::Result<WalletInfo>;
    async fn first_wallet_mut(&self) -> anyhow::Result<tokio::sync::MutexGuard<'_, WalletSession>>;
    async fn get_did(&self) -> anyhow::Result<String>;
    async fn get_token(&self) -> anyhow::Result<String>;
    async fn get_did_doc(&self) -> anyhow::Result<Value>;
    async fn get_key(&self) -> anyhow::Result<KeyDefinition>;
    // RETRIEVE FROM WALLET
    async fn retrieve_wallet_info(&self) -> anyhow::Result<()>;
    async fn retrieve_keys(&self) -> anyhow::Result<()>;
    async fn retrieve_wallet_dids(&self) -> anyhow::Result<()>;
    // REGISTER STUFF IN WALLET
    async fn register_key(&self) -> anyhow::Result<()>;
    async fn register_did(&self) -> anyhow::Result<()>;
    async fn set_default_did(&self) -> anyhow::Result<()>;
    // DELETE STUFF FROM WALLET
    async fn delete_key(&self, key: &KeyDefinition) -> anyhow::Result<()>;
    async fn delete_did(&self, did_info: &DidsInfo) -> anyhow::Result<()>;
    async fn resolve_credential_offer(&self, payload: &OidcUri) -> anyhow::Result<CredentialOfferResponse>;
    async fn resolve_credential_issuer(&self, cred_offer: &CredentialOfferResponse) -> anyhow::Result<Value>;
    async fn use_offer_req(&self, payload: &OidcUri, cred_offer: &CredentialOfferResponse) -> anyhow::Result<()>;
    async fn get_vpd(&self, payload: &OidcUri) -> anyhow::Result<Vpd>;
    fn parse_vpd(&self, vpd_as_string: &str) -> anyhow::Result<Vpd>;
    async fn get_matching_vcs(&self, vpd: &Vpd) -> anyhow::Result<Vec<String>>;
    async fn match_vc4vp(&self, vp_def: Value) -> anyhow::Result<Vec<MatchingVCs>>;
    async fn present_vp(&self, payload: &OidcUri, vcs_id: Vec<String>) -> anyhow::Result<Option<String>>;
}
