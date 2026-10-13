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

//! The DSP negotiation routes end to end through the flow runner: real adapter, rules, loader,
//! handlers and persistence over an in-memory store, with the catalog and auth mocked.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::Request;
use axum::response::Response;
use axum::Router;
use chrono::Utc;
use common::facades::catalog::{CatalogOffer, MockCatalogFacadeTrait};
use common::facades::grants_facade::{MockGrantsFacadeTrait, VerifiedPeer};
use common::facades::mates_facade::MockMatesFacadeTrait;
use common::oauth::{RolePath, Visibility};
use http::StatusCode;
use negotiation_agent::data::repo::negotiation_process::MockNegotiationProcessRepoTrait;
use negotiation_agent::data::repo::negotiation_process_identifier::MockNegotiationIdentifierRepoTrait;
use negotiation_agent::data::repo::offer::MockOfferRepoTrait;
use negotiation_agent::data::sea_orm::orm::negotiation_process::Model as ProcessModel;
use negotiation_agent::protocols::dsp::http::dsp::DspRouter;
use negotiation_agent::protocols::dsp::setup::context::DspContext;
use negotiation_agent::services::agreement::views::AgreementView;
use negotiation_agent::services::agreement::MockAgreementServiceTrait;
use negotiation_agent::services::negotiation_message::views::NegotiationMessageView;
use negotiation_agent::services::negotiation_message::MockNegotiationMessageServiceTrait;
use negotiation_agent::services::negotiation_process::views::NegotiationProcessView;
use negotiation_agent::services::negotiation_process::MockNegotiationProcessServiceTrait;
use negotiation_agent::services::offer::views::OfferView;
use negotiation_agent::services::offer::MockOfferServiceTrait;
use serde_json::{json, Value};
use tower::ServiceExt;
use ymir::errors::Errors;

use crate::support::context::TestContext;
use crate::support::fixtures::{agreement_row, message_row, offer_row, process_row};

const PEER: &str = "did:web:consumer";
const OFFER: &str = "urn:offer:published";
const DATASET: &str = "urn:dataset:1";
const CONSUMER_PID: &str = "8e290e24-def3-4e24-8a6c-24a1a5c1e6af";

/// A stored process and its identifiers.
#[derive(Clone)]
struct Row {
    model: ProcessModel,
    ids: HashMap<String, String>,
}

/// What the fake services write and the fake repos read back.
#[derive(Clone, Default)]
struct Store {
    processes: Arc<Mutex<Vec<Row>>>,
    agreements: Arc<Mutex<Vec<String>>>,
}

impl Store {
    /// A process we play as `role` in `state` with `peer`, under both pids.
    fn seed(&self, role: &str, state: &str, peer: &str) -> (String, String) {
        let n = self.processes.lock().unwrap().len() as u32 + 1;
        let (consumer, provider) = (format!("urn:uuid:c{n}"), format!("urn:uuid:p{n}"));
        let mut model = process_row(n, role, state);
        model.associated_agent_peer = peer.into();
        let ids = HashMap::from([
            ("consumerPid".to_string(), consumer.clone()),
            ("providerPid".to_string(), provider.clone()),
        ]);
        self.processes.lock().unwrap().push(Row { model, ids });
        (consumer, provider)
    }

    fn find(&self, pid: &str) -> Option<Row> {
        self.processes
            .lock()
            .unwrap()
            .iter()
            .find(|r| r.ids.values().any(|v| v == pid))
            .cloned()
    }

    fn state_of(&self, pid: &str) -> String {
        self.find(pid).unwrap().model.state
    }

    fn view(row: &Row) -> NegotiationProcessView {
        NegotiationProcessView::assemble(row.model.clone(), row.ids.clone(), vec![], vec![], None)
    }

    fn repos(
        &self,
    ) -> (
        MockNegotiationProcessRepoTrait,
        MockNegotiationIdentifierRepoTrait,
    ) {
        let mut processes = MockNegotiationProcessRepoTrait::new();
        let s = self.clone();
        processes
            .expect_get_negotiation_process_by_key_value()
            .returning(move |_, pid| Ok(s.find(pid).map(|r| r.model)));
        let mut identifiers = MockNegotiationIdentifierRepoTrait::new();
        let s = self.clone();
        identifiers
            .expect_get_identifiers_by_process_id()
            .returning(move |id| {
                let all = s.processes.lock().unwrap();
                let row = all.iter().find(|r| r.model.id == id.to_string()).unwrap();
                Ok(row
                    .ids
                    .iter()
                    .map(|(k, v)| crate::support::fixtures::identifier_row(1, k, v))
                    .collect())
            });
        (processes, identifiers)
    }

    fn process_service(&self) -> MockNegotiationProcessServiceTrait {
        let mut svc = MockNegotiationProcessServiceTrait::new();
        let s = self.clone();
        svc.expect_create().returning(move |_, cmd| {
            let mut all = s.processes.lock().unwrap();
            let mut model = process_row(all.len() as u32 + 1, &cmd.role, &cmd.state);
            model.associated_agent_peer = cmd.associated_agent_peer.clone();
            model.callback_address = cmd.callback_address.clone();
            let row = Row {
                model,
                ids: cmd.identifiers.clone().unwrap_or_default(),
            };
            all.push(row.clone());
            Ok(Self::view(&row))
        });
        let s = self.clone();
        svc.expect_edit().returning(move |_, id, edit| {
            let mut all = s.processes.lock().unwrap();
            let row = all
                .iter_mut()
                .find(|r| r.model.id == id.to_string())
                .unwrap();
            if let Some(state) = &edit.state {
                row.model.state = state.clone();
            }
            row.model.updated_at = Some(Utc::now().into());
            Ok(Self::view(row))
        });
        svc
    }

    fn agreement_service(&self) -> MockAgreementServiceTrait {
        let mut svc = MockAgreementServiceTrait::new();
        let s = self.clone();
        svc.expect_create().returning(move |_, cmd| {
            s.agreements
                .lock()
                .unwrap()
                .push(cmd.id.as_ref().map(ToString::to_string).unwrap_or_default());
            Ok(AgreementView::assemble(agreement_row(1)))
        });
        svc
    }
}

fn peer(id: &str) -> VerifiedPeer {
    VerifiedPeer {
        participant_id: id.into(),
        role: RolePath::root(),
        visibility: Visibility::Public,
    }
}

/// The DSP routes over `store`, with `OFFER` published on `DATASET`; a token is `as_peer`, or
/// rejected when there is none.
fn router(store: &Store, as_peer: Option<&str>) -> Router {
    let mut catalog = MockCatalogFacadeTrait::new();
    catalog.expect_get_offer().returning(|id| {
        if id.to_string() == OFFER {
            Ok(CatalogOffer {
                id: OFFER.into(),
                entity: DATASET.into(),
                entity_type: "Dataset".into(),
                odrl_offer: json!({}),
            })
        } else {
            Err(Errors::missing_resource(
                id.to_string(),
                "no such offer",
                None,
            ))
        }
    });
    let mut grants = MockGrantsFacadeTrait::new();
    match as_peer.map(str::to_string) {
        Some(id) => {
            grants
                .expect_verify_token()
                .returning(move |_| Ok(peer(&id)));
        }
        None => {
            grants
                .expect_verify_token()
                .returning(|_| Err(Errors::unauthorized("nope", None)));
        }
    }
    let mut mates = MockMatesFacadeTrait::new();
    mates
        .expect_get_mate_by_id()
        .returning(|_, id| Err(Errors::missing_resource(id, "not a mate", None)));
    let mut messages = MockNegotiationMessageServiceTrait::new();
    messages.expect_create().returning(|_, _| {
        Ok(NegotiationMessageView::assemble(
            message_row(200),
            None,
            None,
        ))
    });
    let mut offers = MockOfferServiceTrait::new();
    offers
        .expect_create()
        .returning(|_, _| Ok(OfferView::assemble(offer_row(1))));
    let mut offer_repo = MockOfferRepoTrait::new();
    offer_repo
        .expect_get_offers_by_negotiation_process()
        .returning(|_, _| Ok(vec![offer_row(1)]));
    let (processes, identifiers) = store.repos();
    let app = TestContext::new()
        .process_repo(processes)
        .identifier_repo(identifiers)
        .offer_repo(offer_repo)
        .processes(store.process_service())
        .messages(messages)
        .offers(offers)
        .agreements(store.agreement_service())
        .catalog(catalog)
        .grants(grants)
        .mates(mates)
        .build();
    let (ctx, _) = DspContext::build(app);
    DspRouter::new(ctx).router()
}

fn post(uri: &str, body: Value) -> Request {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", "Bearer tok")
        .body(axum::body::Body::from(body.to_string()))
        .unwrap()
}

async fn send(router: Router, request: Request) -> (StatusCode, Value) {
    let response: Response = router.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

fn request(offer: &str, target: &str) -> Value {
    json!({
        "@context": ["https://w3id.org/dspace/2025/1/context.jsonld"],
        "@type": "ContractRequestMessage",
        "consumerPid": CONSUMER_PID,
        "offer": {"@id": offer, "@type": "Offer", "target": target, "permission": [{"action": "use"}]},
        "callbackAddress": "http://consumer/dsp/current"
    })
}

fn pids_message(kind: &str, consumer: &str, provider: &str) -> Value {
    json!({
        "@context": "https://w3id.org/dspace/2025/1/context.jsonld",
        "@type": kind,
        "consumerPid": consumer,
        "providerPid": provider
    })
}

/// DSP 8.2.2: a request for a published offer opens the negotiation, 201 with its ACK.
#[tokio::test]
async fn a_request_for_a_published_offer_opens_a_negotiation() {
    let store = Store::default();
    let (status, ack) = send(
        router(&store, Some(PEER)),
        post("/request", request(OFFER, DATASET)),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{ack}");
    assert_eq!(ack["@type"], "ContractNegotiation");
    assert_eq!(ack["state"], "REQUESTED");
    assert_eq!(ack["consumerPid"], CONSUMER_PID);
    let provider_pid = ack["providerPid"].as_str().unwrap();
    assert!(provider_pid.starts_with("urn:uuid:"), "{ack}");
    let row = store.find(provider_pid).unwrap();
    assert_eq!(row.model.role, "Provider");
    assert_eq!(row.model.associated_agent_peer, PEER);
}

/// A repeated request gets the first answer and opens nothing more.
#[tokio::test]
async fn a_repeated_request_replays_the_first_ack() {
    let store = Store::default();
    let app = router(&store, Some(PEER));
    let first = send(app.clone(), post("/request", request(OFFER, DATASET))).await;
    let second = send(app, post("/request", request(OFFER, DATASET))).await;
    assert_eq!(first, second);
    assert_eq!(store.processes.lock().unwrap().len(), 1);
}

/// H6: an offer the catalog does not publish, or on another target, is refused.
#[tokio::test]
async fn a_request_for_an_unpublished_offer_is_refused() {
    let store = Store::default();
    for body in [
        request("urn:offer:unknown", DATASET),
        request(OFFER, "urn:dataset:other"),
    ] {
        let (status, error) = send(router(&store, Some(PEER)), post("/request", body)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{error}");
        assert_eq!(error["@type"], "ContractNegotiationError");
        assert_eq!(error["consumerPid"], CONSUMER_PID);
    }
    assert!(store.processes.lock().unwrap().is_empty());
}

/// P1: a peer whose token is refused gets the 404 of a missing negotiation.
#[tokio::test]
async fn an_unknown_peer_gets_404() {
    let (status, _) = send(
        router(&Store::default(), None),
        post("/request", request(OFFER, DATASET)),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

/// A message for an unknown negotiation, or someone else's, is 404.
#[tokio::test]
async fn unknown_and_foreign_negotiations_are_404() {
    let store = Store::default();
    let (consumer, provider) = store.seed("Provider", "AGREED", "did:web:someone-else");
    let verification = pids_message("ContractAgreementVerificationMessage", &consumer, &provider);
    let (status, _) = send(
        router(&store, Some(PEER)),
        post(&format!("/{provider}/agreement/verification"), verification),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let unknown = pids_message(
        "ContractAgreementVerificationMessage",
        "urn:uuid:x",
        "urn:uuid:y",
    );
    let (status, _) = send(
        router(&store, Some(PEER)),
        post("/urn:uuid:y/agreement/verification", unknown),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(store.state_of(&provider), "AGREED");
}

/// DSP 8.2.3: a consumer counters our offer with a request.
#[tokio::test]
async fn a_counter_request_moves_an_offered_negotiation_back() {
    let store = Store::default();
    let (consumer, provider) = store.seed("Provider", "OFFERED", PEER);
    let mut body = request(OFFER, DATASET);
    body["consumerPid"] = json!(consumer);
    body["providerPid"] = json!(provider);
    let (status, ack) = send(
        router(&store, Some(PEER)),
        post(&format!("/{provider}/request"), body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{ack}");
    assert_eq!(ack["state"], "REQUESTED");
    assert_eq!(store.state_of(&provider), "REQUESTED");
}

/// Out of order messages are refused with the state they broke, and move nothing.
#[tokio::test]
async fn a_verification_before_the_agreement_is_refused() {
    let store = Store::default();
    let (consumer, provider) = store.seed("Provider", "REQUESTED", PEER);
    let (status, error) = send(
        router(&store, Some(PEER)),
        post(
            &format!("/{provider}/agreement/verification"),
            pids_message("ContractAgreementVerificationMessage", &consumer, &provider),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{error}");
    assert_eq!(store.state_of(&provider), "REQUESTED");
}

/// A provider does not take offers, whatever the state allows.
#[tokio::test]
async fn a_provider_refuses_an_offer() {
    let store = Store::default();
    let (consumer, provider) = store.seed("Provider", "REQUESTED", PEER);
    let mut body = pids_message("ContractOfferMessage", &consumer, &provider);
    body["offer"] = json!({"@id": OFFER, "@type": "Offer", "target": DATASET});
    let (status, error) = send(
        router(&store, Some(PEER)),
        post(&format!("/{provider}/offers"), body),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{error}");
}

/// The route's pid must be ours.
#[tokio::test]
async fn the_route_must_carry_our_pid() {
    let store = Store::default();
    let (consumer, provider) = store.seed("Provider", "AGREED", PEER);
    let (status, _) = send(
        router(&store, Some(PEER)),
        post(
            &format!("/{consumer}/agreement/verification"),
            pids_message("ContractAgreementVerificationMessage", &consumer, &provider),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

/// DSP 8.3.3: an offer opens a negotiation we consume, 201 with our new consumerPid.
#[tokio::test]
async fn an_initial_offer_opens_a_negotiation_we_consume() {
    let store = Store::default();
    let body = json!({
        "@context": "https://w3id.org/dspace/2025/1/context.jsonld",
        "@type": "ContractOfferMessage",
        "providerPid": "urn:uuid:their-pid",
        "offer": {"@id": "urn:offer:any", "@type": "Offer", "target": DATASET},
        "callbackAddress": "http://provider/dsp/current"
    });
    let (status, ack) = send(
        router(&store, Some("did:web:provider")),
        post("/offers", body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{ack}");
    assert_eq!(ack["state"], "OFFERED");
    assert_eq!(ack["providerPid"], "urn:uuid:their-pid");
    assert_eq!(
        store.find("urn:uuid:their-pid").unwrap().model.role,
        "Consumer"
    );
}

/// DSP 8.2.4: the events route takes ACCEPTED on an offered negotiation we provide.
#[tokio::test]
async fn an_accepted_event_moves_an_offered_negotiation() {
    let store = Store::default();
    let (consumer, provider) = store.seed("Provider", "OFFERED", PEER);
    let mut body = pids_message("ContractNegotiationEventMessage", &consumer, &provider);
    body["eventType"] = json!("ACCEPTED");
    let (status, ack) = send(
        router(&store, Some(PEER)),
        post(&format!("/{provider}/events"), body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{ack}");
    assert_eq!(store.state_of(&provider), "ACCEPTED");
}

/// A consumer does not take ACCEPTED: it is the one that sends it.
#[tokio::test]
async fn a_consumer_refuses_an_accepted_event() {
    let store = Store::default();
    let (consumer, provider) = store.seed("Consumer", "OFFERED", "did:web:provider");
    let mut body = pids_message("ContractNegotiationEventMessage", &consumer, &provider);
    body["eventType"] = json!("ACCEPTED");
    let (status, _) = send(
        router(&store, Some("did:web:provider")),
        post(&format!("/{consumer}/events"), body),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

/// DSP 8.3.5: an agreement moves a negotiation we consume to AGREED and is stored; a UUID id,
/// as the TCK sends, is kept under `urn:agreement:`.
#[tokio::test]
async fn an_agreement_is_stored_on_the_consumer() {
    let store = Store::default();
    let (consumer, provider) = store.seed("Consumer", "ACCEPTED", "did:web:provider");
    let mut body = pids_message("ContractAgreementMessage", &consumer, &provider);
    body["agreement"] = json!({
        "@id": "6c89f109-3c06-4586-b4d5-0b2dc924c967",
        "@type": "Agreement",
        "target": DATASET,
        "assigner": "did:web:provider",
        "assignee": "did:web:consumer",
        "permission": [{"action": "use"}]
    });
    let (status, ack) = send(
        router(&store, Some("did:web:provider")),
        post(&format!("/{consumer}/agreement"), body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{ack}");
    assert_eq!(store.state_of(&consumer), "AGREED");
    assert_eq!(
        *store.agreements.lock().unwrap(),
        ["urn:agreement:6c89f109-3c06-4586-b4d5-0b2dc924c967"]
    );
}

/// DSP 8.2.1: the negotiation as the peer knows it; someone else's is 404.
#[tokio::test]
async fn get_answers_the_ack_to_its_peer_only() {
    let store = Store::default();
    let (_, provider) = store.seed("Provider", "VERIFIED", PEER);
    let get = || {
        Request::builder()
            .uri(format!("/{provider}"))
            .header("authorization", "Bearer tok")
            .body(axum::body::Body::empty())
            .unwrap()
    };
    let (status, ack) = send(router(&store, Some(PEER)), get()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(ack["state"], "VERIFIED");
    let (status, _) = send(router(&store, Some("did:web:other")), get()).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
