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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::config::traits::DatabaseConfigTrait;

#[async_trait]
pub trait VaultTrait: Send + Sync + 'static {
    async fn read<T>(&self, mount: Option<&str>, path: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned + Send;
    async fn basic_read(&self, mount: &str, path: &str) -> anyhow::Result<Value>;
    async fn write<T>(&self, mount: Option<&str>, path: &str, secret: &T) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync;
    async fn write_all_secrets(&self) -> anyhow::Result<()>;
    fn secrets(&self) -> anyhow::Result<HashMap<String, Value>>;
    async fn get_db_connection<T>(&self, config: T) -> DatabaseConnection
    where
        T: DatabaseConfigTrait + Send + Sync;
}
