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

//! TCK scenarios: the connector's part of each test, looked up through the seeded mapping.

use transfer_agent::protocols::dsp::entities::tck::{TckEvent, TckMessage, TckStep};
use transfer_agent::protocols::dsp::services::tck::service::TckScenarios;

fn mapping_file() -> std::path::PathBuf {
    std::env::temp_dir().join(format!("tck-mapping-{}.json", uuid::Uuid::new_v4()))
}

fn write_mapping(path: &std::path::Path) {
    std::fs::write(
        path,
        r#"{"provider_agreements": {"TP0101": "urn:agreement:p1", "TP0205": "urn:agreement:p2"},
            "consumer_agreements": {"TPC0205": "urn:agreement:c5"}}"#,
    )
    .unwrap();
}

/// A provider test plays start, pause and termination after accepting the request.
#[test]
fn provider_scenario_follows_the_tck_script() {
    let path = mapping_file();
    write_mapping(&path);
    let steps = TckScenarios::new(&path).steps("urn:agreement:p1", TckEvent::ProviderRequested);
    assert_eq!(
        steps,
        &[
            TckStep::Send(TckMessage::Start),
            TckStep::Pause,
            TckStep::Send(TckMessage::Terminate)
        ]
    );
}

/// Tests where the TCK drives every message leave the connector idle.
#[test]
fn passive_tests_have_no_steps() {
    let path = mapping_file();
    write_mapping(&path);
    let scenarios = TckScenarios::new(&path);
    assert!(scenarios
        .steps("urn:agreement:p2", TckEvent::ProviderRequested)
        .is_empty());
    assert!(scenarios
        .steps("urn:agreement:c5", TckEvent::ConsumerStarted)
        .is_empty());
    assert!(scenarios
        .steps("urn:agreement:unknown", TckEvent::ProviderRequested)
        .is_empty());
}

/// A consumer test can act right after its request, before any start.
#[test]
fn consumer_scenario_can_follow_the_request() {
    let path = mapping_file();
    write_mapping(&path);
    let steps = TckScenarios::new(&path).steps("urn:agreement:c5", TckEvent::ConsumerRequested);
    assert_eq!(
        steps,
        &[TckStep::Pause, TckStep::Send(TckMessage::Terminate)]
    );
}

/// The mapping is read on lookup: one seeded after start is picked up, a missing one is empty.
#[test]
fn the_mapping_is_read_when_needed() {
    let path = mapping_file();
    let scenarios = TckScenarios::new(&path);
    assert!(scenarios
        .steps("urn:agreement:p1", TckEvent::ProviderRequested)
        .is_empty());
    write_mapping(&path);
    assert_eq!(
        scenarios
            .steps("urn:agreement:p1", TckEvent::ProviderRequested)
            .len(),
        3
    );
}
