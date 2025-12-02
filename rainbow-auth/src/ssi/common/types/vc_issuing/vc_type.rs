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
use anyhow::bail;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use tracing::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VcType {
    LegalPerson,
    TermsAndConditions,
    Unknown,
}

impl FromStr for VcType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LegalPerson" => Ok(VcType::LegalPerson),
            "TermsAndConditions" => Ok(VcType::TermsAndConditions),
            _ => {
                let error = CommonErrors::parse_new(&format!("Unknown credential format: {}", s));
                error!("{}", error.log());
                bail!(error)
            }
        }
    }
}

impl fmt::Display for VcType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            VcType::LegalPerson => "LegalPerson".to_string(),
            VcType::TermsAndConditions => "TermsAndConditions".to_string(),
            _ => "Unknown".to_string(),
        };

        write!(f, "{s}")
    }
}

impl VcType {
    pub fn to_conf(&self) -> String {
        match self {
            VcType::LegalPerson => "LegalPerson_jwt_vc_json".to_string(),
            VcType::TermsAndConditions => "TermsAndConditions_jwt_vc_json".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn variants() -> Vec<VcType> {
        vec![
            VcType::LegalPerson,
            VcType::TermsAndConditions,
            VcType::Unknown,
            // TODO ADD MORE
        ]
    }
}
