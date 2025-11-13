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
use crate::ssi::common::core::CoreWalletTrait;
use crate::ssi::common::services::client::ClientServiceTrait;
use crate::ssi::common::services::wallet::WalletServiceTrait;
use crate::ssi::consumer::config::AuthConsumerConfigTrait;
use crate::ssi::consumer::core::traits::CoreConsumerTrait;
use rainbow_db::auth::common::traits::MatesRepoTrait;
use rainbow_db::auth::consumer::factory::AuthConsumerRepoTrait;
use std::sync::Arc;

pub struct AuthConsumer {
    wallet: Arc<dyn WalletServiceTrait>,
    repo: Arc<dyn AuthConsumerRepoTrait>,
    client: Arc<dyn ClientServiceTrait>,
    config: Arc<dyn AuthConsumerConfigTrait>,
}

impl AuthConsumer {
    pub fn new(
        wallet: Arc<dyn WalletServiceTrait>,
        repo: Arc<dyn AuthConsumerRepoTrait>,
        client: Arc<dyn ClientServiceTrait>,
        config: Arc<dyn AuthConsumerConfigTrait>,
    ) -> AuthConsumer {
        AuthConsumer { wallet, repo, client, config }
    }
}
impl CoreConsumerTrait for AuthConsumer {}

impl CoreWalletTrait for AuthConsumer {
    fn wallet(&self) -> Arc<dyn WalletServiceTrait> {
        self.wallet.clone()
    }

    fn mates(&self) -> Arc<dyn MatesRepoTrait> {
        self.repo.mates().clone()
    }
}
