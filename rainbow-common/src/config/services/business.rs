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

use super::super::traits::CommonConfigTraits;
use crate::config::services::CommonConfig;
use crate::config::traits::{DatabaseConfigTrait, HostConfigTrait, IsLocalTrait, KeysPathTrait, RoleTrait};
use crate::config::types::roles::RoleConfig;
use crate::config::types::HostConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BusinessConfig {
    #[serde(flatten)]
    common: CommonConfig,
    extra_field: bool,
}

impl HostConfigTrait for BusinessConfig {
    fn http(&self) -> &HostConfig {
        self.common.http()
    }
}

impl DatabaseConfigTrait for BusinessConfig {}

impl IsLocalTrait for BusinessConfig {
    fn is_local(&self) -> bool {
        self.common.is_local()
    }
}

impl RoleTrait for BusinessConfig {
    fn role(&self) -> &RoleConfig {
        self.common.role()
    }
}

impl KeysPathTrait for BusinessConfig {
    fn keys_path(&self) -> &str {
        self.common.keys_path()
    }
}

impl CommonConfigTraits for BusinessConfig {}
