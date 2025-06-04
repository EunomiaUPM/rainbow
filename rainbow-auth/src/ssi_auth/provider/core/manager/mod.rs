/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under terms of the GNU General Public License as published by
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
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::HeaderMap;
use rainbow_common::auth::gnap::GrantRequest;
use serde_json::Value;
use tracing::info;
use rainbow_common::mates::Mates;

pub mod manager;

#[async_trait]
pub trait RainbowSSIAuthProviderManagerTrait: Send + Sync {
    async fn generate_exchange_uri(&self, payload: GrantRequest) -> anyhow::Result<(String, String, String)>;
    async fn generate_vp_def(&self, state: String) -> anyhow::Result<Value>;
    async fn verify_all(&self, state: String, vp_token: String) -> anyhow::Result<Option<String>>;
    async fn verify_vp(
        &self,
        exchange: String,
        state: String,
        vp_token: String,
    ) -> anyhow::Result<(Vec<String>, String)>;
    async fn verify_vc(&self, vc_token: String, vp_holder: String) -> anyhow::Result<()>;

    async fn continue_req(&self, interact_ref: String) -> anyhow::Result<Value>;
    async fn save_mate(&self, id: String, token: String, token_actions: String) -> anyhow::Result<()>;
}
