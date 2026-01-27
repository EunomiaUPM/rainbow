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

use crate::config::services::CommonConfig;
use crate::config::traits::{CommonConfigTrait, ConfigLoader};
use crate::config::types::ClientConfig;
use serde::{Deserialize, Serialize};
use tracing::error;
use ymir::errors::{ErrorLogTrait, Errors};
use ymir::types::dids::did_config::DidConfig;
use ymir::types::issuing::StuffToIssue;
use ymir::types::verifying::RequirementsToVerify;
use ymir::types::wallet::WalletConfig;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SsiAuthConfig {
    common: CommonConfig,
    wallet: Option<WalletConfig>,
    client: ClientConfig,
    did: DidConfig,
    stuff_to_issue: StuffToIssue,
    requirements_to_verify: RequirementsToVerify,
    gaia_active: bool,
}

impl SsiAuthConfig {
    pub fn wallet(&self) -> WalletConfig {
        let wallet = match self.wallet.as_ref() {
            Some(wallet) => Some(wallet.clone()),
            None => {
                let error = Errors::module_new("wallet");
                error!("{}", error.log());
                None
            }
        };
        wallet.expect("Wallet not active")
    }
    pub fn client(&self) -> ClientConfig {
        self.client.clone()
    }
    pub fn is_gaia_active(&self) -> bool {
        self.gaia_active
    }
    pub fn is_wallet_active(&self) -> bool {
        self.wallet.is_some()
    }
    pub fn did(&self) -> DidConfig {
        self.did.clone()
    }
    pub fn stuff_to_issue(&self) -> StuffToIssue {
        self.stuff_to_issue.clone()
    }
    pub fn requirements_to_verify(&self) -> RequirementsToVerify {
        self.requirements_to_verify.clone()
    }
}

impl ConfigLoader for SsiAuthConfig {
    fn load(env_file: String) -> Self {
        match Self::global_load(env_file.clone()) {
            Ok(data) => data.ssi_auth(),
            Err(_) => Self::local_load(env_file).expect("Unable to load auth config"),
        }
    }
}

impl CommonConfigTrait for SsiAuthConfig {
    fn common(&self) -> &CommonConfig {
        &self.common
    }
}
