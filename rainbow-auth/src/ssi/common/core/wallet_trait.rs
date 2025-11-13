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
use crate::ssi::common::services::wallet::WalletServiceTrait;
use crate::ssi::common::types::wallet::{DidsInfo, KeyDefinition};
use rainbow_db::auth::common::entities::mates;
use serde_json::Value;
use std::sync::Arc;
use axum::async_trait;
use rainbow_db::auth::common::traits::MatesTrait;

#[async_trait]
pub trait CoreWalletTrait: Send + Sync + 'static {
    fn wallet(&self) -> Arc<dyn WalletServiceTrait>;
    fn mates(&self) -> Arc<dyn MatesTrait>;
    async fn register(&self) -> anyhow::Result<()> {
        self.wallet().register().await
    }
    async fn login(&self) -> anyhow::Result<()> {
        self.wallet().login().await
    }
    async fn logout(&self) -> anyhow::Result<()> {
        self.wallet().logout().await
    }
    async fn onboard(&self) -> anyhow::Result<mates::Model> {
        let mate = self.wallet().onboard().await?;
        self.mates().force_create(mate).await
    }
    async fn partial_onboard(&self) -> anyhow::Result<()> {
        self.wallet().partial_onboard().await
    }
    async fn get_did_doc(&self) -> anyhow::Result<Value> {
        self.wallet().get_did_doc().await
    }
    async fn register_key(&self) -> anyhow::Result<()> {
        self.wallet().register_key().await
    }
    async fn register_did(&self) -> anyhow::Result<()> {
        self.wallet().register_did().await
    }
    async fn delete_key(&self, key: KeyDefinition) -> anyhow::Result<()> {
        self.wallet().delete_key(key).await
    }
    async fn delete_did(&self, did_info: DidsInfo) -> anyhow::Result<()> {
        self.wallet().delete_did(did_info).await
    }
}
