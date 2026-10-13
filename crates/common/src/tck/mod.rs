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

//! The DSP TCK harness: the connector's own part of each TCK scenario, played on its own.
//!
//! The DSP TCK drives one side of a process and expects the connector to drive the other: after
//! a request the provider must start, suspend or terminate as the test says. Which test a
//! process belongs to comes from the resource `scripts/populate_tck.sh` seeded for it (an
//! agreement, an offer), written to the file named by `DSP_TCK_MAPPING`.
//!
//! Each agent brings three things: its script table ([`TckScriptTrait`]), a guard that turns
//! committed changes into events for [`TckQueue::enqueue`], and a [`TckPlayerTrait`] that sends
//! one message through its outbound flow. The [`TckWorker`] plays the queued steps with pauses
//! and retries, one task per scenario.
//!
//! ## 1. The script table
//!
//! ```rust,ignore
//! use common::tck::{TckScriptTrait, TckStep};
//!
//! struct MyScript;
//!
//! impl TckScriptTrait for MyScript {
//!     type Message = MyMessage;
//!     type Event = MyEvent;
//!     const SECTIONS: &'static [&'static str] = &["provider_agreements"];
//!
//!     fn script(test: &str, event: MyEvent) -> &'static [TckStep<MyMessage>] {
//!         match (test, event) {
//!             ("TP0101", MyEvent::Requested) => &[TckStep::Send(MyMessage::Start)],
//!             _ => &[],
//!         }
//!     }
//! }
//! ```
//!
//! ## 2. Wiring it
//!
//! The queue goes to the guards before the outbound runner exists; the worker gets the jobs and
//! a player over that runner afterwards.
//!
//! ```rust,ignore
//! use common::tck::{TckMapping, TckQueue, TckWorker};
//!
//! if let Some(mapping) = TckMapping::from_env() {
//!     let (queue, jobs) = TckQueue::<MyScript>::open(mapping);
//!     let runner = build_outbound(MyTckGuard::new(queue));
//!     let worker = TckWorker::new("my-tck-scenarios", jobs, Arc::new(MyPlayer::new(runner)));
//! }
//!
//! // In a guard, after a commit:
//! queue.enqueue(&agreement_id, MyEvent::Requested, pid);
//! ```

pub mod mapping;
pub mod scenarios;
pub mod worker;

pub use mapping::{TckMapping, TCK_MAPPING_ENV};
pub use scenarios::{TckJob, TckJobs, TckQueue, TckScenarios, TckScriptTrait, TckStep};
pub use worker::{TckPlayerTrait, TckWorker};

#[cfg(test)]
mod tests;
