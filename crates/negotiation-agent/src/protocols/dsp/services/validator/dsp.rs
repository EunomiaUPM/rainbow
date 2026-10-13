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

//! Rules of the inbound DSP flow: shape over the input, then the loaded facts.

use common::dsp_common::DspRules;
use common::validation::{Stage, ValidationRegistry, Validator, Violations};

use crate::protocols::dsp::entities::context_dsp::{NegotiationDspDomain, NegotiationDspTyped};
use crate::protocols::dsp::entities::message_types::NegotiationMessageType;
use crate::protocols::dsp::entities::protocol_fields::NegotiationProtocolFields;
use crate::protocols::dsp::services::validator::rules::NegotiationRules;

/// Registries of the inbound DSP flow, and the message shapes the RPC flow reuses.
pub struct NegotiationDspRules;

impl NegotiationDspRules {
    /// The messages a peer sends us.
    pub const INBOUND: [NegotiationMessageType; 7] = [
        NegotiationMessageType::ContractRequestMessage,
        NegotiationMessageType::ContractOfferMessage,
        NegotiationMessageType::EventAccepted,
        NegotiationMessageType::EventFinalized,
        NegotiationMessageType::ContractAgreementMessage,
        NegotiationMessageType::ContractAgreementVerificationMessage,
        NegotiationMessageType::ContractNegotiationTerminationMessage,
    ];

    /// Inbound DSP shape rules over the extracted fields, before any lookup.
    pub fn input_rules() -> ValidationRegistry<NegotiationMessageType, NegotiationDspTyped> {
        Self::INBOUND.into_iter().fold(
            ValidationRegistry::new("negotiation.dsp.input")
                .with_any(Stage::Shape, Self::route_matches_body()),
            |rules, message| {
                rules.with(
                    message,
                    Stage::Shape,
                    Self::on_fields(Self::message_shape(message)),
                )
            },
        )
    }

    /// Inbound DSP rules over loaded facts: state machine, role, pids and the offer requested.
    pub fn fact_rules() -> ValidationRegistry<NegotiationMessageType, NegotiationDspDomain> {
        let rules = Self::INBOUND.into_iter().fold(
            ValidationRegistry::new("negotiation.dsp.facts"),
            |rules, message| {
                rules.with(
                    message,
                    Stage::Domain,
                    Validator::new()
                        .rule(Self::transition)
                        .rule(Self::role)
                        .rule(Self::correlated)
                        .rule(Self::path_matches_pid),
                )
            },
        );
        rules.with(
            NegotiationMessageType::ContractRequestMessage,
            Stage::Domain,
            Validator::new().when(
                |f: &NegotiationDspDomain| f.process.existing().is_none(),
                Self::offer_published,
            ),
        )
    }

    /// The DSP shape of a message, sent or received; pids are opaque, DSP does not type them.
    pub fn message_shape(message: NegotiationMessageType) -> Validator<NegotiationProtocolFields> {
        use NegotiationMessageType::*;
        match message {
            ContractRequestMessage => Self::consumer_pid()
                .ensure_not_empty("offer.@id", |f: &NegotiationProtocolFields| {
                    f.offer.as_ref()?.id.as_deref()
                })
                .ensure(
                    "callbackAddress",
                    "a request opening a negotiation needs a callbackAddress",
                    |f: &NegotiationProtocolFields| {
                        f.provider_pid.is_some() || Self::present(&f.callback_address)
                    },
                ),
            ContractOfferMessage => Self::provider_pid()
                .ensure_not_empty("offer.@id", |f: &NegotiationProtocolFields| {
                    f.offer.as_ref()?.id.as_deref()
                })
                .ensure(
                    "callbackAddress",
                    "an offer opening a negotiation needs a callbackAddress",
                    |f: &NegotiationProtocolFields| {
                        f.consumer_pid.is_some() || Self::present(&f.callback_address)
                    },
                ),
            ContractAgreementMessage => Self::pids()
                .ensure_not_empty("agreement.@id", |f: &NegotiationProtocolFields| {
                    f.agreement.as_ref()?.id.as_deref()
                })
                .ensure_urn("agreement.target", |f: &NegotiationProtocolFields| {
                    f.agreement.as_ref()?.target.as_deref()
                })
                .ensure_not_empty("agreement.assigner", |f: &NegotiationProtocolFields| {
                    f.agreement.as_ref()?.assigner.as_deref()
                })
                .ensure_not_empty("agreement.assignee", |f: &NegotiationProtocolFields| {
                    f.agreement.as_ref()?.assignee.as_deref()
                }),
            _ => Self::pids(),
        }
    }

    fn consumer_pid() -> Validator<NegotiationProtocolFields> {
        Validator::new().ensure_not_empty("consumerPid", |f: &NegotiationProtocolFields| {
            f.consumer_pid.as_deref()
        })
    }

    fn provider_pid() -> Validator<NegotiationProtocolFields> {
        Validator::new().ensure_not_empty("providerPid", |f: &NegotiationProtocolFields| {
            f.provider_pid.as_deref()
        })
    }

    fn pids() -> Validator<NegotiationProtocolFields> {
        Self::consumer_pid().merge(Self::provider_pid())
    }

    fn present(value: &Option<String>) -> bool {
        value.as_deref().is_some_and(|v| !v.trim().is_empty())
    }

    /// Runs a message-shape validator on the fields extracted from an inbound message.
    fn on_fields(fields: Validator<NegotiationProtocolFields>) -> Validator<NegotiationDspTyped> {
        Validator::new().rule(move |typed: &NegotiationDspTyped| fields.validate(&typed.fields))
    }

    /// The body's `@type` must be what the route handles; the events route takes both events.
    fn route_matches_body() -> Validator<NegotiationDspTyped> {
        Validator::new().ensure(
            "@type",
            "message type does not match the endpoint",
            |typed: &NegotiationDspTyped| typed.message.wire_type() == typed.route().wire_type(),
        )
    }

    fn transition(facts: &NegotiationDspDomain) -> Result<(), Violations> {
        let current = facts.process.existing().map(|p| p.state);
        NegotiationRules::state_transition(current.as_ref(), &facts.typed.message, "state")
    }

    fn role(facts: &NegotiationDspDomain) -> Result<(), Violations> {
        NegotiationRules::may_receive(facts.role, facts.typed.message, "role")
    }

    fn correlated(facts: &NegotiationDspDomain) -> Result<(), Violations> {
        let Some(process) = facts.process.existing() else {
            return Ok(());
        };
        let fields = &facts.typed.fields;
        DspRules::correlate_process_pids(
            (
                process.consumer_pid.as_deref(),
                process.provider_pid.as_deref(),
            ),
            (
                fields.consumer_pid.as_deref(),
                fields.provider_pid.as_deref(),
            ),
            "pids",
        )
    }

    fn path_matches_pid(facts: &NegotiationDspDomain) -> Result<(), Violations> {
        match (facts.typed.path_id(), facts.process.existing()) {
            (Some(id), Some(process)) => {
                NegotiationRules::path_is_own_pid(id, process.own_pid(), "uriId")
            }
            _ => Ok(()),
        }
    }

    fn offer_published(facts: &NegotiationDspDomain) -> Result<(), Violations> {
        NegotiationRules::offer_published(
            facts.catalog_offer.as_ref(),
            facts.typed.fields.offer.as_ref(),
            "offer",
        )
    }
}
