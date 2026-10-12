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

//! The DSP message type names, written and read back.

use transfer_agent::protocols::dsp::entities::message_types::*;

/// Every message type reads back from the name it writes.
#[test]
fn names_round_trip() {
    for t in [
        TransferDSPMessageType::TransferRequestMessage,
        TransferDSPMessageType::TransferStartMessage,
        TransferDSPMessageType::TransferCompletionMessage,
        TransferDSPMessageType::TransferSuspensionMessage,
        TransferDSPMessageType::TransferTerminationMessage,
        TransferDSPMessageType::TransferProcess,
        TransferDSPMessageType::TransferError,
    ] {
        assert_eq!(t.to_string().parse::<TransferDSPMessageType>().unwrap(), t);
    }
}

/// A prefixed or full-IRI type is accepted.
#[test]
fn accepts_prefixed_and_iri_forms() {
    let prefixed: TransferDSPMessageType = "dspace:TransferStartMessage".parse().unwrap();
    assert_eq!(prefixed, TransferDSPMessageType::TransferStartMessage);
    let iri: TransferDSPMessageType = "https://w3id.org/dspace/2025/1/TransferStartMessage"
        .parse()
        .unwrap();
    assert_eq!(iri, TransferDSPMessageType::TransferStartMessage);
}

/// An unknown name is an error.
#[test]
fn an_unknown_name_is_an_error() {
    assert!("ContractRequestMessage".parse::<TransferDSPMessageType>().is_err());
}
