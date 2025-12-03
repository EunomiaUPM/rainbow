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
    ApiConfigTrait, CommonConfigTrait, DatabaseConfigTrait, HostConfigTrait, IsLocalTrait, KeysPathTrait, RoleTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransferConfig {
    #[serde(flatten)]
    common: CommonConfig,
}

impl CommonConfigTrait for TransferConfig {
    fn common(&self) -> &CommonConfig {
        &self.common
    }
}

impl HostConfigTrait for TransferConfig {}

impl DatabaseConfigTrait for TransferConfig {}

impl IsLocalTrait for TransferConfig {}

impl KeysPathTrait for TransferConfig {}

impl RoleTrait for TransferConfig {}

impl ApiConfigTrait for TransferConfig {}
