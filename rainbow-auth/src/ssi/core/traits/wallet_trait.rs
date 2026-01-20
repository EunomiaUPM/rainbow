/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use axum::async_trait;
use serde_json::Value;

use crate::ssi::data::entities::mates;
use crate::ssi::services::repo::repo_trait::AuthRepoTrait;
use crate::ssi::services::wallet::WalletServiceTrait;
use crate::ssi::types::wallet::{DidsInfo, KeyDefinition, OidcUri};

#[async_trait]
pub trait CoreWalletTrait: Send + Sync + 'static {
    fn wallet(&self) -> Arc<dyn WalletServiceTrait>;
    fn repo(&self) -> Arc<dyn AuthRepoTrait>;
    async fn register(&self) -> anyhow::Result<()> { self.wallet().register().await }
    async fn login(&self) -> anyhow::Result<()> { self.wallet().login().await }
    async fn logout(&self) -> anyhow::Result<()> { self.wallet().logout().await }
    async fn onboard(&self) -> anyhow::Result<mates::Model> {
        let mate = self.wallet().onboard().await?;
        self.repo().mates().force_create(mate).await
    }
    async fn partial_onboard(&self) -> anyhow::Result<()> { self.wallet().partial_onboard().await }
    async fn get_did_doc(&self) -> anyhow::Result<Value> { self.wallet().get_did_doc().await }
    async fn register_key(&self) -> anyhow::Result<()> { self.wallet().register_key().await }
    async fn register_did(&self) -> anyhow::Result<()> { self.wallet().register_did().await }
    async fn delete_key(&self, key: KeyDefinition) -> anyhow::Result<()> {
        self.wallet().delete_key(&key).await
    }
    async fn delete_did(&self, did_info: DidsInfo) -> anyhow::Result<()> {
        self.wallet().delete_did(&did_info).await
    }
    async fn process_oidc4vci(&self, payload: OidcUri) -> anyhow::Result<()> {
        let cred_offer = self.wallet().resolve_credential_offer(&payload).await?;
        let _issuer_metadata = self.wallet().resolve_credential_issuer(&cred_offer).await?;
        self.wallet().use_offer_req(&payload, &cred_offer).await
    }
    async fn process_oidc4vp(&self, payload: OidcUri) -> anyhow::Result<Option<String>> {
        let vpd = self.wallet().get_vpd(&payload).await?;
        let vcs_id = self.wallet().get_matching_vcs(&vpd).await?;
        self.wallet().present_vp(&payload, vcs_id).await
    }
}
