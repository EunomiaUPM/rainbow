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

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{Database, DatabaseConnection};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tracing::error;
use vaultrs::api::sys::requests::EnableEngineRequestBuilder;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;
use vaultrs::sys::mount;

use super::super::secrets::{DbSecrets, PemHelper};
use super::super::VaultTrait;
use crate::config::traits::DatabaseConfigTrait;
use crate::errors::{CommonErrors, ErrorLog};
use crate::utils::{expect_from_env, read, read_json};

#[derive(Clone)]
pub struct VaultService {
    client: Arc<VaultClient>,
}

impl VaultService {
    pub fn new() -> Self {
        let settings = VaultClientSettingsBuilder::default()
            .build()
            .map_err(|e| {
                let error = CommonErrors::vault_new(e.to_string());
                error!("{}", error.log());
                error
            })
            .expect("Error creating vault settings");

        let client = VaultClient::new(settings)
            .map_err(|e| {
                let error = CommonErrors::vault_new(e.to_string());
                error!("{}", error.log());
                error
            })
            .expect("Error creating the client vault");

        Self { client: Arc::new(client) }
    }
}

impl VaultService {}

#[async_trait]
impl VaultTrait for VaultService {
    async fn read<T>(&self, mount: Option<&str>, path: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let basic_mount = expect_from_env("VAULT_MOUNT");
        let mount = mount.unwrap_or(&basic_mount);
        let secret = self.basic_read(mount, path).await?;
        let secret = serde_json::from_value(secret)?;
        Ok(secret)
    }
    async fn basic_read(&self, mount: &str, path: &str) -> anyhow::Result<Value> {
        let secret = kv2::read(&*self.client, mount, path).await.map_err(|e| {
            let error = CommonErrors::vault_new(e.to_string());
            error!("{}", error.log());
            error
        })?;

        Ok(secret)
    }
    async fn write<T>(&self, mount: Option<&str>, path: &str, secret: &T) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync,
    {
        let basic_mount = expect_from_env("VAULT_MOUNT");
        let mount = mount.unwrap_or(&basic_mount);
        kv2::set(&*self.client, mount, path, secret).await.map_err(|e| {
            let error = CommonErrors::vault_new(e.to_string());
            error!("{}", error.log());
            error
        })?;

        Ok(())
    }
    async fn write_all_secrets(&self) -> anyhow::Result<()> {
        let mount = expect_from_env("VAULT_MOUNT");

        let mut opts = HashMap::new();
        opts.insert("version".to_string(), "2".to_string());
        let mut data = EnableEngineRequestBuilder::default();
        let data = data.options(opts);

        mount::enable(&*self.client, &mount, "kv", Some(data)).await.map_err(|e| {
            let error = CommonErrors::vault_new(e.to_string());
            error!("{}", error.log());
            error
        })?;

        for (path, secret) in self.secrets()? {
            self.write(Some(&mount), &path, &secret).await?
        }
        Ok(())
    }
    fn secrets(&self) -> anyhow::Result<HashMap<String, Value>> {
        let mut map: HashMap<String, Value> = HashMap::new();

        // DB -----------------------------------------------
        let db_path = expect_from_env("VAULT_DB");
        let db_json = read_json("/vault/secrets/db.json")?;
        map.insert(db_path, db_json);

        // WALLET  -----------------------------------------------
        if let Ok(data) = read_json("/vault/secrets/wallet.json") {
            let db_path = expect_from_env("VAULT_WALLET");
            map.insert(db_path, data);
        }

        // PRIV_KEY
        let priv_key_path = expect_from_env("VAULT_F_PRIV_KEY");
        let data = read("/vault/secrets/private_key.pem")?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        map.insert(priv_key_path, data);

        // PUB_KEY
        let pub_key_path = expect_from_env("VAULT_F_PUB_PKEY");
        let data = read("/vault/secrets/public_key.pem")?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        map.insert(pub_key_path, data);

        // CERT
        let cert_path = expect_from_env("VAULT_F_CERT");
        let data = read("/vault/secrets/cert.pem")?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        map.insert(cert_path, data);

        // REAL CERT
        let cert_path = expect_from_env("VAULT_CLIENT_CERT");
        let data = read("/vault/config/vault-cert.pem")?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        map.insert(cert_path, data);

        // REAL KEY
        let key_path = expect_from_env("VAULT_CLIENT_KEY");
        let data = read("/vault/config/vault-key.pem")?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        map.insert(key_path, data);

        Ok(map)
    }
    async fn get_db_connection<T>(&self, config: T) -> DatabaseConnection
    where
        T: DatabaseConfigTrait + Send + Sync,
    {
        let db_path = expect_from_env("VAULT_DB");

        let db_secrets: DbSecrets = self.read(None, &db_path).await.expect("Not able to retrieve env files");
        Database::connect(config.get_full_db_url(db_secrets)).await.expect("Database can't connect")
    }
}
