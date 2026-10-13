/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! The mapping file the TCK seeding writes: per section, the resource seeded for each test.

use std::collections::HashMap;
use std::path::PathBuf;

use serde_json::Value;
use ymir::errors::{Errors, Outcome};

/// Environment variable naming the TCK mapping file; set, it turns TCK mode on.
pub const TCK_MAPPING_ENV: &str = "DSP_TCK_MAPPING";

/// The mapping file, read on every lookup because the TCK seeds it after the agents start.
#[derive(Debug, Clone)]
pub struct TckMapping {
    path: PathBuf,
}

impl TckMapping {
    /// TCK mode when `DSP_TCK_MAPPING` is set; the file itself may not exist yet.
    pub fn from_env() -> Option<Self> {
        std::env::var(TCK_MAPPING_ENV).ok().map(Self::new)
    }

    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// The test whose seeded resource in one of `sections` is `resource`.
    pub fn test_of(&self, sections: &[&str], resource: &str) -> Outcome<Option<String>> {
        let text = std::fs::read_to_string(&self.path).map_err(|e| {
            Errors::env_var(
                format!("{TCK_MAPPING_ENV}={}: {e}", self.path.display()),
                None,
            )
        })?;
        let mapping: HashMap<String, Value> = serde_json::from_str(&text)
            .map_err(|e| Errors::parse(format!("invalid TCK mapping: {e}"), None))?;
        Ok(sections
            .iter()
            .filter_map(|section| mapping.get(*section)?.as_object())
            .flat_map(|tests| tests.iter())
            .find(|(_, seeded)| seeded.as_str() == Some(resource))
            .map(|(test, _)| test.clone()))
    }
}
