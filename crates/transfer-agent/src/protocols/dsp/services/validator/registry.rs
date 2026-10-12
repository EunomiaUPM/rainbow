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

//! Rule registries for inbound DSP messages: shape over the input, then the loaded facts.

use common::dsp_common::DspRules;
use common::validation::{Rule, Stage, ValidationRegistry, Validator, Violations};
use std::str::FromStr;

use crate::protocols::dsp::entities::context_common::TransferContextProcessSlot;
use crate::protocols::dsp::entities::context_dsp::{TransferDspFacts, TransferDspInput};
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::protocol_fields::TransferProtocolFields;
use crate::protocols::dsp::entities::state_metadata::TransferDSPStateAttribute;
use crate::protocols::dsp::services::validator::rules::TransferRules;

/// Validator registries for inbound DSP messages and the message shapes the RPC side reuses.
pub struct TransferValidators;

impl TransferValidators {
    /// Inbound DSP shape rules over the extracted fields, before any lookup.
    pub fn input_rules() -> ValidationRegistry<TransferDSPMessageType, TransferDspInput> {
        use TransferDSPMessageType::*;
        let pids = || Self::on_fields(Self::pids_message());
        ValidationRegistry::new("transfer.dsp.input")
            .with_any(Stage::Shape, Self::route_matches_body())
            .with(TransferRequestMessage, Stage::Shape, Self::on_fields(Self::request_message()))
            .with(TransferStartMessage, Stage::Shape, pids().rule(Self::path_id_is_urn))
            .with(TransferCompletionMessage, Stage::Shape, pids())
            .with(TransferSuspensionMessage, Stage::Shape, pids())
            .with(TransferTerminationMessage, Stage::Shape, pids())
    }

    /// Inbound DSP rules over loaded facts: state machine, role, semaphore and correlation.
    pub fn fact_rules() -> ValidationRegistry<TransferDSPMessageType, TransferDspFacts> {
        use TransferDSPMessageType::*;
        let correlated = || {
            Validator::new()
                .rule(Self::transition)
                .rule(Self::correlated)
        };
        ValidationRegistry::new("transfer.dsp.facts")
            .with(
                TransferRequestMessage,
                Stage::Domain,
                Validator::new().rule(Self::transition).rule(Self::role),
            )
            .with(
                TransferStartMessage,
                Stage::Domain,
                correlated()
                    .rule(Self::role)
                    .rule(Self::semaphore)
                    .rule(Self::data_address)
                    .rule(Self::path_matches_pid),
            )
            .with(TransferCompletionMessage, Stage::Domain, correlated())
            .with(TransferSuspensionMessage, Stage::Domain, correlated())
            .with(TransferTerminationMessage, Stage::Domain, correlated())
    }

    /// Shape of a request message, sent or received.
    pub fn request_message() -> Validator<TransferProtocolFields> {
        Validator::new()
            .ensure_urn("consumerPid", |f: &TransferProtocolFields| {
                f.consumer_pid.as_deref()
            })
            .ensure_urn("agreementId", |f: &TransferProtocolFields| {
                f.agreement_id.as_deref()
            })
            .ensure_not_empty("format", |f: &TransferProtocolFields| f.format.as_deref())
    }

    /// Shape of a start, completion, suspension or termination message: both pids, as URNs.
    pub fn pids_message() -> Validator<TransferProtocolFields> {
        Validator::new()
            .ensure_urn("consumerPid", |f: &TransferProtocolFields| {
                f.consumer_pid.as_deref()
            })
            .ensure_urn("providerPid", |f: &TransferProtocolFields| {
                f.provider_pid.as_deref()
            })
    }

    /// Runs a message-shape validator on the fields extracted from an inbound message.
    fn on_fields(fields: Validator<TransferProtocolFields>) -> Validator<TransferDspInput> {
        Validator::new().rule(move |input: &TransferDspInput| fields.check(&input.body.fields))
    }

    /// The body's `@type` must be the message the route handles.
    fn route_matches_body() -> Validator<TransferDspInput> {
        Validator::new().ensure(
            "@type",
            "message type does not match the endpoint",
            |input: &TransferDspInput| input.body.message == input.route,
        )
    }

    fn path_id_is_urn(input: &TransferDspInput) -> Result<(), Violations> {
        match input.path_id.as_deref() {
            Some(id) => DspRules::is_urn(id, "uriId"),
            None => Ok(()),
        }
    }

    fn transition(facts: &TransferDspFacts) -> Result<(), Violations> {
        TransferRules::state_transition(
            TransferRules::current_state(&facts.process).as_ref(),
            &facts.input.body.message,
            "state",
        )
    }

    fn role(facts: &TransferDspFacts) -> Result<(), Violations> {
        TransferRules::role_for_message(&facts.role, &facts.input.body.message, "role")
    }

    fn semaphore(facts: &TransferDspFacts) -> Result<(), Violations> {
        TransferRules::semaphore(
            &Self::stored_attribute(&facts.process),
            &facts.input.body.message,
            &facts.role,
            "stateAttribute",
        )
    }

    fn data_address(facts: &TransferDspFacts) -> Result<(), Violations> {
        TransferRules::data_address_format(
            facts.input.body.fields.data_address.is_some(),
            &facts.role,
            "dataAddress",
        )
    }

    fn path_matches_pid(facts: &TransferDspFacts) -> Result<(), Violations> {
        let fields = &facts.input.body.fields;
        match facts.input.path_id.as_deref() {
            Some(id) => TransferRules::uri_and_pid(
                id,
                fields.consumer_pid.as_deref(),
                fields.provider_pid.as_deref(),
                &facts.role,
                "uriId",
            ),
            None => Ok(()),
        }
    }

    fn correlated(facts: &TransferDspFacts) -> Result<(), Violations> {
        let fields = &facts.input.body.fields;
        DspRules::correlate_process_pids(
            Self::stored_pids(&facts.process),
            (fields.consumer_pid.as_deref(), fields.provider_pid.as_deref()),
            "pids",
        )
    }

    fn stored_attribute(process: &TransferContextProcessSlot) -> TransferDSPStateAttribute {
        match process {
            TransferContextProcessSlot::Existing(p) => p
                .state_metadata()
                .attribute
                .as_deref()
                .and_then(|s| TransferDSPStateAttribute::from_str(s).ok())
                .unwrap_or(TransferDSPStateAttribute::OnRequest),
            TransferContextProcessSlot::New { .. } => TransferDSPStateAttribute::OnRequest,
        }
    }

    fn stored_pids(process: &TransferContextProcessSlot) -> (Option<&str>, Option<&str>) {
        match process {
            TransferContextProcessSlot::Existing(p) => (
                p.correlation().consumer_pid.as_deref(),
                p.correlation().provider_pid.as_deref(),
            ),
            TransferContextProcessSlot::New { consumer_pid } => (Some(consumer_pid.as_str()), None),
        }
    }
}
