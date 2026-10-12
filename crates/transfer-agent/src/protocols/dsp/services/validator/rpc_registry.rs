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

//! Rules for outbound transfer messages: what the RPC body needs, what the role and state
//! allow, the shape of the message as it will go out, and the peer's answer.

use common::validation::{Rule, Stage, ValidationRegistry, Validator};
use serde_json::Value;

use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::context_rpc::TransferRpcCommand;
use crate::protocols::dsp::entities::context_rpc::TransferRpcFacts;
use crate::protocols::dsp::services::validator::{TransferRules, TransferValidators};

/// Rule registries of the outbound transfer flow.
pub struct TransferRpcRules;

impl TransferRpcRules {
    const CONTINUATIONS: [TransferDSPMessageType; 4] = [
        TransferDSPMessageType::TransferStartMessage,
        TransferDSPMessageType::TransferCompletionMessage,
        TransferDSPMessageType::TransferSuspensionMessage,
        TransferDSPMessageType::TransferTerminationMessage,
    ];

    /// What the RPC body must carry before anything is looked up.
    pub fn input_rules() -> ValidationRegistry<TransferDSPMessageType, TransferRpcCommand> {
        let request = Validator::new()
            .ensure_not_empty("associatedAgentPeer", |c: &TransferRpcCommand| {
                c.body.associated_agent_peer.as_deref()
            })
            .ensure_not_empty("providerAddress", |c: &TransferRpcCommand| {
                c.body.provider_address.as_deref()
            })
            .ensure_not_empty("format", |c: &TransferRpcCommand| c.body.format.as_deref())
            .ensure_urn("agreementId", |c: &TransferRpcCommand| {
                c.body.agreement_id.as_deref()
            });
        Self::CONTINUATIONS.into_iter().fold(
            ValidationRegistry::new("transfer.rpc.input").with(
                TransferDSPMessageType::TransferRequestMessage,
                Stage::Shape,
                request,
            ),
            |rules, message| {
                rules.with(
                    message,
                    Stage::Shape,
                    Validator::new()
                        .ensure_urn("providerPid|consumerPid", |c: &TransferRpcCommand| c.pid()),
                )
            },
        )
    }

    /// Our role and state must allow sending it, and the message must have the DSP shape.
    pub fn fact_rules() -> ValidationRegistry<TransferDSPMessageType, TransferRpcFacts> {
        use TransferDSPMessageType::*;
        ValidationRegistry::new("transfer.rpc.facts")
            .with_any(
                Stage::Domain,
                Validator::new()
                    .rule(|f: &TransferRpcFacts| {
                        TransferRules::role_may_send(&f.role, &f.command.message, "role")
                    })
                    .rule(|f: &TransferRpcFacts| {
                        TransferRules::state_transition(
                            f.current_state().as_ref(),
                            &f.command.message,
                            "state",
                        )
                    }),
            )
            .with(
                TransferRequestMessage,
                Stage::Shape,
                Self::outgoing(TransferValidators::request_message()),
            )
            .with(
                TransferStartMessage,
                Stage::Shape,
                Self::outgoing(TransferValidators::pids_message()),
            )
            .with(
                TransferCompletionMessage,
                Stage::Shape,
                Self::outgoing(TransferValidators::pids_message()),
            )
            .with(
                TransferSuspensionMessage,
                Stage::Shape,
                Self::outgoing(TransferValidators::pids_message()),
            )
            .with(
                TransferTerminationMessage,
                Stage::Shape,
                Self::outgoing(TransferValidators::pids_message()),
            )
    }

    /// The answer to a request is the provider's `TransferProcess`, with its new pid.
    pub fn reply_rules() -> ValidationRegistry<TransferDSPMessageType, Value> {
        let request = Validator::new()
            .ensure(
                "@type",
                "the provider must answer with a TransferProcess",
                |v: &Value| v["type"] == "TransferProcess",
            )
            .ensure_urn("providerPid", |v: &Value| {
                v["fields"]["providerPid"].as_str()
            });
        Self::CONTINUATIONS.into_iter().fold(
            ValidationRegistry::new("transfer.rpc.reply").with(
                TransferDSPMessageType::TransferRequestMessage,
                Stage::Shape,
                request,
            ),
            |rules, message| rules.with(message, Stage::Shape, Validator::new()),
        )
    }

    /// Runs a message-shape validator on the message as it will be sent.
    fn outgoing(
        fields: Validator<crate::protocols::dsp::entities::protocol_fields::TransferProtocolFields>,
    ) -> Validator<TransferRpcFacts> {
        Validator::new().rule(move |f: &TransferRpcFacts| fields.check(&f.outgoing.fields))
    }
}
