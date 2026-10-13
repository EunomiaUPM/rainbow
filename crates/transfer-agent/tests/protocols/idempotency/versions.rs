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

//! Process versions: restarts after a suspension, in-flight retries and superseded records.

use super::*;

/// A restart after a suspension repeats the start byte for byte; the process version tells
/// it from a replay, which identity alone could not.
#[tokio::test]
async fn a_restart_after_a_suspension_is_not_a_replay() {
    let guard = Guard::new();

    // Start #1 arrives with the process at version 5 and leaves it at 6.
    let start = start_message().await;
    let IdempotencyVerdict::New(key) = guard.check(&start, Some(5)).await.unwrap() else {
        panic!("first start must be New");
    };
    let record = guard.claim(key, &start, Some(5)).await.unwrap();
    guard
        .record_response(record, serde_json::Value::Null, Some(6))
        .await
        .unwrap();

    // A genuine retry: nothing else has touched the process.
    let retry = start_message().await;
    assert!(
        matches!(
            guard.check(&retry, Some(6)).await.unwrap(),
            IdempotencyVerdict::Replay(_)
        ),
        "a retry with the process untouched must still replay"
    );

    // The transfer is suspended (version 7), then restarted with the *same*
    // message. Same pids, same bytes, same canonical hash — but a new transition.
    let restart = start_message().await;
    let IdempotencyVerdict::Superseded { key, previous } =
        guard.check(&restart, Some(7)).await.unwrap()
    else {
        panic!("a restart after an intervening transition must not be a replay");
    };
    assert_eq!(previous.process_version_after, Some(6));

    // And the restart's own retry replays again, against the new version.
    let record = guard.claim(key, &restart, Some(7)).await.unwrap();
    guard
        .record_response(record, serde_json::Value::Null, Some(8))
        .await
        .unwrap();
    let restart_retry = start_message().await;
    assert!(matches!(
        guard.check(&restart_retry, Some(8)).await.unwrap(),
        IdempotencyVerdict::Replay(_)
    ));
}

/// A retry that arrives while the first request is still running has no
/// `process_version_after` yet, and must replay rather than execute twice.
#[tokio::test]
async fn a_retry_while_still_in_flight_replays() {
    let guard = Guard::new();

    let start = start_message().await;
    let IdempotencyVerdict::New(key) = guard.check(&start, Some(5)).await.unwrap() else {
        panic!("first start must be New");
    };
    guard.claim(key, &start, Some(5)).await.unwrap();

    let retry = start_message().await;
    assert!(matches!(
        guard.check(&retry, Some(5)).await.unwrap(),
        IdempotencyVerdict::Replay(_)
    ));
}

/// A different message conflicts until the process moves on; after that it is a new transition
/// (a restart may send another data address). The difference must be a term the start defines.
#[tokio::test]
async fn different_content_conflicts_until_the_process_moves_on() {
    let guard = Guard::new();

    let start = start_message().await;
    let IdempotencyVerdict::New(key) = guard.check(&start, Some(5)).await.unwrap() else {
        panic!("first start must be New");
    };
    let record = guard.claim(key, &start, Some(5)).await.unwrap();
    guard
        .record_response(record, serde_json::Value::Null, Some(6))
        .await
        .unwrap();

    let tampered = typed_at(
        "did:example:consumer",
        format!(
            r#"{{"@context":"{CTX}","@type":"TransferStartMessage","consumerPid":"urn:uuid:cc","providerPid":"urn:uuid:pp","dataAddress":{{"@type":"DataAddress","endpointType":"https://w3id.org/idsa/v4.1/HTTP","endpoint":"https://attacker.example/exfil"}}}}"#
        ),
        None,
        "/urn:uuid:pp/start",
        TransferDSPMessageType::TransferStartMessage,
    )
    .await;
    assert_ne!(
        start_message().await.rdf.canonical_hash,
        tampered.rdf.canonical_hash,
        "a differing dataAddress really is different content"
    );
    assert!(matches!(
        guard.check(&tampered, Some(6)).await.unwrap(),
        IdempotencyVerdict::Conflict(_)
    ));
    assert!(matches!(
        guard.check(&tampered, Some(7)).await.unwrap(),
        IdempotencyVerdict::Superseded { .. }
    ));
}

/// Both versions' routes arrive as `/request` once the prefix is stripped, so the version in
/// the key keeps a pid from colliding across them (DSP 4.3).
#[tokio::test]
async fn the_protocol_version_separates_otherwise_identical_messages() {
    let guard = Guard::new();
    let body = message("urn:uuid:cc", "https://c/cb");

    let v2025 = typed_versioned(
        "did:example:consumer",
        body.clone(),
        None,
        "/request",
        TransferDSPMessageType::TransferRequestMessage,
        ProtocolId::Dsp2025_1,
    )
    .await;
    let v2024 = typed_versioned(
        "did:example:consumer",
        body,
        None,
        "/request",
        TransferDSPMessageType::TransferRequestMessage,
        ProtocolId::Dsp2024,
    )
    .await;

    assert_ne!(
        v2025.idempotency_key.as_str(),
        v2024.idempotency_key.as_str()
    );

    let IdempotencyVerdict::New(key) = guard.check(&v2025, None).await.unwrap() else {
        panic!("first sighting must be New");
    };
    guard.claim(key, &v2025, None).await.unwrap();
    assert!(
        matches!(
            guard.check(&v2024, None).await.unwrap(),
            IdempotencyVerdict::New(_)
        ),
        "the other version's message must not be mistaken for a replay"
    );
}
