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

pub mod impls;
pub mod traits;

use crate::ssi_auth::common::types::ssi::{keys::KeyDefinition, wallet::WalletSession};
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_db::auth_provider::repo_factory::factory_trait::AuthRepoFactoryTrait;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Manager<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    pub wallet_session: Mutex<WalletSession>,
    pub wallet_onboard: bool,
    pub repo: Arc<T>,
    pub key_data: Mutex<Vec<KeyDefinition>>,
    client: Client,
    config: ApplicationProviderConfig,
}

impl<T> Manager<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    pub fn new(repo: Arc<T>, config: ApplicationProviderConfig) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self {
            wallet_session: Mutex::new(WalletSession {
                account_id: None,
                token: None,
                token_exp: None,
                wallets: Vec::new(),
            }),
            wallet_onboard: false,
            repo,
            key_data: Mutex::new(Vec::new()),
            client,
            config,
        }
    }
}
