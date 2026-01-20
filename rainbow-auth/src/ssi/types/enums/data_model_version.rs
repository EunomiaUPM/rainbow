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

use std::fmt;
use std::str::FromStr;

use anyhow::bail;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum VcDataModelVersion {
    V1,
    V2,
}

impl FromStr for VcDataModelVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "V1" => Ok(VcDataModelVersion::V1),
            "V2" => Ok(VcDataModelVersion::V2),
            _ => {
                let error = CommonErrors::parse_new("Invalid data model version");
                error!("{}", error.log());
                bail!(error)
            }
        }
    }
}

impl fmt::Display for VcDataModelVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VcDataModelVersion::V1 => "V1",
            VcDataModelVersion::V2 => "V2",
        };

        write!(f, "{s}")
    }
}
