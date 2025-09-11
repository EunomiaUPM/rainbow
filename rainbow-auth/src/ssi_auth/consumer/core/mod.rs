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

pub mod impls;
pub mod traits;

use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
use rainbow_common::ssi_wallet::WalletSession;
use rainbow_db::auth_consumer::repo_factory::factory_trait::AuthRepoFactoryTrait;
use reqwest::Client;
use serde_json::Value;
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
    client: Client,
    config: ApplicationConsumerConfig,
    didweb: Value,
}

impl<T> Manager<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    pub fn new(repo: Arc<T>, config: ApplicationConsumerConfig) -> Self {
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
            client,
            config,
            didweb: Value::Null,
        }
    }
}
