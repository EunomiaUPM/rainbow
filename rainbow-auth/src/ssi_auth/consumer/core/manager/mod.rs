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

pub mod manager;
use crate::ssi_auth::consumer::core::types::MatchingVCs;
use axum::async_trait;
pub use manager::Manager;
use rainbow_db::auth_consumer::entities::auth_verification::Model;
use serde_json::Value;

#[async_trait]
pub trait RainbowSSIAuthConsumerWalletTrait: Send + Sync {
    async fn register_wallet(&self) -> anyhow::Result<()>;
    async fn login_wallet(&mut self) -> anyhow::Result<()>;
    async fn logout_wallet(&mut self) -> anyhow::Result<()>;
    async fn get_wallet_info(&mut self) -> anyhow::Result<()>;
    async fn get_wallet_dids(&mut self) -> anyhow::Result<()>;
    async fn onboard(&mut self) -> anyhow::Result<()>; //ESTA
    async fn token_expired(&self) -> anyhow::Result<bool>;
    async fn update_token(&mut self) -> anyhow::Result<()>;
    async fn ok(&mut self) -> anyhow::Result<()>;
}

#[async_trait]
pub trait RainbowSSIAuthConsumerManagerTrait: Send + Sync {
    async fn request_access(&self, url: String, provider: String, actions: Vec<String>) -> anyhow::Result<Model>;
    async fn join_exchange(&self, exchange_url: String) -> anyhow::Result<String>;
    async fn parse_vpd(&self, vpd_as_string: String) -> anyhow::Result<Value>;
    async fn match_vc4vp(&self, vp_def: Value) -> anyhow::Result<Vec<MatchingVCs>>;
    async fn present_vp(&self, preq: String, creds: Vec<String>) -> anyhow::Result<()>;
    async fn continue_request(&self, id: String, nonce: String) -> anyhow::Result<()>;
}
