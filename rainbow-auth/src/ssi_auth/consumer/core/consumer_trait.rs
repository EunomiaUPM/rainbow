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

use crate::ssi_auth::types::{MatchingVCs, RedirectResponse};
use axum::async_trait;
use rainbow_db::auth_consumer::entities::{auth_request, mates};
use serde_json::Value;

#[async_trait]
pub trait RainbowSSIAuthConsumerManagerTrait: Send + Sync {
    async fn request_onboard_provider(
        &self,
        url: String,
        provider_id: String,
        provider_slug: String,
    ) -> anyhow::Result<String>;
    async fn check_callback(&self, id: String, interact_ref: String, hash: String) -> anyhow::Result<()>;
    async fn continue_request(&self, id: String, interact_ref: String) -> anyhow::Result<auth_request::Model>;
    async fn save_mate(&self, mate: mates::NewModel) -> anyhow::Result<mates::Model>;

    // EXTRAS ------------------------------------------------------------------------------------->
    async fn join_exchange(&self, exchange_url: String) -> anyhow::Result<String>;
    async fn parse_vpd(&self, vpd_as_string: String) -> anyhow::Result<Value>;
    async fn match_vc4vp(&self, vp_def: Value) -> anyhow::Result<Vec<MatchingVCs>>;
    async fn present_vp(&self, preq: String, creds: Vec<String>) -> anyhow::Result<RedirectResponse>;
}
