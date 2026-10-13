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

//! What both negotiation flows say about the process a message refers to.

use crate::protocols::dsp::entities::process::NegotiationProcess;

/// Whether the local process already exists or the message opens one.
#[derive(Debug)]
pub enum NegotiationProcessSlot {
    Existing(Box<NegotiationProcess>),
    /// A request or offer opening a negotiation, with the pid the peer minted.
    New {
        peer_pid: String,
    },
}

impl NegotiationProcessSlot {
    pub fn existing(&self) -> Option<&NegotiationProcess> {
        match self {
            Self::Existing(process) => Some(process),
            Self::New { .. } => None,
        }
    }
}
