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

//! How a flow failed, in terms a codec can map onto any transport.

use crate::validation::{codes, Violations};
use std::fmt::{self, Display, Formatter};
use ymir::errors::Errors;

/// Failure kinds a codec turns into a status and a body.
#[derive(Debug)]
pub enum FlowError {
    /// The input or the facts broke a rule: 400 with every violation.
    Rejected(Violations),
    /// The facts changed under us (stale version, duplicate claim): 409.
    Conflict(Violations),
    /// The target does not exist or the caller may not see it: 404.
    NotFound(String),
    /// A peer this flow called refused or failed: 502, with what the peer answered.
    Peer { status: u16, answer: String },
    /// Anything else: 500, logged, never detailed to the caller.
    Failed(Errors),
}

impl FlowError {
    /// Rule failures are a rejection, unless one reports a stale version.
    pub fn from_violations(violations: Violations) -> Self {
        if violations.iter().any(|v| v.code == codes::STALE) {
            Self::Conflict(violations)
        } else {
            Self::Rejected(violations)
        }
    }

    /// HTTP status a codec would answer with.
    pub fn status(&self) -> u16 {
        match self {
            Self::Rejected(_) => 400,
            Self::NotFound(_) => 404,
            Self::Conflict(_) => 409,
            Self::Peer { .. } => 502,
            Self::Failed(_) => 500,
        }
    }

    /// Reasons safe to send back; internal failures carry none.
    pub fn reasons(&self) -> Vec<String> {
        match self {
            Self::Rejected(v) | Self::Conflict(v) => v.to_reasons(),
            Self::NotFound(reason) => vec![reason.clone()],
            Self::Peer { status, answer } => vec![format!("peer answered {status}: {answer}")],
            Self::Failed(_) => vec!["internal error".to_string()],
        }
    }
}

impl From<Errors> for FlowError {
    fn from(err: Errors) -> Self {
        match err {
            Errors::MissingResourceError { .. } => Self::NotFound(err.reason().to_string()),
            Errors::PetitionError { ctx, reason, .. } => Self::Peer {
                status: ctx.http_code.map(|c| c.as_u16()).unwrap_or(0),
                answer: reason,
            },
            Errors::FormatError { .. } | Errors::ParseError { .. } => Self::Rejected(
                crate::validation::violation("body", codes::MALFORMED, err.reason().to_string()),
            ),
            other => Self::Failed(other),
        }
    }
}

impl Display for FlowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rejected(v) => write!(f, "rejected: {v}"),
            Self::Conflict(v) => write!(f, "conflict: {v}"),
            Self::NotFound(r) => write!(f, "not found: {r}"),
            Self::Peer { status, answer } => write!(f, "peer answered {status}: {answer}"),
            Self::Failed(e) => write!(f, "failed: {}", e.reason()),
        }
    }
}
