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

//! FlowRunner stage order, with a fake flow that counts calls and stub guards.

use super::*;
use crate::validation::{codes, violation, Stage, ValidationRegistry, Validator};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Default)]
struct CountingFlow {
    loads: AtomicUsize,
    sends: AtomicUsize,
    applies: AtomicUsize,
    send_reply: Option<Value>,
}

#[async_trait::async_trait]
impl FlowTrait for CountingFlow {
    type Key = ();
    type Caller = ();
    type Input = u32;
    type Facts = u32;
    type Output = u32;

    fn key(&self, _: &u32) {}

    async fn load(&self, _: &FlowCx<'_, ()>, input: u32) -> Outcome<u32> {
        self.loads.fetch_add(1, Ordering::SeqCst);
        Ok(input * 10)
    }

    async fn send(&self, _: &FlowCx<'_, ()>, _: &u32) -> Outcome<Option<Value>> {
        self.sends.fetch_add(1, Ordering::SeqCst);
        Ok(self.send_reply.clone())
    }

    async fn apply(&self, cx: &FlowCx<'_, ()>, facts: u32, _: Option<Value>) -> Outcome<u32> {
        self.applies.fetch_add(1, Ordering::SeqCst);
        cx.uow.on_commit(|| {});
        Ok(facts + 1)
    }
}

impl CountingFlow {
    fn counts(&self) -> (usize, usize, usize) {
        (
            self.loads.load(Ordering::SeqCst),
            self.sends.load(Ordering::SeqCst),
            self.applies.load(Ordering::SeqCst),
        )
    }
}

fn rules(pass_when: fn(&u32) -> bool) -> Arc<ValidationRegistry<(), u32>> {
    Arc::new(ValidationRegistry::new("test").with(
        (),
        Stage::Shape,
        Validator::new().rule(move |n: &u32| {
            if pass_when(n) {
                Ok(())
            } else {
                Err(violation("n", codes::NOT_ALLOWED, "rejected"))
            }
        }),
    ))
}

fn runner(
    flow: Arc<CountingFlow>,
    input_ok: fn(&u32) -> bool,
    facts_ok: fn(&u32) -> bool,
) -> FlowRunnerBuilder<CountingFlow> {
    FlowRunner::builder(
        flow,
        Arc::new(AutocommitUnitOfWorkFactory),
        rules(input_ok),
        rules(facts_ok),
    )
}

/// A clean run loads, sends, applies and returns the output.
#[tokio::test]
async fn clean_run_goes_through_every_stage() {
    let flow = Arc::new(CountingFlow::default());
    let out = runner(flow.clone(), |_| true, |_| true)
        .build()
        .run(&(), 4)
        .await
        .unwrap();
    assert_eq!(out, 41);
    assert_eq!(flow.counts(), (1, 1, 1));
}

/// Input rule failures stop the run before any load or write.
#[tokio::test]
async fn input_rules_stop_before_load() {
    let flow = Arc::new(CountingFlow::default());
    let err = runner(flow.clone(), |_| false, |_| true)
        .build()
        .run(&(), 4)
        .await
        .unwrap_err();
    assert!(matches!(err, FlowError::Rejected(_)));
    assert_eq!(flow.counts(), (0, 0, 0));
}

/// Fact rule failures stop the run after load and before send or apply.
#[tokio::test]
async fn fact_rules_stop_before_apply() {
    let flow = Arc::new(CountingFlow::default());
    let err = runner(flow.clone(), |_| true, |n| *n < 10)
        .build()
        .run(&(), 4)
        .await
        .unwrap_err();
    assert_eq!(err.status(), 400);
    assert_eq!(flow.counts(), (1, 0, 0));
}

/// A stale-version violation is reported as a conflict.
#[tokio::test]
async fn stale_violations_become_conflicts() {
    let flow = Arc::new(CountingFlow::default());
    let facts = Arc::new(ValidationRegistry::new("t").with(
        (),
        Stage::Integrity,
        Validator::new().rule(|_: &u32| Err(violation("version", codes::STALE, "changed"))),
    ));
    let err = FlowRunner::builder(
        flow,
        Arc::new(AutocommitUnitOfWorkFactory),
        rules(|_| true),
        facts,
    )
    .build()
    .run(&(), 1)
    .await
    .unwrap_err();
    assert_eq!(err.status(), 409);
}

/// A reply that breaks the reply rules is rejected and nothing is applied.
#[tokio::test]
async fn reply_rules_stop_before_apply() {
    let flow = Arc::new(CountingFlow {
        send_reply: Some(serde_json::json!({"ok": false})),
        ..Default::default()
    });
    let reply_rules = Arc::new(ValidationRegistry::new("reply").with(
        (),
        Stage::Shape,
        Validator::new().ensure("ok", "peer refused", |v: &Value| v["ok"] == true),
    ));
    let err = runner(flow.clone(), |_| true, |_| true)
        .reply_rules(reply_rules)
        .build()
        .run(&(), 1)
        .await
        .unwrap_err();
    assert!(matches!(err, FlowError::Rejected(_)));
    assert_eq!(flow.counts(), (1, 1, 0));
}

struct ReplayGuard;

#[async_trait::async_trait]
impl FlowGuardTrait<CountingFlow> for ReplayGuard {
    async fn after_load(
        &self,
        _: &FlowCx<'_, ()>,
        _: &(),
        facts: &u32,
    ) -> Result<GuardVerdict<u32>, FlowError> {
        Ok(GuardVerdict::Replay(*facts))
    }
}

/// A guard replay answers with the earlier output and skips the rules and apply.
#[tokio::test]
async fn guard_replay_skips_rules_and_apply() {
    let flow = Arc::new(CountingFlow::default());
    let out = runner(flow.clone(), |_| true, |_| false)
        .guard(Arc::new(ReplayGuard))
        .build()
        .run(&(), 3)
        .await
        .unwrap();
    assert_eq!(out, 30);
    assert_eq!(flow.counts(), (1, 0, 0));
}

#[derive(Default)]
struct RecordingGuard {
    seen: AtomicUsize,
}

#[async_trait::async_trait]
impl FlowGuardTrait<CountingFlow> for RecordingGuard {
    async fn after_commit(&self, _: &(), _: &(), output: &u32) -> Outcome<()> {
        self.seen.store(*output as usize, Ordering::SeqCst);
        Ok(())
    }
}

/// Guards see the output only after a successful commit.
#[tokio::test]
async fn guards_see_the_committed_output() {
    let guard = Arc::new(RecordingGuard::default());
    let ok = runner(Arc::new(CountingFlow::default()), |_| true, |_| true)
        .guard(guard.clone())
        .build();
    ok.run(&(), 2).await.unwrap();
    assert_eq!(guard.seen.load(Ordering::SeqCst), 21);

    let failing = Arc::new(RecordingGuard::default());
    let rejected = runner(Arc::new(CountingFlow::default()), |_| true, |_| false)
        .guard(failing.clone())
        .build();
    assert!(rejected.run(&(), 2).await.is_err());
    assert_eq!(failing.seen.load(Ordering::SeqCst), 0);
}

/// Commit hooks run on commit and are dropped on rollback.
#[tokio::test]
async fn commit_hooks_only_run_on_commit() {
    let hits = Arc::new(AtomicUsize::new(0));
    let committed = UnitOfWork::autocommit();
    let h = hits.clone();
    committed.on_commit(move || {
        h.fetch_add(1, Ordering::SeqCst);
    });
    committed.commit().await.unwrap();

    let rolled_back = UnitOfWork::autocommit();
    let h = hits.clone();
    rolled_back.on_commit(move || {
        h.fetch_add(1, Ordering::SeqCst);
    });
    rolled_back.rollback().await.unwrap();
    assert_eq!(hits.load(Ordering::SeqCst), 1);
}

/// Missing resources map to 404 and parse errors to 400; anything else is a 500.
#[test]
fn ymir_errors_map_to_flow_errors() {
    let missing: FlowError =
        ymir::errors::Errors::missing_resource("x", "process not found", None).into();
    assert_eq!(missing.status(), 404);
    let parse: FlowError = ymir::errors::Errors::parse("bad", None).into();
    assert_eq!(parse.status(), 400);
    let db: FlowError = ymir::errors::Errors::db("down", None).into();
    assert_eq!(db.status(), 500);
    assert_eq!(db.reasons(), vec!["internal error"]);
}

/// A peer's refusal is reported as a 502 that carries what the peer answered.
#[test]
fn peer_failures_map_to_bad_gateway() {
    let refused = ymir::errors::Errors::petition(
        "http://peer/transfers/request",
        "POST",
        Some(axum::http::StatusCode::BAD_REQUEST),
        ymir::errors::PetitionFailure::HttpStatus(axum::http::StatusCode::BAD_REQUEST),
        "code: is required",
        None,
    );
    let err: FlowError = refused.into();
    assert_eq!(err.status(), 502);
    assert_eq!(err.reasons(), vec!["peer answered 400: code: is required"]);
}
