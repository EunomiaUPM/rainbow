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
use crate::config::traits::{
    ApiConfigTrait, CommonConfigTrait, ConfigLoader, DatabaseConfigTrait, HostConfigTrait, IsLocalTrait,
};
use crate::config::types::{ClientConfig, HostConfig, WalletConfig};
use crate::errors::{CommonErrors, ErrorLog};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SsiAuthConfig {
    common: CommonConfig,
    wallet: Option<WalletConfig>,
    client: ClientConfig,
    gaia_active: bool,
}

impl SsiAuthConfig {
    pub fn wallet(&self) -> WalletConfig {
        let wallet = match self.wallet.as_ref() {
            Some(wallet) => Some(wallet.clone()),
            None => {
                let error = CommonErrors::module_new("wallet");
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
        self.wallet.is_none()
    }
}

impl ConfigLoader for SsiAuthConfig {
    fn default(common_config: CommonConfig) -> Self {
        Self {
            common: common_config,
            wallet: Some(WalletConfig {
                api: HostConfig {
                    protocol: "http".to_string(),
                    url: "127.0.0.1".to_string(),
                    port: Some("7001".to_string()),
                },
                id: None,
            }),
            client: ClientConfig { class_id: "rainbow".to_string(), display: None },
            gaia_active: false,
        }
    }

    fn load(env_file: Option<String>) -> Self {
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

impl HostConfigTrait for SsiAuthConfig {}

impl DatabaseConfigTrait for SsiAuthConfig {}

impl IsLocalTrait for SsiAuthConfig {}

impl ApiConfigTrait for SsiAuthConfig {}
