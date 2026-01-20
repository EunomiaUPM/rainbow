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

use anyhow::bail;
use async_trait::async_trait;
use sea_orm::{Database, DatabaseConnection};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{error, info};
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
        let mount_name = expect_from_env("VAULT_MOUNT");

        let existing_mounts = mount::list(&*self.client).await.map_err(|e| {
            error!("Error listing mounts: {}", e);
            CommonErrors::vault_new(e.to_string())
        })?;

        let mount_path = format!("{}/", mount_name);
        if !existing_mounts.contains_key(&mount_path) {
            let mut opts = HashMap::new();
            opts.insert("version".to_string(), "2".to_string());
            let mut data = EnableEngineRequestBuilder::default();
            let data = data.options(opts);

            mount::enable(&*self.client, &mount_name, "kv", Some(data)).await.map_err(|e| {
                let error = CommonErrors::vault_new(e.to_string());
                error!("{}", error.log());
                error
            })?;

            info!("Mount '{}' created successfully", mount_name);
        } else {
            info!("Mount '{}' already exists, omitting step", mount_name);
        }

        // 3. Escribir secretos
        for (path, secret) in Self::secrets()? {
            self.write(Some(&mount_name), &path, &secret).await?
        }
        Ok(())
    }
    fn secrets() -> anyhow::Result<HashMap<String, Value>> {
        let mut map: HashMap<String, Value> = HashMap::new();

        let secret_path = PathBuf::from(expect_from_env("VAULT_PATH")).join("secrets");
        let config_path = PathBuf::from(expect_from_env("VAULT_PATH")).join("config");

        Self::insert_json(&mut map, secret_path.join("db.json"), "VAULT_DB", true)?;
        Self::insert_json(&mut map, secret_path.join("wallet.json"), "VAULT_WALLET", false)?;

        Self::insert_pem(&mut map, secret_path.join("private_key.pem"), "VAULT_F_PRIV_KEY")?;
        Self::insert_pem(&mut map, secret_path.join("public_key.pem"), "VAULT_F_PUB_PKEY")?;
        Self::insert_pem(&mut map, secret_path.join("cert.pem"), "VAULT_F_CERT")?;

        Self::insert_pem(&mut map, config_path.join("vault-cert.pem"), "VAULT_CLIENT_CERT")?;
        Self::insert_pem(&mut map, config_path.join("vault-key.pem"), "VAULT_CLIENT_KEY")?;

        let secret_path = expect_from_env("SECRET_PATH");
        let config_path = expect_from_env("CONFIG_PATH");

        // DB -----------------------------------------------
        let binding = PathBuf::from(&secret_path).join("db.json");
        let to_read_db = binding.to_str().expect("Error parsing db path");
        let db_path = expect_from_env("VAULT_DB");
        let db_json = read_json(to_read_db)?;
        map.insert(db_path, db_json);

        // WALLET  -----------------------------------------------
        let binding = PathBuf::from(&secret_path).join("wallet.json");
        let to_read_wallet = binding.to_str().expect("Error parsing wallet path");
        if let Ok(data) = read_json(to_read_wallet) {
            let db_path = expect_from_env("VAULT_WALLET");
            map.insert(db_path, data);
        }

        // PRIV_KEY
        let binding = PathBuf::from(&secret_path).join("private_key.pem");
        let to_read_f_pkey = binding.to_str().expect("Error parsing priv key path");
        let priv_key_path = expect_from_env("VAULT_F_PRIV_KEY");
        let data = read(to_read_f_pkey)?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        map.insert(priv_key_path, data);

        // PUB_KEY
        let binding = PathBuf::from(&secret_path).join("public_key.pem");
        let to_read_f_pkey = binding.to_str().expect("Error parsing f pub key path");
        let pub_key_path = expect_from_env("VAULT_F_PUB_PKEY");
        let data = read(to_read_f_pkey)?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        map.insert(pub_key_path, data);

        // CERT
        let binding = PathBuf::from(&secret_path).join("cert.pem");
        let to_read_f_pkey = binding.to_str().expect("Error parsing f cert path");
        let cert_path = expect_from_env("VAULT_F_CERT");
        let data = read(to_read_f_pkey)?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        map.insert(cert_path, data);

        // REAL CERT
        let binding = PathBuf::from(&config_path).join("vault-cert.pem");
        let to_read_cert = binding.to_str().expect("Error parsing cert path");
        let cert_path = expect_from_env("VAULT_CLIENT_CERT");
        let data = read(to_read_cert)?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        map.insert(cert_path, data);

        // REAL KEY
        let binding = PathBuf::from(&config_path).join("vault-key.pem");
        let to_read_pkey = binding.to_str().expect("Error parsing f cert path");
        let key_path = expect_from_env("VAULT_CLIENT_KEY");
        let data = read(to_read_pkey)?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        map.insert(key_path, data);

        Ok(map)
    }

    fn insert_json<T>(mapa: &mut HashMap<String, Value>, to_read: T, env: &str, required: bool) -> anyhow::Result<()>
    where
        T: AsRef<Path>
    {
        let vault_path = expect_from_env(env);
        let db_json = match read_json(to_read) {
            Ok(db_json) => db_json,
            Err(e) => {
                if required {
                    bail!(e)
                } else {
                    return Ok(());
                }
            }
        };
        mapa.insert(vault_path, db_json);
        Ok(())
    }

    fn insert_pem<T>(mapa: &mut HashMap<String, Value>, to_read: T, env: &str) -> anyhow::Result<()>
    where
        T: AsRef<Path>
    {
        let vault_path = expect_from_env(env);
        let data = read(to_read)?;
        let data = serde_json::to_value(PemHelper::new(data))?;
        mapa.insert(vault_path, data);
        Ok(())
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
