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
use transfer_agent::protocols::dsp::entities::message_types::TransferDSPMessageType;
use transfer_agent::protocols::dsp::entities::message_types::TransferDSPMessageType::{
    TransferRequestMessage, TransferStartMessage, TransferTerminationMessage,
};
use serde_json::json;
use transfer_agent::protocols::dsp::entities::context_rpc::{
    TransferRpcBody, TransferRpcRaw, TransferRpcRdf, TransferRpcTyped,
};
use transfer_agent::protocols::dsp::services::validator::rpc_registry::TransferRpcRules;
use transfer_agent::protocols::dsp::services::validator::TransferRules;

async fn typed(route: TransferDSPMessageType, body: TransferRpcBody) -> TransferRpcTyped {
    TransferRpcTyped::from_body(&route, &body).await.unwrap()
}

/// A request without peer, address, format or agreement reports all four at once.
#[tokio::test]
async fn an_empty_request_reports_every_missing_field() {
    let command = typed(TransferRequestMessage, TransferRpcBody::default()).await;
    let err = TransferRpcRules::input_rules()
        .validate(&TransferRequestMessage, &command)
        .unwrap_err();
    assert_eq!(err.len(), 4, "{err}");
}

/// A state change needs the pid of the process it moves.
#[tokio::test]
async fn a_state_change_needs_a_pid() {
    let without = typed(TransferTerminationMessage, TransferRpcBody::default()).await;
    assert!(TransferRpcRules::input_rules()
        .validate(&TransferTerminationMessage, &without)
        .is_err());
    let with = typed(
        TransferTerminationMessage,
        TransferRpcBody {
            provider_pid: Some("urn:uuid:p".into()),
            ..TransferRpcBody::default()
        },
    )
    .await;
    assert!(TransferRpcRules::input_rules()
        .validate(&TransferTerminationMessage, &with)
        .is_ok());
}

/// A body typed as another RPC than the route's is rejected by a shape rule.
#[tokio::test]
async fn a_body_for_another_route_is_rejected() {
    let raw = TransferRpcRaw::new(
        &TransferStartMessage,
        json!({
            "@context": "https://w3id.org/eunomia/v1/context.jsonld",
            "@type": "RpcTransferTerminationMessage",
            "providerPid": "urn:uuid:p"
        }),
    );
    let command = TransferRpcTyped::from_rdf(TransferRpcRdf::from_raw(raw).await.unwrap()).unwrap();
    let err = TransferRpcRules::input_rules()
        .validate(&TransferStartMessage, &command)
        .unwrap_err();
    assert!(err.to_string().contains("@type"), "{err}");
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
