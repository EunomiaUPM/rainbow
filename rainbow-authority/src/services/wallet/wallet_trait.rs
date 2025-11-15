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
use crate::data::entities::minions::NewModel;
use crate::types::issuing::WellKnownJwks;
use crate::types::wallet::{DidsInfo, KeyDefinition, WalletInfo, WalletSession};
use axum::async_trait;
use serde_json::Value;

#[async_trait]
pub trait WalletTrait: Send + Sync + 'static {
    // BASIC
    async fn register(&self) -> anyhow::Result<()>;
    async fn login(&self) -> anyhow::Result<()>;
    async fn logout(&self) -> anyhow::Result<()>;
    async fn onboard(&self) -> anyhow::Result<NewModel>;
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
    fn get_jwks_data(&self) -> anyhow::Result<WellKnownJwks>;
    // REGISTER STUFF IN WALLET
    async fn register_key(&self) -> anyhow::Result<()>;
    async fn register_did(&self) -> anyhow::Result<()>;
    async fn set_default_did(&self) -> anyhow::Result<()>;
    // DELETE STUFF FROM WALLET
    async fn delete_key(&self, key: KeyDefinition) -> anyhow::Result<()>;
    async fn delete_did(&self, did_info: DidsInfo) -> anyhow::Result<()>;
}
