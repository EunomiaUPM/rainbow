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

//! Plays the connector's part of TCK scenarios through the outbound flow.

use std::sync::Arc;
use std::time::Duration;

use common::boot::workers::BackgroundWorker;
use common::flow::FlowRunner;
use common::oauth::UserInfo;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_rpc::{TransferRpcBody, TransferRpcCommand};
use crate::protocols::dsp::services::rpc_flow::service::TransferRpcFlow;
use crate::protocols::dsp::entities::tck::{TckMessage, TckStep};

/// Steps to play on the process with pid `pid`.
#[derive(Debug)]
pub struct TckJob {
    pub pid: String,
    pub steps: &'static [TckStep],
}

/// Receives scenario jobs and plays each in its own task, so scenarios can overlap.
pub struct TckWorker {
    jobs: UnboundedReceiver<TckJob>,
    runner: Arc<FlowRunner<TransferRpcFlow>>,
}

impl TckWorker {
    /// Gap the TCK mocks leave between two actions, and before the first.
    const PAUSE: Duration = Duration::from_millis(1000);
    const ATTEMPTS: u64 = 5;

    pub fn new(jobs: UnboundedReceiver<TckJob>, runner: Arc<FlowRunner<TransferRpcFlow>>) -> Self {
        Self { jobs, runner }
    }

    async fn play(runner: Arc<FlowRunner<TransferRpcFlow>>, job: TckJob) {
        tokio::time::sleep(Self::PAUSE).await;
        for step in job.steps {
            match step {
                TckStep::Pause => tokio::time::sleep(Self::PAUSE).await,
                TckStep::Send(message) => Self::send(&runner, &job.pid, *message).await,
            }
        }
    }

    /// Sends one message, retrying while the peer is not ready for it yet.
    async fn send(runner: &FlowRunner<TransferRpcFlow>, pid: &str, message: TckMessage) {
        for attempt in 1..=Self::ATTEMPTS {
            let command = TransferRpcCommand::new(
                message.message_type(),
                TransferRpcBody {
                    provider_pid: Some(pid.to_string()),
                    ..TransferRpcBody::default()
                },
            );
            match runner.run(&UserInfo::system(), command).await {
                Ok(_) => return,
                Err(err) => {
                    tracing::warn!(pid, ?message, attempt, error = %err, "TCK scenario step failed");
                    tokio::time::sleep(Duration::from_millis(400 * attempt)).await;
                }
            }
        }
        tracing::error!(pid, ?message, "TCK scenario step gave up");
    }
}

#[async_trait::async_trait]
impl BackgroundWorker for TckWorker {
    fn name(&self) -> &'static str {
        "transfer-tck-scenarios"
    }

    async fn run(mut self: Box<Self>, token: CancellationToken) -> Outcome<()> {
        let mut scenarios = JoinSet::new();
        loop {
            tokio::select! {
                _ = token.cancelled() => break,
                job = self.jobs.recv() => match job {
                    Some(job) => {
                        scenarios.spawn(Self::play(self.runner.clone(), job));
                    }
                    None => token.cancelled().await,
                },
                Some(_) = scenarios.join_next(), if !scenarios.is_empty() => {}
            }
        }
        scenarios.shutdown().await;
        Ok(())
    }
}
