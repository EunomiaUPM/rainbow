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
use crate::ssi::common::services::client::ClientServiceTrait;
use crate::ssi::common::services::wallet::WalletServiceTrait;
use crate::ssi::provider::config::AuthProviderConfig;
use rainbow_db::auth::provider::factory::factory_trait::AuthProviderRepoTrait;
use std::sync::Arc;

pub struct AuthProvider {
    wallet: Arc<dyn WalletServiceTrait>,
    repo: Arc<dyn AuthProviderRepoTrait>,
    client: Arc<dyn ClientServiceTrait>,
    config: Arc<AuthProviderConfig>,
}

impl AuthProvider {
    pub fn new(
        wallet: Arc<dyn WalletServiceTrait>,
        repo: Arc<dyn AuthProviderRepoTrait>,
        client: Arc<dyn ClientServiceTrait>,
        config: Arc<AuthProviderConfig>,
    ) -> AuthProvider {
        AuthProvider { wallet, repo, client, config }
    }

    pub fn wallet(&self) -> Arc<dyn WalletServiceTrait> {
        self.wallet.clone()
    }
    pub fn repo(&self) -> Arc<dyn AuthProviderRepoTrait> {
        self.repo.clone()
    }
    pub fn client(&self) -> Arc<dyn ClientServiceTrait> {
        self.client.clone()
    }
    pub fn config(&self) -> Arc<AuthProviderConfig> {
        self.config.clone()
    }
}
