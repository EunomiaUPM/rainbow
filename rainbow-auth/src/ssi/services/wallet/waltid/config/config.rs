/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use rainbow_common::config::services::SsiAuthConfig;
use rainbow_common::config::traits::CommonConfigTrait;
use rainbow_common::config::types::{CommonHostsConfig, WalletConfig};

use crate::ssi::services::wallet::waltid::config::config_trait::WaltIdConfigTrait;

#[derive(Clone)]
pub struct WaltIdConfig {
    hosts: CommonHostsConfig,
    wallet: WalletConfig,
}

impl From<SsiAuthConfig> for WaltIdConfig {
    fn from(value: SsiAuthConfig) -> Self {
        WaltIdConfig {
            hosts: value.common().hosts.clone(),
            wallet: value.wallet(),
        }
    }
}

impl WaltIdConfigTrait for WaltIdConfig {
    fn get_raw_wallet_config(&self) -> WalletConfig { self.wallet.clone() }
    fn get_wallet_api_url(&self) -> String {
        let data = self.get_raw_wallet_config().api;
        match data.port.as_ref() {
            Some(port) => format!("{}://{}:{}", data.protocol, data.url, port),
            None => format!("{}://{}", data.protocol, data.url)
        }
    }

    fn hosts(&self) -> &CommonHostsConfig { &self.hosts }
}
