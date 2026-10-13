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

//! Reading the Contract Negotiation Protocol fields (DSP 8.2, 8.3) off an expanded message.
//! Everything protocol-agnostic lives in `common::rdf`.

use common::dsp_common::context_field::ContextField;
use common::dsp_common::rdf::DspProfile;
use common::rdf::{RdfCanonicalizer, RdfEngine, RdfNode, RdfProtocolTrait};
use serde_json::{Map, Value};
use ymir::errors::Outcome;

use crate::protocols::dsp::entities::message_types::NegotiationMessageType;
use crate::protocols::dsp::entities::protocol_fields::{
    AgreementFields, NegotiationProtocolFields, OfferFields,
};

/// Namespace every DSP term expands into.
const DSPACE: &str = "https://w3id.org/dspace/2025/1/";
/// Namespace of the policy terms the DSP context imports.
const ODRL: &str = "http://www.w3.org/ns/odrl/2/";

/// The negotiation protocol, as an extraction target.
#[derive(Debug, Clone)]
pub struct DspNegotiation;

impl RdfProtocolTrait for DspNegotiation {
    type MessageType = NegotiationMessageType;
    type Fields = NegotiationProtocolFields;

    fn engine() -> &'static RdfEngine {
        DspProfile::shared()
    }

    fn context() -> Value {
        serde_json::to_value(ContextField::default()).unwrap_or_default()
    }

    fn compact_type(message: &NegotiationMessageType) -> String {
        message.wire_type().to_string()
    }

    fn type_iri(message: &NegotiationMessageType) -> String {
        Self::dspace(message.wire_type())
    }

    /// Only the DSP namespace counts; an event message is typed by its `eventType` too.
    fn message_type(node: &RdfNode<'_, '_>) -> Option<NegotiationMessageType> {
        let event = node.iri_or_literal(&Self::dspace("eventType"));
        node.types()
            .filter_map(|iri| iri.strip_prefix(DSPACE))
            .find_map(|term| NegotiationMessageType::from_wire(term, event))
    }

    fn extract(node: &RdfNode<'_, '_>) -> Outcome<NegotiationProtocolFields> {
        Ok(NegotiationProtocolFields {
            consumer_pid: Self::id(node, &Self::dspace("consumerPid")),
            provider_pid: Self::id(node, &Self::dspace("providerPid")),
            callback_address: Self::owned(node, &Self::dspace("callbackAddress")),
            offer: node
                .object(&Self::dspace("offer"))
                .map(|offer| OfferFields {
                    id: offer.id().map(Self::as_sent),
                    target: Self::id(&offer, &Self::odrl("target")),
                }),
            agreement: node
                .object(&Self::dspace("agreement"))
                .map(|agreement| AgreementFields {
                    id: agreement.id().map(Self::as_sent),
                    target: Self::id(&agreement, &Self::odrl("target")),
                    assigner: Self::id(&agreement, &Self::odrl("assigner")),
                    assignee: Self::id(&agreement, &Self::odrl("assignee")),
                    timestamp: Self::owned(&agreement, &Self::dspace("timestamp")),
                }),
            event_type: Self::term(node, &Self::dspace("eventType")),
            state: Self::term(node, &Self::dspace("state")),
            code: Self::owned(node, &Self::dspace("code")),
            reason: node
                .values(&Self::dspace("reason"))
                .filter_map(|v| v.get("@value").and_then(Value::as_str))
                .map(str::to_string)
                .collect(),
        })
    }
}

impl DspNegotiation {
    /// The members of `message` plus the `eventType` an event message needs.
    pub fn members(
        message: NegotiationMessageType,
        mut members: Map<String, Value>,
    ) -> Map<String, Value> {
        if let Some(event) = message.event_type() {
            members.insert("eventType".into(), Value::String(event.into()));
        }
        members
    }

    /// An `@id`-typed member read back as sent: a value that is not an absolute IRI, such as a
    /// bare UUID pid, expands under the document base.
    fn id(node: &RdfNode<'_, '_>, predicate: &str) -> Option<String> {
        node.iri_or_literal(predicate).map(Self::as_sent)
    }

    fn as_sent(expanded: &str) -> String {
        expanded
            .strip_prefix(RdfCanonicalizer::DOCUMENT_BASE)
            .map(|rest| rest.trim_start_matches('/'))
            .unwrap_or(expanded)
            .to_string()
    }

    /// A `@vocab`-typed member back as its DSP term.
    fn term(node: &RdfNode<'_, '_>, predicate: &str) -> Option<String> {
        let value = node.iri_or_literal(predicate)?;
        Some(value.strip_prefix(DSPACE).unwrap_or(value).to_string())
    }

    fn dspace(term: &str) -> String {
        format!("{DSPACE}{term}")
    }

    fn odrl(term: &str) -> String {
        format!("{ODRL}{term}")
    }

    fn owned(node: &RdfNode<'_, '_>, predicate: &str) -> Option<String> {
        node.iri_or_literal(predicate).map(str::to_string)
    }
}
