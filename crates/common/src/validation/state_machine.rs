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

//! A protocol state machine as a table: which messages each state accepts.

use std::fmt::Display;

use crate::validation::{codes, violation, Path, Violations};

/// The messages a process accepts in each state; a `None` state is a process not opened yet.
pub struct StateMachine<S: 'static, M: 'static> {
    transitions: &'static [(M, &'static [Option<S>])],
}

impl<S: PartialEq + Display, M: PartialEq + Display> StateMachine<S, M> {
    /// One row per message, with the states it may arrive in.
    pub const fn new(transitions: &'static [(M, &'static [Option<S>])]) -> Self {
        Self { transitions }
    }

    /// Whether `message` may arrive when the process is in `current`.
    pub fn check(
        &self,
        current: Option<&S>,
        message: &M,
        path: impl Into<Path>,
    ) -> Result<(), Violations> {
        let Some((_, from)) = self.transitions.iter().find(|(m, _)| m == message) else {
            return Err(violation(
                path,
                codes::NOT_ALLOWED,
                format!("{message} is not a lifecycle transition"),
            ));
        };
        if from.iter().any(|state| state.as_ref() == current) {
            return Ok(());
        }
        let current = current.map_or_else(|| "<new>".to_string(), ToString::to_string);
        Err(violation(
            path,
            codes::NOT_ALLOWED,
            format!("{message} is not allowed from state {current}"),
        ))
    }
}
