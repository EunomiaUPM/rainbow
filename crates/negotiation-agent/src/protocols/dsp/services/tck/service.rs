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

//! The connector's part of each negotiation test of the DSP TCK 1.0.0-RC6.

use common::tck::{TckScriptTrait, TckStep};

use crate::protocols::dsp::entities::tck::{TckEvent, TckMessage};

/// What negotiation plays in each TCK test, keyed by the dataset seeded for it: the TCK makes up
/// the consumer's offer ids, but every offer targets the seeded dataset.
pub struct NegotiationTckScript;

impl TckScriptTrait for NegotiationTckScript {
    type Message = TckMessage;
    type Event = TckEvent;
    const SECTIONS: &'static [&'static str] = &["datasets"];

    /// As the TCK's own mocks play it; the invalid transitions of CN03, CNC03 are its own.
    fn script(test: &str, event: TckEvent) -> &'static [TckStep<TckMessage>] {
        use TckEvent::*;
        use TckMessage::*;
        use TckStep::*;
        match (test, event) {
            (
                "CN0101" | "CN0102" | "CN0103" | "CN0204" | "CN0206" | "CN0302" | "CN0303"
                | "CN0304",
                ProviderRequested,
            ) => &[Send(Offer)],
            ("CN0102", ProviderCounterRequested) => &[Send(Terminate)],
            ("CN0103", ProviderAccepted) => &[Send(Agreement)],
            ("CN0103" | "CN0104" | "CN0301", ProviderVerified) => &[Send(Finalize)],
            ("CN0104" | "CN0203" | "CN0207" | "CN0301", ProviderRequested) => &[Send(Agreement)],
            ("CN0201", ProviderRequested) => &[Send(Terminate)],
            ("CN0205", ProviderRequested) => &[Send(Offer), Pause, Send(Terminate)],
            ("CN0206", ProviderAccepted) => &[Send(Terminate)],
            ("CN0207", ProviderVerified) => &[Send(Terminate)],
            ("CNC0101" | "CNC0205" | "CNC0304" | "CNC0305" | "CNC0306", ConsumerOffered) => {
                &[Send(Accept)]
            }
            ("CNC0101" | "CNC0104" | "CNC0206", ConsumerAgreed) => &[Send(Verify)],
            ("CNC0102", ConsumerOffered) => &[Send(CounterRequest)],
            ("CNC0103", ConsumerOffered) => &[Send(Terminate)],
            ("CNC0202", ConsumerRequested) => &[Pause, Send(Terminate)],
            ("CNC0203", ConsumerAgreed) => &[Send(Terminate)],
            _ => &[],
        }
    }
}
