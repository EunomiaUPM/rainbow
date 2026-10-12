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
use common::validation::{Rule, Stage, ValidationRegistry, Validator, Violations, codes};
use std::str::FromStr;

use crate::protocols::dsp::entities::context_common::TransferContextProcessSlot;
use crate::protocols::dsp::entities::context_dsp::{
    TransferDSPContextDomain, TransferDSPContextTyped,
};
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::protocol_fields::TransferProtocolFields;
use crate::protocols::dsp::entities::state_metadata::TransferDSPStateAttribute;
use crate::protocols::dsp::services::validator::rules::TransferRules;

/// Validator registries for inbound DSP messages and the message shapes the RPC side reuses.
pub struct TransferValidators;

impl TransferValidators {
    /// Inbound DSP rules over loaded facts: state machine, role, semaphore and correlation.
    pub fn fact_rules() -> ValidationRegistry<TransferDSPMessageType, TransferDSPContextDomain> {
        use TransferDSPMessageType::*;
        ValidationRegistry::new("transfer.dsp.facts")
            .with(TransferRequestMessage, Stage::Domain, Self::request_dsp())
            .with(TransferStartMessage, Stage::Domain, Self::start_dsp())
            .with(TransferCompletionMessage, Stage::Domain, Self::completion_dsp())
            .with(TransferSuspensionMessage, Stage::Domain, Self::suspension_dsp())
            .with(TransferTerminationMessage, Stage::Domain, Self::termination_dsp())
    }

    /// Inbound DSP shape rules over the extracted fields, before any lookup.
    pub fn input_rules() -> ValidationRegistry<TransferDSPMessageType, TransferDSPContextTyped> {
        use TransferDSPMessageType::*;
        ValidationRegistry::new("transfer.dsp.input")
            .with_any(Stage::Shape, Self::route_matches_body())
            .with(TransferRequestMessage, Stage::Shape, Self::on_fields(Self::request_message()))
            .with(TransferStartMessage, Stage::Shape, Self::on_fields(Self::start_message()))
            .with(TransferCompletionMessage, Stage::Shape, Self::on_fields(Self::completion_message()))
            .with(TransferSuspensionMessage, Stage::Shape, Self::on_fields(Self::suspension_message()))
            .with(TransferTerminationMessage, Stage::Shape, Self::on_fields(Self::termination_message()))
    }

    /// Runs a message-shape validator on the fields extracted from an inbound message.
    fn on_fields(fields: Validator<TransferProtocolFields>) -> Validator<TransferDSPContextTyped> {
        Validator::new().rule(move |ctx: &TransferDSPContextTyped| fields.check(&ctx.fields))
    }

    /// The body's `@type` must be the message the route handles.
    fn route_matches_body() -> Validator<TransferDSPContextTyped> {
        Validator::new().ensure(
            "@type",
            "message type does not match the endpoint",
            |ctx: &TransferDSPContextTyped| ctx.message == ctx.rdf.parsed.dsp_message_type,
        )
    }

    /// Shape of a request message, sent or received.
    pub fn request_message() -> Validator<TransferProtocolFields> {
        Validator::new().rule(|ctx: &TransferProtocolFields| {
            let mut v = Violations::new();
            if let Some(consumer_pid) = &ctx.consumer_pid {
                if let Err(vs) = DspRules::is_urn(consumer_pid, "consumerPid") {
                    v.extend(vs);
                }
            } else {
                v.push(common::validation::Violation::new(
                    "consumerPid",
                    codes::MISSING,
                    "is required",
                ));
            }
            if let Some(agreement_id) = &ctx.agreement_id {
                if let Err(vs) = DspRules::is_urn(agreement_id, "agreementId") {
                    v.extend(vs);
                }
            } else {
                v.push(common::validation::Violation::new(
                    "agreementId",
                    codes::MISSING,
                    "is required",
                ));
            }
            if ctx.format.as_deref().unwrap_or("").trim().is_empty() {
                v.push(common::validation::Violation::new(
                    "format",
                    codes::MISSING,
                    "is required",
                ));
            }
            v.into_result()
        })
    }

    /// Shape of a start message, sent or received.
    pub fn start_message() -> Validator<TransferProtocolFields> {
        Validator::new().rule(|ctx: &TransferProtocolFields| {
            let mut v = Violations::new();
            Self::check_urn_opt(&mut v, "consumerPid", ctx.consumer_pid.as_deref());
            Self::check_urn_opt(&mut v, "providerPid", ctx.provider_pid.as_deref());
            v.into_result()
        })
    }

    /// Shape of a completion message, sent or received.
    pub fn completion_message() -> Validator<TransferProtocolFields> {
        Validator::new().rule(|ctx: &TransferProtocolFields| {
            let mut v = Violations::new();
            Self::check_urn_opt(&mut v, "consumerPid", ctx.consumer_pid.as_deref());
            Self::check_urn_opt(&mut v, "providerPid", ctx.provider_pid.as_deref());
            v.into_result()
        })
    }

    /// Shape of a suspension message, sent or received.
    pub fn suspension_message() -> Validator<TransferProtocolFields> {
        Validator::new().rule(|ctx: &TransferProtocolFields| {
            let mut v = Violations::new();
            Self::check_urn_opt(&mut v, "consumerPid", ctx.consumer_pid.as_deref());
            Self::check_urn_opt(&mut v, "providerPid", ctx.provider_pid.as_deref());
            v.into_result()
        })
    }

    /// Shape of a termination message, sent or received.
    pub fn termination_message() -> Validator<TransferProtocolFields> {
        Validator::new().rule(|ctx: &TransferProtocolFields| {
            let mut v = Violations::new();
            Self::check_urn_opt(&mut v, "consumerPid", ctx.consumer_pid.as_deref());
            Self::check_urn_opt(&mut v, "providerPid", ctx.provider_pid.as_deref());
            v.into_result()
        })
    }

    fn request_dsp() -> Validator<TransferDSPContextDomain> {
        Validator::new()
            .rule(|ctx: &TransferDSPContextDomain| {
                let mut v = Violations::new();
                Self::check_urn_opt(
                    &mut v,
                    "consumerPid",
                    ctx.typed.fields.consumer_pid.as_deref(),
                );
                Self::check_urn_opt(
                    &mut v,
                    "agreementId",
                    ctx.typed.fields.agreement_id.as_deref(),
                );
                v.into_result()
            })
            .then()
            .rule(|ctx: &TransferDSPContextDomain| {
                let mut v = Violations::new();
                let current_state = TransferRules::current_state(&ctx.process);
                if let Err(vs) = TransferRules::state_transition(
                    current_state.as_ref(),
                    &ctx.typed.message,
                    "state",
                ) {
                    v.extend(vs);
                }
                if let Err(vs) =
                    TransferRules::role_for_message(&ctx.role, &ctx.typed.message, "role")
                {
                    v.extend(vs);
                }
                v.into_result()
            })
    }

    fn start_dsp() -> Validator<TransferDSPContextDomain> {
        Validator::new()
            .rule(|ctx: &TransferDSPContextDomain| {
                let mut v = Violations::new();
                Self::check_urn_opt(
                    &mut v,
                    "consumerPid",
                    ctx.typed.fields.consumer_pid.as_deref(),
                );
                Self::check_urn_opt(
                    &mut v,
                    "providerPid",
                    ctx.typed.fields.provider_pid.as_deref(),
                );
                if let Some(uri_id) = ctx.typed.rdf.parsed.raw.path_id.as_deref() {
                    Self::check_urn_opt(&mut v, "uriId", Some(uri_id));
                }
                v.into_result()
            })
            .then()
            .rule(|ctx: &TransferDSPContextDomain| {
                let mut v = Violations::new();
                let current_state = TransferRules::current_state(&ctx.process);
                let attr = Self::extract_attribute(&ctx.process);

                if let Err(vs) = TransferRules::state_transition(
                    current_state.as_ref(),
                    &ctx.typed.message,
                    "state",
                ) {
                    v.extend(vs);
                }
                if let Err(vs) =
                    TransferRules::role_for_message(&ctx.role, &ctx.typed.message, "role")
                {
                    v.extend(vs);
                }
                if let Err(vs) =
                    TransferRules::semaphore(&attr, &ctx.typed.message, &ctx.role, "stateAttribute")
                {
                    v.extend(vs);
                }
                if let Err(vs) = TransferRules::data_address_format(ctx.typed.fields.data_address.is_some(), &ctx.role, "dataAddress") {
                    v.extend(vs);
                }
                if let Some(uri_id) = ctx.typed.rdf.parsed.raw.path_id.as_deref() {
                    if let Err(vs) = TransferRules::uri_and_pid(
                        uri_id,
                        ctx.typed.fields.consumer_pid.as_deref(),
                        ctx.typed.fields.provider_pid.as_deref(),
                        &ctx.role,
                        "uriId",
                    ) {
                        v.extend(vs);
                    }
                }
                let (proc_cons, proc_prov) = Self::extract_stored_pids(&ctx.process);
                if let Err(vs) = TransferRules::correlation(
                    proc_cons,
                    proc_prov,
                    ctx.typed.fields.consumer_pid.as_deref(),
                    ctx.typed.fields.provider_pid.as_deref(),
                    "pids",
                ) {
                    v.extend(vs);
                }
                v.into_result()
            })
    }

    fn completion_dsp() -> Validator<TransferDSPContextDomain> {
        Validator::new()
            .rule(|ctx: &TransferDSPContextDomain| {
                let mut v = Violations::new();
                Self::check_urn_opt(
                    &mut v,
                    "consumerPid",
                    ctx.typed.fields.consumer_pid.as_deref(),
                );
                Self::check_urn_opt(
                    &mut v,
                    "providerPid",
                    ctx.typed.fields.provider_pid.as_deref(),
                );
                v.into_result()
            })
            .then()
            .rule(|ctx: &TransferDSPContextDomain| {
                let mut v = Violations::new();
                let current_state = TransferRules::current_state(&ctx.process);
                if let Err(vs) = TransferRules::state_transition(
                    current_state.as_ref(),
                    &ctx.typed.message,
                    "state",
                ) {
                    v.extend(vs);
                }
                let (proc_cons, proc_prov) = Self::extract_stored_pids(&ctx.process);
                if let Err(vs) = TransferRules::correlation(
                    proc_cons,
                    proc_prov,
                    ctx.typed.fields.consumer_pid.as_deref(),
                    ctx.typed.fields.provider_pid.as_deref(),
                    "pids",
                ) {
                    v.extend(vs);
                }
                v.into_result()
            })
    }

    fn suspension_dsp() -> Validator<TransferDSPContextDomain> {
        Validator::new()
            .rule(|ctx: &TransferDSPContextDomain| {
                let mut v = Violations::new();
                Self::check_urn_opt(
                    &mut v,
                    "consumerPid",
                    ctx.typed.fields.consumer_pid.as_deref(),
                );
                Self::check_urn_opt(
                    &mut v,
                    "providerPid",
                    ctx.typed.fields.provider_pid.as_deref(),
                );
                v.into_result()
            })
            .then()
            .rule(|ctx: &TransferDSPContextDomain| {
                let mut v = Violations::new();
                let current_state = TransferRules::current_state(&ctx.process);
                if let Err(vs) = TransferRules::state_transition(
                    current_state.as_ref(),
                    &ctx.typed.message,
                    "state",
                ) {
                    v.extend(vs);
                }
                let (proc_cons, proc_prov) = Self::extract_stored_pids(&ctx.process);
                if let Err(vs) = TransferRules::correlation(
                    proc_cons,
                    proc_prov,
                    ctx.typed.fields.consumer_pid.as_deref(),
                    ctx.typed.fields.provider_pid.as_deref(),
                    "pids",
                ) {
                    v.extend(vs);
                }
                v.into_result()
            })
    }

    fn termination_dsp() -> Validator<TransferDSPContextDomain> {
        Validator::new()
            .rule(|ctx: &TransferDSPContextDomain| {
                let mut v = Violations::new();
                Self::check_urn_opt(
                    &mut v,
                    "consumerPid",
                    ctx.typed.fields.consumer_pid.as_deref(),
                );
                Self::check_urn_opt(
                    &mut v,
                    "providerPid",
                    ctx.typed.fields.provider_pid.as_deref(),
                );
                v.into_result()
            })
            .then()
            .rule(|ctx: &TransferDSPContextDomain| {
                let mut v = Violations::new();
                let current_state = TransferRules::current_state(&ctx.process);
                if let Err(vs) = TransferRules::state_transition(
                    current_state.as_ref(),
                    &ctx.typed.message,
                    "state",
                ) {
                    v.extend(vs);
                }
                let (proc_cons, proc_prov) = Self::extract_stored_pids(&ctx.process);
                if let Err(vs) = TransferRules::correlation(
                    proc_cons,
                    proc_prov,
                    ctx.typed.fields.consumer_pid.as_deref(),
                    ctx.typed.fields.provider_pid.as_deref(),
                    "pids",
                ) {
                    v.extend(vs);
                }
                v.into_result()
            })
    }

    fn check_urn_opt(v: &mut Violations, field: &'static str, val: Option<&str>) {
        match val {
            Some(s) if s.starts_with("urn:") => {}
            Some(_) => v.push(common::validation::Violation::new(
                field,
                codes::MALFORMED,
                "must be a valid URN",
            )),
            None => v.push(common::validation::Violation::new(
                field,
                codes::MISSING,
                "is required",
            )),
        }
    }

    fn extract_attribute(process: &TransferContextProcessSlot) -> TransferDSPStateAttribute {
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

    fn extract_stored_pids(process: &TransferContextProcessSlot) -> (Option<&str>, Option<&str>) {
        match process {
            TransferContextProcessSlot::Existing(p) => (
                p.correlation().consumer_pid.as_deref(),
                p.correlation().provider_pid.as_deref(),
            ),
            TransferContextProcessSlot::New { consumer_pid } => (Some(consumer_pid.as_str()), None),
        }
    }
}
