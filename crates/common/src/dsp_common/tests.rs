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

//! The DSP namespace normalizer, alone and as axum middleware, and the DspRules checks.

use axum::{routing::post, Router};
use serde_json::json;
use tower::ServiceExt;

use crate::dsp_common::normalizer::{
    dsp_namespace_normalizer, normalise_value, WireBody, MAX_BODY_BYTES,
};
use crate::dsp_common::{DspErrorBody, DspRules};
use crate::validation::codes;

/// A literal's `xsd:` datatype survives normalisation: stripping it would leave a relative
/// IRI that expands to `x-string:///dateTime`.
#[test]
fn keeps_xsd_datatypes_on_literals() {
    let out = normalise_value(json!({
        "odrl:constraint": [{
            "odrl:leftOperand": "odrl:dateTime",
            "odrl:rightOperand": {
                "@type": "xsd:dateTime",
                "@value": "2026-12-31T23:59:59Z"
            }
        }]
    }));
    assert_eq!(
        out["constraint"][0]["rightOperand"]["@type"],
        json!("xsd:dateTime")
    );
    // The DSP/ODRL prefixes are still stripped from keys as before.
    assert_eq!(out["constraint"][0]["leftOperand"], json!("odrl:dateTime"));
}

/// DSP prefixes are stripped from keys and from `@type`.
#[test]
fn still_strips_dsp_prefixes_from_keys_and_types() {
    let out = normalise_value(json!({
        "@type": "dspace:TransferRequestMessage",
        "dspace:consumerPid": "urn:uuid:cc",
        "dct:format": "HttpData-PULL"
    }));
    assert_eq!(out["@type"], json!("TransferRequestMessage"));
    assert_eq!(out["consumerPid"], json!("urn:uuid:cc"));
    assert_eq!(out["format"], json!("HttpData-PULL"));
}

/// The middleware keeps the bytes the peer sent, so signatures and audits can still use
/// them after the body is rewritten.
#[tokio::test]
async fn stashes_the_pre_rewrite_body() {
    const SENT: &str =
        r#"{"@type":"dspace:TransferRequestMessage","dspace:consumerPid":"urn:uuid:cc"}"#;

    async fn handler(request: axum::extract::Request) -> String {
        let wire = request
            .extensions()
            .get::<WireBody>()
            .expect("normalizer must stash the wire body")
            .0
            .clone();
        let seen = axum::body::to_bytes(request.into_body(), MAX_BODY_BYTES)
            .await
            .unwrap();
        format!(
            "{}|{}",
            String::from_utf8(wire.to_vec()).unwrap(),
            String::from_utf8(seen.to_vec()).unwrap()
        )
    }

    let app = Router::new()
        .route("/request", post(handler))
        .layer(axum::middleware::from_fn(dsp_namespace_normalizer));

    let response = app
        .oneshot(
            axum::extract::Request::builder()
                .method("POST")
                .uri("/request")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(SENT))
                .unwrap(),
        )
        .await
        .unwrap();
    let out = axum::body::to_bytes(response.into_body(), MAX_BODY_BYTES)
        .await
        .unwrap();
    let out = String::from_utf8(out.to_vec()).unwrap();
    let (wire, seen) = out.split_once('|').unwrap();

    assert_eq!(wire, SENT, "wire body must be untouched");
    assert_ne!(seen, SENT, "the handler's body really was rewritten");
    assert!(
        seen.contains(r#""consumerPid""#) && !seen.contains("dspace:consumerPid"),
        "got: {seen}"
    );
}

/// A pid must be a URN: a malformed one is MALFORMED and a missing optional one MISSING.
#[test]
fn is_urn_rejects_malformed_and_missing_pids() {
    assert!(DspRules::is_urn("urn:uuid:1234-5678", "pid").is_ok());
    assert!(DspRules::is_urn("urn:custom:item", "pid").is_ok());

    let malformed = DspRules::is_urn("not-a-urn", "pid").unwrap_err();
    assert_eq!(malformed.code(), Some(codes::MALFORMED));

    assert!(DspRules::is_urn_opt(Some("urn:uuid:abc"), "pid").is_ok());
    let missing = DspRules::is_urn_opt(None, "pid").unwrap_err();
    assert_eq!(missing.code(), Some(codes::MISSING));
}

/// The path pid and the body pid must match.
#[test]
fn correlate_pids_rejects_a_mismatch() {
    assert!(DspRules::correlate_pids("urn:uuid:a", "urn:uuid:a", "pid").is_ok());

    let mismatch = DspRules::correlate_pids("urn:uuid:a", "urn:uuid:b", "pid").unwrap_err();
    assert_eq!(mismatch.code(), Some(codes::NOT_ALLOWED));
}

/// A stored agreement keeps its parties and target with partial content, and its rules when
/// the content has well-formed ones.
#[test]
fn stored_agreement_reads_rules_leniently() {
    let id: urn::Urn = "urn:agreement:1".parse().unwrap();
    let target: urn::Urn = "urn:dataset:1".parse().unwrap();
    let partial = crate::dsp_common::odrl::OdrlAgreement::from_stored(
        id.clone(),
        target.clone(),
        "did:web:provider".into(),
        "did:web:consumer".into(),
        &json!({"uid": "urn:agreement:1", "permission": "not a list"}),
    );
    assert_eq!(partial.assigner, "did:web:provider");
    assert_eq!(partial.target, target);
    assert!(partial.permission.is_none());

    let ruled = crate::dsp_common::odrl::OdrlAgreement::from_stored(
        id,
        target,
        "did:web:provider".into(),
        "did:web:consumer".into(),
        &json!({"permission": [{"action": "use"}]}),
    );
    assert_eq!(ruled.permission.map(|p| p.len()), Some(1));
}

/// A message's pids must be the stored ones on both sides, absent ones included.
#[test]
fn process_pids_must_match_the_stored_ones() {
    let stored = (Some("urn:uuid:c"), Some("urn:uuid:p"));
    assert!(DspRules::correlate_process_pids(stored, stored, "pids").is_ok());

    let other = DspRules::correlate_process_pids(stored, (Some("urn:uuid:c"), None), "pids")
        .unwrap_err();
    assert_eq!(other.code(), Some(codes::NOT_ALLOWED));
}

/// An error body without reasons serializes without the member and reads back the same.
#[test]
fn error_body_round_trips_without_reasons() {
    let body = DspErrorBody {
        code: "404".into(),
        reason: vec![],
    };
    let json = serde_json::to_value(&body).unwrap();
    assert_eq!(json, json!({"code": "404"}));
    assert_eq!(serde_json::from_value::<DspErrorBody>(json).unwrap(), body);
}

/// The DSP context and the expected type are found whether given as a string or an array.
#[test]
fn context_and_type_accept_string_or_array_forms() {
    let doc_str = json!({
        "@context": "https://w3id.org/dspace/2025/1/context.json",
        "@type": "dspace:TransferRequestMessage"
    });
    assert!(DspRules::valid_dsp_context(&doc_str, "@context").is_ok());
    assert!(DspRules::expected_type(&doc_str, "TransferRequestMessage", "@type").is_ok());

    let doc_arr = json!({
        "@context": ["https://w3id.org/dspace/2025/1/context.json", "https://example.org/extra"],
        "@type": ["dspace:TransferRequestMessage", "CustomType"]
    });
    assert!(DspRules::valid_dsp_context(&doc_arr, "@context").is_ok());
    assert!(DspRules::expected_type(&doc_arr, "TransferRequestMessage", "@type").is_ok());

    let bad_ctx = json!({ "@context": "https://schema.org" });
    assert!(DspRules::valid_dsp_context(&bad_ctx, "@context").is_err());

    let bad_type = json!({ "@type": "TransferStartMessage" });
    assert!(DspRules::expected_type(&bad_type, "TransferRequestMessage", "@type").is_err());
}

/// A data address without properties leaves the member out, and reads back without it.
#[test]
fn data_address_omits_empty_endpoint_properties() {
    let address = crate::dsp_common::data_address::DataAddress {
        _type: "DataAddress".into(),
        endpoint_type: "HTTP".into(),
        endpoint: Some("http://provider/data".into()),
        endpoint_properties: vec![],
    };
    let json = serde_json::to_value(&address).unwrap();
    assert!(json.get("endpointProperties").is_none(), "{json}");
    let back: crate::dsp_common::data_address::DataAddress = serde_json::from_value(json).unwrap();
    assert_eq!(back, address);
}

/// A DSP error body carries one reason per violation, in order.
#[test]
fn error_body_lists_every_violation() {
    let mut vs =
        crate::validation::violation("consumerPid", codes::MALFORMED, "must be a valid URN");
    vs.extend(crate::validation::violation(
        "offer",
        codes::MISSING,
        "field is required",
    ));
    let body = DspErrorBody::from_violations("400", &vs);
    assert_eq!(body.code, "400");
    assert_eq!(
        body.reason,
        vec![
            "consumerPid: must be a valid URN",
            "offer: field is required"
        ]
    );
}
