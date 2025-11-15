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
use crate::core::traits::{
    CoreGatekeeperTrait, CoreIssuerTrait, CoreTrait, CoreVcsTrait, CoreVerifierTrait, CoreWalletTrait,
};
use crate::services::client::ClientServiceTrait;
use crate::services::gatekeeper::GateKeeperTrait;
use crate::services::issuer::IssuerTrait;
use crate::services::repo::RepoTrait;
use crate::services::verifier::VerifierTrait;
use crate::services::wallet::WalletTrait;
use crate::setup::AuthorityApplicationConfig;
use std::sync::Arc;

pub struct Authority {
    wallet: Arc<dyn WalletTrait>,
    gatekeeper: Arc<dyn GateKeeperTrait>,
    issuer: Arc<dyn IssuerTrait>,
    verifier: Arc<dyn VerifierTrait>,
    repo: Arc<dyn RepoTrait>,
    client: Arc<dyn ClientServiceTrait>,
    config: AuthorityApplicationConfig,
}

impl Authority {
    pub fn new(
        wallet: Arc<dyn WalletTrait>,
        gatekeeper: Arc<dyn GateKeeperTrait>,
        issuer: Arc<dyn IssuerTrait>,
        verifier: Arc<dyn VerifierTrait>,
        repo: Arc<dyn RepoTrait>,
        client: Arc<dyn ClientServiceTrait>,
        config: AuthorityApplicationConfig,
    ) -> Self {
        Self { wallet, gatekeeper, issuer, verifier, repo, client, config }
    }
}

impl CoreTrait for Authority {}
impl CoreVerifierTrait for Authority {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait> {
        self.gatekeeper.clone()
    }

    fn verifier(&self) -> Arc<dyn VerifierTrait> {
        self.verifier.clone()
    }

    fn repo(&self) -> Arc<dyn RepoTrait> {
        self.repo.clone()
    }
}

impl CoreIssuerTrait for Authority {
    fn issuer(&self) -> Arc<dyn IssuerTrait> {
        self.issuer.clone()
    }

    fn wallet(&self) -> Arc<dyn WalletTrait> {
        self.wallet.clone()
    }

    fn repo(&self) -> Arc<dyn RepoTrait> {
        self.repo.clone()
    }
}

impl CoreVcsTrait for Authority {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait> {
        self.gatekeeper.clone()
    }

    fn repo(&self) -> Arc<dyn RepoTrait> {
        self.repo.clone()
    }
}

impl CoreGatekeeperTrait for Authority {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait> {
        self.gatekeeper.clone()
    }

    fn verifier(&self) -> Arc<dyn VerifierTrait> {
        self.verifier.clone()
    }

    fn issuer(&self) -> Arc<dyn IssuerTrait> {
        self.issuer.clone()
    }

    fn repo(&self) -> Arc<dyn RepoTrait> {
        self.repo.clone()
    }
}

impl CoreWalletTrait for Authority {
    fn wallet(&self) -> Arc<dyn WalletTrait> {
        self.wallet.clone()
    }

    fn repo(&self) -> Arc<dyn RepoTrait> {
        self.repo.clone()
    }
}
