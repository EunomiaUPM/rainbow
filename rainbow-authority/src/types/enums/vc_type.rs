/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use crate::errors::{ErrorLogTrait, Errors};
use crate::types::enums::errors::BadFormat;
use anyhow::bail;
use serde::Serialize;
use tracing::error;

pub enum VcType {
    DataSpaceParticipant,
    Identity,
}

impl VcType {
    pub fn to_conf(self) -> String {
        match self {
            VcType::DataSpaceParticipant => "DataspaceParticipantCredential_jwt_vc_json".to_string(),
            VcType::Identity => "IdentityCredential_jwt_vc_json".to_string(),
        }
    }

    pub fn from_str(s: &str) -> anyhow::Result<VcType> {
        match s {
            "DataspaceParticipantCredential" => Ok(VcType::DataSpaceParticipant),
            "IdentityCredential" => Ok(VcType::Identity),
            _ => {
                let error = Errors::format_new(BadFormat::Received, &format!("Unknown format: {}", s));
                error!("{}", error.log());
                bail!(error)
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            VcType::DataSpaceParticipant => "DataspaceParticipantCredential".to_string(),
            VcType::Identity => "IdentityCredential".to_string(),
        }
    }
}
