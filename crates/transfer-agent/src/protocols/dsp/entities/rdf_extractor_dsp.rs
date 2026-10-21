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

//! Reading the Transfer Process Protocol fields (DSP 9.2) off an expanded
//! message. Everything protocol-agnostic lives in `common::rdf`.

use common::dsp_common::context_field::ContextField;
use common::dsp_common::data_address::{DataAddress, EndpointProperty};
use common::dsp_common::rdf::DspProfile;
use common::eunomia::EUNOMIA_NS;
use common::rdf::{RdfCanonicalizer, RdfEngine, RdfNode, RdfProtocolTrait};
use serde_json::{json, Value};
use ymir::errors::{BadFormat, Errors, Outcome};

use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::protocol_fields::TransferProtocolFields;

/// Namespace every DSP term expands into.
const DSPACE: &str = "https://w3id.org/dspace/2025/1/";
/// `format` is Dublin Core in the DSP context, not a `dspace:` term.
const DCT_FORMAT: &str = "http://purl.org/dc/terms/format";
/// Our extension term on a request; not part of DSP, so a standard peer ignores it.
const AUTO_START: &str = "autoStart";

/// The transfer protocol, as an extraction target.
#[derive(Debug, Clone)]
pub struct DspTransfer;

impl RdfProtocolTrait for DspTransfer {
    type MessageType = TransferDSPMessageType;
    type Fields = TransferProtocolFields;

    fn engine() -> &'static RdfEngine {
        DspProfile::shared()
    }

    fn context() -> Value {
        serde_json::to_value(ContextField::default()).unwrap_or_default()
    }

    fn compact_type(message: &TransferDSPMessageType) -> String {
        message.to_string()
    }

    fn type_iri(message: &TransferDSPMessageType) -> String {
        Self::dspace(&message.to_string())
    }

    /// Only the DSP namespace counts: in expanded form every `@type` is a full
    /// IRI, so a same-named term from elsewhere is a different message.
    fn message_type(node: &RdfNode<'_, '_>) -> Option<TransferDSPMessageType> {
        node.types()
            .filter_map(|iri| iri.strip_prefix(DSPACE))
            .find_map(|term| term.parse().ok())
    }

    fn extract(node: &RdfNode<'_, '_>) -> Outcome<TransferProtocolFields> {
        Ok(TransferProtocolFields {
            consumer_pid: Self::id(node, &Self::dspace("consumerPid")),
            provider_pid: Self::id(node, &Self::dspace("providerPid")),
            agreement_id: Self::id(node, &Self::dspace("agreementId")),
            callback_address: Self::owned(node, &Self::dspace("callbackAddress")),
            format: node.iri_or_literal(DCT_FORMAT).map(Self::compact_format),
            data_address: Self::data_address(node)?,
            code: Self::owned(node, &Self::dspace("code")),
            reason: node
                .values(&Self::dspace("reason"))
                .filter_map(|v| v.get("@value").and_then(serde_json::Value::as_str))
                .map(str::to_string)
                .collect(),
            auto_start: node
                .get_opt_bool(&format!("{EUNOMIA_NS}{AUTO_START}"))?
                .unwrap_or(false),
        })
    }
}

impl DspTransfer {
    /// The DSP context plus the eunomia prefix, for a request carrying `eunomia:autoStart`.
    pub(crate) fn auto_start_context() -> Value {
        let mut contexts = match Self::context() {
            Value::Array(contexts) => contexts,
            single => vec![single],
        };
        contexts.push(json!({ "eunomia": EUNOMIA_NS }));
        Value::Array(contexts)
    }

    /// The compact member asking a provider to start the transfer unasked.
    pub(crate) fn auto_start_member() -> String {
        format!("eunomia:{AUTO_START}")
    }

    /// A `dataAddress` is optional, but a present one missing a required member is
    /// malformed rather than absent.
    pub(crate) fn data_address(node: &RdfNode<'_, '_>) -> Outcome<Option<DataAddress>> {
        let Some(address) = node.object(&Self::dspace("dataAddress")) else {
            return Ok(None);
        };
        let endpoint_type = Self::owned(&address, &Self::dspace("endpointType"))
            .ok_or_else(|| Self::missing("dataAddress.endpointType"))?;

        // RDF is unordered: this array is not the sender's order. Look properties up
        // by name, never by index.
        let endpoint_properties = address
            .objects(&Self::dspace("endpointProperties"))
            .map(|p| Self::endpoint_property(&p))
            .collect::<Outcome<Vec<_>>>()?;

        Ok(Some(DataAddress {
            _type: Self::term_of(&address).unwrap_or_else(|| "DataAddress".to_string()),
            endpoint_type,
            // OPTIONAL per DSP Appendix A; whether a message needs one is a domain
            // rule, since it depends on the connector behind the `format`.
            endpoint: Self::owned(&address, &Self::dspace("endpoint")),
            endpoint_properties,
        }))
    }

    fn endpoint_property(node: &RdfNode<'_, '_>) -> Outcome<EndpointProperty> {
        Ok(EndpointProperty {
            _type: Self::term_of(node).unwrap_or_else(|| "EndpointProperty".to_string()),
            name: Self::owned(node, &Self::dspace("name"))
                .ok_or_else(|| Self::missing("dataAddress.endpointProperties[].name"))?,
            value: Self::owned(node, &Self::dspace("value"))
                .ok_or_else(|| Self::missing("dataAddress.endpointProperties[].value"))?,
        })
    }

    /// `format` is `@vocab`-typed, so a bare term (`http+pull`) expands to an IRI under the
    /// document base; it is read back as sent, which is what a distribution stores.
    fn compact_format(expanded: &str) -> String {
        Self::below_base(expanded)
            .or_else(|| expanded.strip_prefix(DSPACE))
            .unwrap_or(expanded)
            .to_string()
    }

    /// An `@id`-typed member (pids, agreement) read back as sent: a value that is not an
    /// absolute IRI, such as a bare UUID pid, expands under the document base.
    fn id(node: &RdfNode<'_, '_>, predicate: &str) -> Option<String> {
        node.iri_or_literal(predicate)
            .map(|value| Self::below_base(value).unwrap_or(value).to_string())
    }

    fn below_base(expanded: &str) -> Option<&str> {
        expanded
            .strip_prefix(RdfCanonicalizer::DOCUMENT_BASE)
            .map(|rest| rest.trim_start_matches('/'))
    }

    /// The full IRI a DSP term expands to.
    fn dspace(term: &str) -> String {
        format!("{DSPACE}{term}")
    }

    fn owned(node: &RdfNode<'_, '_>, predicate: &str) -> Option<String> {
        node.iri_or_literal(predicate).map(str::to_string)
    }

    /// The node's `@type` back as the compact DSP term, the shape entities store.
    fn term_of(node: &RdfNode<'_, '_>) -> Option<String> {
        let iri = node.types().next()?;
        Some(iri.strip_prefix(DSPACE).unwrap_or(iri).to_string())
    }

    fn missing(field: &str) -> Errors {
        Errors::format(
            BadFormat::Received,
            format!("expanded message is missing {field}"),
            None,
        )
    }
}
