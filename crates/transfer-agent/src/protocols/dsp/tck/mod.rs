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

//! What the DSP TCK expects this connector to do on its own in each transfer test.
//!
//! The TCK names an agreement per test (`TP_01_01_AGREEMENTID`…) and waits for the connector
//! under test to send its part of the scenario. Only active when `DSP_TCK_MAPPING` points at
//! the mapping `scripts/populate_tck.sh` writes (`tck/tck_mapping.json`); it also mounts the
//! TCK's consumer trigger, which is not served otherwise.

pub mod guard;
pub mod worker;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;
use ymir::errors::{Errors, Outcome};

use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;

/// Environment variable naming the TCK mapping file.
pub const TCK_MAPPING_ENV: &str = "DSP_TCK_MAPPING";

/// One thing the connector does in a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TckStep {
    Send(TckMessage),
    Pause,
}

/// The messages a scenario makes the connector send.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TckMessage {
    Start,
    Suspend,
    Complete,
    Terminate,
}

impl TckMessage {
    pub fn message_type(self) -> TransferDSPMessageType {
        match self {
            Self::Start => TransferDSPMessageType::TransferStartMessage,
            Self::Suspend => TransferDSPMessageType::TransferSuspensionMessage,
            Self::Complete => TransferDSPMessageType::TransferCompletionMessage,
            Self::Terminate => TransferDSPMessageType::TransferTerminationMessage,
        }
    }
}

/// What just happened to a process, from which a scenario continues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TckEvent {
    /// We are the provider and accepted a request.
    ProviderRequested,
    /// We are the consumer and our request was accepted.
    ConsumerRequested,
    /// We are the consumer and the provider started the transfer.
    ConsumerStarted,
}

#[derive(Debug, Default, Deserialize)]
struct TckMapping {
    #[serde(default)]
    provider_agreements: HashMap<String, String>,
    #[serde(default)]
    consumer_agreements: HashMap<String, String>,
}

/// Scenario lookup over the mapping file, read on each lookup: the TCK seeds it after start.
#[derive(Debug, Clone)]
pub struct TckScenarios {
    mapping: PathBuf,
}

impl TckScenarios {
    /// TCK mode when `DSP_TCK_MAPPING` is set; the file itself may not exist yet.
    pub fn from_env() -> Option<Self> {
        std::env::var(TCK_MAPPING_ENV).ok().map(Self::new)
    }

    pub fn new(mapping: impl Into<PathBuf>) -> Self {
        Self {
            mapping: mapping.into(),
        }
    }

    /// What to do after `event` on a process under `agreement`; empty when not a TCK test.
    pub fn steps(&self, agreement: &str, event: TckEvent) -> &'static [TckStep] {
        match self.test_of(agreement) {
            Ok(Some(test)) => Self::script(&test, event),
            Ok(None) => &[],
            Err(err) => {
                tracing::warn!(error = %err.reason(), "TCK mapping unreadable");
                &[]
            }
        }
    }

    /// The test an agreement was seeded for, from the current mapping.
    fn test_of(&self, agreement: &str) -> Outcome<Option<String>> {
        let text = std::fs::read_to_string(&self.mapping).map_err(|e| {
            Errors::env_var(
                format!("{TCK_MAPPING_ENV}={}: {e}", self.mapping.display()),
                None,
            )
        })?;
        let mapping: TckMapping = serde_json::from_str(&text)
            .map_err(|e| Errors::parse(format!("invalid TCK mapping: {e}"), None))?;
        Ok(mapping
            .provider_agreements
            .into_iter()
            .chain(mapping.consumer_agreements)
            .find(|(_, seeded)| seeded == agreement)
            .map(|(test, _)| test))
    }

    /// The connector's part of each scenario, as the TCK's own mocks play it.
    fn script(test: &str, event: TckEvent) -> &'static [TckStep] {
        use TckMessage::*;
        use TckStep::*;
        match (test, event) {
            ("TP0101", TckEvent::ProviderRequested) => &[Send(Start), Pause, Send(Terminate)],
            ("TP0102", TckEvent::ProviderRequested) => &[Send(Start), Pause, Send(Complete)],
            ("TP0103", TckEvent::ProviderRequested) => {
                &[Send(Start), Pause, Send(Suspend), Pause, Send(Terminate)]
            }
            ("TP0104", TckEvent::ProviderRequested) => &[
                Send(Start),
                Pause,
                Send(Suspend),
                Pause,
                Send(Start),
                Pause,
                Send(Complete),
            ],
            ("TP0105", TckEvent::ProviderRequested) => &[Send(Terminate)],
            (
                "TP0201" | "TP0202" | "TP0203" | "TP0204" | "TP0303" | "TP0304" | "TP0305"
                | "TP0306",
                TckEvent::ProviderRequested,
            ) => &[Send(Start)],
            ("TPC0201", TckEvent::ConsumerStarted) => &[Send(Terminate)],
            ("TPC0202", TckEvent::ConsumerStarted) => &[Send(Complete)],
            ("TPC0203", TckEvent::ConsumerStarted) => &[Send(Suspend), Pause, Send(Terminate)],
            ("TPC0205", TckEvent::ConsumerRequested) => &[Pause, Send(Terminate)],
            _ => &[],
        }
    }
}
