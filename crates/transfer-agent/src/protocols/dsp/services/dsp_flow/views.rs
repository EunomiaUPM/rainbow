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

//! What the inbound transfer flow answers once a message is applied.

use serde::{Deserialize, Serialize};

use crate::entities::ids::IdempotencyKey;
use crate::entities::protocol::TransferRole;
use crate::protocols::dsp::entities::ack::TransferProcessAck;

/// What applying a message produced: the ACK, and what idempotency needs to replay it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferDspReply {
    pub ack: TransferProcessAck,
    /// A request opened the process (201); any other message moved it (200).
    pub created: bool,
    pub process_version: u64,
    pub idempotency_key: IdempotencyKey,
    /// What the process runs under and our side of it, for whoever reacts to the change.
    pub agreement_id: String,
    pub role: TransferRole,
}
