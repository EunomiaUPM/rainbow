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

//! IdempotencyGuard over an in-memory store with transfer's keys, split into how messages are
//! keyed and how a process version tells restarts from retries.

mod keys;
mod versions;

use std::sync::Arc;

use axum::extract::Request;
use common::facades::grants_facade::VerifiedPeer;
use common::flow::idempotency::{IdempotencyRecord, IdempotencyVerdict};
use common::flow::{IdempotencyGuard, IdempotencyKey, InMemoryIdempotencyStore};
use ymir::types::oauth::RolePath;
use transfer_agent::entities::protocol::ProtocolId;
use transfer_agent::protocols::dsp::entities::context_dsp::{
    TransferDspParsed, TransferDspRaw, TransferDspRdf, TransferDspTyped,
};
use transfer_agent::protocols::dsp::entities::message_types::TransferDSPMessageType;
use ymir::errors::Outcome;

const CTX: &str = "https://w3id.org/dspace/2025/1/context.jsonld";

/// The common guard driven with what a transfer input carries.
struct Guard(IdempotencyGuard);

impl Guard {
    fn new() -> Self {
        Self(IdempotencyGuard::new(Arc::new(InMemoryIdempotencyStore::new())))
    }

    async fn check(
        &self,
        input: &TransferDspTyped,
        version: Option<u64>,
    ) -> Outcome<IdempotencyVerdict> {
        self.0
            .check(&input.idempotency_key, &input.rdf.canonical_hash, version)
            .await
    }

    async fn claim(
        &self,
        key: IdempotencyKey,
        input: &TransferDspTyped,
        version: Option<u64>,
    ) -> Outcome<IdempotencyRecord> {
        self.0.claim(key, input.rdf.canonical_hash, version).await
    }

    async fn record_response(
        &self,
        record: IdempotencyRecord,
        response: serde_json::Value,
        version_after: Option<u64>,
    ) -> Outcome<()> {
        self.0
            .record_response(&record.key, response, version_after)
            .await
    }
}

fn mate(participant_id: &str) -> VerifiedPeer {
    VerifiedPeer {
        participant_id: participant_id.into(),
        role: RolePath::root(),
        visibility: ymir::types::participants::Visibility::Public,
    }
}

/// The typed stage a handler would get, for a body sent by `peer`.
async fn typed_from(peer: &str, body: String) -> TransferDspTyped {
    typed_from_with_key(peer, body, None).await
}

async fn typed_from_with_key(
    peer: &str,
    body: String,
    supplied_key: Option<&str>,
) -> TransferDspTyped {
    typed_at(
        peer,
        body,
        supplied_key,
        "/request",
        TransferDSPMessageType::TransferRequestMessage,
    )
    .await
}

async fn typed_at(
    peer: &str,
    body: String,
    supplied_key: Option<&str>,
    uri: &str,
    message_type: TransferDSPMessageType,
) -> TransferDspTyped {
    typed_versioned(
        peer,
        body,
        supplied_key,
        uri,
        message_type,
        ProtocolId::Dsp2025_1,
    )
    .await
}

async fn typed_versioned(
    peer: &str,
    body: String,
    supplied_key: Option<&str>,
    uri: &str,
    message_type: TransferDSPMessageType,
    dsp_version: ProtocolId,
) -> TransferDspTyped {
    let mut builder = Request::builder().method("POST").uri(uri.to_string());
    if let Some(key) = supplied_key {
        builder = builder.header("idempotency-key", key);
    }
    let req = builder.body(axum::body::Body::from(body)).unwrap();
    let raw = TransferDspRaw::from_request(mate(peer), req).await.unwrap();
    let parsed = TransferDspParsed::from_raw(raw, &dsp_version, &message_type).unwrap();
    TransferDspTyped::from_rdf(TransferDspRdf::from_parsed(parsed).await.unwrap()).unwrap()
}

fn message(consumer_pid: &str, callback: &str) -> String {
    format!(
        r#"{{"@context":"{CTX}","@type":"TransferRequestMessage","consumerPid":"{consumer_pid}","callbackAddress":"{callback}"}}"#
    )
}

/// A `TransferStartMessage` on the state-transition route: same pids every
/// time, so protocol identity alone cannot tell a retry from a restart.
async fn start_message() -> TransferDspTyped {
    typed_at(
        "did:example:consumer",
        format!(
            r#"{{"@context":"{CTX}","@type":"TransferStartMessage","consumerPid":"urn:uuid:cc","providerPid":"urn:uuid:pp"}}"#
        ),
        None,
        "/urn:uuid:pp/start",
        TransferDSPMessageType::TransferStartMessage,
    )
    .await
}
