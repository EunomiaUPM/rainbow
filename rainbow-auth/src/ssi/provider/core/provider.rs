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
use rainbow_db::auth::common::traits::{MatesTrait, ReqInteractionTrait, ReqVcTrait, ReqVerificationTrait};
use crate::ssi::common::core::{CoreMateTrait, CoreVcRequesterTrait, CoreWalletTrait};
use crate::ssi::common::services::vc_request::VcRequesterTrait;
use crate::ssi::provider::core::traits::CoreProviderTrait;

pub struct AuthProvider {
    wallet: Arc<dyn WalletServiceTrait>,
    vc_requester: Arc<dyn VcRequesterTrait>,
    repo: Arc<dyn AuthProviderRepoTrait>,
    client: Arc<dyn ClientServiceTrait>,
    config: AuthProviderConfig,
}

impl AuthProvider {
    pub fn new(
        wallet: Arc<dyn WalletServiceTrait>,
        vc_requester: Arc<dyn VcRequesterTrait>,
        repo: Arc<dyn AuthProviderRepoTrait>,
        client: Arc<dyn ClientServiceTrait>,
        config: AuthProviderConfig,
    ) -> AuthProvider {
        AuthProvider { wallet, vc_requester, repo, client, config }
    }
}



impl CoreProviderTrait for AuthProvider {}

impl CoreWalletTrait for AuthProvider {
    fn wallet(&self) -> Arc<dyn WalletServiceTrait> {
        self.wallet.clone()
    }

    fn mate_repo(&self) -> Arc<dyn MatesTrait> {
        self.repo.mates()
    }
}

impl CoreVcRequesterTrait for AuthProvider {
    fn vc_req(&self) -> Arc<dyn VcRequesterTrait> {
        self.vc_requester.clone()
    }

    fn vc_req_repo(&self) -> Arc<dyn ReqVcTrait> {
        self.repo.vc_req().clone()
    }

    fn verification_req_repo(&self) -> Arc<dyn ReqVerificationTrait> {
        self.repo.verification_req().clone()
    }

    fn interaction_req_repo(&self) -> Arc<dyn ReqInteractionTrait> {
        self.repo.interaction_req().clone()
    }
}

impl CoreMateTrait for AuthProvider {
    fn mate_repo(&self) -> Arc<dyn MatesTrait> {
        self.repo.mates().clone()
    }
}
