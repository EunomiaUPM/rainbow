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

//! Reading eunomia RPC bodies into transfer commands: fields, data address, route type and
//! bodies that are not eunomia JSON-LD.

use serde_json::json;
use transfer_agent::protocols::dsp::entities::context_rpc::{TransferRpcBody, TransferRpcCommand};
use transfer_agent::protocols::dsp::entities::message_types::TransferDSPMessageType::{
    TransferRequestMessage, TransferStartMessage, TransferTerminationMessage,
};

const EUNOMIA: &str = "https://w3id.org/eunomia/v1/context.jsonld";

/// A request body is read into its fields, data address included.
#[tokio::test]
async fn a_request_body_is_read_through_rdf() {
    let body = json!({
        "@context": EUNOMIA,
        "@type": "RpcTransferRequestMessage",
        "associatedAgentPeer": "did:web:provider.example",
        "agreementId": "urn:uuid:e8dc8655-44c2-46ef-b701-4cffdc2faa44",
        "format": "HttpData-PUSH",
        "providerAddress": "https://provider.example/dsp/current",
        "dataAddress": {
            "endpointType": "HTTP",
            "endpoint": "https://sink.example/data",
            "endpointProperties": [{"name": "authorization", "value": "TOKEN"}]
        }
    });
    let command = TransferRpcCommand::read(&TransferRequestMessage, body.clone())
        .await
        .unwrap();

    assert_eq!(command.body.associated_agent_peer.as_deref(), Some("did:web:provider.example"));
    assert_eq!(command.body.format.as_deref(), Some("HttpData-PUSH"));
    assert_eq!(
        command.body.provider_address.as_deref(),
        Some("https://provider.example/dsp/current")
    );
    let address = command.body.data_address.unwrap();
    assert_eq!(address.endpoint.as_deref(), Some("https://sink.example/data"));
    assert_eq!(address.endpoint_properties.unwrap()[0].name, "authorization");
    assert_eq!(command.json, body, "the JSON-LD is kept as sent");
}

/// Reasons of a termination come back as a list, code alongside.
#[tokio::test]
async fn a_termination_keeps_code_and_reasons() {
    let body = json!({
        "@context": EUNOMIA,
        "@type": "RpcTransferTerminationMessage",
        "providerPid": "urn:uuid:p",
        "code": "STOPPED",
        "reason": ["no longer needed"]
    });
    let command = TransferRpcCommand::read(&TransferTerminationMessage, body)
        .await
        .unwrap();
    assert_eq!(command.body.code.as_deref(), Some("STOPPED"));
    assert_eq!(command.body.reason, Some(vec!["no longer needed".to_string()]));
}

/// Plain JSON without the eunomia context declares no RPC type and is rejected.
#[tokio::test]
async fn plain_json_is_rejected() {
    let body = json!({"providerPid": "urn:uuid:p"});
    assert!(TransferRpcCommand::read(&TransferStartMessage, body).await.is_err());
}

/// A DSP message is not an RPC body, even with the right fields.
#[tokio::test]
async fn a_dsp_message_is_rejected() {
    let body = json!({
        "@context": "https://w3id.org/dspace/2025/1/context.jsonld",
        "@type": "TransferStartMessage",
        "providerPid": "urn:uuid:p",
        "consumerPid": "urn:uuid:c"
    });
    assert!(TransferRpcCommand::read(&TransferStartMessage, body).await.is_err());
}

/// A body typed as another RPC than the route's is rejected.
#[tokio::test]
async fn a_body_for_another_route_is_rejected() {
    let body = json!({
        "@context": EUNOMIA,
        "@type": "RpcTransferTerminationMessage",
        "providerPid": "urn:uuid:p"
    });
    let err = TransferRpcCommand::read(&TransferStartMessage, body)
        .await
        .unwrap_err();
    assert!(format!("{err:?}").contains("RpcTransferStartMessage"), "{err:?}");
}

/// A command built in process renders JSON-LD that reads back to the same body.
#[tokio::test]
async fn an_in_process_command_round_trips() {
    let built = TransferRpcCommand::new(
        TransferStartMessage,
        TransferRpcBody {
            provider_pid: Some("urn:uuid:p".into()),
            consumer_pid: Some("urn:uuid:c".into()),
            ..TransferRpcBody::default()
        },
    );
    assert_eq!(built.json["@type"], "RpcTransferStartMessage");
    let read = TransferRpcCommand::read(&TransferStartMessage, built.json.clone())
        .await
        .unwrap();
    assert_eq!(read.body.provider_pid.as_deref(), Some("urn:uuid:p"));
    assert_eq!(read.body.consumer_pid.as_deref(), Some("urn:uuid:c"));
}
