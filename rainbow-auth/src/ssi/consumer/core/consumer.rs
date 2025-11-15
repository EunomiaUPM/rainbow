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
use crate::ssi::common::core::{CoreMateTrait, CoreVcRequesterTrait, CoreWalletTrait};
use crate::ssi::common::services::client::ClientServiceTrait;
use crate::ssi::common::services::vc_requester::VcRequesterTrait;
use crate::ssi::common::services::wallet::WalletServiceTrait;
use crate::ssi::consumer::config::AuthConsumerConfigTrait;
use crate::ssi::consumer::core::traits::{CoreConsumerTrait, CoreOnboarderTrait};
use crate::ssi::consumer::services::onboarder::OnboarderTrait;
use rainbow_db::auth::common::traits::{MatesTrait, ReqInteractionTrait, ReqVcTrait, ReqVerificationTrait};
use rainbow_db::auth::consumer::factory::AuthConsumerRepoTrait;
use std::sync::Arc;

pub struct AuthConsumer {
    wallet: Arc<dyn WalletServiceTrait>,
    vc_requester: Arc<dyn VcRequesterTrait>,
    onboarder: Arc<dyn OnboarderTrait>,
    repo: Arc<dyn AuthConsumerRepoTrait>,
    #[allow(dead_code)] // as an orchestrator, it should have access even though it's not used
    client: Arc<dyn ClientServiceTrait>,
    config: Arc<dyn AuthConsumerConfigTrait>,
}

impl AuthConsumer {
    pub fn new(
        wallet: Arc<dyn WalletServiceTrait>,
        vc_requester: Arc<dyn VcRequesterTrait>,
        onboarder: Arc<dyn OnboarderTrait>,
        repo: Arc<dyn AuthConsumerRepoTrait>,
        client: Arc<dyn ClientServiceTrait>,
        config: Arc<dyn AuthConsumerConfigTrait>,
    ) -> AuthConsumer {
        AuthConsumer { wallet, vc_requester, onboarder, repo, client, config }
    }
}

impl CoreConsumerTrait for AuthConsumer {
    fn config(&self) -> Arc<dyn AuthConsumerConfigTrait> {
        self.config.clone()
    }
}

impl CoreWalletTrait for AuthConsumer {
    fn wallet(&self) -> Arc<dyn WalletServiceTrait> {
        self.wallet.clone()
    }

    fn mate_repo(&self) -> Arc<dyn MatesTrait> {
        self.repo.mates().clone()
    }
}

impl CoreVcRequesterTrait for AuthConsumer {
    fn vc_req(&self) -> Arc<dyn VcRequesterTrait> {
        self.vc_requester.clone()
    }

    fn vc_req_repo(&self) -> Arc<dyn ReqVcTrait> {
        self.repo.vc_req().clone()
    }

    fn verification_req_repo(&self) -> Arc<dyn ReqVerificationTrait> {
        self.repo.verification_req()
    }

    fn interaction_req_repo(&self) -> Arc<dyn ReqInteractionTrait> {
        self.repo.interaction_req()
    }
}

impl CoreMateTrait for AuthConsumer {
    fn mate_repo(&self) -> Arc<dyn MatesTrait> {
        self.repo.mates().clone()
    }
}

impl CoreOnboarderTrait for AuthConsumer {
    fn onboarder(&self) -> Arc<dyn OnboarderTrait> {
        self.onboarder.clone()
    }

    fn repo(&self) -> Arc<dyn AuthConsumerRepoTrait> {
        self.repo.clone()
    }
}
