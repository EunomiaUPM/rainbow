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

//! Outbound messages as RDF: built in JSON-LD, expanded and read back with the same extractor
//! the inbound side uses, and kept with their canonical form.

use common::dsp_common::context_field::ContextField;
use common::dsp_common::rdf::DspProfile;
use common::rdf::ExpandedDoc;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use ymir::errors::{BadFormat, Errors, Outcome};

use crate::entities::message_envelope::MessageEnvelope;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::protocol_fields::TransferProtocolFields;
use crate::protocols::dsp::entities::rdf_extractor_dsp::{DspTransfer, ExtractProtocolFields};

/// A DSP message in compacted JSON-LD with what its RDF expansion says.
#[derive(Debug, Clone)]
pub struct RdfDspMessage {
    pub json: Value,
    pub message: TransferDSPMessageType,
    pub fields: TransferProtocolFields,
    pub canonical_n_quads: String,
    pub canonical_hash: [u8; 32],
}

impl RdfDspMessage {
    /// Builds a message of type `message` from its members, then reads it back through RDF.
    pub async fn build(
        message: &TransferDSPMessageType,
        members: Map<String, Value>,
    ) -> Outcome<Self> {
        let mut json = json!({
            "@context": ContextField::default(),
            "@type": message.to_string(),
        });
        if let Value::Object(object) = &mut json {
            object.extend(members.into_iter().filter(|(_, v)| !v.is_null()));
        }
        Self::read(json).await
    }

    /// Expands a message (ours or a peer's) and extracts its type and DSP fields.
    pub async fn read(json: Value) -> Outcome<Self> {
        let expansion = DspProfile::shared().expand(&json).await?;
        let canonical_hash = Sha256::digest(expansion.canonical_n_quads.as_bytes()).into();
        let doc = ExpandedDoc::new(&expansion.expanded).ok_or_else(|| {
            Errors::format(
                BadFormat::Received,
                "expanded JSON-LD is not an array of node objects",
                None,
            )
        })?;
        let (message, node) = DspTransfer::root_message(&doc)?;
        let fields = DspTransfer::extract(&node)?;
        Ok(Self {
            json,
            message,
            fields,
            canonical_n_quads: expansion.canonical_n_quads,
            canonical_hash,
        })
    }

    /// How the message is kept in the process history.
    pub fn envelope(&self) -> MessageEnvelope {
        MessageEnvelope::new(
            self.json.clone(),
            Some((self.canonical_n_quads.clone(), self.canonical_hash)),
        )
    }
}
