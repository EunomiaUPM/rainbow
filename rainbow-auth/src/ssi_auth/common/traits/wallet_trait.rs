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
use crate::ssi_auth::common::types::oidc::{CredentialOfferResponse, Vpd};
use crate::ssi_auth::common::types::ssi::dids::DidsInfo;
use crate::ssi_auth::common::types::ssi::keys::KeyDefinition;
use crate::ssi_auth::common::types::ssi::other::{MatchingVCs, RedirectResponse};
use crate::ssi_auth::common::types::ssi::wallet::WalletInfo;
use axum::async_trait;
use serde_json::Value;

#[mockall::automock]
#[async_trait]
pub trait RainbowSSIAuthWalletTrait: Send + Sync {
    // BASIC
    async fn register_wallet(&self) -> anyhow::Result<()>;
    async fn login_wallet(&self) -> anyhow::Result<()>;
    async fn logout_wallet(&self) -> anyhow::Result<()>;
    async fn onboard_wallet(&self) -> anyhow::Result<()>;
    async fn partial_onboard(&self) -> anyhow::Result<()>;
    // GET FROM MANAGER (It gives a cloned Value, not a reference)
    async fn get_wallet(&self) -> anyhow::Result<WalletInfo>;
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
    async fn delete_key(&self, key: KeyDefinition) -> anyhow::Result<()>;
    async fn delete_did(&self, did_info: DidsInfo) -> anyhow::Result<()>;
    // OIDC
    async fn resolve_credential_offer(&self, uri: String) -> anyhow::Result<CredentialOfferResponse>;
    async fn resolve_credential_issuer(&self, issuer_uri: String) -> anyhow::Result<()>;
    async fn use_offer_req(&self, uri: String, pin: String) -> anyhow::Result<()>;
    async fn join_exchange(&self, exchange_url: String) -> anyhow::Result<String>;
    async fn parse_vpd(&self, vpd_as_string: String) -> anyhow::Result<Vpd>;
    async fn get_matching_vcs(&self, vpd: Vpd) -> anyhow::Result<Vec<String>>;
    async fn match_vc4vp(&self, vp_def: Value) -> anyhow::Result<Vec<MatchingVCs>>;
    async fn present_vp(&self, preq: String, creds: Vec<String>) -> anyhow::Result<RedirectResponse>;
    // OTHER
    async fn token_expired(&self) -> anyhow::Result<bool>;
    async fn update_token(&self) -> anyhow::Result<()>;
    async fn ok(&self) -> anyhow::Result<()>;
}
