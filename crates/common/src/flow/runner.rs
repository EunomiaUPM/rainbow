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

//! Runs a flow in the fixed order; every controller goes through here.

use crate::flow::{
    FlowAdapterTrait, FlowCx, FlowError, FlowGuardTrait, FlowTrait, GuardVerdict,
    UnitOfWorkFactoryTrait,
};
use crate::validation::ValidationRegistry;
use serde_json::Value;
use std::sync::Arc;

/// A flow with its rules, unit of work factory and guards.
pub struct FlowRunner<F: FlowTrait> {
    flow: Arc<F>,
    input_rules: Arc<ValidationRegistry<F::Key, F::Input>>,
    fact_rules: Arc<ValidationRegistry<F::Key, F::Facts>>,
    reply_rules: Option<Arc<ValidationRegistry<F::Key, Value>>>,
    uow: Arc<dyn UnitOfWorkFactoryTrait>,
    guards: Vec<Arc<dyn FlowGuardTrait<F>>>,
}

/// Builder of a [`FlowRunner`]; both rule registries are required so no stage is skipped.
pub struct FlowRunnerBuilder<F: FlowTrait> {
    runner: FlowRunner<F>,
}

impl<F: FlowTrait> FlowRunnerBuilder<F> {
    /// Rules for the peer's answer to `send`.
    pub fn reply_rules(mut self, rules: Arc<ValidationRegistry<F::Key, Value>>) -> Self {
        self.runner.reply_rules = Some(rules);
        self
    }

    /// Adds a guard; guards run in the order they are added.
    pub fn guard(mut self, guard: Arc<dyn FlowGuardTrait<F>>) -> Self {
        self.runner.guards.push(guard);
        self
    }

    pub fn build(self) -> FlowRunner<F> {
        self.runner
    }
}

impl<F: FlowTrait> FlowRunner<F> {
    pub fn builder(
        flow: Arc<F>,
        uow: Arc<dyn UnitOfWorkFactoryTrait>,
        input_rules: Arc<ValidationRegistry<F::Key, F::Input>>,
        fact_rules: Arc<ValidationRegistry<F::Key, F::Facts>>,
    ) -> FlowRunnerBuilder<F> {
        FlowRunnerBuilder {
            runner: FlowRunner {
                flow,
                input_rules,
                fact_rules,
                reply_rules: None,
                uow,
                guards: Vec::new(),
            },
        }
    }

    pub fn input_rules(&self) -> &Arc<ValidationRegistry<F::Key, F::Input>> {
        &self.input_rules
    }

    pub fn fact_rules(&self) -> &Arc<ValidationRegistry<F::Key, F::Facts>> {
        &self.fact_rules
    }

    /// Extract with `adapter`, run, and answer with the output or the error.
    pub async fn serve<A: FlowAdapterTrait<F>>(
        &self,
        adapter: &A,
        caller: &F::Caller,
        key: F::Key,
        raw: A::Raw,
    ) -> A::Response {
        let (context, result) = match adapter.extract(&key, raw).await {
            Ok(input) => (adapter.error_context(&input), self.run(caller, input).await),
            Err(err) => (A::ErrorContext::default(), Err(err)),
        };
        match result {
            Ok(output) => adapter.respond(&key, output),
            Err(err) => {
                Self::log(&key, &err);
                adapter.reject(&key, context, err)
            }
        }
    }

    /// Runs every stage in order; the unit of work commits only if all of them pass.
    #[tracing::instrument(level = "info", skip_all, err(Display), fields(key = ?self.flow.key(&input)))]
    pub async fn run(&self, caller: &F::Caller, input: F::Input) -> Result<F::Output, FlowError> {
        let key = self.flow.key(&input);
        self.input_rules
            .validate(&key, &input)
            .map_err(FlowError::from_violations)?;

        let uow = self.uow.begin().await?;
        let cx = FlowCx { caller, uow: &uow };
        let applied = self.load_and_apply(&cx, &key, input).await;
        let output = match applied {
            Ok(Applied::Done(output)) => output,
            Ok(Applied::Replayed(output)) => {
                uow.rollback().await?;
                return Ok(output);
            }
            Err(err) => {
                uow.rollback().await?;
                return Err(err);
            }
        };
        uow.commit().await?;

        for guard in &self.guards {
            if let Err(err) = guard.after_commit(caller, &key, &output).await {
                tracing::warn!(error = %err.reason(), "flow guard failed after commit");
            }
        }
        Ok(output)
    }

    /// Everything between opening and closing the unit of work.
    async fn load_and_apply(
        &self,
        cx: &FlowCx<'_, F::Caller>,
        key: &F::Key,
        input: F::Input,
    ) -> Result<Applied<F::Output>, FlowError> {
        let facts = self.flow.load(cx, input).await?;
        for guard in &self.guards {
            if let GuardVerdict::Replay(output) = guard.after_load(cx, key, &facts).await? {
                return Ok(Applied::Replayed(output));
            }
        }
        self.fact_rules
            .validate(key, &facts)
            .map_err(FlowError::from_violations)?;

        // With real transactions, the call to the peer must run outside one.
        let reply = self.flow.send(cx, &facts).await?;
        if let (Some(reply), Some(rules)) = (&reply, &self.reply_rules) {
            rules
                .validate(key, reply)
                .map_err(FlowError::from_violations)?;
        }
        Ok(Applied::Done(self.flow.apply(cx, facts, reply).await?))
    }

    /// Internal failures are logged with their detail, which never reaches the caller.
    fn log(key: &F::Key, err: &FlowError) {
        match err {
            FlowError::Failed(e) => tracing::error!(?key, error = %e.reason(), "flow failed"),
            other => tracing::info!(?key, error = %other, "flow rejected the request"),
        }
    }
}

/// How the guarded part of a run ended.
enum Applied<O> {
    Done(O),
    Replayed(O),
}
