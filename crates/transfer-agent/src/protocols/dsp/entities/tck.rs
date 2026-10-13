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

//! The messages a DSP TCK scenario makes the connector send, and the events it continues from.

use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;

/// The messages a scenario makes the connector send.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TckMessage {
    Start,
    Suspend,
    Complete,
    Terminate,
}

impl TckMessage {
    pub fn message_type(self) -> TransferDSPMessageType {
        match self {
            Self::Start => TransferDSPMessageType::TransferStartMessage,
            Self::Suspend => TransferDSPMessageType::TransferSuspensionMessage,
            Self::Complete => TransferDSPMessageType::TransferCompletionMessage,
            Self::Terminate => TransferDSPMessageType::TransferTerminationMessage,
        }
    }
}

/// What just happened to a process, from which a scenario continues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TckEvent {
    /// We are the provider and accepted a request.
    ProviderRequested,
    /// We are the consumer and our request was accepted.
    ConsumerRequested,
    /// We are the consumer and the provider started the transfer.
    ConsumerStarted,
}
