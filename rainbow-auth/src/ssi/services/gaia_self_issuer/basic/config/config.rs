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

use super::GaiaGaiaSelfIssuerConfigTrait;
use rainbow_common::config::services::SsiAuthConfig;
use rainbow_common::config::traits::CommonConfigTrait;
use ymir::config::traits::ApiConfigTrait;
use ymir::config::types::CommonHostsConfig;
use ymir::types::vcs::W3cDataModelVersion;

pub struct GaiaSelfIssuerConfig {
    hosts: CommonHostsConfig,
    is_local: bool,
    api_path: String,
    vc_data_model: W3cDataModelVersion,
}

impl From<SsiAuthConfig> for GaiaSelfIssuerConfig {
    fn from(value: SsiAuthConfig) -> Self {
        let api_path = value.common().get_api_version();
        Self {
            hosts: value.common().hosts.clone(),
            is_local: value.common().is_local(),
            api_path,
            vc_data_model: W3cDataModelVersion::V1,
        }
    }
}

impl GaiaGaiaSelfIssuerConfigTrait for GaiaSelfIssuerConfig {
    fn hosts(&self) -> &CommonHostsConfig {
        &self.hosts
    }
    fn is_local(&self) -> bool {
        self.is_local
    }
    fn get_api_path(&self) -> String {
        self.api_path.clone()
    }
    fn get_data_model_version(&self) -> W3cDataModelVersion {
        self.vc_data_model.clone()
    }
}
