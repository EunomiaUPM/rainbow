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
use std::format;
use anyhow::bail;
use axum::async_trait;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::HeaderMap;
use serde_json::Value;
use tracing::{error, info};
use crate::errors::CommonErrors;
use crate::errors::helpers::MissingAction;
use crate::ssi_wallet::{DidsInfo, KeyDefinition, WalletInfo};

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
    // DELETE STUFF FROM WALLET
    async fn delete_key(&self, key: KeyDefinition) -> anyhow::Result<()>;
    async fn delete_did(&self, did_info: DidsInfo) -> anyhow::Result<()>;
    // OTHER
    async fn token_expired(&self) -> anyhow::Result<bool>;
    async fn update_token(&self) -> anyhow::Result<()>;
    async fn ok(&self) -> anyhow::Result<()>;
}