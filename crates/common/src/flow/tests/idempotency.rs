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

//! IdempotencyGuard over an in-memory store: verdicts by key, hash and process version, and
//! the guard plugged into a runner with a fake flow that counts applies.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ymir::errors::Outcome;

use crate::flow::idempotency::IdempotencyVerdict;
use crate::flow::*;
use crate::validation::{Stage, ValidationRegistry, Validator};

fn guard() -> IdempotencyGuard {
    IdempotencyGuard::new(Arc::new(InMemoryIdempotencyStore::new()))
}

fn key(s: &str) -> IdempotencyKey {
    IdempotencyKey::new(s)
}

/// A first message is new; once answered, the same message replays the answer.
#[tokio::test]
async fn first_message_is_new_then_replays() {
    let guard = guard();
    let IdempotencyVerdict::New(k) = guard.check(&key("a"), &[1; 32], None).await.unwrap() else {
        panic!("first sighting must be new");
    };
    guard.claim(k.clone(), [1; 32], None).await.unwrap();
    guard
        .record_response(&k, json!({"ok": true}), Some(1))
        .await
        .unwrap();

    let IdempotencyVerdict::Replay(record) = guard.check(&k, &[1; 32], Some(1)).await.unwrap()
    else {
        panic!("same message must replay");
    };
    assert_eq!(record.response, Some(json!({"ok": true})));
}

/// The same key with another canonical hash is a conflict, not a retry.
#[tokio::test]
async fn same_key_with_other_content_conflicts() {
    let guard = guard();
    guard.claim(key("a"), [1; 32], None).await.unwrap();
    assert!(matches!(
        guard.check(&key("a"), &[2; 32], None).await.unwrap(),
        IdempotencyVerdict::Conflict(_)
    ));
}

/// A retry still in flight replays (with no answer yet); after the process moved on it is new.
#[tokio::test]
async fn the_process_version_tells_a_retry_from_a_restart() {
    let guard = guard();
    guard.claim(key("start"), [1; 32], Some(5)).await.unwrap();
    assert!(matches!(
        guard.check(&key("start"), &[1; 32], Some(5)).await.unwrap(),
        IdempotencyVerdict::Replay(r) if r.response.is_none()
    ));

    guard
        .record_response(&key("start"), json!({}), Some(6))
        .await
        .unwrap();
    assert!(matches!(
        guard.check(&key("start"), &[1; 32], Some(6)).await.unwrap(),
        IdempotencyVerdict::Replay(_)
    ));
    assert!(matches!(
        guard.check(&key("start"), &[1; 32], Some(8)).await.unwrap(),
        IdempotencyVerdict::Superseded { .. }
    ));
}

/// A restart carrying other content (another data address) is new once the process moved on.
#[tokio::test]
async fn a_restart_with_other_content_is_superseded() {
    let guard = guard();
    guard.claim(key("start"), [1; 32], Some(5)).await.unwrap();
    guard
        .record_response(&key("start"), json!({}), Some(6))
        .await
        .unwrap();
    assert!(matches!(
        guard.check(&key("start"), &[2; 32], Some(6)).await.unwrap(),
        IdempotencyVerdict::Conflict(_)
    ));
    assert!(matches!(
        guard.check(&key("start"), &[2; 32], Some(8)).await.unwrap(),
        IdempotencyVerdict::Superseded { .. }
    ));
}

struct Facts {
    key: IdempotencyKey,
    hash: [u8; 32],
}

impl IdempotentFactsTrait for Facts {
    fn idempotency_key(&self) -> &IdempotencyKey {
        &self.key
    }
    fn canonical_hash(&self) -> &[u8; 32] {
        &self.hash
    }
    fn process_version(&self) -> Option<u64> {
        None
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Reply {
    key: IdempotencyKey,
    applied: usize,
}

impl IdempotentOutputTrait for Reply {
    fn idempotency_key(&self) -> &IdempotencyKey {
        &self.key
    }
    fn process_version(&self) -> Option<u64> {
        Some(1)
    }
}

#[derive(Default)]
struct OnceFlow {
    applies: AtomicUsize,
}

#[async_trait::async_trait]
impl FlowTrait for OnceFlow {
    type Key = ();
    type Caller = ();
    type Input = u8;
    type Facts = Facts;
    type Output = Reply;

    fn key(&self, _: &u8) {}

    async fn load(&self, _: &FlowCx<'_, ()>, input: u8) -> Outcome<Facts> {
        Ok(Facts {
            key: IdempotencyKey::new("k"),
            hash: [input; 32],
        })
    }

    async fn apply(&self, _: &FlowCx<'_, ()>, facts: Facts, _: Option<Value>) -> Outcome<Reply> {
        let applied = self.applies.fetch_add(1, Ordering::SeqCst) + 1;
        Ok(Reply {
            key: facts.key,
            applied,
        })
    }
}

/// As a guard, a repeated message gets the first reply without applying twice, and a
/// different message under the same key is a conflict.
#[tokio::test]
async fn as_a_guard_a_repeat_replays_without_applying() {
    let flow = Arc::new(OnceFlow::default());
    let runner = FlowRunner::builder(
        flow.clone(),
        Arc::new(AutocommitUnitOfWorkFactory),
        Arc::new(ValidationRegistry::new("in").with((), Stage::Shape, Validator::new())),
        Arc::new(ValidationRegistry::new("facts").with((), Stage::Domain, Validator::new())),
    )
    .guard(Arc::new(guard()))
    .build();

    let first = runner.run(&(), 1).await.unwrap();
    let again = runner.run(&(), 1).await.unwrap();
    assert_eq!(first, again);
    assert_eq!(flow.applies.load(Ordering::SeqCst), 1);

    assert!(matches!(
        runner.run(&(), 2).await,
        Err(FlowError::Conflict(_))
    ));
}
