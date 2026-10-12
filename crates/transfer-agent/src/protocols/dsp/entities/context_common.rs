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

//! What both transfer flows say about the process and connector a message refers to.

use connector::ConnectorInstanceDto;

use crate::entities::transfer_process::TransferProcess;

/// Whether the local process for this transfer already exists or must be minted.
#[derive(Debug)]
pub enum TransferContextProcessSlot {
    Existing(Box<TransferProcess>),
    New { consumer_pid: String },
}

/// The connector backing this transfer: a provider resolves one from the
/// agreement; a consumer has none.
#[derive(Debug, Clone)]
pub enum TransferContextConnectorRole {
    ConsumerNotHavingConnector,
    ProviderHavingConnector(Box<ConnectorInstanceDto>),
}
