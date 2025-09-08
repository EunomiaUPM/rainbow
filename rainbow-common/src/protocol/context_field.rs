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

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub static CONTEXT: &str = "https://w3id.org/dspace/2025/1/context.jsonld";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextField {
    Single(String),
    Multiple(Vec<String>),
}

impl ContextField {
    pub fn validate(&self) -> anyhow::Result<()> {
        match self {
            ContextField::Single(s) => {
                if s == CONTEXT {
                    Ok(())
                } else {
                    Err(anyhow!("Invalid @context value"))
                }
            }
            ContextField::Multiple(v) => {
                if v.iter().any(|s| s == CONTEXT) {
                    Ok(())
                } else {
                    Err(anyhow!("Invalid @context value"))
                }
            }
        }
    }
}

impl Display for ContextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(CONTEXT)
    }
}

impl Default for ContextField {
    fn default() -> Self {
        ContextField::Multiple(vec![CONTEXT.to_string()])
    }
}
