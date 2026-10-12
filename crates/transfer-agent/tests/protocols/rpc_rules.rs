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

//! Outbound transfer rules: what the RPC body needs and what each role may send.

use transfer_agent::entities::protocol::TransferRole::{Consumer, Provider, Relay};
use transfer_agent::protocols::dsp::entities::message_types::TransferDSPMessageType::{
    TransferRequestMessage, TransferStartMessage, TransferTerminationMessage,
};
use transfer_agent::protocols::dsp::rpc_flow::input::{TransferRpcBody, TransferRpcCommand};
use transfer_agent::protocols::dsp::rpc_flow::rules::TransferRpcRules;
use transfer_agent::protocols::dsp::services::validator::TransferRules;

/// A request without peer, address, format or agreement reports all four at once.
#[test]
fn an_empty_request_reports_every_missing_field() {
    let command = TransferRpcCommand {
        message: TransferRequestMessage,
        body: TransferRpcBody::default(),
    };
    let err = TransferRpcRules::input_rules()
        .validate(&TransferRequestMessage, &command)
        .unwrap_err();
    assert_eq!(err.len(), 4, "{err}");
}

/// A state change needs the pid of the process it moves.
#[test]
fn a_state_change_needs_a_pid() {
    let without = TransferRpcCommand {
        message: TransferTerminationMessage,
        body: TransferRpcBody::default(),
    };
    assert!(TransferRpcRules::input_rules()
        .validate(&TransferTerminationMessage, &without)
        .is_err());
    let with = TransferRpcCommand {
        message: TransferTerminationMessage,
        body: TransferRpcBody {
            provider_pid: Some("urn:uuid:p".into()),
            ..TransferRpcBody::default()
        },
    };
    assert!(TransferRpcRules::input_rules()
        .validate(&TransferTerminationMessage, &with)
        .is_ok());
}

/// Only a consumer requests, only a provider starts, and a relay sends nothing.
#[test]
fn each_role_sends_only_its_messages() {
    assert!(TransferRules::role_may_send(&Consumer, &TransferRequestMessage, "role").is_ok());
    assert!(TransferRules::role_may_send(&Provider, &TransferRequestMessage, "role").is_err());
    assert!(TransferRules::role_may_send(&Provider, &TransferStartMessage, "role").is_ok());
    assert!(TransferRules::role_may_send(&Consumer, &TransferStartMessage, "role").is_err());
    assert!(TransferRules::role_may_send(&Consumer, &TransferTerminationMessage, "role").is_ok());
    assert!(TransferRules::role_may_send(&Relay, &TransferTerminationMessage, "role").is_err());
}
