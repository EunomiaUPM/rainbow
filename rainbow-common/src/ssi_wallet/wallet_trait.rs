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

use axum::async_trait;
use serde_json::Value;
#[async_trait]
pub trait RainbowSSIAuthWalletTrait: Send + Sync {
    async fn register_wallet(&self) -> anyhow::Result<()>;
    async fn login_wallet(&self) -> anyhow::Result<()>;
    async fn logout_wallet(&self) -> anyhow::Result<()>;
    async fn get_wallet_info(&self) -> anyhow::Result<()>;
    async fn get_wallet_dids(&self) -> anyhow::Result<()>;
    async fn onboard(&self) -> anyhow::Result<()>; //ESTA
    async fn token_expired(&self) -> anyhow::Result<bool>;
    async fn update_token(&self) -> anyhow::Result<()>;
    async fn ok(&self) -> anyhow::Result<()>;
    async fn didweb(&self) -> anyhow::Result<Value>;
}