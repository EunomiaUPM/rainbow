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

//! The DSP catalog routes behind PeerAuth, with mocked orchestrator, grants and mates.

use std::sync::Arc;

use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::Router;
use catalog_agent::protocols::dsp::http::protocol::DspRouter;
use catalog_agent::protocols::dsp::orchestrator::protocol::MockProtocolOrchestratorTrait;
use catalog_agent::protocols::dsp::orchestrator::MockOrchestratorTrait;
use common::dsp_common::PeerAuth;
use common::facades::grants_facade::{MockGrantsFacadeTrait, VerifiedPeer};
use common::facades::mates_facade::MockMatesFacadeTrait;
use ymir::types::oauth::RolePath;
use ymir::types::participants::Visibility;
use serde_json::Value;
use tower::ServiceExt;
use ymir::errors::Errors;

const DATASET: &str = "urn:uuid:9f8c7a2e-0000-0000-0000-000000000001";

/// A router whose peer is always verified and whose datasets are all unknown.
fn router() -> Router {
    let mut protocol = MockProtocolOrchestratorTrait::new();
    protocol
        .expect_on_dataset_request()
        .returning(|_, _| Err(Errors::missing_resource(DATASET, "dataset not found", None)));
    let protocol = Arc::new(protocol);
    let mut orchestrator = MockOrchestratorTrait::new();
    orchestrator
        .expect_get_protocol_service()
        .returning(move || protocol.clone());
    let mut grants = MockGrantsFacadeTrait::new();
    grants.expect_verify_token().returning(|_| {
        Ok(VerifiedPeer {
            participant_id: "did:example:consumer".into(),
            role: RolePath::root(),
            visibility: Visibility::Public,
        })
    });
    let mut mates = MockMatesFacadeTrait::new();
    mates
        .expect_get_mate_by_id()
        .returning(|_, id| Err(Errors::missing_resource(id, "not a mate", None)));
    let auth = PeerAuth::new(Arc::new(grants), Arc::new(mates));
    DspRouter::new(Arc::new(orchestrator), auth).router()
}

async fn get(path: &str) -> (StatusCode, Value) {
    let request = Request::builder()
        .uri(path)
        .header("authorization", "Bearer tok")
        .body(Body::empty())
        .unwrap();
    let response = router().oneshot(request).await.unwrap();
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), 64 * 1024)
        .await
        .unwrap();
    (status, serde_json::from_slice(&body).unwrap())
}

/// An unknown dataset is a 404 with a `CatalogError` body (DSP 6.3.3).
#[tokio::test]
async fn unknown_dataset_is_a_404_catalog_error() {
    let (status, body) = get(&format!("/datasets/{DATASET}")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["@type"], "CatalogError");
    assert_eq!(body["code"], "404");
    assert!(body["reason"].as_array().is_some_and(|r| !r.is_empty()));
}

/// A dataset id that is not a URN is a 400 `CatalogError`.
#[tokio::test]
async fn malformed_dataset_id_is_a_400_catalog_error() {
    let (status, body) = get("/datasets/not-a-urn").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["@type"], "CatalogError");
}
