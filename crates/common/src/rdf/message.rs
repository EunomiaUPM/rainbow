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

//! A protocol message read through RDF: its JSON-LD, declared type, extracted fields and
//! canonical form, for any protocol described by an [`RdfProtocolTrait`].

use std::fmt::{Debug, Display};

use serde_json::{json, Map, Value};
use ymir::errors::{BadFormat, Errors, Outcome};

use crate::rdf::engine::RdfEngine;
use crate::rdf::node::{ExpandedDoc, RdfNode};

/// A protocol's messages as RDF: its engine, how its types expand and how its fields are read.
pub trait RdfProtocolTrait: Send + Sync + 'static {
    type MessageType: Display + Debug + Clone + PartialEq + Send + Sync;
    type Fields: Debug + Clone + Send + Sync;

    /// The engine whose loader knows this protocol's contexts.
    fn engine() -> &'static RdfEngine;
    /// The `@context` written on messages built on this side.
    fn context() -> Value;
    /// The compact `@type` term of `message`.
    fn compact_type(message: &Self::MessageType) -> String;
    /// The full IRI `message`'s type expands to.
    fn type_iri(message: &Self::MessageType) -> String;
    /// The message type a node declares, if it is one of this protocol's.
    fn message_type(node: &RdfNode<'_, '_>) -> Option<Self::MessageType>;
    /// Reads the fields off the message node; extraction is total, requiredness is a rule.
    fn extract(node: &RdfNode<'_, '_>) -> Outcome<Self::Fields>;

    /// The one node declaring a message type of this protocol.
    fn root_message<'d, 'a>(
        doc: &'d ExpandedDoc<'a>,
    ) -> Outcome<(Self::MessageType, RdfNode<'d, 'a>)> {
        let mut matching = doc
            .nodes()
            .filter_map(|node| Self::message_type(&node).map(|kind| (kind, node)));
        let found = matching.next().ok_or_else(|| {
            Errors::format(
                BadFormat::Received,
                "expanded body declares no message type",
                None,
            )
        })?;
        if matching.next().is_some() {
            return Err(Errors::format(
                BadFormat::Received,
                "expanded body declares more than one message node",
                None,
            ));
        }
        Ok(found)
    }
}

/// A message in compacted JSON-LD with what its RDF expansion says.
#[derive(Debug, Clone)]
pub struct RdfMessage<P: RdfProtocolTrait> {
    pub json: Value,
    pub message: P::MessageType,
    pub fields: P::Fields,
    pub canonical_n_quads: String,
    pub canonical_hash: [u8; 32],
}

impl<P: RdfProtocolTrait> RdfMessage<P> {
    /// Builds a message of type `message` from its members, then reads it back through RDF.
    pub async fn build(message: &P::MessageType, members: Map<String, Value>) -> Outcome<Self> {
        let mut json = json!({
            "@context": P::context(),
            "@type": P::compact_type(message),
        });
        if let Value::Object(object) = &mut json {
            object.extend(members.into_iter().filter(|(_, v)| !v.is_null()));
        }
        Self::read(json).await
    }

    /// Expands a message (ours or a peer's) and extracts its type and fields.
    pub async fn read(json: Value) -> Outcome<Self> {
        let expansion = P::engine().expand(&json).await?;
        let canonical_hash = expansion.canonical_hash();
        let doc = ExpandedDoc::new(&expansion.expanded).ok_or_else(|| {
            Errors::format(
                BadFormat::Received,
                "expanded JSON-LD is not an array of node objects",
                None,
            )
        })?;
        let (message, node) = P::root_message(&doc)?;
        let fields = P::extract(&node)?;
        Ok(Self {
            json,
            message,
            fields,
            canonical_n_quads: expansion.canonical_n_quads,
            canonical_hash,
        })
    }

    /// Like `read`, but the message must be of type `expected`.
    pub async fn read_as(expected: &P::MessageType, json: Value) -> Outcome<Self> {
        let read = Self::read(json).await?;
        if &read.message != expected {
            return Err(Errors::format(
                BadFormat::Received,
                format!(
                    "body is a {} where a {} is expected",
                    P::compact_type(&read.message),
                    P::compact_type(expected)
                ),
                None,
            ));
        }
        Ok(read)
    }
}
