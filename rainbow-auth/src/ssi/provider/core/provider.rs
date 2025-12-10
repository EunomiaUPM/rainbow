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
use crate::ssi::provider::core::traits::{
    CoreBusinessTrait, CoreGateKeeperTrait, CoreProviderTrait, CoreVerifierTrait,
};
use crate::ssi::provider::services::business::BusinessTrait;
use crate::ssi::provider::services::gatekeeper::GateKeeperTrait;
use crate::ssi::provider::services::repo::AuthProviderRepoTrait;
use crate::ssi::provider::services::verifier::VerifierTrait;
use rainbow_common::config::services::SsiAuthConfig;
use std::sync::Arc;

pub struct AuthProvider {
    wallet: Arc<dyn WalletServiceTrait>,
    vc_requester: Arc<dyn VcRequesterTrait>,
    gatekeeper: Arc<dyn GateKeeperTrait>,
    verifier: Arc<dyn VerifierTrait>,
    callback: Arc<dyn CallbackTrait>,
    business: Arc<dyn BusinessTrait>,
    repo: Arc<dyn AuthProviderRepoTrait>,
    #[allow(dead_code)] // as an orchestrator, it should have access even though it's not used
    client: Arc<dyn ClientServiceTrait>,
    // EXTRA MODULES
    self_issuer: Option<Arc<dyn GaiaSelfIssuerTrait>>,
    config: Arc<SsiAuthConfig>,
}

impl AuthProvider {
    pub fn new(
        wallet: Arc<dyn WalletServiceTrait>,
        vc_requester: Arc<dyn VcRequesterTrait>,
        gatekeeper: Arc<dyn GateKeeperTrait>,
        verifier: Arc<dyn VerifierTrait>,
        callback: Arc<dyn CallbackTrait>,
        business: Arc<dyn BusinessTrait>,
        repo: Arc<dyn AuthProviderRepoTrait>,
        client: Arc<dyn ClientServiceTrait>,
        self_issuer: Option<Arc<dyn GaiaSelfIssuerTrait>>,
        config: Arc<SsiAuthConfig>,
    ) -> AuthProvider {
        AuthProvider {
            wallet,
            vc_requester,
            gatekeeper,
            verifier,
            callback,
            business,
            repo,
            client,
            config,
            self_issuer,
        }
    }
}

impl CoreProviderTrait for AuthProvider {
    fn config(&self) -> Arc<SsiAuthConfig> {
        self.config.clone()
    }
    fn gaia_active(&self) -> bool {
        match self.self_issuer {
            Some(_) => true,
            None => false,
        }
    }
}

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

    fn mates_repo(&self) -> Arc<dyn MatesTrait> {
        self.repo.mates().clone()
    }

    fn verification_req_repo(&self) -> Arc<dyn ReqVerificationTrait> {
        self.repo.verification_req().clone()
    }

    fn interaction_req_repo(&self) -> Arc<dyn ReqInteractionTrait> {
        self.repo.interaction_req().clone()
    }

    fn callback(&self) -> Arc<dyn CallbackTrait> {
        self.callback.clone()
    }
}

impl CoreMateTrait for AuthProvider {
    fn mate_repo(&self) -> Arc<dyn MatesTrait> {
        self.repo.mates().clone()
    }
}

impl CoreGateKeeperTrait for AuthProvider {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait> {
        self.gatekeeper.clone()
    }

    fn verifier(&self) -> Arc<dyn VerifierTrait> {
        self.verifier.clone()
    }

    fn repo(&self) -> Arc<dyn AuthProviderRepoTrait> {
        self.repo.clone()
    }
}

impl CoreVerifierTrait for AuthProvider {
    fn verifier(&self) -> Arc<dyn VerifierTrait> {
        self.verifier.clone()
    }

    fn repo(&self) -> Arc<dyn AuthProviderRepoTrait> {
        self.repo.clone()
    }

    fn business(&self) -> Arc<dyn BusinessTrait> {
        self.business.clone()
    }
}

impl CoreBusinessTrait for AuthProvider {
    fn business(&self) -> Arc<dyn BusinessTrait> {
        self.business.clone()
    }

    fn repo(&self) -> Arc<dyn AuthProviderRepoTrait> {
        self.repo.clone()
    }

    fn verifier(&self) -> Arc<dyn VerifierTrait> {
        self.verifier.clone()
    }
}

impl CoreGaiaSelfIssuerTrait for AuthProvider {
    fn self_issuer(&self) -> Arc<dyn GaiaSelfIssuerTrait> {
        self.self_issuer.clone().unwrap().clone()
    }

    fn wallet(&self) -> Arc<dyn WalletServiceTrait> {
        self.wallet.clone()
    }
}
