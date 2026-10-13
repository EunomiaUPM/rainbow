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

//! The consumer gets the participant credential from the authority and presents it to the
//! provider over GNAP and OID4VP; the token it gets back then authenticates it at the provider.

use std::time::Duration;

use chrono::Utc;
use serde_json::{json, Value};

use crate::agent::Agent;

const VC_TYPE: &str = "DataSpaceParticipant_jwt_vc_json";

/// Grants of `path` created since `since` for `participant`.
async fn grants_since(agent: &Agent, path: &str, since: &str, participant: &str) -> Vec<Value> {
    let page = agent.get(&format!("{path}?createdAfter={since}")).await;
    page["items"]
        .as_array()
        .expect("paginated grants")
        .iter()
        .filter(|g| g["participant_id"] == participant)
        .cloned()
        .collect()
}

/// Token the consumer holds for the provider, once it differs from `previous`.
async fn new_provider_token(
    consumer: &Agent,
    provider_did: &str,
    previous: Option<String>,
) -> String {
    for _ in 0..60 {
        let requests = consumer.get("/api/v1/peer-connection/request/all").await;
        if let Some(token) = requests["items"]
            .as_array()
            .and_then(|items| {
                items.iter().find(|r| {
                    r["participant_id"] == provider_did && r["status"] == "Approved"
                })
            })
            .and_then(|r| r["final_token"].as_str())
            .filter(|t| Some(t.to_string()) != previous)
        {
            return token.to_string();
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    panic!("the consumer never got a new token from the provider");
}

/// Token stored for the provider before this run, if the consumer already onboarded.
async fn current_provider_token(consumer: &Agent, provider_did: &str) -> Option<String> {
    let requests = consumer.get("/api/v1/peer-connection/request/all").await;
    requests["items"]
        .as_array()?
        .iter()
        .find(|r| r["participant_id"] == provider_did && r["status"] == "Approved")
        .and_then(|r| r["final_token"].as_str().map(str::to_string))
}

/// Steps 1 to 4 of the script, checking what each one leaves behind.
#[tokio::test]
#[ignore = "needs the dev stack: authority, consumer and provider with their wallets"]
async fn consumer_onboards_with_the_provider() {
    let authority = Agent::login("AUTHORITY_URL", "http://127.0.0.1:1500").await;
    let consumer = Agent::login("CONSUMER_URL", "http://127.0.0.1:1100").await;
    let provider = Agent::login("PROVIDER_URL", "http://127.0.0.1:1200").await;
    let since = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    for agent in [&authority, &consumer, &provider] {
        agent.post("/api/v1/wallet/link", json!({})).await;
    }
    let authority_did = authority.did().await;
    let consumer_did = consumer.did().await;
    let provider_did = provider.did().await;

    // The credential is issued and redeemed within the request, since it is automatic.
    consumer
        .post(
            "/api/v1/vc-request/beg",
            json!({
                "id": authority_did,
                "nick": "authority",
                "url": format!("{}/api/v1/gate/access", authority.url),
                "vc_type": VC_TYPE,
                "method": "cert",
                "auto": true
            }),
        )
        .await;
    let requests = grants_since(&consumer, "/api/v1/vc-request/all", &since, &authority_did).await;
    assert_eq!(requests.len(), 1, "one credential request: {requests:?}");
    assert_eq!(requests[0]["status"], "Finalized");

    // The presentation and the GNAP continuation run through callbacks, after `connect` answers.
    let previous = current_provider_token(&consumer, &provider_did).await;
    consumer
        .post(
            "/api/v1/peer-connection/connect",
            json!({
                "id": provider_did,
                "nick": "provider",
                "url": format!("{}/api/v1/gate/access", provider.url),
                "actions": ["talk"],
                "visibility": "Public",
                "auto": true
            }),
        )
        .await;
    let token = new_provider_token(&consumer, &provider_did, previous).await;

    let mate = provider
        .post_json("/api/v1/gate/token/verify", json!({ "token": token }))
        .await;
    assert_eq!(mate["participant_id"], consumer_did);

    // Provider also requests credential from authority.
    provider
        .post(
            "/api/v1/vc-request/beg",
            json!({
                "id": authority_did,
                "nick": "authority",
                "url": format!("{}/api/v1/gate/access", authority.url),
                "vc_type": VC_TYPE,
                "method": "cert",
                "auto": true
            }),
        )
        .await;

    // Provider authenticates with consumer.
    let previous_c = current_provider_token(&provider, &consumer_did).await;
    provider
        .post(
            "/api/v1/peer-connection/connect",
            json!({
                "id": consumer_did,
                "nick": "consumer",
                "url": format!("{}/api/v1/gate/access", consumer.url),
                "actions": ["talk"],
                "visibility": "Public",
                "auto": true
            }),
        )
        .await;
    let token_c = new_provider_token(&provider, &consumer_did, previous_c).await;

    let mate_c = consumer
        .post_json("/api/v1/gate/token/verify", json!({ "token": token_c }))
        .await;
    assert_eq!(mate_c["participant_id"], provider_did);
}
