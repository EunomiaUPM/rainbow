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

//! One skeleton for every controller: decode, check shape, load facts, check invariants,
//! apply, commit, encode.
//!
//! A [`FlowTrait`] holds only what varies per agent: how to load the facts a message needs and
//! how to apply it. The [`FlowRunner`] owns the order, so no controller can apply before
//! validating or validate against facts it did not load:
//!
//! ```text
//! codec.decode → Input ─input rules─▶ begin unit of work → load → Facts ─guards─▶
//!   ─fact rules─▶ send (optional) ─reply rules─▶ apply → commit → guards → codec.encode
//! ```
//!
//! ```rust,ignore
//! let runner = FlowRunner::builder(Arc::new(MyFlow::new(ctx)), Arc::new(AutocommitUnitOfWorkFactory))
//!     .input_rules(Arc::new(MyRules::input()))
//!     .fact_rules(Arc::new(MyRules::facts()))
//!     .guard(Arc::new(MyIdempotencyGuard::new(store)))
//!     .build();
//! let response = runner.serve(&codec, &peer, MessageType::Request, request).await;
//! ```

mod codec;
mod error;
mod runner;
mod uow;

pub use codec::FlowCodecTrait;
pub use error::FlowError;
pub use runner::{FlowRunner, FlowRunnerBuilder};
pub use uow::{AutocommitUnitOfWorkFactory, UnitOfWork, UnitOfWorkFactoryTrait};

use crate::registry::RegistryKey;
use serde_json::Value;
use ymir::errors::Outcome;

/// What a controller does for one agent and direction; the runner decides when.
#[async_trait::async_trait]
pub trait FlowTrait: Send + Sync + 'static {
    /// Closed set of messages or commands the flow handles; picks the rules.
    type Key: RegistryKey;
    /// Who acts: a verified peer over DSP, a user over RPC or management.
    type Caller: Send + Sync;
    /// What arrived, decoded but not checked against anything stored.
    type Input: Send + Sync;
    /// The input plus everything loaded for it; what invariants are checked on.
    type Facts: Send + Sync;
    type Output: Send + Sync;

    fn key(&self, input: &Self::Input) -> Self::Key;

    /// Gathers the facts; the only I/O before the invariants are checked.
    async fn load(&self, cx: &FlowCx<'_, Self::Caller>, input: Self::Input)
        -> Outcome<Self::Facts>;

    /// Outbound call to a peer, made before `apply`; its answer is checked by the reply rules.
    async fn send(
        &self,
        _cx: &FlowCx<'_, Self::Caller>,
        _facts: &Self::Facts,
    ) -> Outcome<Option<Value>> {
        Ok(None)
    }

    /// Performs the change; every write goes through `cx.uow`.
    async fn apply(
        &self,
        cx: &FlowCx<'_, Self::Caller>,
        facts: Self::Facts,
        reply: Option<Value>,
    ) -> Outcome<Self::Output>;
}

/// What a flow sees while it runs: who acts and the unit of work its writes join.
pub struct FlowCx<'a, C> {
    pub caller: &'a C,
    pub uow: &'a UnitOfWork,
}

/// Optional checks around a flow (idempotency, peer authorization, policies).
#[async_trait::async_trait]
pub trait FlowGuardTrait<F: FlowTrait>: Send + Sync {
    /// Runs after `load`, before the fact rules; `Replay` answers without applying.
    async fn after_load(
        &self,
        _cx: &FlowCx<'_, F::Caller>,
        _key: &F::Key,
        _facts: &F::Facts,
    ) -> Result<GuardVerdict<F::Output>, FlowError> {
        Ok(GuardVerdict::Continue)
    }

    /// Runs once the change is committed; failures are logged, not returned.
    async fn after_commit(
        &self,
        _caller: &F::Caller,
        _key: &F::Key,
        _output: &F::Output,
    ) -> Outcome<()> {
        Ok(())
    }
}

/// A guard's answer: carry on, or answer now with an earlier output.
pub enum GuardVerdict<O> {
    Continue,
    Replay(O),
}

#[cfg(test)]
mod tests;
