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

//! Validation stages shared by every contributor of a [`ValidationRegistry`](super::ValidationRegistry).

use std::fmt::{self, Display, Formatter};

/// Global barrier levels: a stage runs only if every earlier stage passed for all origins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stage {
    /// Presence, format and schema of what arrived; no stored state needed.
    Shape,
    /// Agreement with what is stored: identifiers, versions, existence.
    Integrity,
    /// Protocol and business rules: state transitions, roles.
    Domain,
    /// Ecosystem rules added by profiles or plugins.
    Profile,
}

impl Stage {
    /// Every stage in run order.
    pub const ALL: [Stage; 4] = [
        Stage::Shape,
        Stage::Integrity,
        Stage::Domain,
        Stage::Profile,
    ];
}

impl Display for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Stage::Shape => "shape",
            Stage::Integrity => "integrity",
            Stage::Domain => "domain",
            Stage::Profile => "profile",
        };
        f.write_str(name)
    }
}
