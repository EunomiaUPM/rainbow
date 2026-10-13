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

//! Atomic rules of the DSP 2025-1 contract negotiation: state machine, roles, pids and offers.

use common::dsp_common::DspRules;
use common::facades::catalog::CatalogOffer;
use common::validation::{codes, violation, Path, Violations};

use crate::protocols::dsp::entities::message_types::NegotiationMessageType;
use crate::protocols::dsp::entities::protocol_fields::OfferFields;
use crate::protocols::dsp::entities::role::NegotiationRole;
use crate::protocols::dsp::entities::state::NegotiationState;

/// Negotiation-specific rules for the validator registries.
pub struct NegotiationRules;

impl NegotiationRules {
    /// DSP 8.1 state machine, as the table next to the states.
    pub fn state_transition(
        current: Option<&NegotiationState>,
        message: &NegotiationMessageType,
        path: impl Into<Path>,
    ) -> Result<(), Violations> {
        NegotiationState::MACHINE.check(current, message, path)
    }

    /// We may receive a message only if the other side is the one that sends it.
    pub fn may_receive(
        role: NegotiationRole,
        message: NegotiationMessageType,
        path: impl Into<Path>,
    ) -> Result<(), Violations> {
        match NegotiationRole::sender_of(message) {
            Some(sender) if sender == role => Err(violation(
                path,
                codes::NOT_ALLOWED,
                format!("a {role} does not receive a {message}"),
            )),
            _ => Ok(()),
        }
    }

    /// We may send a message only if our side is the one that sends it.
    pub fn may_send(
        role: NegotiationRole,
        message: NegotiationMessageType,
        path: impl Into<Path>,
    ) -> Result<(), Violations> {
        match NegotiationRole::sender_of(message) {
            Some(sender) if sender != role => Err(violation(
                path,
                codes::NOT_ALLOWED,
                format!("only a {sender} sends a {message}"),
            )),
            _ => Ok(()),
        }
    }

    /// The pid in the route must be the one this connector minted.
    pub fn path_is_own_pid(
        path_id: &str,
        own_pid: Option<&str>,
        path: impl Into<Path>,
    ) -> Result<(), Violations> {
        let path = path.into();
        match own_pid {
            Some(pid) => DspRules::correlate_pids(path_id, pid, path),
            None => Err(violation(
                path,
                codes::MISSING,
                "the process has no pid of ours",
            )),
        }
    }

    /// A requested offer must be one we published, on the target the request names.
    pub fn offer_published(
        published: Option<&CatalogOffer>,
        requested: Option<&OfferFields>,
        path: impl Into<Path>,
    ) -> Result<(), Violations> {
        let path = path.into();
        let Some(published) = published else {
            return Err(violation(
                path,
                codes::NOT_ALLOWED,
                "the offer is not published in the catalog",
            ));
        };
        match requested.and_then(|o| o.target.as_deref()) {
            Some(target) if target != published.entity => Err(violation(
                path,
                codes::NOT_ALLOWED,
                format!(
                    "the offer is published on {}, not on {target}",
                    published.entity
                ),
            )),
            _ => Ok(()),
        }
    }
}
