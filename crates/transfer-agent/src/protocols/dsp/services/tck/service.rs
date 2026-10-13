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

//! The connector's part of each transfer test of the DSP TCK 1.0.0-RC6.

use common::tck::{TckScriptTrait, TckStep};
use crate::protocols::dsp::entities::tck::{TckEvent, TckMessage};

/// What transfer plays in each TCK test, keyed by the agreement seeded for it.
pub struct TransferTckScript;

impl TckScriptTrait for TransferTckScript {
    type Message = TckMessage;
    type Event = TckEvent;
    const SECTIONS: &'static [&'static str] = &["provider_agreements", "consumer_agreements"];

    /// As the TCK's own mocks play it.
    fn script(test: &str, event: TckEvent) -> &'static [TckStep<TckMessage>] {
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
