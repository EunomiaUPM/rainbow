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

use serde::{Deserialize, Serialize};

use super::{DidsInfo, WalletInfo};

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInfoResponse {
    pub account: String,
    pub wallets: Vec<ModifiedWalletInfo>
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
pub struct ModifiedWalletInfo {
    pub id: String,
    pub name: String,
    #[serde(rename = "createdOn")]
    pub created_on: String,
    #[serde(rename = "addedOn")]
    pub added_on: String,
    pub permission: String, // TODO
    pub dids: Option<Vec<DidsInfo>>
}

impl ModifiedWalletInfo {
    pub fn to_normal(self) -> WalletInfo {
        WalletInfo {
            id: self.id,
            name: self.name,
            created_on: self.created_on,
            added_on: self.added_on,
            permission: self.permission,
            dids: Vec::<DidsInfo>::new()
        }
    }
}
