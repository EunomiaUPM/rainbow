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
use crate::ssi::common::core::{CoreGaiaSelfIssuerTrait, CoreMateTrait, CoreVcRequesterTrait, CoreWalletTrait};
use crate::ssi::common::services::callback::CallbackTrait;
use crate::ssi::common::services::client::ClientServiceTrait;
use crate::ssi::common::services::gaia_self_issuer::GaiaSelfIssuerTrait;
use crate::ssi::common::services::repo::subtraits::{
    MatesTrait, ReqInteractionTrait, ReqVcTrait, ReqVerificationTrait,
};
use crate::ssi::common::services::vc_requester::VcRequesterTrait;
use crate::ssi::common::services::wallet::WalletServiceTrait;
use crate::ssi::consumer::core::traits::{CoreConsumerTrait, CoreOnboarderTrait};
use crate::ssi::consumer::services::onboarder::OnboarderTrait;
use crate::ssi::consumer::services::repo::AuthConsumerRepoTrait;
use rainbow_common::config::services::SsiAuthConfig;
use std::sync::Arc;

pub struct AuthConsumer {
    wallet: Arc<dyn WalletServiceTrait>,
    vc_requester: Arc<dyn VcRequesterTrait>,
    onboarder: Arc<dyn OnboarderTrait>,
    callback: Arc<dyn CallbackTrait>,
    repo: Arc<dyn AuthConsumerRepoTrait>,
    #[allow(dead_code)] // as an orchestrator, it should have access even though it's not used
    client: Arc<dyn ClientServiceTrait>,
    config: Arc<SsiAuthConfig>,
    // EXTRA MODULES
    self_issuer: Option<Arc<dyn GaiaSelfIssuerTrait>>,
}

impl AuthConsumer {
    pub fn new(
        wallet: Arc<dyn WalletServiceTrait>,
        vc_requester: Arc<dyn VcRequesterTrait>,
        onboarder: Arc<dyn OnboarderTrait>,
        callback: Arc<dyn CallbackTrait>,
        repo: Arc<dyn AuthConsumerRepoTrait>,
        client: Arc<dyn ClientServiceTrait>,
        config: Arc<SsiAuthConfig>,
        self_issuer: Option<Arc<dyn GaiaSelfIssuerTrait>>,
    ) -> AuthConsumer {
        AuthConsumer { wallet, vc_requester, onboarder, callback, repo, client, self_issuer, config }
    }
}

impl CoreConsumerTrait for AuthConsumer {
    fn gaia_active(&self) -> bool {
        match self.self_issuer {
            Some(_) => true,
            None => false,
        }
    }
    fn config(&self) -> Arc<SsiAuthConfig> {
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

    fn mates_repo(&self) -> Arc<dyn MatesTrait> {
        self.repo.mates().clone()
    }

    fn verification_req_repo(&self) -> Arc<dyn ReqVerificationTrait> {
        self.repo.verification_req()
    }

    fn interaction_req_repo(&self) -> Arc<dyn ReqInteractionTrait> {
        self.repo.interaction_req()
    }

    fn callback(&self) -> Arc<dyn CallbackTrait> {
        self.callback.clone()
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

    fn callback(&self) -> Arc<dyn CallbackTrait> {
        self.callback.clone()
    }
}

impl CoreGaiaSelfIssuerTrait for AuthConsumer {
    fn self_issuer(&self) -> Arc<dyn GaiaSelfIssuerTrait> {
        self.self_issuer.clone().unwrap().clone()
    }

    fn wallet(&self) -> Arc<dyn WalletServiceTrait> {
        self.wallet.clone()
    }
}
