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

//! The scenarios of an agent: its script table over the mapping, and the queue of steps to play.

use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::tck::TckMapping;

/// One thing the connector does in a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TckStep<M> {
    Send(M),
    Pause,
}

/// The connector's part of each TCK test, as the TCK's own mocks play it.
pub trait TckScriptTrait: Send + Sync + 'static {
    type Message: Copy + Debug + Send + Sync + 'static;
    type Event: Copy + Debug + Send + Sync;
    /// Mapping sections where this agent's resources are seeded.
    const SECTIONS: &'static [&'static str];

    fn script(test: &str, event: Self::Event) -> &'static [TckStep<Self::Message>];
}

/// The script table of `S` over the mapping.
#[derive(Debug, Clone)]
pub struct TckScenarios<S> {
    mapping: TckMapping,
    script: PhantomData<S>,
}

impl<S: TckScriptTrait> TckScenarios<S> {
    pub fn new(mapping: TckMapping) -> Self {
        Self {
            mapping,
            script: PhantomData,
        }
    }

    /// What to do after `event` on a process under `resource`; empty when not a TCK test.
    pub fn steps(&self, resource: &str, event: S::Event) -> &'static [TckStep<S::Message>] {
        match self.mapping.test_of(S::SECTIONS, resource) {
            Ok(Some(test)) => S::script(&test, event),
            Ok(None) => &[],
            Err(err) => {
                tracing::warn!(error = %err.reason(), "TCK mapping unreadable");
                &[]
            }
        }
    }
}

/// Steps to play on the process with pid `pid`.
#[derive(Debug)]
pub struct TckJob<M: 'static> {
    pub pid: String,
    pub steps: &'static [TckStep<M>],
}

/// The receiving end of a queue, handed to the worker.
pub struct TckJobs<M: 'static>(pub(crate) UnboundedReceiver<TckJob<M>>);

/// Turns committed changes into jobs for the worker; the agent's guards hold it.
pub struct TckQueue<S: TckScriptTrait> {
    scenarios: Arc<TckScenarios<S>>,
    jobs: UnboundedSender<TckJob<S::Message>>,
}

impl<S: TckScriptTrait> Clone for TckQueue<S> {
    fn clone(&self) -> Self {
        Self {
            scenarios: self.scenarios.clone(),
            jobs: self.jobs.clone(),
        }
    }
}

impl<S: TckScriptTrait> TckQueue<S> {
    /// A queue over `mapping`, and the jobs its worker plays.
    pub fn open(mapping: TckMapping) -> (Self, TckJobs<S::Message>) {
        let (jobs, queue) = mpsc::unbounded_channel();
        let queue_end = Self {
            scenarios: Arc::new(TckScenarios::new(mapping)),
            jobs,
        };
        (queue_end, TckJobs(queue))
    }

    /// Queues what the scenario of `resource` does after `event`, if anything.
    pub fn enqueue(&self, resource: &str, event: S::Event, pid: String) {
        let steps = self.scenarios.steps(resource, event);
        if steps.is_empty() {
            return;
        }
        tracing::info!(resource, ?event, pid, "TCK scenario continues");
        let _ = self.jobs.send(TckJob { pid, steps });
    }
}
