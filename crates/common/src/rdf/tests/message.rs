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

//! RdfMessage over a toy protocol on the DSP engine: build, read, read_as and canonical hash.

use serde_json::{json, Map, Value};
use ymir::errors::Outcome;

use crate::dsp_common::rdf::{DspProfile, DSP_CONTEXT_URL};
use crate::rdf::{RdfEngine, RdfMessage, RdfNode, RdfProtocolTrait};

const DSPACE: &str = "https://w3id.org/dspace/2025/1/";

/// Transfer start and termination, reading only the consumer pid.
#[derive(Debug, Clone)]
struct ToyTransfer;

impl RdfProtocolTrait for ToyTransfer {
    type MessageType = String;
    type Fields = Option<String>;

    fn engine() -> &'static RdfEngine {
        DspProfile::shared()
    }
    fn context() -> Value {
        json!(DSP_CONTEXT_URL)
    }
    fn compact_type(message: &String) -> String {
        message.clone()
    }
    fn type_iri(message: &String) -> String {
        format!("{DSPACE}{message}")
    }
    fn message_type(node: &RdfNode<'_, '_>) -> Option<String> {
        node.types()
            .filter_map(|iri| iri.strip_prefix(DSPACE))
            .find(|t| *t == "TransferStartMessage" || *t == "TransferTerminationMessage")
            .map(str::to_string)
    }
    fn extract(node: &RdfNode<'_, '_>) -> Outcome<Option<String>> {
        Ok(node
            .iri_or_literal(&format!("{DSPACE}consumerPid"))
            .map(str::to_string))
    }
}

fn start() -> Value {
    json!({
        "@context": DSP_CONTEXT_URL,
        "@type": "TransferStartMessage",
        "consumerPid": "urn:uuid:c",
        "providerPid": "urn:uuid:p"
    })
}

/// Reading keeps the JSON, finds the type, extracts the fields and hashes the canonical form.
#[tokio::test]
async fn read_extracts_type_fields_and_hash() {
    let message = RdfMessage::<ToyTransfer>::read(start()).await.unwrap();
    assert_eq!(message.message, "TransferStartMessage");
    assert_eq!(message.fields.as_deref(), Some("urn:uuid:c"));
    assert_eq!(message.json, start());
    let expansion = DspProfile::shared().expand(&start()).await.unwrap();
    assert_eq!(message.canonical_hash, expansion.canonical_hash());
}

/// A built message carries the protocol's context and type and reads back like a received one.
#[tokio::test]
async fn build_reads_back_what_it_wrote() {
    let mut members = Map::new();
    members.insert("consumerPid".into(), json!("urn:uuid:c"));
    members.insert("providerPid".into(), json!("urn:uuid:p"));
    members.insert("code".into(), Value::Null);
    let built = RdfMessage::<ToyTransfer>::build(&"TransferStartMessage".to_string(), members)
        .await
        .unwrap();
    assert_eq!(built.json["@context"], DSP_CONTEXT_URL);
    assert!(built.json.get("code").is_none(), "nulls are left out");
    let received = RdfMessage::<ToyTransfer>::read(start()).await.unwrap();
    assert_eq!(built.canonical_hash, received.canonical_hash);
}

/// `read_as` refuses a message of another type, naming both.
#[tokio::test]
async fn read_as_refuses_another_type() {
    let err = RdfMessage::<ToyTransfer>::read_as(&"TransferTerminationMessage".into(), start())
        .await
        .unwrap_err();
    assert!(format!("{err:?}").contains("TransferTerminationMessage"));
}

/// A body declaring none of the protocol's types is rejected.
#[tokio::test]
async fn a_body_of_another_protocol_is_rejected() {
    let body = json!({"@context": DSP_CONTEXT_URL, "@type": "CatalogRequestMessage"});
    assert!(RdfMessage::<ToyTransfer>::read(body).await.is_err());
}
